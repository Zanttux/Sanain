//! Operating-system integration that the window library does not offer
//! (ADR 0003). Every `unsafe` block is allowed individually and carries a
//! `SAFETY:` comment.
//!
//! - macOS: files opened from Finder (Open With, double click, drag onto
//!   the Dock icon, `open -a Text³ file`) arrive as an `odoc` Apple
//!   event, not as program arguments. [`prepare_open_documents`] handles
//!   that event and [`take_opened_files`] hands the paths to the app.
//! - Windows: [`attach_parent_console`] makes CLI output visible in the
//!   invoking console without allocating a console window, and
//!   [`system_program`] names programs that ship with the system by their
//!   full path.
//! - macOS: [`default_editor`] and [`set_default_editor`] read and set
//!   the user's default application for a content type (Launch
//!   Services), for the Default apps dialog (#164).
//! - macOS: [`bookmark`] and [`open_bookmark`] keep access to files the
//!   user chose, across restarts, inside the App Store sandbox
//!   (security-scoped bookmarks, #79).

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

/// Where a program that ships with the operating system lives.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemDir {
    /// Windows `System32` (`reg.exe`); `/usr/bin` on macOS (`open`).
    System,
    /// The Windows folder itself (`explorer.exe`); `/usr/bin` on macOS.
    Windows,
}

/// Full path of `name`, a program that ships with Windows or macOS, so it
/// is started without searching for it. On Windows `Command::new("reg")`
/// looks in the folder of the running `text3.exe` before `System32`, so a
/// `reg.exe` planted next to a downloaded copy (e.g. in Downloads) would
/// run instead (Notepad++ CVE-2025-49144 class). `None` on other systems
/// (PATH is the user's own choice there and the program folder is not
/// searched), for a `name` that is not a bare file name, or when the
/// folder cannot be found; callers must then not fall back to the bare
/// name on Windows.
pub fn system_program(name: &str, dir: SystemDir) -> Option<PathBuf> {
    if name.is_empty() || name.contains(['/', '\\', ':']) || name == "." || name == ".." {
        return None;
    }
    system_dir(dir).map(|d| d.join(name))
}

#[cfg(windows)]
fn system_dir(dir: SystemDir) -> Option<PathBuf> {
    use std::os::windows::ffi::OsStringExt;
    use windows_sys::Win32::System::SystemInformation::{
        GetSystemDirectoryW, GetSystemWindowsDirectoryW,
    };
    let mut buf = vec![0u16; 1024];
    let cap = u32::try_from(buf.len()).ok()?;
    // SAFETY: `buf` is valid for `cap` u16 writes. Both functions write at
    // most `cap` characters including the terminating NUL and return the
    // length without it, the size needed when `buf` is too small (always
    // >= `cap`), or 0 on failure; both cases are rejected below before
    // the buffer is read.
    #[allow(unsafe_code)]
    let n = unsafe {
        match dir {
            SystemDir::System => GetSystemDirectoryW(buf.as_mut_ptr(), cap),
            SystemDir::Windows => GetSystemWindowsDirectoryW(buf.as_mut_ptr(), cap),
        }
    } as usize;
    if n == 0 || n >= buf.len() {
        return None;
    }
    buf.truncate(n);
    let path = PathBuf::from(std::ffi::OsString::from_wide(&buf));
    path.is_absolute().then_some(path)
}

#[cfg(target_os = "macos")]
fn system_dir(_: SystemDir) -> Option<PathBuf> {
    Some(PathBuf::from("/usr/bin"))
}

#[cfg(not(any(windows, target_os = "macos")))]
fn system_dir(_: SystemDir) -> Option<PathBuf> {
    None
}

/// Files the system asked Text³ to open since the last call.
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

