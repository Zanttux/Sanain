//! Operating-system integration that the window library does not offer
//! (ADR 0003). Every `unsafe` block is allowed individually and carries a
//! `SAFETY:` comment.
//!
//! - macOS: files opened from Finder (Open With, double click, drag onto
//!   the Dock icon, `open -a Sanain file`) arrive as an `odoc` Apple
//!   event, not as program arguments. [`prepare_open_documents`] handles
//!   that event and [`take_opened_files`] hands the paths to the app.
//! - Windows: [`attach_parent_console`] makes CLI output visible in the
//!   invoking console without allocating a console window.

use std::path::PathBuf;
use std::sync::Mutex;

static OPENED: Mutex<Vec<PathBuf>> = Mutex::new(Vec::new());
static WAKE: Mutex<Option<Box<dyn Fn() + Send>>> = Mutex::new(None);

/// Attach to the parent's console for CLI commands that print and exit.
/// Returns true only when attachment succeeds; returns false on other
/// platforms, when there is no parent console, or when already attached.
/// Never allocates a console window. Do not call on the normal GUI path.
///
/// On Windows, attachment sets the standard handles for a process without
/// a console, so Rust's `println!`/`eprintln!` (which use `GetStdHandle`)
/// work normally. Output may appear after the shell prompt because the
/// shell does not wait for GUI-subsystem programs; this is not an error.
pub fn attach_parent_console() -> bool {
    #[cfg(windows)]
    {
        use windows_sys::Win32::System::Console::{ATTACH_PARENT_PROCESS, AttachConsole};

        // SAFETY: ATTACH_PARENT_PROCESS is a valid process selector. This
        // call takes no pointers and requires no owned resources; failure
        // (including an existing attachment) is reported by a zero result.
        #[allow(unsafe_code)]
        unsafe {
            AttachConsole(ATTACH_PARENT_PROCESS) != 0
        }
    }
    #[cfg(not(windows))]
    {
        false
    }
}

/// Files the system asked Sanain to open since the last call.
pub fn take_opened_files() -> Vec<PathBuf> {
    OPENED
        .lock()
        .map(|mut v| std::mem::take(&mut *v))
        .unwrap_or_default()
}

// Only the macOS handler (and tests) push files.
#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
fn push_opened(paths: Vec<PathBuf>) {
    if paths.is_empty() {
        return;
    }
    if let Ok(mut v) = OPENED.lock() {
        v.extend(paths);
    }
    if let Ok(w) = WAKE.lock()
        && let Some(wake) = w.as_ref()
    {
        wake();
    }
}

/// Prepare to receive "open documents" requests. Call once on the main
/// thread before the event loop starts, so a file opened together with
/// the launch is not missed. Does nothing on platforms that pass files
/// as program arguments.
pub fn prepare_open_documents() {
    #[cfg(target_os = "macos")]
    macos::prepare();
}

/// Called when files arrive (e.g. to request a repaint).
pub fn on_opened_files(wake: impl Fn() + Send + 'static) {
    if let Ok(mut w) = WAKE.lock() {
        *w = Some(Box::new(wake));
    }
}

/// Turn a `file://` URL string into a path (percent-decoded).
pub fn file_url_to_path(url: &str) -> Option<PathBuf> {
    let rest = url.strip_prefix("file://")?;
    let rest = rest.strip_prefix("localhost").unwrap_or(rest);
    let mut bytes = Vec::with_capacity(rest.len());
    let mut it = rest.bytes();
    while let Some(b) = it.next() {
        if b == b'%' {
            let h = it.next()?;
            let l = it.next()?;
            let hex = |c: u8| (c as char).to_digit(16);
            bytes.push((hex(h)? * 16 + hex(l)?) as u8);
        } else {
            bytes.push(b);
        }
    }
    let s = String::from_utf8(bytes).ok()?;
    let s = s.trim_end_matches('/');
    (!s.is_empty()).then(|| PathBuf::from(s))
}

#[cfg(target_os = "macos")]
mod macos {
    use objc2::rc::Retained;
    use objc2::runtime::NSObject;
    use objc2::{AllocAnyThread, MainThreadMarker, define_class, msg_send, sel};
    use objc2_foundation::{
        NSAppleEventDescriptor, NSAppleEventManager, NSNotification, NSNotificationCenter, NSString,
    };

    /// Four-character codes from the Apple Event Manager headers.
    const fn four_cc(s: &[u8; 4]) -> u32 {
        u32::from_be_bytes(*s)
    }
    const K_CORE_EVENT_CLASS: u32 = four_cc(b"aevt");
    const K_AE_OPEN_DOCUMENTS: u32 = four_cc(b"odoc");
    const KEY_DIRECT_OBJECT: u32 = four_cc(b"----");
    const TYPE_FILE_URL: u32 = four_cc(b"furl");

