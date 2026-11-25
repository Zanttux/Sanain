# Product requirements
## Mission
Sanain is a product, not a separate research program. One editor for everyday text, code, configuration, Markdown and logs. File-first, not project-first. Plain text opens immediately; richer views are optional and reversible. No IDE, notebook database or collaboration platform in v1.

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