/// Turn a `file://` URL string into a path (percent-decoded). URLs that
/// name another host (`file://server/share/x`) give `None`: they are not
/// local files, and turning them into a relative path would be wrong.
pub fn file_url_to_path(url: &str) -> Option<PathBuf> {
    let rest = url.strip_prefix("file://")?;
    let rest = rest.strip_prefix("localhost").unwrap_or(rest);
    if !rest.starts_with('/') {
        return None;
    }
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

/// Bundle identifier of the user's default application for the content
/// type `uti` (e.g. `public.plain-text`), any role. `None` when there is
/// none or on other platforms.
pub fn default_editor(uti: &str) -> Option<String> {
    #[cfg(target_os = "macos")]
    return launch_services::default_handler(uti);
    #[cfg(not(target_os = "macos"))]
    {
        let _ = uti;
        None
    }
}

/// Make the application `bundle_id` the user's default for the content
/// type `uti`, all roles. The system may ask the user to confirm. Errors
/// carry the Launch Services status; other platforms return an error.
pub fn set_default_editor(uti: &str, bundle_id: &str) -> std::io::Result<()> {
    #[cfg(target_os = "macos")]
    return launch_services::set_default_handler(uti, bundle_id);
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (uti, bundle_id);
        Err(std::io::Error::other("only available on macOS"))
    }
}

/// Show `path` selected in Finder (NSWorkspace). Works inside the App
/// Store sandbox, where starting `/usr/bin/open` is not allowed (#79).
/// Returns false on other platforms or when Finder could not be asked.
pub fn reveal_in_finder(path: &std::path::Path) -> bool {
    #[cfg(target_os = "macos")]
    return finder::reveal(path);
    #[cfg(not(target_os = "macos"))]
    {
        let _ = path;
        false
    }
}

#[cfg(target_os = "macos")]
mod finder {
    use objc2::msg_send;
    use objc2::rc::Retained;
    use objc2::runtime::{AnyClass, AnyObject};
    use objc2_foundation::{NSArray, NSString, NSURL};

    pub(super) fn reveal(path: &std::path::Path) -> bool {
        let Some(s) = path.to_str() else { return false };
        let url = NSURL::fileURLWithPath(&NSString::from_str(s));
        let urls = NSArray::from_retained_slice(&[url]);
        // AppKit is loaded by the window library; without it there is no
        // NSWorkspace and nothing to do.
        let Some(cls) = AnyClass::get(c"NSWorkspace") else {
            return false;
        };
        // SAFETY: `sharedWorkspace` returns the shared NSWorkspace and
        // `activateFileViewerSelectingURLs:` takes an NSArray<NSURL> and
        // returns nothing; both may be called from any thread.
        #[allow(unsafe_code)]
        unsafe {
            let ws: Option<Retained<AnyObject>> = msg_send![cls, sharedWorkspace];
            let Some(ws) = ws else { return false };
            let _: () = msg_send![&*ws, activateFileViewerSelectingURLs: &*urls];
        }
        true
    }
}

/// A security-scoped bookmark for `path`, a file or folder this process
/// may use now (the user chose it). Stored and handed to
/// [`open_bookmark`] after a restart, it gives the sandboxed App Store
/// build access to the file again. `None` on other platforms or when the
/// system refuses (outside the sandbox it usually works too).
pub fn bookmark(path: &std::path::Path) -> Option<Vec<u8>> {
    #[cfg(target_os = "macos")]
    return bookmarks::create(path);
    #[cfg(not(target_os = "macos"))]
    {
        let _ = path;
        None
    }
}

/// Resolve a bookmark from [`bookmark`] and start using the file it
/// names; access lasts until the process exits. Returns the file's
/// current path and, when the bookmark is out of date (the file moved),
/// fresh bookmark data to store instead.
pub fn open_bookmark(data: &[u8]) -> Option<(PathBuf, Option<Vec<u8>>)> {
    #[cfg(target_os = "macos")]
    return bookmarks::open(data);
    #[cfg(not(target_os = "macos"))]
    {
        let _ = data;
        None
    }
}

