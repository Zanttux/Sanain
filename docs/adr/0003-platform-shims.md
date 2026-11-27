# ADR 0003: A small crate for audited platform code

Status: Accepted (2026-11-26, Claude; owner request #78)

## Context
The workspace forbids `unsafe` code. Some operating-system features that
users expect are not offered by the window library (winit/eframe):

- macOS sends files opened from Finder (Open With, double click, dropping
  on the Dock icon, `open -a Sanain file`) as an `odoc` Apple event, not as
  program arguments. winit does not handle it, and it does not install an
  application delegate either.
- Windows GUI-subsystem builds need to attach to the parent console so
  CLI commands such as `--help` can print there.

Reaching these needs Objective-C or Win32 calls that the binding crates
mark `unsafe`.

## Decision
`crates/text3-platform` holds such shims, and only them.

- The crate sets `unsafe_code = "deny"` (the rest of the workspace keeps
  `forbid`); each block opts in with `#[allow(unsafe_code)]` and a
  `SAFETY:` comment; clippy's `undocumented_unsafe_blocks` is on.
- The public API is safe and platform-neutral
  (`prepare_open_documents`, `on_opened_files`, `take_opened_files`,
  `attach_parent_console`); unsupported platform operations are no-ops.
- macOS: before the event loop starts, a handler object observes
  `NSApplicationWillFinishLaunchingNotification` and then registers itself
  with `NSAppleEventManager` for `kAEOpenDocuments` — the point Apple
  documents for replacing AppKit's default handlers, before launch files
  are delivered. Paths are queued and opened by the shell like dropped
  files. `Info.plist` declares text document types with
  `LSHandlerRank = Alternate`, so Sanain appears in Open With without
  claiming any type by default.
- Bindings: `objc2` and `objc2-foundation`, already in the dependency tree
  through winit (MIT).
- Windows: `attach_parent_console` calls `AttachConsole(ATTACH_PARENT_PROCESS)`
  only for CLI paths that print and exit, never for normal GUI launches.
  It returns whether attachment succeeded (false on non-Windows), without
  allocating a console. Bindings: Windows-only `windows-sys` 0.59, already
  locked, with only `Win32_System_Console` and `Win32_Foundation` enabled
  directly (MIT OR Apache-2.0).

## Consequences
- The existing macOS unsafe sites and one Windows unsafe block stay in
  `text3-platform/src/lib.rs`, individually documented and reviewable.
- Windows attachment sets standard handles for processes without a console;
  Rust printing uses `GetStdHandle`, so no stream rebinding is needed.
  Output may appear after the shell prompt because shells do not wait for
  GUI-subsystem programs. This is expected Windows behaviour, not an error.
- Non-Windows no-op behaviour is unit tested; Windows runtime console
  behaviour still needs verification on Windows.
- Tested on macOS 27.2 with `open -a` both at launch and while running.
  Finder's Open With uses the same event.
