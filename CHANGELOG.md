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
- `sanain-i18n`: Fluent localization with en-US and draft fi-FI, locale
  negotiation and resource consistency tests.

### Changed
- Wordmark set in a single colour.