#[cfg(target_os = "macos")]
mod bookmarks {
    use std::path::{Path, PathBuf};

    use objc2::runtime::Bool;
    use objc2_foundation::{
        NSData, NSString, NSURL, NSURLBookmarkCreationOptions, NSURLBookmarkResolutionOptions,
    };

    pub(super) fn create(path: &Path) -> Option<Vec<u8>> {
        let s = path.to_str()?;
        let url = NSURL::fileURLWithPath(&NSString::from_str(s));
        url.bookmarkDataWithOptions_includingResourceValuesForKeys_relativeToURL_error(
            NSURLBookmarkCreationOptions::WithSecurityScope,
            None,
            None,
        )
        .ok()
        .map(|d| d.to_vec())
    }

    pub(super) fn open(data: &[u8]) -> Option<(PathBuf, Option<Vec<u8>>)> {
        let data = NSData::with_bytes(data);
        let mut stale = Bool::NO;
        // SAFETY: `stale` is a valid, writable Bool for the whole call;
        // the other arguments are owned Objective-C objects or None.
        #[allow(unsafe_code)]
        let url = unsafe {
            NSURL::URLByResolvingBookmarkData_options_relativeToURL_bookmarkDataIsStale_error(
                &data,
                NSURLBookmarkResolutionOptions::WithSecurityScope
                    | NSURLBookmarkResolutionOptions::WithoutUI,
                None,
                &mut stale,
            )
        }
        .ok()?;
        // SAFETY: `url` came from a security-scoped bookmark. The access
        // is never stopped on purpose: the file stays usable until exit.
        #[allow(unsafe_code)]
        let started = unsafe { url.startAccessingSecurityScopedResource() };
        if !started {
            return None;
        }
        let path = PathBuf::from(url.path()?.to_string());
        let fresh = if stale.as_bool() { create(&path) } else { None };
        Some((path, fresh))
    }
}

#[cfg(target_os = "macos")]
mod launch_services {
    use std::ffi::c_void;

    use objc2::rc::Retained;
    use objc2_foundation::NSString;

    /// `kLSRolesAll` from LSConstants.h.
    const ROLES_ALL: u32 = 0xFFFF_FFFF;

    // SAFETY: the declarations match LSInfo.h (CFStringRef arguments and
    // result, LSRolesMask is UInt32, OSStatus is SInt32).
    #[allow(unsafe_code)]
    #[link(name = "CoreServices", kind = "framework")]
    unsafe extern "C" {
        // Deprecated in favour of NSWorkspace's asynchronous calls (macOS
        // 12+), but still supported; Text³ runs on macOS 11 as well.
        fn LSCopyDefaultRoleHandlerForContentType(
            content_type: *const c_void,
            role: u32,
        ) -> *mut c_void;
        fn LSSetDefaultRoleHandlerForContentType(
            content_type: *const c_void,
            role: u32,
            bundle_id: *const c_void,
        ) -> i32;
    }

    /// NSString is toll-free bridged to CFString: the same pointer is a
    /// valid CFStringRef for as long as the NSString lives.
    fn cf(s: &NSString) -> *const c_void {
        (s as *const NSString).cast()
    }

    pub(super) fn default_handler(uti: &str) -> Option<String> {
        let uti = NSString::from_str(uti);
        // SAFETY: `uti` is a valid CFStringRef (toll-free bridged) that
        // outlives the call; the function only reads it.
        #[allow(unsafe_code)]
        let raw = unsafe { LSCopyDefaultRoleHandlerForContentType(cf(&uti), ROLES_ALL) };
        // SAFETY: a non-null result is a CFStringRef we own (the Copy
        // rule, +1); CFString is bridged to NSString, so `Retained` takes
        // that reference over and releases it once.
        #[allow(unsafe_code)]
        let id: Option<Retained<NSString>> = unsafe { Retained::from_raw(raw.cast()) };
        id.map(|s| s.to_string())
    }

