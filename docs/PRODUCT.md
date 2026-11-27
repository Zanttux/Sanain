# Product requirements
## Mission
Text³ is a product, not a separate research program. One editor for everyday text, code, configuration, Markdown and logs. File-first, not project-first. Plain text opens immediately; richer views are optional and reversible. No IDE, notebook database or collaboration platform in v1.

## Users
Windows power users, developers switching platforms, sysadmins, technical writers and casual scratchpad users. Core jobs: open/edit/save, recover scratch notes, search many files, inspect structured text, read Markdown, compare files.

## Requirements and acceptance
| ID | Priority | Requirement / acceptance |
|---|---|---|
| E01 | P0 | New/open/save/save as; round-trip unchanged supported files byte-for-byte |
| E02 | P0 | Tabs, reorder, pin, close/reopen, split editor; dirty indicators |
| E03 | P0 | Undo/redo, multi-cursor, rectangular selection, clipboard, indent, line operations |
| E04 | P0 | Literal/regex search, replace, whole word, case, preview replace-all, cancel |
| E05 | P0 | UTF-8/BOM, UTF-16 LE/BE, common legacy encodings; explicit conversion; preserve LF/CRLF and mixed endings |
| E06 | P0 | Session restore including unnamed buffers; crash recovery and disk-full handling |
| E07 | P0 | Syntax highlighting, line numbers, folding, themes, zoom, keyboard navigation |
| E08 | P0 | Markdown source/split/preview, GFM, Mermaid offline, outline |
| E09 | P0 | JSON validate/format/minify, tree view; XML/YAML safe inspection; CSV grid |
| E10 | P0 | Side-by-side diff, apply selected changes as undoable edits |
| E11 | P0 | Command palette, configurable shortcuts, familiar Notepad++ preset |
| E12 | P0 | Accessible menus, focus, high contrast, IME, Unicode graphemes |
| E13 | P1 | Large log viewer, tail, filters, cancellation |
| E14 | P1 | Local history with retention and explicit purge |
| E15 | P1 | Macro recording, replay, search across directories |
| E16 | P2 | Optional local/BYOK AI, WASM extensions, hex viewer, rich Markdown editing |

P0 means stable v1 scope, not first prototype. P1 follows stable unless human reprioritizes. All features remain free. Default UI has tabs, editor, menu, status bar; no permanent AI/sidebar clutter. Settings include restore behavior, privacy, font, tab width, encoding, theme, keymap. No destructive operation without preview/confirmation where appropriate.

## Competitor feature map (2026-11)
Checked 2026-11-27. Text³ status verified against STATUS.md, `locales/en-US/main.ftl` and `crates/text3-app/src`: **Has**, **Partial**, **Missing**. Ranks are judgement from the cited evidence, not a measured poll; review-site pages (G2, Capterra) block automated reading and were seen only through search summaries.

**Choice of the three.** Stack Overflow Developer Survey, "IDE" question, all respondents: 2025 — VS Code 75.9 %, Visual Studio 29 %, Notepad++ 27.4 %, IntelliJ 27.1 %, Vim 24.3 %, Neovim 14 %, Nano 12.2 %, Sublime Text 10.5 % [S1]; 2026 — VS Code 70.9 %, Notepad++ 27.8 % [S2]. Picked: **Notepad++** (most-used dedicated text editor among developers), **Windows Notepad** (bundled with Windows, about 1.4 billion monthly active devices [S3]; not in the survey, so its rank rests on install base, not preference — weak evidence for "most liked"), **Sublime Text** (highest-ranked GUI notepad-style editor after Notepad++). Excluded: VS Code and Visual Studio (IDE/extension platforms, not notepad-style; Sanain's mission excludes an IDE), Vim/Neovim/Nano (terminal editors), Notepad3, Kate, gedit/GNOME Text Editor, BBEdit, CotEditor (not in the survey's top list; no comparable usage figures found).

### Notepad++
| # | Liked feature (evidence) | Text³ |
|---|---|---|
| 1 | Lightweight, fast, free and open source [S4][S5] | Partial — free/FOSS; macOS warm start 135 ms median, idle RSS 108 MiB (budget 100), Windows/Linux unmeasured (BENCHMARKS) |
| 2 | Syntax highlighting and folding for ~90 languages [S4] | Partial — 22 languages, folding |
| 3 | Regex find/replace, Find in Files, Mark All [S4] | Has (no line bookmarks) |
| 4 | Tabs [S5] | Has |
| 5 | Unsaved tabs survive a restart without prompting (session snapshot, periodic backup) [S6] | Partial — crash recovery and file session restore; quit still asks per unsaved tab, untitled tabs not kept |
| 6 | Plugin ecosystem (Plugins Admin; NppExec ~22 k, JSON Viewer ~21 k Softpedia downloads) [S7][S8] | Missing (WASM extensions are E16 P2) |
| 7 | Popular plugins' jobs: Compare, JSON Viewer, XML Tools [S7][S9] | Has, built in |
| 8 | Column mode, multi-caret, Column Editor (insert text/numbers down a column) [S9] | Partial — no Column Editor / number sequence |
| 9 | Macro record and playback [S4] | Has |
| 10 | Encoding detection and conversion [S5] | Has |

