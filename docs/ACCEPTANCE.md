# Acceptance scenarios
- E01/E05: open UTF-8 BOM, UTF-16 and CRLF fixtures; save without edits; compare hashes. Invalid encoding must prompt rather than replace bytes.
- E06: type unnamed note, wait for recovery checkpoint, kill process, restart; note restored. Simulate full disk; dirty state retained and error visible.
- E01: external process changes open file; save must not overwrite silently; choose diff or save copy.
- E04: regex replace across selection, preview count, apply, undo; invalid pattern gives error; cancellation works.
- E08: open README with tables, task lists, code, local image and Mermaid; source/split/preview work offline; source hash unchanged; malicious script not executed.
- E09: malformed JSON and CSV with quoted newlines; clear errors, no source mutation on inspection.
- E10: compare two tabs, apply one hunk, undo; stale revision rejected.
- E12: navigate all primary actions without mouse; test IME composition and emoji graphemes on each supported OS.
- Funding: free download exposes all features; no account or network needed; no purchase prompts during editing.

Evidence per scenario: commit, OS, fixture, expected/actual, test command, pass/fail/blocked, reviewer.

## Results
| Scenario | Date | OS | Result | Evidence |
|---|---|---|---|---|
| E06 recovery after kill | 2026-11-25 | macOS 27.2.1 | Pass | Typed into untitled buffer, waited 3 s, `kill -9`, relaunched: text restored, marked unsaved, status shown. Don't Save removed the checkpoint. Disk-full part not run. |
| E08 preview (partial) | 2026-11-25 | macOS 27.2.1 | Pass for implemented parts | GFM demo with front matter, task lists, table, code, Mermaid block and `<script>`: split and preview render; script shown as text; SHA-256 of the file identical after viewing and quitting. Images, link opening and Mermaid not implemented. |
| E10 compare (partial) | 2026-11-25 | macOS 27.2.1 | Pass for implemented parts | Two tabs compared; first hunk copied left; saved file bytes as expected; untouched hunk unchanged. Apply-all-hunks equals the other side (property test). Stale-revision rejection implemented, not yet exercised by an automated test. |
| E01 external change | 2026-11-25 | macOS, CI all OS | Pass (automated) | `crates/sanain-io/tests/file_lifecycle.rs`, `crates/sanain-app/src/editor.rs` tests |
