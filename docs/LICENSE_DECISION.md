# License gate

Status: decided. MPL-2.0, approved by the owner on 2026-11-25. The
verbatim text from mozilla.org is in LICENSE.

## Reasons
- File-level copyleft: improvements to Sanain's own files stay open, while
  the code can be combined with code under other licenses.
- Compatible with the current dependency set (41 runtime crates, all
  MIT/Apache-2.0/Unicode-3.0/Unlicense as of 2026-11-25) and with both GUI
  candidates (egui MIT/Apache-2.0, GPUI Apache-2.0).
- Permits app-store distribution, which GPL-3.0 makes difficult. The
  optional paid store build (D09) remains possible.
- Explicit patent grant and termination clause.
- Precedent: Helix (Rust editor) uses MPL-2.0.

## Alternatives
- MIT or Apache-2.0: maximum reuse; forks may close their changes.
- GPL-3.0: strongest copyleft; complicates store distribution.

Fonts, grammars, icons, Mermaid renderer and any reused code (Ferrite, Zed)
need separate checks. Do not copy Notepad++ or Zed application code. Name
and trademark rights are separate from the code license.
