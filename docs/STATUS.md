# Status

Updated 2026-11-25. Version 0.0.1. M0 complete except trademark search.
M1 in progress.

## Done
- Environment: macOS 27.2.1 arm64 (Apple M5), Rust 1.100.0 via rustup,
  git 2.54, gh 2.101 authenticated.
- Repository: github.com/Zanttux/sanain (public, MPL-2.0). Commits use the
  owner's GitHub noreply address.
- Workspace at 0.0.1: `sanain-core`, `sanain-io`, `sanain-i18n`.
  First vertical slice (open UTF-8 file → edit → undo → atomic save →
  reopen → bytes verified) passes as an automated test.
- Checks on this machine: `cargo fmt --check` clean;
  `cargo clippy --workspace --all-targets --locked -D warnings` clean;
  `cargo test --workspace --locked` 45 passed, 0 failed.
- Multi-cursor selections and grapheme-aware editing commands.
- Fresh-context review of the IO and core crates: no silent data loss
  found; five gaps fixed (ADR 0002). Hard links still split on save.
- CI passes on ubuntu-latest, windows-latest and macos-latest
  (run 45646960690).
- GUI spike (GPUI vs egui) recorded in docs/adr/0001-gui-toolkit.md.
- Runtime dependency licenses: 41 crates, all permissive.

## Requirement evidence
| ID | State | Evidence |
|---|---|---|
| E01 | Partial | Round-trip and conflict tests in `crates/sanain-io/tests/file_lifecycle.rs` |
| E03 | Partial | Undo/redo, grouped transactions, property test, multi-cursor insert/delete |
| E12 | Partial | Grapheme-cluster navigation and deletion (`crates/sanain-core/src/grapheme.rs`) |
| E05 | Partial | UTF-8, UTF-8 BOM, UTF-16 LE/BE BOM, line endings preserved; legacy encodings not started |
| Others | Not started | |

Windows and Linux: automated tests only (CI). No GUI exists yet.

## Name availability (2026-11-25, automated lookups)
crates.io `sanain*`: free. GitHub user `sanain`: taken (repository must
live under the owner account or another org name). RDAP: sanain.com
registered; .org, .dev, .app, .io returned no registration. Trademark
registers (EUIPO, PRH, USPTO, WIPO) not searched: needs owner or counsel.

## Owner decisions (2026-11-25)
Approved: MPL-2.0, egui/eframe, public repository with delegated routine
GitHub operations, single-colour wordmark and icon as working assets.
Open: trademark search (EUIPO, PRH, USPTO, WIPO) before any launch
announcement.

## Repository settings (verified 2026-11-25)
Squash merge only, branches deleted on merge, wiki off, discussions on.
Private vulnerability reporting, Dependabot alerts and security updates,
secret scanning and push protection on. Actions: GitHub-owned actions
only, default token read-only, cannot approve PRs. Ruleset on `main`
(no bypass): PR required, three CI checks required and up to date, no
force push or deletion. Labels from config/labels.json; milestones M0–M8.
Solo-owner policy: PRs need passing CI, no approving review is required
because nobody independent exists; no fake approvals.

## Next
- Line/column mapping, vertical movement, word movement, line operations.
- Hard-link-safe save strategy.
- `sanain-app` with the first egui editor window: open, type, save.
