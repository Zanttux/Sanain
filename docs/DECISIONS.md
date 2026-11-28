# Decision register
| ID | Status | Decision |
|---|---|---|
| D01 | Confirmed | Rust implementation |
| D02 | Confirmed | Full FOSS, all features free, sponsorship |
| D03 | Confirmed | First-class Markdown |
| D04 | Superseded by D27 | Sanain display name; sanain technical slug and CLI; legal clearance pending |
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
| D15 | Superseded by D27 | Graphite/mint brand, single-colour wordmark and icon approved as working assets 2026-11-25; trademark clearance pending; en-US and fi-FI locales |
| D16 | Confirmed | Start at 0.0.1; Semantic Versioning; no 1.0.0 before M7 gates (docs/RELEASE.md) |
| D18 | Confirmed | Public repository github.com/Zanttux/Sanain (renamed to github.com/Zanttux/text3 under D27); routine GitHub operations delegated per AUTONOMY (owner approved 2026-11-25) |
| D19 | Confirmed | Owner standing order (2026-11-25): Claude does not pause to ask questions. It decides, records the decision here or in STATUS, and continues. Owner-only physical tasks are listed in STATUS. |
| D20 | Superseded by D27 | Owner (2026-11-25): the name is "Sanain" everywhere people see it — app menu, window titles, bundle, repository (github.com/Zanttux/Sanain), docs. Lowercase `sanain` only where tooling requires it: crate names, the CLI executable, config/data directory names on Linux. |
| D21 | Accepted | Soft wrap on by default (2026-11-25, Claude): Sanain targets everyday text and Markdown as much as code, and Windows Notepad, TextEdit and most prose editors wrap by default. View > Word Wrap and Settings turn it off. Wrapping is computed by `text3_core::wrap` (word boundaries, monospace columns) so rows, hit testing and carets agree; documents over 1 M characters are not wrapped until wrapping is incremental (a full rebuild takes ~4.5 ms per 1 M chars on Apple M5, release). |
| D22 | Accepted | Dependency policy (2026-11-26, Claude): only permissive licenses compatible with MPL-2.0 distribution (allow-list in deny.toml: MIT, Apache-2.0, BSD, ISC, Zlib, BSL-1.0, Unlicense, Unicode, CDLA-Permissive-2.0, OFL-1.1/Ubuntu font for egui fonts), crates.io only, no wildcard versions, RustSec vulnerabilities block. Adding a license to the list needs an entry here. |
| D23 | Confirmed | Owner (2026-11-26): release builds are made automatically and as soon as possible. CI publishes unsigned pre-releases: every push to main replaces the `nightly` pre-release, a `v*` tag creates a pre-release for that version (Package workflow, separate `contents: write` publish job, pushes only). Signed, notarized and store builds wait for certificates and accounts from the owner. The in-app update check ignores pre-releases. |
| D24 | Confirmed | Owner (2026-11-26): machine translations are accepted automatically for now; merging does not wait for native review. They stay labelled as machine translations (never presented as human-verified) and corrections from speakers are welcome. |
| D25 | Confirmed | Owner (2026-11-26): every piece of owner feedback or feature request is recorded as a GitHub issue before it is worked on; the fixing PR closes it. |
| D26 | Accepted | Mermaid (2026-11-26, Claude): Markdown preview draws a safe Mermaid subset natively in Rust (parser and layered layout in `text3-markdown::mermaid`, egui painting). No bundled mermaid.js, JS engine or web view: they would break the offline/no-execution rule (docs/MARKDOWN.md) and add large dependencies. Unsupported diagrams show their source. Reversal: an isolated, offline renderer could replace it if one meets the same rules. |
| D27 | Confirmed | Owner (2026-11-27): the product is renamed **Text³** (spoken "Text Cubed"; supersedes the name in D04, D15 and D20). Visible name Text³ everywhere users see it; ASCII `text3` for crates, the CLI, paths, package ids and URLs; repository rename to github.com/Zanttux/text3 approved (cutover by the owner session; Pages moves to zanttux.github.io/text3/). The approved lynx artwork (`assets/brand/approved-reference.png`) is the visual authority; no slogans. Compatibility: the old Sanain data folders are copied (not moved) on first start, `SANAIN_*` variables, the `sanain-portable` marker and a `sanain` command symlink (Linux) keep working, `--register`/`--unregister` remove Sanain's registrations, the Inno Setup AppId and the macOS bundle id `io.github.zanttux.sanain` stay so upgrades work, on-disk formats unchanged. Historical changelog entries and tags keep the old name. Trademark/domain clearance and final visual sign-off remain with the owner. Details and rollback: docs/REBRAND.md. |
| D28 | Confirmed | Owner (2026-11-28): release builds get signed. macOS: Developer ID signing and notarization through the owner's Apple Developer account. Windows: signing through a free open-source signing programme (SignPath Foundation first). Linux unsigned with checksums. Secrets held only by the owner, used only on main and tags, never on pull requests (#174). |
| D29 | Accepted | Default apps (2026-11-28, Claude, #164): Text³ offers once on first start (and in Settings › General) to become the default app for five groups of file types, web pages unticked. macOS sets it through Launch Services, Linux through `xdg-mime`; Windows only registers Text³ as a capable handler and opens Settings › Default apps, because apps may not set the protected user choice. Portable copies and macOS copies outside Text³.app do not offer it. |
| D30 | Confirmed | Owner (2026-11-28): every UI/UX change is first analysed by an agent acting as a UI designer; the UI must be the best possible experience, follow current standards and not bloat or lose its system. Claude added further automatic gates (performance and lightness as the main thread, accessibility, data safety, dependencies, security, localization, review, scope): docs/QUALITY_GATES.md. |
| D17 | Accepted | Workspace layout and first slice (docs/adr/0002-workspace-and-first-slice.md) |

ADRs live in docs/adr/. Record ADR date, context, alternatives, evidence, consequences and reversal plan. Claude may decide routine implementation details; reserved decisions follow AUTONOMY. Proposed license and external authority require explicit owner approval.
