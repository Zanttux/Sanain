# Changelog

All notable changes are listed here. Versions follow Semantic Versioning.

## [Unreleased] — 0.0.1

### Added
- Comparison: "Ignore whitespace" option, so indentation and line-ending
  differences no longer hide the real changes.
- Markdown preview: fenced code blocks are coloured by language
  (` ```rust `, ` ```py `, …) using the same highlighting as the editor.
- Packaging script and workflow: unsigned archives for Linux, Windows and
  macOS (Sanain.app) with license, notices and checksums. Nothing is
  published automatically.
- `.editorconfig` files set indentation (indent_style, indent_size,
  tab_width) per file; the status bar shows the indentation in use. Other
  EditorConfig properties are not applied yet.
- Tabs: Close Other Tabs and Close All Tabs (pinned tabs and tabs with
  unsaved changes stay open), Copy File Path, and Show in Finder /
  Explorer / File Manager, in the tab context menu and the File menu.
- Enter keeps the current line's indentation, and the bracket matching
  the one next to the cursor is outlined.
- Indent and outdent lines: Tab with a multi-line selection indents it,
  Shift+Tab outdents; also Edit > Indent/Outdent Lines (Cmd/Ctrl+] and
  [). Edit > Toggle Line Comment (Cmd/Ctrl+/) for languages with line
  comments (// or #), placed at the block's common indentation.
- File > Open Recent (Cmd/Ctrl+Shift+E, the last 20 files), Save All
  (Cmd/Ctrl+Option/Alt+S) and Revert to Saved (asks first when there are
  unsaved changes). Files dropped onto the window open in tabs.
- Log viewer: lines are coloured by level (errors, warnings, debug) and
  Go to Line (Cmd/Ctrl+L) jumps to a line, also when filtering.
- Tools > Validate XML and Validate YAML report whether the document is
  well-formed and put the cursor on the first error.
- About Sanain (app menu on macOS, Help menu elsewhere): version,
  license, how Sanain is made and where the source is.
- Release tooling: cargo-deny policy (deny.toml) and a third-party notices
  generator (scripts/third-party-notices.py), both run in CI.
- UTF-16 files without a byte order mark are recognised and opened
  directly (and saved without one). For other files that are not UTF-8,
  Reopen with Encoding suggests an encoding guessed from the contents.
  The status bar names UTF-16 with a BOM as "UTF-16 LE BOM" / "BE BOM".
- Split editor (View > Split Editor, Cmd/Ctrl+\): two tabs side by side.
  The pane you work in is the active one (marked with a line on top), so
  menus and shortcuts act on it.
- Comparisons highlight the changed characters within changed lines.
- Replace All first shows the lines it would change, before and after,
  and replaces only when confirmed.
- Syntax highlighting for C, C++, C#, Go, HTML, Java, Ruby, TypeScript,
  TSX and XML (18 languages in total).
- Saving to a full disk, a read-only disk or a location without write
  permission now explains what happened in plain words and that the file
  on disk is unchanged.
- Tools > Compare with Saved Version and Compare with Clipboard show the
  current document side by side with the file on disk or the clipboard
  text; changes can be copied into the document one at a time.
- Macros: Tools > Start or Stop Macro Recording (Cmd/Ctrl+Shift+R)
  records typing, deleting, cursor keys and editing commands; Play Macro
  (Cmd/Ctrl+Shift+M) replays them at the current cursor, and each run
  undoes in one step.
- Select Next Occurrence (Cmd/Ctrl+D): selects the word at the caret,
  then adds the next occurrence of the selection as another cursor.
- With word wrap on, Home and End (Cmd+Left/Right on macOS) go to the
  start and end of the display row.
- Structured view for XML and YAML (View > Structured View): a read-only
  tree of elements, attributes and text, or of mappings and sequences,
  with the error position for invalid files. Nothing is expanded or
  fetched: XML entities other than the predefined ones and YAML aliases
  are shown as written, so entity-expansion attacks do nothing.
- Go to Line (Cmd/Ctrl+L, accepts `line` or `line:column`) and, in
  Markdown, Go to Heading (Cmd/Ctrl+Shift+O) with a filterable list of
  headings. The command palette list now scrolls through every command.
- Word wrap: long lines wrap at word boundaries to the window width (on
  by default; View > Word Wrap or Settings to turn it off). Up/Down move
  by display row. Documents over 1 M characters are not wrapped yet.
- Multiple cursors from the UI: Option/Alt+click adds or removes a
  cursor, Option/Alt+drag makes a rectangular (column) selection, Edit >
  Add Cursor Above/Below (Cmd+Option+Up/Down on macOS, Ctrl+Alt+Up/Down
  elsewhere) adds one at the same column, and Esc returns to one cursor.
- Tabs can be reordered by dragging and pinned (right-click a tab, or
  Window > Pin Tab). Pinned tabs stay at the front, show a pin instead of
  a close button, and are remembered in the session.
- Log viewer for large and growing files: File > Open in Log Viewer, and
  files of 256 MB or more open there automatically. Read-only; lines are
  read from disk as they scroll into view. Filter lines by text or
  regular expression, and Follow to keep showing the end as the file
  grows (a file that shrinks is read again from the start). Reading and
  filtering run in the background and stop when the viewer closes.
- Local history: each save keeps a copy of the file on this computer (up
  to 50 versions per file, 30 days, files up to 10 MB). File > Local
  History lists them; open a version in a new tab or restore it as one
  undoable edit. It can be turned off in Settings and cleared with File >
  Clear Local History.
- Update check: Sanain > Check for Updates (File menu on Windows/Linux)
  asks GitHub for the latest release and links to its download page.
  Settings can turn on a daily check; it is off by default. Sanain never
  downloads or installs updates by itself.
- 24 more UI languages: Czech, Danish, Dutch, French, German, Greek,
  Hungarian, Indonesian, Italian, Japanese, Korean, Norwegian Bokmål,
  Polish, Portuguese (Brazil and Portugal), Romanian, Russian, Simplified
  and Traditional Chinese, Spanish, Swedish, Turkish, Ukrainian and
  Vietnamese. They are machine translations by Claude and await review by
  native speakers; corrections are welcome. The language picker in
  Settings lists every language by its own name.
- Cargo workspace at 0.0.1 with pinned toolchain and CI for Linux, Windows
  and macOS.
- `sanain-core`: rope document, grouped transactions, undo/redo, revision
  counter, save-point tracking, line-ending detection.
- `sanain-io`: strict UTF-8 / UTF-8 BOM / UTF-16 BOM decoding with
  byte-exact re-encoding; atomic save that preserves permissions and
  symlinks and refuses to overwrite files changed on disk.
- Multi-cursor selections and grapheme-aware insert, Backspace, Delete
  and arrow movement (combining marks, emoji sequences, flags, CRLF move
  and delete as one unit).
- `sanain-i18n`: Fluent localization with en-US and draft fi-FI, locale
  negotiation and resource consistency tests.

- MPL-2.0 license, security policy with private reporting, CODEOWNERS,
  Dependabot.

- First editor window (egui): open from the command line or File > Open,
  typing with IME support, mouse and keyboard selection, multi-line
  undo/redo, cut/copy/paste, Save and Save As with native dialogs, prompts
  for unsaved changes on close/new/open, external-change and read-only
  confirmations, status bar with position, line ending and encoding,
  light and dark themes from the brand tokens.

- Native macOS menu bar (Sanain, File, Edit, Window) with standard
  shortcuts; restyled in-window menus on Windows and Linux.
- Cut, Copy and Paste work (system clipboard was not enabled before).
- macOS app bundle script (`scripts/macos-bundle.sh`).
- Find in Folder (Cmd/Ctrl+Shift+F): search the text files under a
  folder with the same options as Find; results stream in, grouped by
  file, and a click opens the file with the match selected. Hidden and
  build folders, binary and non-UTF-8 files and files over 10 MB are
  skipped; folder links are not followed. Stop cancels at once.
- Keyboard shortcuts are configurable in Settings: pick a preset (Sanain
  or Notepad++) and change or reset any command's shortcut by pressing
  the new keys. A shortcut taken from another command is reported.
  Menus, the command palette and the native macOS menu follow the keymap.
- Code folding by indentation, for any file type: click the ▾/▸ marker
  next to the line number, or use View > Fold / Unfold / Unfold All.
  Folds stay on their lines while you edit above them and open when the
  caret moves into them.
- Screen readers: the editor exposes the caret's line as its text value,
  with the caret and selection, so VoiceOver can read where you are
  (verified through the macOS accessibility API).
- High contrast appearance (Settings): black and white with a yellow
  accent and stronger outlines.
- Markdown preview shows local images (PNG, JPEG, GIF, WebP) from the
  document's folder, decoded in the background with size and memory
  limits. Remote images are never fetched; images outside the folder are
  not shown; each case gets a short note.
- Legacy encodings: files that are not UTF-8/UTF-16 open an encoding
  picker with a live preview (Windows-125x, ISO-8859-2/15, KOI8-R,
  Shift_JIS, EUC-JP, GB18030, Big5, EUC-KR). A file opens only if the
  chosen encoding reproduces its bytes exactly. File > Reopen with
  Encoding and Save with Encoding convert explicitly; saving refuses
  characters the encoding cannot hold and selects the first one.
- Markdown preview links: web links open in the browser after a
  confirmation, links to local files open in a tab, `#anchors` scroll to
  the heading, and javascript:, data:, file: and unknown schemes are never
  opened. Hovering a link shows its target.
- Structured View for CSV, TSV and JSON files (View menu, palette),
  read-only so the source stays authoritative: a CSV grid with delimiter
  detection and override, quoted fields with line breaks, and a warning
  for unterminated quotes; a collapsible JSON tree that shows keys and
  values exactly as written.
- Compare two tabs side by side (Tools > Compare With…, also in the
  command palette): changed, added and removed lines marked with −/+ and
  colour, a change count, and per-change buttons to copy a change to the
  left or right as one undoable edit. Applying is refused if either
  document changed since the comparison was computed.
- Tools menu (also in the command palette), acting on the selection or
  the whole document as one undoable edit: Validate / Format / Minify
  JSON (keeps duplicate keys, number spellings and escapes exactly; errors
  show line and column and move the caret there), sort lines, sort
  descending, remove duplicate lines, trim trailing whitespace, upper and
  lower case, convert line endings to LF or CRLF.
- Markdown source, split and preview modes for .md files (View menu,
  command palette, Cmd/Ctrl+Shift+V for split). CommonMark and GFM:
  headings, emphasis, strikethrough, inline code, links, task lists,
  nested lists, tables with alignment, quotes, code blocks, front matter.
  The preview follows the caret. Raw HTML is shown as text and never
  run; links are not opened yet; images show their alt text; Mermaid
  blocks show their source.
- Command palette (Cmd/Ctrl+Shift+P): fuzzy search over every menu
  command with its shortcut; arrow keys, Enter and Esc.
- Settings window (Cmd+, on macOS, File > Settings elsewhere): font size,
  tab width, spaces for Tab, light/dark/system appearance, session
  restore, UI language. Saved immediately to a versioned settings file.
- Zoom In/Out/Actual Size (Cmd/Ctrl + "+", "-", "0"; Cmd/Ctrl+= also
  zooms in), new View menu.
- macOS menu shortcuts use logical keys, so punctuation shortcuts work on
  non-US keyboard layouts.
- Syntax highlighting with Tree-sitter for Rust, Python, JavaScript,
  JSON, TOML, YAML, CSS and shell scripts, chosen by file extension and
  shown in the status bar. Highlighting runs on a background thread and
  is skipped for files over 8 MB. All syntax colours meet WCAG AA
  contrast on the editor background.
- Session restore: open files and the active tab return on the next
  launch when Sanain starts without file arguments (`SANAIN_NO_SESSION=1`
  disables it until a setting exists).
- Find and replace: Cmd/Ctrl+F, Find Next/Previous (Cmd/Ctrl+G,
  Shift+Cmd/Ctrl+G, Enter/Shift+Enter), Replace (Cmd+Option+F on macOS,
  Ctrl+H elsewhere). Match case, whole word and regular expressions with
  `$1` capture references. All matches are highlighted with a count;
  Replace All is one undo step. Regular expressions run in linear time.
- Tabs: open several files (command line, File > Open with multi-select),
  New Tab, Close Tab (Cmd/Ctrl+W), Reopen Closed Tab (Cmd/Ctrl+Shift+T),
  Next/Previous Tab (Ctrl+Tab / Ctrl+Shift+Tab), middle-click to close,
  unsaved marker per tab. Opening a file that is already open switches to
  it. Quitting asks about each unsaved tab in turn. Every crash-recovery
  snapshot is restored as its own tab.
- Line operations: move line up/down (Option/Alt+Up/Down), duplicate
  (Option/Alt+Shift+Down), delete (Cmd/Ctrl+Shift+K), also in the Edit
  menu. They work on every selected line and keep the file's line endings.
- Up/Down keep the caret's column across shorter lines.
- Word movement and deletion: Option+Arrow / Option+Backspace on macOS,
  Ctrl+Arrow / Ctrl+Backspace / Ctrl+Delete elsewhere.
- Crash recovery: unsaved text is checkpointed within 2 seconds of an
  edit and restored on the next launch; checkpoints are deleted on save
  or when changes are discarded.

### Fixed
- Saving a file with several hard links (macOS, Linux) replaced it with a
  new file, so the other names kept the old contents. Such files are now
  overwritten in place, after a backup copy of the old contents is
  written next to them (removed when the save succeeds).
- Fewer whole-document copies on the UI thread: syntax highlighting got a
  copy on every keystroke (even for files too large to highlight), the
  comparison copied both documents every frame, and Go to Heading parsed
  the document every frame while open.
- Crash-recovery checkpoints copied the whole document on the UI thread;
  the copy now happens on the writer thread (large files no longer pause
  every few seconds while edited).
- Typing in the find field of a large document froze the window while
  the whole text was copied and searched on every keystroke. Documents
  over 1 M characters are now searched in the background; a newer search
  cancels the older one.
- A large file opened from the menu, or dropped onto the window, could get
  a log viewer that did not refresh while reading (it was given a stale
  UI context).
- The status bar copied and scanned the whole document every frame to
  show its line ending; large files used several times more memory and
  CPU than needed (147 MB file: 1.43 GB → 571 MB).
- Opening a file with a very long line (such as a multi-megabyte minified
  JSON file) crashed Sanain when word wrap was off or the file was too
  large to wrap: the GPU vertex buffer exceeded its limit. Long lines are
  now laid out only around the visible columns.
- With the native macOS menu, Cmd+A, Cmd+Z and clipboard shortcuts in
  the find bar or other text fields now act on that field, not on the
  document.
- Esc now closes the find bar after searching with Enter.

### Changed
- README describes what works today and shows screenshots from a real
  build.
- Repository renamed to github.com/Zanttux/Sanain.
- Wordmark set in a single colour.
