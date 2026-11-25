# Status

Updated 2026-11-25. Version 0.0.1. M0 complete except trademark search.
M1 in progress.

## Done
- Environment: macOS 27.2.1 arm64 (Apple M5), Rust 1.100.0 via rustup,
  git 2.54, gh 2.101 authenticated.
- Repository: github.com/Zanttux/Sanain (public, MPL-2.0). Commits use the
  owner's GitHub noreply address.
- Workspace at 0.0.1: `sanain-core`, `sanain-io`, `sanain-i18n`.
  First vertical slice (open UTF-8 file → edit → undo → atomic save →
  reopen → bytes verified) passes as an automated test.
- Checks on this machine: `cargo fmt --check` clean;
  `cargo clippy --workspace --all-targets --locked -D warnings` clean;
  `cargo test --workspace --locked` 45 passed, 0 failed.
- Multi-cursor selections and grapheme-aware editing commands.
- First egui editor window (`cargo run -p sanain-app -- file`). Verified
  by hand on macOS with synthetic keystrokes: typing, Backspace, arrows,
  Enter (inserts the file's CRLF), Cmd+S writes the expected bytes; close
  with unsaved changes prompts; Save / Don't Save / Cancel each behave
  correctly; dialog buttons are reachable through the macOS accessibility
  tree; Finnish UI renders.
- Fresh-context review of the IO and core crates: no silent data loss
  found; five gaps fixed (ADR 0002). Hard links still split on save.
- CI passes on ubuntu-latest, windows-latest and macos-latest
  (run 45646960690).
- GUI spike (GPUI vs egui) recorded in docs/adr/0001-gui-toolkit.md.
- Runtime dependency licenses (2026-11-25): 405 crates, none copyleft-only;
  two are dual-licensed with a GPL/LGPL option and are used under their
  permissive option. Automated checking (cargo-deny) not yet in CI.

## Requirement evidence
| ID | State | Evidence |
|---|---|---|
| E01 | Partial | Round-trip and conflict tests in `crates/sanain-io/tests/file_lifecycle.rs` |
| E03 | Partial | Undo/redo, grouped transactions, property test, multi-cursor editing created by Alt+click, Add Cursor Above/Below (keyboard) and Alt+drag rectangular selection; Esc collapses. Not yet: rectangular selection by keyboard, virtual space past line ends, add next occurrence |
| E12 | Partial | Grapheme-cluster navigation and deletion; editor exposes the caret line, caret and selection to AccessKit (checked via the macOS accessibility API: AXValue and AXSelectedText follow the caret); high-contrast appearance; every command reachable by keyboard (menus, palette). Not yet: tested with VoiceOver speech by a person, NVDA/Orca, IME composition by a person |
| E04 | Partial | Find/replace in the current tab: literal, regex, case, whole word, highlight, count, replace all as one undo. Not yet: preview before replace-all, cancellation for huge files, search across files |
| E08 | Partial | Source/split/preview, GFM blocks and inlines, front matter, caret-following preview, raw HTML inert, source bytes unchanged (hash checked). Links: web with confirmation, local files, anchors, unsafe schemes blocked. Local images inside the document folder (limits enforced; remote never fetched). Go to Heading (filterable heading list). Not yet: permission flow for images outside the folder, Mermaid, persistent outline panel, syntax colours in code blocks |
| E10 | Partial | Two-tab side-by-side line diff (time-bounded), apply hunk in either direction as one undo, stale-revision check. Not yet: intraline highlight, whitespace options, files/clipboard as sources |
| E15 | Partial | Search across a folder (background, cancellable, bounded), results open at the match. Not yet: replace across files, macros |
| E13 | Partial | Log viewer: background indexing and filtering (cancellable), follow/tail, truncation detection, auto-open at 256 MB. UTF-8 only (lossy display); tested on macOS with a 302 MB file |
| E14 | Done | Local history: copy per save (50 versions / 30 days / 10 MB cap, owner-only files), list, open, restore as one undo step, disable, purge. Tested on macOS |
| L10n | Partial | 26 UI languages (en-US canonical; fi-FI draft; 24 machine translations by Claude awaiting native review). Every message formats in every locale (test). RTL languages not shipped |
| E05 | Partial | UTF-8, UTF-8 BOM, UTF-16 LE/BE BOM, 17 legacy encodings by explicit choice (exact round trip enforced), explicit encoding and LF/CRLF conversion. Not yet: UTF-16 without BOM, automatic legacy detection |
| E09 | Partial | JSON validate/format/minify preserving lexemes; read-only JSON, XML and YAML trees (no entity or alias expansion, node cap, iterative builders; tested against billion-laughs inputs); read-only CSV/TSV grid with delimiter detection. Not yet: XML/YAML formatting and validation tools, editing in structured views |
| E02 | Partial | Tabs, close/reopen, keyboard switching, dirty markers, per-tab recovery, drag reorder, pin (kept in front, saved in session), tab context menu. Not yet: split editor, keyboard reordering |
| E07 | Partial | Tree-sitter highlighting for 8 languages, background worker with cancellation, line numbers, indentation folding, keyboard navigation, light/dark/system/high-contrast themes, zoom. Not yet: syntax-aware folding, incremental parsing, more languages |
| E11 | Done for v1 scope | Menus with standard shortcuts (logical keys on macOS), command palette with fuzzy search, configurable shortcuts with conflict handling, Notepad++ preset. Not yet: importing Notepad++ shortcuts.xml |
| E06 | Partial | Unsaved-changes prompts; crash recovery per tab (kill -9 → relaunch restores text); session restore of open files and active tab. Not yet: settings opt-out UI, disk-full drill |
| Others | Not started | |

Known M1 limits: native menus only on macOS (Windows/Linux use in-window
menus); running the bare binary on macOS shows "sanain" in the menu bar,
use `scripts/macos-bundle.sh` for the proper app name;
word wrap only for documents up to 1 M characters (full rebuild per
edit), Home/End act on the whole line rather than the display row;
every visible line
is re-laid out each frame (very long lines will be slow); dialog button
order follows macOS on all platforms; IME composition and VoiceOver
reading of document text not yet verified by a person.

Windows and Linux: automated tests and builds only (CI); the GUI has not
been run on them.

## Name availability (2026-11-25, automated lookups)
crates.io `sanain*`: free. GitHub user `sanain`: taken (repository must
live under the owner account or another org name). RDAP: sanain.com
registered; .org, .dev, .app, .io returned no registration. Trademark
registers (EUIPO, PRH, USPTO, WIPO) not searched: needs owner or counsel.

## Owner decisions (2026-11-25)
Approved: MPL-2.0, egui/eframe, public repository with delegated routine
GitHub operations, single-colour wordmark and icon as working assets.
Standing order: no questions to the owner (D19).

## Owner-only items (never block work)
- Automatic installation of updates needs code-signing certificates
  (Apple Developer ID, Windows Authenticode) and a key for signing update
  manifests. Only the check-and-notify part is implemented.
- Trademark search for "Sanain" (EUIPO, PRH, USPTO, WIPO) before any
  launch announcement.

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
- Log viewer next steps (E13): jump to line, level highlighting, encodings other than UTF-8, very long files beyond ~16 M lines (f32 scroll precision).
- Manual IME and screen-reader pass on macOS; Windows and Linux GUI smoke
  test (needs hardware or VM access).
