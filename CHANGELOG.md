# Changelog

All notable changes are listed here. Versions follow Semantic Versioning.

## [Unreleased] — 0.0.1

### Added
- Cargo workspace at 0.0.1 with pinned toolchain and CI for Linux, Windows
  and macOS.
- `sanain-core`: rope document, grouped transactions, undo/redo, revision
  counter, save-point tracking, line-ending detection.
- `sanain-io`: strict UTF-8 / UTF-8 BOM / UTF-16 BOM decoding with
  byte-exact re-encoding; atomic save that preserves permissions and
  symlinks and refuses to overwrite files changed on disk.
- Multi-cursor selections and grapheme-aware insert, Backspace, Delete
  and arrow movement (combining marks, emoji sequences, flags, CRLF move
  and delete as one unit).
- `sanain-i18n`: Fluent localization with en-US and draft fi-FI, locale
  negotiation and resource consistency tests.

- MPL-2.0 license, security policy with private reporting, CODEOWNERS,
  Dependabot.

- First editor window (egui): open from the command line or File > Open,
  typing with IME support, mouse and keyboard selection, multi-line
  undo/redo, cut/copy/paste, Save and Save As with native dialogs, prompts
  for unsaved changes on close/new/open, external-change and read-only
  confirmations, status bar with position, line ending and encoding,
  light and dark themes from the brand tokens.

- Native macOS menu bar (Sanain, File, Edit, Window) with standard
  shortcuts; restyled in-window menus on Windows and Linux.
- Cut, Copy and Paste work (system clipboard was not enabled before).
- macOS app bundle script (`scripts/macos-bundle.sh`).
- Find and replace: Cmd/Ctrl+F, Find Next/Previous (Cmd/Ctrl+G,
  Shift+Cmd/Ctrl+G, Enter/Shift+Enter), Replace (Cmd+Option+F on macOS,
  Ctrl+H elsewhere). Match case, whole word and regular expressions with
  `$1` capture references. All matches are highlighted with a count;
  Replace All is one undo step. Regular expressions run in linear time.
- Tabs: open several files (command line, File > Open with multi-select),
  New Tab, Close Tab (Cmd/Ctrl+W), Reopen Closed Tab (Cmd/Ctrl+Shift+T),
  Next/Previous Tab (Ctrl+Tab / Ctrl+Shift+Tab), middle-click to close,
  unsaved marker per tab. Opening a file that is already open switches to
  it. Quitting asks about each unsaved tab in turn. Every crash-recovery
  snapshot is restored as its own tab.
- Line operations: move line up/down (Option/Alt+Up/Down), duplicate
  (Option/Alt+Shift+Down), delete (Cmd/Ctrl+Shift+K), also in the Edit
  menu. They work on every selected line and keep the file's line endings.
- Up/Down keep the caret's column across shorter lines.
- Word movement and deletion: Option+Arrow / Option+Backspace on macOS,
  Ctrl+Arrow / Ctrl+Backspace / Ctrl+Delete elsewhere.
- Crash recovery: unsaved text is checkpointed within 2 seconds of an
  edit and restored on the next launch; checkpoints are deleted on save
  or when changes are discarded.

### Changed
- Repository renamed to github.com/Zanttux/Sanain.
- Wordmark set in a single colour.
