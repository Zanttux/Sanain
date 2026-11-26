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
  copies permissions, renames atomically, syncs the directory on Unix,
  follows symlinks (including dangling ones) to replace the target, and
  refuses to overwrite
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
Directory fsync is best effort. Saving splits hard links (other names keep
the old content). Update 2026-11-26: on Unix, files with more than one link are now overwritten in place behind a backup copy (`write_in_place` in sanain-io); Windows keeps the rename. Unix owner/group, Windows ACLs and extended attributes
are not preserved yet. Read-only targets are refused; there is no
"save anyway" path yet. UTF-32 files are rejected on load.

## Review
Fresh-context review of 4d264c7 found no silent data loss. Fixed after
review: dangling symlinks, 0600 mode on new files, silent replacement of
read-only files, a save that could be reported failed after succeeding,
UTF-32 misread as UTF-16.
