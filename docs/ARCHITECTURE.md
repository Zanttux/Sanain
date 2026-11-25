# Architecture
## Stack
Rust stable, Cargo workspace. GUI: egui/eframe recommended, GPUI alternative (docs/adr/0001-gui-toolkit.md). Ropey or equivalent rope buffer; Tree-sitter incremental parsing; Comrak Markdown AST; encoding_rs plus explicit UTF-16 handling; regex and optional bounded advanced engine; similar diff; notify watcher; serde/TOML settings; SQLite or atomic journal for recovery. Avoid redundant text shaping stack if GUI supplies it. No dependency is approved merely by listing it.

## Workspace
Implemented: `sanain-core`, `sanain-io`, `sanain-i18n` (see
docs/adr/0002-workspace-and-first-slice.md). The rest are planned and are
added when a slice needs them.
- sanain-core: document IDs, rope, byte/character/grapheme mappings, transactions, undo, selections; no GUI/network.
- sanain-io: encoding, newline preservation, file identity, atomic save, watcher, recovery.
- sanain-i18n: Fluent resources, locale negotiation, fallback.
- sanain-language: syntax grammars, incremental parsing, cancellation.
- sanain-tools: pure transformations, diff, CSV/JSON/XML/YAML adapters.
- sanain-markdown: AST, preview model, source mapping, safe links.
- sanain-app: commands, state, settings, task orchestration.
- sanain-ui: rendering, accessibility, platform adapters.
- sanain-cli: open paths, diff, new instance; later tail.

## Invariants
Document revision increases on edits. Every mutation is a transaction with inverse or undo snapshot. Background tasks receive immutable snapshots and revision IDs; stale results discarded. UI thread never performs unbounded disk I/O or parsing. Use bounded queues and cancellation. Explicit byte/char/grapheme coordinate types; no unchecked indexing. Source text remains authoritative for every Smart View.

## File lifecycle
Read raw bytes, detect BOM and encoding, prompt on ambiguity or invalid sequences; never silently lossy decode and save. Retain original newline representation. Track file identity, metadata and content fingerprint; check external changes before save. Write temporary file in same directory, flush, atomically replace using platform-appropriate APIs, preserve permissions where feasible; handle symlinks and replacement failures explicitly. Document durability limits per OS. On conflict offer reload, diff, save copy, or explicit overwrite. Never mark clean before successful save.

## Recovery
Persist journal/snapshots atomically with versioned schema and checksums; recover unnamed and dirty buffers, distinguish disk and recovered versions. Debounce with bounded maximum interval (proposed 2s); expose last recovery status. Handle corrupt journal, full disk and permission failure. Local recovery may contain sensitive data: user-controlled location, purge and disable options; never claim encryption unless implemented.

## GUI spike gate
Implement same small editor in GPUI and one fallback: typing, selection, clipboard, IME, Unicode, screen reader, 100k lines, tabs, dark/light, packaging. Record startup/RSS/input latency, build size, accessibility gaps, OS results, dependency licenses. Select only after human review. Native rendering does not imply native widgets. Do not reuse Zed editor code without license analysis.
