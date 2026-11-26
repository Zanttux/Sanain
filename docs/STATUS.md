# Status

Updated 2026-11-26. Version 0.0.1 (pre-alpha, no release published).
M0 complete except trademark search. M1 feature work largely done; next
are cross-platform verification and release readiness.

## Summary for the owner
- 59 pull requests merged; every one passed CI on Linux, Windows and
  macOS plus the dependency check (cargo-deny, third-party notices).
  195 automated tests pass.
- Sanain is a usable everyday editor on macOS: tabs and split view,
  multiple cursors, macros, word wrap, find/replace with preview and
  folder search, Markdown preview, data views (JSON/XML/YAML/CSV),
  comparison, local history, crash recovery, a log viewer for huge
  files, 18 highlighted languages and 26 UI languages (24 of them
  unreviewed machine translations).
- Robustness checked by stress tests: 147 MB file, 2.7 MB single line,
  full disk, missing permissions, random binary data, deeply nested JSON.
- Not verified yet: running on Windows and Linux (CI builds and tests
  only), screen readers (VoiceOver, NVDA, Orca) and IME input by a person.
- Needs you before a public release: code-signing certificates and the
  decision to publish (see "Owner-only items").

## Done
- Environment: macOS 27.2.1 arm64 (Apple M5), Rust 1.100.0 via rustup,
  git 2.54, gh 2.101 authenticated.
- Repository: github.com/Zanttux/Sanain (public, MPL-2.0). Commits use the
  owner's GitHub noreply address.
- Workspace at 0.0.1: `sanain-core`, `sanain-io`, `sanain-i18n`.
  First vertical slice (open UTF-8 file → edit → undo → atomic save →
  reopen → bytes verified) passes as an automated test.
- Checks on this machine (2026-11-26): `cargo fmt --check` clean;
  `cargo clippy --workspace --all-targets --locked -D warnings` clean;
  `cargo test --workspace --locked` 195 passed, 0 failed; `cargo deny
  --locked check` clean.