### Sublime Text
| # | Liked feature (evidence) | Text³ |
|---|---|---|
| 1 | Instant startup, low resource use [S10][S11] | Partial — macOS measured (warm p95 150 ms; idle RSS 108 MiB misses the 100 MiB budget); Windows unmeasured |
| 2 | Large files with highlighting and folding [S10][S11] | Partial — 147 MB file edits (571 MB RSS); log viewer for any size |
| 3 | Multiple selections, Ctrl+D rename [S12][S11] | Has |
| 4 | Goto Anything (Ctrl+P: fuzzy file, @symbol, :line) [S12] | Yes — Cmd/Ctrl+P: open tabs, recent files and the project folder; `:line`, `file:line`, `@symbol`, `#text`; no preview on highlight |
| 5 | Command palette [S12] | Has |
| 6 | Packages (Package Control 25.6 M installs; Emmet 6.2 M, LSP) [S13][S10] | Missing |
| 7 | Hot exit: quit without prompts, unsaved buffers come back [S12] | Partial (as Notepad++ #5) |
| 8 | Customisable keybindings, settings and colour schemes [S10] | Has (shortcuts, Notepad++ import, user themes) |
| 9 | Split editing, tab multi-select [S12] | Has |
| 10 | Folder sidebar (SideBarEnhancements 3.0 M installs) [S13] | Missing (only Find in Folder panel) |

### Windows Notepad
| # | Liked feature (evidence) | Text³ |
|---|---|---|
| 1 | Always there, opens instantly, plain text only [S14][S15] | Partial — plain text default; not preinstalled; Windows startup unmeasured |
| 2 | Tabs [S16] | Has |
| 3 | Session restore incl. unsaved text, autosave [S16][S17] | Partial (as Notepad++ #5) |
| 4 | Dark theme following the system [S17] | Has |
| 5 | Spell check and autocorrect [S16] | Missing |
| 6 | Character count in the status bar (selection and document) [S18] | Missing — status bar shows Ln/Col and selection count |
| 7 | Find/replace, word wrap [S17] | Has |
| 8 | Zoom and font choice [S16] | Has |
| 9 | Light Markdown formatting (bold, headings; 25H2) [S16] | Partial — Markdown preview, no formatting commands |
| 10 | AI rewrite/summarise (divisive; backlash for bloat) [S14][S15] | Missing by design (optional local/BYOK only, E16) |

### Gaps
Shared by all three: no-prompt hot exit with untitled tabs kept (E06 already requires it). Then: fuzzy Goto Anything; status-bar character/word count; spell check (prose users); folder sidebar; Column Editor; minimap and bookmarks (Notepad++/Sublime staples, below the top 10); extensions stay P2. Startup/memory are measured on macOS only and idle RSS still misses its budget, so no "lightweight" claim yet (TESTING budgets, BENCHMARKS). Sanain already beats all three out of the box on built-in compare, data trees, Markdown/Mermaid and byte-exact saving.

Sources (accessed 2026-11-27): [S1] https://survey.stackoverflow.co/2025/technology/ (2025-07) · [S2] https://visualstudiomagazine.com/articles/2026/10/06/stack-overflow-2026-survey-vs-code-leads-visual-studio-takes-third.aspx (2026-10-06, not read directly: 403) · [S3] https://www.windowslatest.com/2025/07/01/microsoft-confirms-windows-11-did-not-lose-400-million-monthly-active-pcs (2025-07-01) · [S4] https://www.g2.com/products/notepad/reviews (2,467 reviews, via search summary) · [S5] https://opensource.com/article/16/12/notepad-text-editor (2016) · [S6] https://community.notepad-plus-plus.org/post/32142 · [S7] https://www.softpedia.com/downloadTag/Notepad%2B%2B+plugin (2026-05) · [S8] https://github.com/notepad-plus-plus/nppPluginList · [S9] https://www.capterra.com/p/185965/Notepad/reviews/ (via search summary) · [S10] https://forum.sublimetext.com/t/anyone-else-still-sticking-with-sublime-in-2026/78491 (2026-05) · [S11] https://dev.to/mindninjax/why-do-you-still-use-sublime-1o7l · [S12] https://www.sublimetext.com/ (multiple selections, tab multi-select), https://www.lynda.com/Sublime-Text-tutorials/Using-Goto-Anything-command/513594/566119-4.html (Goto Anything "favorite feature"), https://forum.sublimetext.com/t/hot-exit-as-default/1534 (hot exit) · [S13] https://packagecontrol.io/browse/popular (2026-11-27) · [S14] https://www.theregister.com/2025/05/23/microsoft_ai_notepad (2025-05-23) · [S15] https://www.windowscentral.com/software-apps/microsoft-adds-tables-and-more-ai-to-notepad-what-happened-to-the-app-we-loved (2025-11) · [S16] https://www.thurrott.com/books/windows-11-field-guide/334641/notepad-25h2 (2026-04-07) · [S17] https://www.ghacks.net/?p=201520 · [S18] https://www.bleepingcomputer.com/news/microsoft/windows-11-notepad-gets-a-built-in-character-counter-finally/ (2023-12)
