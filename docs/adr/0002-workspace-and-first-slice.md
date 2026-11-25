# ADR 0002: Workspace layout and first slice

- Date: 2026-11-25
- Status: Accepted (routine implementation decision)

## Decision
Cargo workspace at version 0.0.1, toolchain pinned in `rust-toolchain.toml`,
MSRV 1.88, `unsafe_code = "forbid"`. Crates are added only when a slice
needs them. First crates:

- `sanain-core`: `ropey` rope, `Transaction` of ordered `Change`s with exact
  inverses, undo/redo, monotonic `Revision`, save point tracked by history
  position (undo back to the saved state is clean). Positions are `char`
  indices; byte/grapheme conversions are explicit.
- `sanain-io`: strict decoding of UTF-8, UTF-8 BOM and UTF-16 LE/BE with
  BOM. Invalid input is an error, never replaced. Line endings are kept as
  bytes in the text. Save writes a temp file in the same directory, fsyncs,
  copies permissions, renames atomically (`tempfile`), syncs the directory
  on Unix, follows symlinks to replace the target, and refuses to overwrite
  a file whose length, mtime or content changed since load unless the
  caller passes `SaveMode::Overwrite`.
- `sanain-i18n`: Fluent with embedded resources; negotiation exact →
  language → en-US; per-message fallback; message ID shown when missing
  everywhere. A test checks every FTL file parses, has no duplicate IDs, and
  uses the same variables as en-US.

## Alternatives
`encoding_rs` deferred until legacy encodings (E05) land; std covers UTF-8
and UTF-16 strictly. Fingerprint uses std SipHash in-process; switch to a
persisted cryptographic hash when recovery (E06) stores fingerprints.

## Known limits
Conflict check and rename are not one atomic operation (TOCTOU window).
Directory fsync is best effort. Windows ACLs and extended attributes are
not preserved yet.
