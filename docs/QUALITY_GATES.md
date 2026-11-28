# Quality gates (applied automatically)

Owner rule (2026-11-28, D30). Every change is checked against the gates
that apply to it, before it is merged. A pull request says which gates
applied and how each was met; a gate that could not be run is named as
not run, never as passed. Lightness and speed are the main thread of
development: when gates conflict, the lighter and faster option wins
unless an owner decision says otherwise.

## G1 UI/UX designer review (any visible change)
Applies to: windows, dialogs, menus, settings, status bar, palette,
themes, icons, strings users read, keyboard shortcuts, the website.
- Before implementing, an agent acting as a UI designer (Claude subagent
  or Codex, whichever fits the task) reviews the proposal against
  docs/UX.md and the platform guidelines (macOS HIG, Windows Fluent,
  GNOME HIG). Its findings are fixed or answered in the PR.
- No bloat: a new control, setting, menu item or panel must replace
  something, be reachable from an existing place, or be justified in the
  PR. Prefer a good default over a new option.
- Systematic: reuse existing components, spacing and typography tokens,
  wording patterns ("…" only when more input follows, sentence case) and
  dialog layout; no one-off styles.
- Shown in the PR: before/after screenshots or, when the screen cannot
  be captured, the headless test that shows the result.

## G2 Performance and lightness
Applies to: start-up path, rendering, editing, file loading, memory,
dependencies, bundled assets.
- Budgets in docs/BENCHMARKS.md: warm start ≤ 250 ms, 100 MB file ≤ 2 s,
  idle memory, binary and download size.
- A change on these paths is measured (scripts/measure-startup.py,
  scripts/bench/) against main. A regression above 5 % (or above a
  budget) needs a decision in the PR and an issue to win it back.
- Work that is not needed for the first frame stays off the first frame
  (background threads, lazy loading, files beside the binary instead of
  compiled-in assets).

## G3 Accessibility
Every new control has an AccessKit role and name, is reachable and usable
from the keyboard in a sensible focus order, shows focus, keeps contrast
in all themes, and has a headless test that finds it by its name.

## G4 Data safety
Changes touching open, save, encodings, line endings, recovery, history
or external-change detection keep bytes identical unless the user changes
them, and come with regression tests. Checks are never weakened to pass.

## G5 Dependencies
A new crate or asset needs: licence compatible with MPL-2.0 (D22,
deny.toml), maintained source, its size and start-up cost, and the
alternative of not adding it. Bundled fonts and media carry their licence.

## G6 Security
Programs are started by full path; untrusted input (files, network,
links, pasted content) is size-limited and never executed; no secrets in
code or logs; signing secrets only on main and tags, never on pull
requests. Changes to these areas get a security review agent pass.

## G7 Localization
Every user-visible string is in all locales in the same PR (machine
translations accepted, D24); layouts tolerate 40 % longer text.

## G8 Independent review and checks
fmt, clippy -D warnings and locked workspace tests pass locally; CI
passes on Linux, Windows and macOS before merge. Larger changes get a
review by a fresh agent that sees the code, not the author's conclusion.

## G9 Product scope
A feature must serve everyday text, code, configuration, Markdown or data
editing for most users. Niche features go behind existing surfaces
(palette, menus) rather than new chrome, or stay out.
