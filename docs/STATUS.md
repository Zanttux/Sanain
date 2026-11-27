# Status

Updated 2026-11-27. Version 0.2.1 (pre-alpha, M2 done; tagged v0.2.1). Unsigned `nightly`
pre-release published automatically from main (D23).
M0 complete except trademark search. M1 and M2 done; next
are cross-platform verification and release readiness.

## Summary for the owner
- 0.1.0 published as an unsigned pre-release (installers and portable
  builds for Windows, macOS and Linux); nightly builds follow every
  change; website at https://zanttux.github.io/text3/.
- 102 pull requests merged; every one passed CI on Linux, Windows and
  macOS plus the dependency check. 247 automated tests pass. Reviews by
  fresh Claude agents and by Codex found and fixed several bugs (#74,
  #118).
- Text³ is a usable everyday editor on macOS: tabs and split view (with
  Markdown preview and data views in either pane), multiple cursors,
  rectangular selection, macros, word wrap, syntax-aware folding,
  find/replace with preview, Replace in Files, Markdown preview with an
  outline, data views (JSON/XML/YAML/CSV), comparison, local history,
  crash recovery, a log viewer for huge files, themes, Finder/Explorer/
  Linux "Open With", 22 highlighted languages, 26 UI languages (24
  machine-translated, accepted for now per D24).
- Robustness checked by stress tests: 147 MB file, 2.7 MB single line,
  full disk, missing permissions, random binary data, deeply nested JSON.
- Not verified yet by a person: Windows and Linux at runtime (owner
  testing Windows now: #95 flicker, re-run installer for #115), screen
  readers and IME input.
- Waiting on the owner: Windows test results (#95), the GitHub Sponsors
  page going public (#93), code-signing certificates and store accounts
  (#79).

## Done
- Mouse selection (#112-#114): drags start at the press point; double-click
  selects the word, whitespace or punctuation run under the pointer,
  triple-click the line; right-click menu with Cut/Copy/Paste/Select All.
- Environment: macOS 27.2.1 arm64 (Apple M5), Rust 1.100.0 via rustup,
  git 2.54, gh 2.101 authenticated.
- Repository: github.com/Zanttux/text3 (public, MPL-2.0). Commits use the
  owner's GitHub noreply address.
- Workspace at 0.0.1: `text3-core`, `text3-io`, `text3-i18n`.
  First vertical slice (open UTF-8 file → edit → undo → atomic save →
  reopen → bytes verified) passes as an automated test.
- Checks on this machine (2026-11-26): `cargo fmt --check` clean;
  `cargo clippy --workspace --all-targets --locked -D warnings` clean;
  `cargo test --workspace --locked` 233 passed, 0 failed; `cargo deny
  --locked check` clean.
- Multi-cursor selections and grapheme-aware editing commands.
- First egui editor window (`cargo run -p text3-app -- file`). Verified
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
| E01 | Partial | Round-trip and conflict tests in `crates/text3-io/tests/file_lifecycle.rs` |
| E03 | Partial | Undo/redo, grouped transactions, property test, indent/outdent of selected lines (Tab/Shift+Tab), toggle line comment, multi-cursor editing created by Alt+click, Add Cursor Above/Below (keyboard) and Alt+drag rectangular selection; Esc collapses. Select Next Occurrence (Cmd/Ctrl+D) and Skip to Next Occurrence (Cmd/Ctrl+Shift+D). Keyboard rectangles use Ctrl+Alt+Shift+Arrow (Control+Option+Shift on macOS), preserving Shift+Alt line duplication; virtual space past line ends remains unimplemented. |
| E12 | Partial | Grapheme-cluster navigation and deletion; editor exposes the caret line, caret and selection to AccessKit (checked via the macOS accessibility API: AXValue and AXSelectedText follow the caret); high-contrast appearance; every command reachable by keyboard (menus, palette). Newer views (2026-11-27, checked via the macOS accessibility API, see ACCEPTANCE): CSV grid as AXTable with named cells and the focused cell following the selection; JSON/XML/YAML trees as AXOutline rows with name, value, level and expanded state; named split panes; Mermaid diagrams as images with a summary and the source; named prompts, shortcut buttons and search field; live status and import messages; Shift+F10/Menu opens grid and tree menus; focus returns after menus, inline editors and palette commands. Focus order (2026-11-27, AX API and keyboard only, see ACCEPTANCE): F6/Shift+F6 cycle tab strip → find bar → outline → each pane → folder search → status bar; Tab/Shift+Tab follow the same order at region edges (the editor keeps Tab for indentation; F6, Escape or Ctrl+Tab leave it); Escape from the tab strip or status bar returns to the document; tabs and Settings sections are tab lists with selected state, arrow keys move between Settings sections; status-bar encoding, line ending and indentation are buttons. Not yet: tested with VoiceOver speech by a person, NVDA/Orca, IME composition by a person; focus order checked on macOS only; preview paragraphs with links and the find/folder search fields have no accessible names; row/column indices, level and expanded state reach macOS only through the names (AccessKit's macOS adapter does not map them) |
| E04 | Partial | Find/replace in the current tab: literal, regex, case, whole word, highlight, count, replace all as one undo. Replace All shows a preview (count, changed lines before/after, first 200) and applies only on confirmation, refusing if the document changed meanwhile. Search across files: see E15. Searches in documents over 1 M chars run in the background and are cancelled by newer ones (147 MB file stays responsive). |
| E08 | Partial | Source/split/preview, GFM blocks and inlines, front matter, caret-following preview, raw HTML inert, source bytes unchanged (hash checked). Links: web with confirmation, local files, anchors, unsafe schemes blocked. Local images inside the document folder (limits enforced; remote never fetched). Go to Heading (filterable heading list). Fenced code blocks coloured by language (22 languages and common aliases, blocks up to 64 KB, parsed once per block). Outline side panel (View › Outline: heading tree, current section highlighted, click to jump). Images outside the document folder are shown only after the user allows that folder (per session, button next to the image). Mermaid: flowchart/graph subset drawn natively in the preview (layered layout, theme colours, scaled to fit; no JS, network or link opening; D26), including subgraphs nested up to 3 deep (titled frames that never overlap non-members, tested on generated charts), links to subgraph frames, `<-->`/`--o`/`--x` ends and hexagon, cylinder, parallelogram, trapezoid and double-circle nodes; `sequenceDiagram` with participants/actors, eight message arrows, self-messages, notes, autonumber, activations and loop/alt/opt/par/critical/break/rect frames. `stateDiagram`/`stateDiagram-v2` (states, descriptions, `[*]` start/end per scope, composite states nested 3 deep as frames, choice/fork/join, notes left/right incl. multi-line, `--` concurrent regions as dashed frames, `direction`) and `classDiagram` (attribute/method compartments, static/abstract, generics `~T~` → `<T>`, annotations, all eight relationship kinds with ends on either side, cardinalities, labels, namespace frames, notes) laid out with the flowchart layered layout. Other diagram types and unsupported syntax show the source with a note. Not yet: gantt, pie, ER, journey and other Mermaid diagram types; notes are placed by the layout (a layer before/after their state), not strictly beside it |
| E10 | Partial | Two-tab side-by-side line diff (time-bounded), apply hunk in either direction as one undo, stale-revision check. Compare with the saved file, the clipboard or any file chosen in a file dialog (read-only right side; encoding detected as on open). Intraline highlight of changed characters (pairs with at least 40 % in common, lines up to 2 000 chars). Ignore whitespace option (lines differing only in spaces, tabs or line endings count as equal; applying a hunk still copies exact bytes). From the command line: `text3 --diff LEFT RIGHT` opens both and compares them (both sides editable) |
| E15 | Partial | Search across a folder (background, cancellable, bounded), results open at the match. Keyboard macros: record/stop, play at the cursor, each run one undo step (grouped undo in core); mouse placement not recorded. Play Macro Multiple Times (1–10 000 runs, one undo step). Replace in Files from the folder search results: confirmation first; open files are changed in their tab (undoable, not saved); other files are reloaded, their old bytes copied to local history, saved with the external-change check, encoding/BOM/line endings kept; disabled when results hit a limit. Saved, named macros: Tools → Save Macro… names the last recording (same name replaces), Play Saved Macro… and Delete Saved Macro… pick by name from a filtered list (also in the command palette); playing is one undo step. Stored in `macros` beside the settings (versioned line format, `TEXT3_CONFIG_DIR`/portable mode honoured, atomic write); malformed macros are skipped, caps of 100 macros, 10 000 steps and 1 MiB. |
| E13 | Partial | Log viewer: background indexing and filtering (cancellable), follow/tail, truncation detection, auto-open at 256 MB, level colours, Go to Line. Encoding detected from the first 64 KiB: UTF-16 LE/BE (BOM or zero-byte pattern; line starts indexed per 2-byte unit), UTF-8, or an ASCII-compatible legacy encoding (shown in the header; display and filter decode with it). Tested on macOS with a 302 MB file, a windows-1251 log and a UTF-16 LE log |
| E14 | Done | Local history: copy per save (50 versions / 30 days / 10 MB cap, owner-only files), list, open, restore as one undo step, disable, purge. Tested on macOS |
| L10n | Partial | 26 UI languages (en-US canonical; fi-FI draft; 24 machine translations by Claude awaiting native review). Every message formats in every locale (test). RTL languages not shipped |
| E05 | Partial | UTF-8, UTF-8 BOM, UTF-16 LE/BE BOM, 17 legacy encodings by explicit choice (exact round trip enforced), explicit encoding and LF/CRLF conversion. UTF-16 LE/BE without BOM detected (zero-byte pattern, exact decode) and opened directly; legacy encodings suggested by chardetng in the picker (user confirms, exact round trip still required). Files over 16 MB: guessed from the first 64 KiB, the open still verifies the whole file. ASCII-only UTF-16 without a BOM (which is also valid UTF-8 with NULs) is recognised on open |
| E09 | Partial | JSON validate/format/minify preserving lexemes; editable JSON tree (inline value edit, key rename, add after/inside, delete; each one minimal undoable edit from exact char spans that keeps formatting, CRLF, escapes and number lexemes, result re-validated; off for read-only files); editable XML tree (inline attribute value, element text (CDATA kept when possible) and element/attribute rename (XML Name rules, start and end tag in one undo step), add attribute/child/sibling, delete element or attribute; each change touches only that node's characters, keeps quotes, entity and character references, comments, declaration, CRLF and indentation, copies sibling layout, and is refused unless the result parses (as strictly as the source did); entities never expanded; off for read-only files); editable YAML tree (inline scalar value edit, key rename, add after/inside (`key: null` / `- null` at the same indentation and line ending), delete block entries/items with their lines; spans from saphyr-parser markers plus a quote scanner, each change touches only that node's characters and is refused unless re-parsing gives exactly the expected tree; quote style kept, strings stay strings (YAML 1.1 booleans/null/numbers quoted), comments/CRLF/indentation kept; flow collections, anchored/tagged scalars, aliases, block scalars, complex keys and later documents read-only with the reason shown; property-tested on random block documents; off for read-only files; no alias expansion, node cap, iterative builders; tested against billion-laughs inputs); CSV/TSV grid with delimiter detection and inline cell editing (each edit replaces only that field's source characters as one undo step; insert/delete row; quoting kept or added only when needed; off for read-only files and unterminated quotes). Validate XML/YAML (well-formedness, caret to the error). Format XML (re-indents element-only content; mixed content, CDATA, comments and tags kept byte for byte). Not yet: YAML formatting, editing YAML flow collections, block scalars, anchors/tags and later documents in the tree, schema validation, moving JSON/XML tree nodes or replacing a whole object/array in place, editing XML comments and processing instructions, namespace-prefix checks, CSV column insert/delete |
| E02 | Partial | Tabs, close/reopen, keyboard switching, dirty markers, per-tab recovery, drag reorder, pin (kept in front, saved in session), tab context menu. Split editor: up to four panes (Split Right / Split Down; the first split sets the direction, a split the other way nests one level inside the focused column or row), Close Pane, Focus Next/Previous Pane; focus follows interaction, the same file may be shown in several panes (shared caret). Draggable dividers (min 120 pt where it fits, double-click resets that level to even, sizes renormalised on add/close); the split layout (direction, nesting, file per pane, focus, sizes) is saved in the session and restored, dropping panes whose file did not reopen. Move Tab Left/Right (Ctrl+Shift+Page Up/Down, Window menu; pinned and unpinned tabs stay in their groups). Each pane shows its tab's Markdown preview/structured view as the single area does. Not yet: deeper nesting |
| E07 | Partial | Tree-sitter highlighting for 22 languages (PHP with the HTML around it, Lua, PowerShell and Markdown source with headings, inline code and fenced code in its language added; SQL and Swift left out, their grammars add about 13 MB and 4 MB to the binary), background worker with cancellation and incremental updates (the tree is re-parsed where an edit touched it; embedded languages — HTML scripts and styles, Markdown inline text and fences, the HTML around PHP, JavaScript tagged templates — are kept as layer trees, injections are looked for again only where the enclosing tree changed and a layer is re-parsed only when an edit reaches it; only the changed lines are re-highlighted, merged over the layers like `tree_sitter_highlight`; a fresh analysis settles the result after a 300 ms pause and the incremental state is kept when they agree; 500 KB Markdown 15 ms, HTML 3 ms, PHP 6 ms per keystroke), line numbers, syntax-aware folding from the Tree-sitter tree (indentation folding as fallback), keyboard navigation, light/dark/system/high-contrast themes, zoom. Not yet: incremental folds (collected from the whole tree, about 4 ms for 500 KB Markdown), combined injections (the HTML of a PHP file) re-parsed as one layer on each edit, SQL, Swift, Kotlin (grammar crates too large or without highlight queries) |
| E11 | Done for v1 scope | Menus with standard shortcuts (logical keys on macOS), command palette with fuzzy search, configurable shortcuts with conflict handling, Notepad++ preset, import of a Notepad++ shortcuts.xml (31 menu commands and 9 Scintilla commands mapped; macros, run and plugin commands skipped). Import parsing tested with sample files; Windows file dialog not hand-checked |
| E06 | Partial | Unsaved-changes prompts; crash recovery per tab (kill -9 → relaunch restores text); session restore of open files and active tab. Disk-full and permission-denied saves verified on macOS (file unchanged, text kept, plain error; see ACCEPTANCE). Session restore can be turned off in Settings. Automated disk-full drill (`crates/text3-app/tests/disk_full.rs`, gated by `TEXT3_FULL_DIR`) passes locally on macOS (2 MB HFS+ image, 2026-11-26) and runs in CI on a 4 MiB tmpfs (Linux), 2 MiB HFS+ image (macOS) and 8 MiB NTFS VHD via diskpart (Windows); it passed on all three in CI on PR #141. It found and fixed: a failed in-place save of a hard-linked file left the file truncated (now restored in place, backup removed) and a half-written `macros.N.bak` |
| Others | Not started | |

Keyboard layouts (checked 2026-11-26, Finnish layout, macOS): native
menu shortcuts on punctuation keys are localised by macOS and work as
shown in the menu (Toggle Line Comment Cmd+´, Indent Cmd+Ä, Outdent
Cmd+Ö); Cmd+Shift+7 does not reach the window at all. Not verified: the
same shortcuts with in-window menus on Windows/Linux non-US layouts (they
match egui's logical key; may need per-layout defaults or rebinding in
Settings).

Known M1 limits: native menus only on macOS (Windows/Linux use in-window
menus); running the bare binary on macOS shows "text3" in the menu bar,
use `scripts/macos-bundle.sh` for the proper app name;
word wrap only for documents up to 1 M characters (full rebuild per
edit); visible lines are laid
out each frame (egui caches identical layouts; unwrapped lines over 4 096
characters are laid out only around the visible columns); dialog button
order follows macOS on all platforms; IME composition and VoiceOver
reading of document text not yet verified by a person.

Windows and Linux: automated tests and builds only (CI); the GUI has not
been run on them.

## Name availability (2026-11-25, automated lookups; historical, name was Sanain)
crates.io `sanain*`: free. GitHub user `sanain`: taken (repository must
live under the owner account or another org name). RDAP: sanain.com
registered; .org, .dev, .app, .io returned no registration. Trademark
registers (EUIPO, PRH, USPTO, WIPO) not searched: needs owner or counsel.

## Rebrand to Text³ (2026-11-27)
Owner approved the rename to Text³ (handoff package, D27) and the
repository rename to Zanttux/text3. Branch `rebrand/text3`: assets,
crate rename, branding, data-folder migration with Sanain compatibility,
installer/DMG/website. Report, tests and rollback: docs/REBRAND.md.

## Owner decisions (2026-11-25)
Approved: MPL-2.0, egui/eframe, public repository with delegated routine
GitHub operations, single-colour wordmark and icon as working assets.
Standing order: no questions to the owner (D19).

## Owner-only items (never block work)
- Automatic installation of updates needs code-signing certificates
  (Apple Developer ID, Windows Authenticode) and a key for signing update
  manifests. Only the check-and-notify part is implemented.
- Trademark and domain clearance for "Text³"/"text3" (EUIPO, PRH, USPTO,
  WIPO) before any launch announcement (the Sanain search is moot after
  the rename, D27).
- Text³ rebrand cutover (docs/REBRAND.md): rename the repository to
  Zanttux/text3 (approved 2026-11-27), then Pages, Actions and release
  checks; final visual sign-off of the traced artwork and icons.

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
- Smaller: incremental word wrap (lift the 1 M-character limit), more Mermaid diagram types (class, state) in Markdown preview.
