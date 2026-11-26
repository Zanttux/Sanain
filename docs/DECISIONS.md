# Decision register
| ID | Status | Decision |
|---|---|---|
| D01 | Confirmed | Rust implementation |
| D02 | Confirmed | Full FOSS, all features free, sponsorship |
| D03 | Confirmed | First-class Markdown |
| D04 | Confirmed | Sanain display name; sanain technical slug and CLI; legal clearance pending |
| D05 | Proposed | Windows first validation; macOS/Linux before cross-platform stable claim |
| D06 | Confirmed | MPL-2.0 (owner approved 2026-11-25) |
| D07 | Confirmed | egui/eframe (owner approved 2026-11-25; docs/adr/0001-gui-toolkit.md) |
| D08 | Confirmed | Claude primary operational developer and maintainer; human-led; welcome community contributions and disclose provenance |
| D09 | Proposed | Optional identical $9.99 supporter store distribution |
| D10 | Proposed | No runtime AI initially; optional local/BYOK later |
| D11 | Confirmed | English canonical docs; localization from M0 |
| D12 | Confirmed | Autonomous GitHub/community operations within approved authority |
| D13 | Confirmed | No paid feature gating |
| D14 | Confirmed | Product only; no separate research project, reports or dashboard |
| D15 | Confirmed | Graphite/mint brand, single-colour wordmark and icon approved as working assets 2026-11-25; trademark clearance pending; en-US and fi-FI locales |
| D16 | Confirmed | Start at 0.0.1; Semantic Versioning; no 1.0.0 before M7 gates (docs/RELEASE.md) |
| D18 | Confirmed | Public repository github.com/Zanttux/Sanain; routine GitHub operations delegated per AUTONOMY (owner approved 2026-11-25) |
| D19 | Confirmed | Owner standing order (2026-11-25): Claude does not pause to ask questions. It decides, records the decision here or in STATUS, and continues. Owner-only physical tasks are listed in STATUS. |
| D20 | Confirmed | Owner (2026-11-25): the name is "Sanain" everywhere people see it — app menu, window titles, bundle, repository (github.com/Zanttux/Sanain), docs. Lowercase `sanain` only where tooling requires it: crate names, the CLI executable, config/data directory names on Linux. |
| D21 | Accepted | Soft wrap on by default (2026-11-25, Claude): Sanain targets everyday text and Markdown as much as code, and Windows Notepad, TextEdit and most prose editors wrap by default. View > Word Wrap and Settings turn it off. Wrapping is computed by `sanain_core::wrap` (word boundaries, monospace columns) so rows, hit testing and carets agree; documents over 1 M characters are not wrapped until wrapping is incremental (a full rebuild takes ~4.5 ms per 1 M chars on Apple M5, release). |
| D22 | Accepted | Dependency policy (2026-11-26, Claude): only permissive licenses compatible with MPL-2.0 distribution (allow-list in deny.toml: MIT, Apache-2.0, BSD, ISC, Zlib, BSL-1.0, Unlicense, Unicode, CDLA-Permissive-2.0, OFL-1.1/Ubuntu font for egui fonts), crates.io only, no wildcard versions, RustSec vulnerabilities block. Adding a license to the list needs an entry here. |
| D23 | Confirmed | Owner (2026-11-26): release builds are made automatically and as soon as possible. CI publishes unsigned pre-releases: every push to main replaces the `nightly` pre-release, a `v*` tag creates a pre-release for that version (Package workflow, separate `contents: write` publish job, pushes only). Signed, notarized and store builds wait for certificates and accounts from the owner. The in-app update check ignores pre-releases. |
| D24 | Confirmed | Owner (2026-11-26): machine translations are accepted automatically for now; merging does not wait for native review. They stay labelled as machine translations (never presented as human-verified) and corrections from speakers are welcome. |
| D25 | Confirmed | Owner (2026-11-26): every piece of owner feedback or feature request is recorded as a GitHub issue before it is worked on; the fixing PR closes it. |
| D26 | Accepted | Mermaid (2026-11-26, Claude): Markdown preview draws a safe Mermaid subset natively in Rust (parser and layered layout in `sanain-markdown::mermaid`, egui painting). No bundled mermaid.js, JS engine or web view: they would break the offline/no-execution rule (docs/MARKDOWN.md) and add large dependencies. Unsupported diagrams show their source. Reversal: an isolated, offline renderer could replace it if one meets the same rules. |
| D17 | Accepted | Workspace layout and first slice (docs/adr/0002-workspace-and-first-slice.md) |

ADRs live in docs/adr/. Record ADR date, context, alternatives, evidence, consequences and reversal plan. Claude may decide routine implementation details; reserved decisions follow AUTONOMY. Proposed license and external authority require explicit owner approval.