    define_class!(
        // SAFETY: NSObject has no subclassing requirements; the class keeps
        // no instance variables and does not implement Drop.
        #[unsafe(super(NSObject))]
        #[name = "SanainOpenDocumentsHandler"]
        struct Handler;

        impl Handler {
            // SAFETY: matches `handleOpen:withReply:` as registered with the
            // Apple event manager (two Apple event descriptors).
            #[unsafe(method(handleOpen:withReply:))]
            fn handle_open(
                &self,
                event: &NSAppleEventDescriptor,
                _reply: &NSAppleEventDescriptor,
            ) {
                super::push_opened(paths(event));
            }

            // SAFETY: matches `willFinishLaunching:` as registered with the
            // notification center (one NSNotification).
            #[unsafe(method(willFinishLaunching:))]
            fn will_finish_launching(&self, _note: &NSNotification) {
                // AppKit has installed its default handlers by now; ours
                // replaces the one for odoc before the launch files arrive.
                register(self);
            }
        }
    );

    fn paths(event: &NSAppleEventDescriptor) -> Vec<std::path::PathBuf> {
        let Some(list) = event.paramDescriptorForKeyword(KEY_DIRECT_OBJECT) else {
            return Vec::new();
        };
        let n = list.numberOfItems();
        // A single file may come as the descriptor itself, not a list.
        let items: Vec<Retained<NSAppleEventDescriptor>> = if n == 0 {
            vec![list]
        } else {
            (1..=n).filter_map(|i| list.descriptorAtIndex(i)).collect()
        };
        items
            .iter()
            .filter_map(|d| {
                // fileURLValue also accepts aliases and bookmarks.
                let url = d
                    .fileURLValue()
                    .or_else(|| d.coerceToDescriptorType(TYPE_FILE_URL)?.fileURLValue())?;
                super::file_url_to_path(&url.absoluteString()?.to_string())
            })
            .collect()
    }

    fn register(handler: &Handler) {
        let manager = NSAppleEventManager::sharedAppleEventManager();
        // SAFETY: `handler` implements `handleOpen:withReply:` with the
        // expected signature and lives for the rest of the process (it is
        // leaked in `prepare`).
        #[allow(unsafe_code)]
        unsafe {
            manager.setEventHandler_andSelector_forEventClass_andEventID(
                handler,
                sel!(handleOpen:withReply:),
                K_CORE_EVENT_CLASS,
                K_AE_OPEN_DOCUMENTS,
            );
        }
    }

    pub(super) fn prepare() {
        if MainThreadMarker::new().is_none() {
            return;
        }
        // SAFETY: `init` on a freshly allocated NSObject subclass with no
        // instance variables returns an initialised object.
        #[allow(unsafe_code)]
        let handler: Retained<Handler> = unsafe { msg_send![Handler::alloc(), init] };
        let name = NSString::from_str("NSApplicationWillFinishLaunchingNotification");
        // SAFETY: `handler` implements `willFinishLaunching:` taking one
        // NSNotification and is never deallocated (leaked below), so the
        // notification center never messages a dead object.
        #[allow(unsafe_code)]
        unsafe {
            NSNotificationCenter::defaultCenter().addObserver_selector_name_object(
                &handler,
                sel!(willFinishLaunching:),
                Some(&name),
                None,
            );
        }
        // If the application is already running (it should not be), the
        // notification has passed: register now as well.
        register(&handler);
        std::mem::forget(handler);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(not(windows))]
    #[test]
    fn parent_console_is_noop() {
        assert!(!attach_parent_console());
    }

    #[test]
    fn file_urls() {
        assert_eq!(
            file_url_to_path("file:///Users/a/My%20Notes/caf%C3%A9.md"),
            Some(PathBuf::from("/Users/a/My Notes/café.md"))
        );
        assert_eq!(
            file_url_to_path("file://localhost/tmp/x"),
            Some(PathBuf::from("/tmp/x"))
        );
        assert_eq!(file_url_to_path("https://example.org/x"), None);
        assert_eq!(file_url_to_path("file:///bad%2"), None);
    }

    #[test]
    fn opened_files_queue() {
        push_opened(vec![PathBuf::from("/a"), PathBuf::from("/b")]);
        assert_eq!(take_opened_files().len(), 2);
        assert!(take_opened_files().is_empty());
    }
}
