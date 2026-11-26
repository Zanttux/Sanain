# Quality gates

Run locally and in CI:

    cargo fmt --all -- --check
    cargo clippy --workspace --all-targets --locked -- -D warnings
    cargo test --workspace --locked

Unit: rope offsets, graphemes, undo/redo, transactions, encodings, newline preservation, parser and action errors. Property tests: random edits + undo restore original, coordinate round trips, unchanged file bytes. Integration: save/reopen, crash recovery, external conflict, disk full, permission denial, corrupt session, symlinks, concurrent instances. Fuzz parsers, decoding, recovery and transformations with bounded inputs. UI: keyboard, IME, screen reader, tabs, split, drag/drop, DPI, native dialogs. Security: malicious Markdown/XML/YAML, no outbound traffic in default use, dependency audit and license checks.

Disk-full drill: `crates/sanain-app/tests/disk_full.rs` runs only when `SANAIN_FULL_DIR` names a directory on a small throwaway filesystem (it refuses to fill more than 512 MiB); otherwise it prints "skipped" and passes. It fills the filesystem itself, leaving 64 KiB free. CI runs it after the workspace tests on a 1 MiB tmpfs (Linux), a 2 MiB HFS+ disk image (macOS) and an 8 MiB NTFS VHD created with `diskpart` (Windows; runners are administrators). Locally on macOS:

    hdiutil create -size 2m -fs HFS+ -volname SanainFull -layout NONE full.dmg
    hdiutil attach -nobrowse -mountpoint ./full full.dmg
    SANAIN_FULL_DIR=$PWD/full cargo test -p sanain-app --test disk_full --locked
    hdiutil detach ./full

On Linux: `sudo mount -t tmpfs -o size=1m tmpfs /mnt/sanain-full` and `chown` it to yourself first.

Proposed performance budgets (not measurements): cold start p95 <= 500 ms, warm <= 250 ms, idle RSS <= 100 MiB, typing p95 <= 16 ms on documented reference machine; 100 MB file opens <= 2s where feasible; 10 GB read-only log mode bounded memory (P1). Record OS, CPU, RAM, disk, build, dataset, 20+ runs, cold/warm method, median/p95 and peak RSS. Do not claim targets met without evidence. Compare Notepad++ on same Windows hardware and tasks; no cross-platform apples-to-oranges comparisons.

Release gate: all P0 acceptance tests, no known data-loss/security blockers, signed artifacts where available, dependency inventory, manual smoke on supported OS, recovery drill, accessibility report, human approval. CI should run fmt/clippy/test on Windows/macOS/Linux; platform packaging smoke tests and reproducible build documentation. Never mark skipped tests as passed.
