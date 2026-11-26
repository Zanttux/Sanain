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
- Shell integration: "Open with Sanain" in Windows Explorer, Finder Open With / default app, Linux `.desktop` entry with MIME types (#78). Reversible, no elevation by default.
- Paid Mac App Store and Microsoft Store builds as a support model; free builds and all features stay free (#79). Store accounts and pricing are owner decisions.

## Cross-cutting gates
M0: name clearance/competition review, brand approval, i18n spike, repository governance approval. M1 onward: localized UI keys and regression tests on every slice. M6: launch locale review, GitHub ruleset verification, security and release dry run. M7: owner-approved name, license, actual funding accounts and verified release artifacts.
