# Milestones
| Milestone | Deliverable | Exit gate |
|---|---|---|
| M0 | License/name clearance, GUI spike, ADRs | Owner approves reserved decisions; Claude selects routine implementation details |
| M1 | Core editing + file IO + minimal GUI | Round-trip and undo tests; keyboard typing |
| M2 | Tabs, sessions, recovery, search | Crash and conflict tests; no silent loss |
| M3 | Syntax, settings, shortcuts, accessibility | Platform smoke and IME tests |
| M4 | Markdown source/split/preview | GFM fixtures, safe offline rendering, Mermaid spike |
| M5 | Smart Views/Actions and diff | Undoable transformations and malformed input tests |
| M6 | Cross-platform packaging, docs, beta | CI matrix, benchmark report, security review |
| M7 | Stable v1 | All P0 gates and human release approval |
| M8 | Logs, history, macros, optional AI/plugins | Separate ADR and threat model per feature |

No calendar promises. Each milestone is split into small PR-sized tasks. First vertical slice: open UTF-8 file, edit, undo, save atomically, reopen and verify bytes. Do not build all crates upfront; add boundaries as needed. Track requirement IDs and evidence in STATUS.md.

## Requested by the owner (2026-11-26)
- Shell integration: "Open with Text³" in Windows Explorer, Finder Open With / default app, Linux `.desktop` entry with MIME types (#78). Reversible, no elevation by default.
- Paid Mac App Store and Microsoft Store builds as a support model; free builds and all features stay free (#79). Store accounts and pricing are owner decisions.
- (2026-11-27) Done: new-tab button: a "+" button right after the last tab in the tab strip opens a new untitled tab, as in other editors (#151). Same command as File › New Tab; localized tooltip and accessible name; reachable in the tab-strip focus order.
- (2026-11-27) Gaps from the competitor feature map (PRODUCT.md, Notepad++, Sublime Text, Windows Notepad), by priority:
  - Done (2026-11-27): hot exit — quit without prompts, unsaved and untitled tabs come back on next launch (completes E06; opt-out in Settings › General). Instances sharing the recovery folder leave each other's live snapshots alone (owner files).
  - Partly done (2026-11-27): measured on macOS (scripts/measure-startup.py, BENCHMARKS.md). Warm start p95 150 ms (budget 250) and 100 MB open 0.6 s (budget 2 s) met; fixed the editor-font scan that took RSS to ~530 MB at start and a 20–35 ms first-frame stall. Still missed: idle RSS 108 MiB (budget 100; Text³'s own heap ~3 MB, rest AppKit/Metal), cold-ish p95 1.05 s (first-launch OS check of a new binary; true cold start unmeasured). Open: Windows/Linux runs, typing latency, a decision on the RSS budget versus platform overhead before any "lightweight" claim.
  - Done (2026-11-27): Goto Anything — Cmd/Ctrl+P fuzzy quick open of open tabs, recent files and the project folder (Find in Folder folder, else the active file's folder; background listing, 50k files, fixed ignore list, no `.gitignore`), with `file:line`, `:line`, `@symbol/heading` (Tree-sitter, 15 languages + Markdown) and `#text` jumps; Cmd/Ctrl+Enter opens in the other pane. Not yet: preview on highlight, `file@symbol`, `.gitignore`.
  - Done (2026-11-27): status-bar character and word count for the document and the selection (grapheme characters, UAX #29 words, summary popover with lines and bytes; counted in the background, files over 8 MB on demand; Settings › Editor to hide).
  - Should: folder sidebar (file tree of the opened folder, hidden by default per UX).
  - Should: Column Editor — insert text or a number sequence down a column/multi-cursor.
  - Should: spell check for prose (offline system or bundled dictionaries, off for code).
  - Could: line bookmarks (toggle, next/previous, list).
  - Could: minimap / document map.
  - Could: distraction-free (full-screen, chrome hidden) mode.
  - Could: more highlighted languages (Notepad++ ships about 90; Text³ 22).
  - Could: Markdown formatting commands (bold, italic, heading, list) in source view.
  - Later (M8, separate ADR): extensions API (E16) and optional local/BYOK writing tools.

## Cross-cutting gates
M0: name clearance/competition review, brand approval, i18n spike, repository governance approval. M1 onward: localized UI keys and regression tests on every slice. M6: launch locale review, GitHub ruleset verification, security and release dry run. M7: owner-approved name, license, actual funding accounts and verified release artifacts.
