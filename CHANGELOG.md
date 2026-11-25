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

### Changed
- Wordmark set in a single colour.