- Multi-cursor selections and grapheme-aware editing commands.
- First egui editor window (`cargo run -p sanain-app -- file`). Verified
  by hand on macOS with synthetic keystrokes: typing, Backspace, arrows,
  Enter (inserts the file's CRLF), Cmd+S writes the expected bytes; close
  with unsaved changes prompts; Save / Don't Save / Cancel each behave
  correctly; dialog buttons are reachable through the macOS accessibility
  tree; Finnish UI renders.
- Fresh-context review of the IO and core crates: no silent data loss
  found; five gaps fixed (ADR 0002). Hard-linked files are saved in place
  behind a backup on Unix (2026-11-26); Windows still splits hard links.
- CI passes on ubuntu-latest, windows-latest and macos-latest
  (run 45646960690).
- GUI spike (GPUI vs egui) recorded in docs/adr/0001-gui-toolkit.md.
- Runtime dependency licenses (2026-11-25): 405 crates, none copyleft-only;
  two are dual-licensed with a GPL/LGPL option and are used under their
  permissive option. Since 2026-11-26 CI runs cargo-deny (licenses
  allow-list, RustSec advisories, sources; deny.toml) and generates the
  third-party notices (479 crates, all with license text).

## Requirement evidence
| ID | State | Evidence |
|---|---|---|
| E01 | Partial | Round-trip and conflict tests in `crates/sanain-io/tests/file_lifecycle.rs` |
| E03 | Partial | Undo/redo, grouped transactions, property test, indent/outdent of selected lines (Tab/Shift+Tab), toggle line comment, multi-cursor editing created by Alt+click, Add Cursor Above/Below (keyboard) and Alt+drag rectangular selection; Esc collapses. Select Next Occurrence (Cmd/Ctrl+D) and Skip to Next Occurrence (Cmd/Ctrl+Shift+D). Not yet: rectangular selection by keyboard, virtual space past line ends |
| E12 | Partial | Grapheme-cluster navigation and deletion; editor exposes the caret line, caret and selection to AccessKit (checked via the macOS accessibility API: AXValue and AXSelectedText follow the caret); high-contrast appearance; every command reachable by keyboard (menus, palette). Not yet: tested with VoiceOver speech by a person, NVDA/Orca, IME composition by a person |
| E04 | Partial | Find/replace in the current tab: literal, regex, case, whole word, highlight, count, replace all as one undo. Replace All shows a preview (count, changed lines before/after, first 200) and applies only on confirmation, refusing if the document changed meanwhile. Search across files: see E15. Searches in documents over 1 M chars run in the background and are cancelled by newer ones (147 MB file stays responsive). |
| E08 | Partial | Source/split/preview, GFM blocks and inlines, front matter, caret-following preview, raw HTML inert, source bytes unchanged (hash checked). Links: web with confirmation, local files, anchors, unsafe schemes blocked. Local images inside the document folder (limits enforced; remote never fetched). Go to Heading (filterable heading list). Fenced code blocks coloured by language (18 languages and common aliases, blocks up to 64 KB, parsed once per block). Not yet: permission flow for images outside the folder, Mermaid, persistent outline panel |
| E10 | Partial | Two-tab side-by-side line diff (time-bounded), apply hunk in either direction as one undo, stale-revision check. Compare with the saved file, the clipboard or any file chosen in a file dialog (read-only right side; encoding detected as on open). Intraline highlight of changed characters (pairs with at least 40 % in common, lines up to 2 000 chars). Ignore whitespace option (lines differing only in spaces, tabs or line endings count as equal; applying a hunk still copies exact bytes). From the command line: `sanain --diff LEFT RIGHT` opens both and compares them (both sides editable) |
| E15 | Partial | Search across a folder (background, cancellable, bounded), results open at the match. Keyboard macros: record/stop, play at the cursor, each run one undo step (grouped undo in core); mouse placement not recorded. Play Macro Multiple Times (1–10 000 runs, one undo step). Not yet: replace across files, saved/named macros |
| E13 | Partial | Log viewer: background indexing and filtering (cancellable), follow/tail, truncation detection, auto-open at 256 MB, level colours, Go to Line. UTF-8 only (lossy display); tested on macOS with a 302 MB file |
| E14 | Done | Local history: copy per save (50 versions / 30 days / 10 MB cap, owner-only files), list, open, restore as one undo step, disable, purge. Tested on macOS |
| L10n | Partial | 26 UI languages (en-US canonical; fi-FI draft; 24 machine translations by Claude awaiting native review). Every message formats in every locale (test). RTL languages not shipped |
| E05 | Partial | UTF-8, UTF-8 BOM, UTF-16 LE/BE BOM, 17 legacy encodings by explicit choice (exact round trip enforced), explicit encoding and LF/CRLF conversion. UTF-16 LE/BE without BOM detected (zero-byte pattern, exact decode) and opened directly; legacy encodings suggested by chardetng in the picker (user confirms, exact round trip still required). Files over 16 MB: guessed from the first 64 KiB, the open still verifies the whole file. ASCII-only UTF-16 without a BOM (which is also valid UTF-8 with NULs) is recognised on open |
| E09 | Partial | JSON validate/format/minify preserving lexemes; read-only JSON, XML and YAML trees (no entity or alias expansion, node cap, iterative builders; tested against billion-laughs inputs); read-only CSV/TSV grid with delimiter detection. Validate XML/YAML (well-formedness, caret to the error). Not yet: XML/YAML formatting (would change mixed content/comments), schema validation, editing in structured views |
| E02 | Partial | Tabs, close/reopen, keyboard switching, dirty markers, per-tab recovery, drag reorder, pin (kept in front, saved in session), tab context menu. Split editor: two panes side by side, focus follows interaction, the same file may be shown in both (shared caret). Move Tab Left/Right (Ctrl+Shift+Page Up/Down, Window menu; pinned and unpinned tabs stay in their groups). Not yet: more than two panes, Markdown preview/structured views inside a split |
| E07 | Partial | Tree-sitter highlighting for 18 languages, background worker with cancellation, line numbers, indentation folding, keyboard navigation, light/dark/system/high-contrast themes, zoom. Not yet: syntax-aware folding, incremental parsing, more languages |
| E11 | Done for v1 scope | Menus with standard shortcuts (logical keys on macOS), command palette with fuzzy search, configurable shortcuts with conflict handling, Notepad++ preset. Not yet: importing Notepad++ shortcuts.xml |
| E06 | Partial | Unsaved-changes prompts; crash recovery per tab (kill -9 → relaunch restores text); session restore of open files and active tab. Disk-full and permission-denied saves verified on macOS (file unchanged, text kept, plain error; see ACCEPTANCE). Session restore can be turned off in Settings. Not yet: disk-full drill on Windows/Linux |
| Others | Not started | |

Keyboard layouts (checked 2026-11-26, Finnish layout, macOS): native
menu shortcuts on punctuation keys are localised by macOS and work as
shown in the menu (Toggle Line Comment Cmd+´, Indent Cmd+Ä, Outdent
Cmd+Ö); Cmd+Shift+7 does not reach the window at all. Not verified: the
same shortcuts with in-window menus on Windows/Linux non-US layouts (they
match egui's logical key; may need per-layout defaults or rebinding in
Settings).

Known M1 limits: native menus only on macOS (Windows/Linux use in-window
menus); running the bare binary on macOS shows "sanain" in the menu bar,
use `scripts/macos-bundle.sh` for the proper app name;
word wrap only for documents up to 1 M characters (full rebuild per
edit); visible lines are laid
out each frame (egui caches identical layouts; unwrapped lines over 4 096
characters are laid out only around the visible columns); dialog button
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
- Windows and Linux runtime smoke test (needs a machine or VM): open,
  edit, save, menus, shortcuts on non-US layouts.
- Manual screen-reader (VoiceOver) and IME pass on macOS.
- Release preparation for 0.1.0: unsigned archives exist (`Package`
  workflow); still SBOM, Linux desktop entry and icon, release notes;
  publishing waits for the owner (signing).
- Smaller: incremental word wrap (lift the 1 M-character limit), log
  viewer for non-UTF-8 files, Mermaid diagrams in Markdown preview.