    pub(super) fn set_default_handler(uti: &str, bundle_id: &str) -> std::io::Result<()> {
        let uti = NSString::from_str(uti);
        let id = NSString::from_str(bundle_id);
        // SAFETY: both arguments are valid CFStringRefs (toll-free bridged)
        // that outlive the call; the function only reads them.
        #[allow(unsafe_code)]
        let status = unsafe { LSSetDefaultRoleHandlerForContentType(cf(&uti), ROLES_ALL, cf(&id)) };
        if status == 0 {
            Ok(())
        } else {
            Err(std::io::Error::other(format!(
                "Launch Services error {status}"
            )))
        }
    }
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
        #[name = "Text3OpenDocumentsHandler"]
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
    use std::path::Path;

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
        assert_eq!(file_url_to_path("file://server/share/x"), None);
    }

    #[test]
    fn system_programs_are_full_paths() {
        for bad in ["", ".", "..", "a/reg.exe", r"..\reg.exe", "C:reg.exe"] {
            assert_eq!(system_program(bad, SystemDir::System), None, "{bad}");
        }
        let reg = system_program("reg.exe", SystemDir::System);
        let explorer = system_program("explorer.exe", SystemDir::Windows);
        if cfg!(windows) {
            let reg = reg.expect("System32");
            assert!(reg.is_absolute() && reg.is_file(), "{}", reg.display());
            assert!(
                reg.parent()
                    .and_then(|p| p.file_name())
                    .is_some_and(|n| n.eq_ignore_ascii_case("system32")),
                "{}",
                reg.display()
            );
            let explorer = explorer.expect("Windows folder");
            assert!(explorer.is_file(), "{}", explorer.display());
        } else if cfg!(target_os = "macos") {
            assert_eq!(
                system_program("open", SystemDir::System),
                Some(PathBuf::from("/usr/bin/open"))
            );
            assert!(Path::new("/usr/bin/open").is_file());
        } else {
            assert_eq!((reg, explorer), (None, None));
        }
    }

    /// Reading is harmless; setting is not tested (it would change the
    /// user's defaults).
    #[test]
    fn default_editor_lookup() {
        let plain = default_editor("public.plain-text");
        if cfg!(target_os = "macos") {
            // Every Mac has a handler for plain text (TextEdit at least).
            assert!(plain.is_some_and(|id| id.contains('.')));
            assert_eq!(default_editor("invalid.text3.no-such-type"), None);
        } else {
            assert_eq!(plain, None);
            assert!(set_default_editor("public.plain-text", "x.y").is_err());
        }
    }

    /// Never panics: without AppKit (this test binary) it reports false.
    #[test]
    fn reveal_without_appkit_is_harmless() {
        let d = std::env::temp_dir();
        let shown = reveal_in_finder(&d);
        if !cfg!(target_os = "macos") {
            assert!(!shown);
        }
    }

    #[test]
    fn opened_files_queue() {
        push_opened(vec![PathBuf::from("/a"), PathBuf::from("/b")]);
        assert_eq!(take_opened_files().len(), 2);
        assert!(take_opened_files().is_empty());
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn bookmarks_round_trip_outside_the_sandbox() {
        let dir = std::env::temp_dir().join(format!("text3-bm-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("a b.txt");
        std::fs::write(&file, "x").unwrap();
        let data = bookmark(&file).expect("bookmark");
        let (path, _) = open_bookmark(&data).expect("resolve");
        assert_eq!(path.canonicalize().unwrap(), file.canonicalize().unwrap());
        assert!(open_bookmark(b"not a bookmark").is_none());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[cfg(not(target_os = "macos"))]
    #[test]
    fn bookmarks_are_macos_only() {
        assert!(bookmark(std::path::Path::new("/")).is_none());
        assert!(open_bookmark(b"x").is_none());
    }
}
