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
| E06 disk full | 2026-11-26 | macOS 27.2.1 | Pass | 2 MB RAM disk filled to 0 free; pasted 600 kB into an open file there and saved: dialog "the disk is full… file on disk is unchanged"; file hash unchanged, no temporary file left, tab still unsaved, recovery checkpoint present. After freeing space, Save wrote 600 037 bytes. Permission denial (folder mode 555): own message, file unchanged. |
| E06 disk-full drill (automated) | 2026-11-26 | macOS 27.2.1; CI Linux, macOS, Windows | Pass on macOS; CI pending first PR run | `SANAIN_FULL_DIR=<mount> cargo test -p sanain-app --test disk_full --locked` on a 2 MB HFS+ image (`hdiutil create -size 2m -fs HFS+ -layout NONE`). The test fills the disk to 64 KiB free, then: 1 MiB editor saves (atomic and hard-linked in place) fail with StorageFull, text kept and unsaved; recovery snapshot, settings, session and macros writes (incl. backup of an unreadable macros file) fail; every file byte-identical and no file added. Then small saves succeed, and the large save succeeds after the space is freed. Before the fix the in-place save left the file truncated plus `.linked.txt.sanain-backup`, and macros left a partial `macros.1.bak`. CI: 4 MiB tmpfs (Linux), 2 MiB HFS+ image (macOS), 8 MiB NTFS VHD made with diskpart (Windows). |
| E08 preview (partial) | 2026-11-25 | macOS 27.2.1 | Pass for implemented parts | GFM demo with front matter, task lists, table, code, Mermaid block and `<script>`: split and preview render; script shown as text; SHA-256 of the file identical after viewing and quitting. Images, link opening and Mermaid not implemented. |
| E08 Mermaid | 2026-11-26 | macOS 27.2.1 | Pass | Document with a TD flowchart (all five shapes, solid/dotted/thick links, `|yes|` and `-- no -->` labels, a `click` line), an LR cycle with a chain and `&` group, and a `sequenceDiagram`: preview draws the two flowcharts (cycle edge routed back, click ignored) and shows the sequence source with "Diagram type not supported; showing source." Checked in the dark, light and Dracula themes; document not modified. |
| E10 compare (partial) | 2026-11-25 | macOS 27.2.1 | Pass for implemented parts | Two tabs compared; first hunk copied left; saved file bytes as expected; untouched hunk unchanged. Apply-all-hunks equals the other side (property test). Stale-revision rejection implemented, not yet exercised by an automated test. |
| E01 external change | 2026-11-25 | macOS, CI all OS | Pass (automated) | `crates/sanain-io/tests/file_lifecycle.rs`, `crates/sanain-app/src/editor.rs` tests |
