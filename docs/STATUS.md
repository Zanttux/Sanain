# Status

Updated 2026-11-25. Version 0.0.1. Milestone M0 (exit pending owner
decisions); M1 toolkit-independent work started.

## Done
- Environment: macOS 27.2.1 arm64 (Apple M5), Rust 1.100.0 via rustup,
  git 2.54, gh 2.101 authenticated. Local git repository, branch `main`,
  no remote.
- Workspace at 0.0.1: `sanain-core`, `sanain-io`, `sanain-i18n`.
  First vertical slice (open UTF-8 file → edit → undo → atomic save →
  reopen → bytes verified) passes as an automated test.
- Checks on this machine: `cargo fmt --check` clean;
  `cargo clippy --workspace --all-targets --locked -D warnings` clean;
  `cargo test --workspace --locked` 29 passed, 0 failed.
- CI workflow (`.github/workflows/ci.yml`) written with a pinned
  `actions/checkout` SHA. Not yet run: no remote repository.
- GUI spike (GPUI vs egui) recorded in docs/adr/0001-gui-toolkit.md.
- Runtime dependency licenses: 41 crates, all permissive.

## Requirement evidence
| ID | State | Evidence |
|---|---|---|
| E01 | Partial | Round-trip and conflict tests in `crates/sanain-io/tests/file_lifecycle.rs` |
| E03 | Partial | Undo/redo, grouped transactions, property test |
| E05 | Partial | UTF-8, UTF-8 BOM, UTF-16 LE/BE BOM, line endings preserved; legacy encodings not started |
| Others | Not started | |

Untested platforms: Windows, Linux (CI configured, not run).

## Name availability (2026-11-25, automated lookups)
crates.io `sanain*`: free. GitHub user `sanain`: taken (repository must
live under the owner account or another org name). RDAP: sanain.com
registered; .org, .dev, .app, .io returned no registration. Trademark
registers (EUIPO, PRH, USPTO, WIPO) not searched: needs owner or counsel.

## Owner decisions needed
1. License: MPL-2.0 recommended (docs/LICENSE_DECISION.md).
2. GUI toolkit: egui/eframe recommended (docs/adr/0001-gui-toolkit.md).
   M1 GUI work waits on this.
3. GitHub: create repository (suggest `Zanttux/sanain`, private until
   license and clearance), then activate the AUTONOMY policy for branches,
   PRs and CI.
4. Brand: approve the single-colour wordmark and the icon concept.

## Next
- Cursor, selection and grapheme-aware editing commands in `sanain-core`.
- After decisions 1–3: push, verify CI on all three OSes, then the first
  editor window.
