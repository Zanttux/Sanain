app-name = Text³
menu-file = File
file-open = Open…
file-save = Save
status-tabs = { $count ->
    [one] One tab
   *[other] { $count } tabs
    }
error-open = Could not open { $name }.
menu-edit = Edit
file-new = New Tab
file-save-as = Save As…
edit-undo = Undo
edit-redo = Redo
edit-select-all = Select All
untitled = Untitled
window-title = { $dirty ->
    [yes] ● { $name } — Text³
   *[no] { $name } — Text³
    }
status-position = Ln { $line }, Col { $column }
status-selections = { $count ->
    [one] 1 selection
   *[other] { $count } selections
    }
status-saved = Saved { $name }
encoding-utf8 = UTF-8
encoding-utf8-bom = UTF-8 with BOM
encoding-utf16le = UTF-16 LE
encoding-utf16be = UTF-16 BE
line-ending-lf = LF
line-ending-crlf = CRLF
line-ending-cr = CR
line-ending-mixed = Mixed line endings
editor-accessible-name = Text editor
button-save = Save
button-dont-save = Don't Save
button-cancel = Cancel
button-ok = OK
button-overwrite = Overwrite
button-save-as = Save As…
dialog-unsaved-title = Save changes to { $name }?
dialog-unsaved-body = Your changes will be lost if you don't save them.
dialog-conflict-title = { $name } changed on disk
dialog-conflict-body = Another program modified this file after you opened it. Overwriting replaces those changes.
dialog-readonly-title = { $name } is read-only
dialog-readonly-body = Save a copy under a different name instead.
error-open-encoding = { $name } is not UTF-8 or UTF-16 text. It was not opened, so its contents are unchanged. Use File > Reopen with Encoding to choose its encoding.
error-save = Could not save { $name }: { $reason }
menu-window = Window
edit-cut = Cut
edit-copy = Copy
edit-paste = Paste
app-quit = Quit Text³
app-hide = Hide Text³
app-hide-others = Hide Others
app-show-all = Show All
window-minimize = Minimize
window-zoom = Zoom
status-recovered = Restored unsaved changes from the last session
status-recovery-failed = Could not save a recovery copy: { $reason }
status-hot-exit-failed = Could not keep unsaved changes for the next start ({ $reason }); asking about them instead.
status-hot-exit-too-large = { $name } is too large to keep for the next start; asking about it instead.
status-restored-changed = { $name } changed on disk after your unsaved changes were kept. Your text is in the tab; the file was not touched.
status-restored-missing = { $name } is no longer on disk. Its unsaved text is kept in the tab; saving creates the file again.
edit-duplicate-line = Duplicate Line
edit-delete-line = Delete Line
edit-move-line-up = Move Line Up
edit-move-line-down = Move Line Down
file-close-tab = Close Tab
file-reopen-tab = Reopen Closed Tab
window-next-tab = Next Tab
window-prev-tab = Previous Tab
tab-close = Close tab
tab-missing-on-disk = Not found on disk
edit-find = Find…
edit-find-next = Find Next
edit-find-prev = Find Previous
edit-replace = Replace…
find-placeholder = Find
replace-placeholder = Replace with
find-match-case = Match case
find-whole-word = Whole word
find-regex = Regular expression
find-no-results = No results
find-count = { $total ->
    [one] 1 match
   *[other] { $total } matches
    }
find-count-current = { $current } of { $total }
find-prev = Previous match (Shift+Enter)
find-next = Next match (Enter)
find-close = Close (Esc)
replace-one = Replace
replace-all = Replace All
status-replaced = { $count ->
    [one] Replaced 1 match
   *[other] Replaced { $count } matches
    }
language-plain = Plain Text
menu-view = View
app-settings = Settings…
view-zoom-in = Zoom In
view-zoom-out = Zoom Out
view-zoom-reset = Actual Size
settings-title = Settings
settings-font-size = Font size
settings-tab-width = Tab width
settings-insert-spaces = Insert spaces when pressing Tab
settings-theme = Appearance
settings-theme-system = Match system
settings-theme-light = Light
settings-theme-dark = Dark
settings-restore-session = Reopen files from the last session
settings-hot-exit = Remember unsaved changes on exit
settings-hot-exit-detail = Quit without being asked about each unsaved tab. Unsaved and untitled tabs come back with their text at the next start, even when reopening the last session's files is off. Closing a single tab still asks.
settings-language = Language
settings-language-system = Match system
settings-language-note = Language changes apply after restarting Text³.
settings-save-failed = Could not save settings: { $reason }
view-command-palette = Command Palette…
palette-placeholder = Type a command
view-markdown-source = Markdown Source
view-markdown-split = Markdown Source and Preview
view-markdown-preview = Markdown Preview
preview-too-large = This document is too large to preview.
preview-mermaid-unsupported-type = Diagram type not supported; showing source.
preview-mermaid-unsupported-syntax = Diagram uses syntax that cannot be drawn; showing source.
preview-mermaid-too-large = Diagram is too large to draw; showing source.
menu-tools = Tools
tools-json-validate = Validate JSON
tools-json-format = Format JSON
tools-json-minify = Minify JSON
tools-sort-lines = Sort Lines
tools-sort-lines-desc = Sort Lines Descending
tools-unique-lines = Remove Duplicate Lines
tools-trim-trailing = Trim Trailing Whitespace
tools-upper-case = Upper Case
tools-lower-case = Lower Case
tools-to-lf = Convert Line Endings to LF
tools-to-crlf = Convert Line Endings to CRLF
tools-json-valid = JSON is valid
tools-done = Done. Undo reverts it.
tools-no-change = Nothing to change
tools-json-error = Invalid JSON at line { $line }, column { $column }: { $reason }
json-reason-unterminated-string = unterminated string
json-reason-invalid-u-escape = invalid \u escape
json-reason-invalid-escape = invalid escape
json-reason-control-character-in-string = control character in string
json-reason-invalid-number = invalid number
json-reason-unexpected-character = unexpected character
json-reason-unexpected-content-after-the-value = unexpected content after the value
json-reason-unexpected-comma = unexpected comma
json-reason-expected-a-string-key = expected a string key
json-reason-expected = expected ':'
json-reason-expected-or-a-closing-bracket = expected ',' or a closing bracket
json-reason-expected-a-value = expected a value
json-reason-empty-document = empty document
json-reason-unexpected-end-of-input = unexpected end of input
json-reason-too-deeply-nested-to-format = too deeply nested to format
tools-compare = Compare With…
compare-need-two = Open another tab to compare with.
compare-pick-title = Compare { $name } with
compare-count = { $count ->
    [0] No differences
    [one] 1 change
   *[other] { $count } changes
    }
compare-close = Close Comparison
compare-to-left = Copy this change to the left
compare-to-right = Copy this change to the right
compare-stale = A document changed during the comparison. The comparison was refreshed; try again.
view-structured = Structured View (CSV, JSON, XML, YAML)
csv-delimiter = Delimiter
csv-rows = { $count ->
    [one] 1 row
   *[other] { $count } rows
    }
csv-unterminated = Unterminated quote starting on line { $line }
csv-truncated = Showing the first 200,000 rows
csv-edit-cell = Edit Cell
csv-insert-row-below = Insert Row Below
csv-delete-row = Delete Row
csv-read-only = Read-only file: the grid can't be edited
json-more = … { $count } more
json-edit-value = Edit Value
json-rename-key = Rename Key
json-add-after = Add After
json-add-inside = Add Inside
json-delete-node = Delete
json-read-only = Read-only file: the tree can't be edited
json-invalid-value = Not a valid JSON value
json-duplicate-key = The object already has a key with this name
xml-edit-value = Edit Value
xml-edit-text = Edit Text
xml-rename = Rename
xml-add-attribute = Add Attribute
xml-add-child = Add Child
xml-add-after = Add After
xml-delete = Delete
xml-invalid-name = Not a valid XML name
xml-duplicate-attribute = The element already has an attribute with this name
xml-not-well-formed = This change would make the XML invalid
yaml-not-preserved = This change would alter other parts of the YAML
yaml-key-too-long = A key can be at most 1024 characters long
yaml-read-only-flow = Flow style (written inside brackets) can't be edited in the tree yet
yaml-read-only-anchor = Values with an anchor (&) can't be edited in the tree
yaml-read-only-tag = Values with a tag (!) can't be edited in the tree
yaml-read-only-alias = Aliases (*) can't be edited in the tree
yaml-read-only-block-scalar = Block text (| or >) can't be edited in the tree yet
yaml-read-only-complex-key = Complex keys can't be edited in the tree
yaml-read-only-document = Only the first document can be edited in the tree
yaml-read-only-layout = This entry shares its lines with other content, so it can't be changed here
yaml-read-only-only-child = The only entry can't be removed
dialog-open-link-title = Open this link in your web browser?
button-open = Open
link-blocked = This kind of link is not opened, for safety.
link-not-found = Linked file not found: { $target }
enc-windows-1252 = Western European
enc-iso-8859-15 = Western European
enc-windows-1250 = Central European
enc-iso-8859-2 = Central European
enc-windows-1251 = Cyrillic
enc-koi8-r = Cyrillic
enc-windows-1253 = Greek
enc-windows-1254 = Turkish
enc-windows-1255 = Hebrew
enc-windows-1256 = Arabic
enc-windows-1257 = Baltic
enc-windows-874 = Thai
enc-shift_jis = Japanese
enc-euc-jp = Japanese
enc-gb18030 = Chinese, Simplified
enc-big5 = Chinese, Traditional
enc-euc-kr = Korean
file-reopen-encoding = Reopen with Encoding…
file-save-encoding = Save with Encoding…
encoding-reopen-title = Reopen with Encoding
encoding-save-title = Save with Encoding
encoding-not-exact = This encoding cannot open the file without changing it: { $reason }
encoding-unencodable = “{ $character }” cannot be saved as { $encoding }. Choose another encoding or change the text; nothing was written.
encoding-save-first = Save or undo your changes before reopening with another encoding.
image-remote = Remote images are not loaded.
image-blocked = Only images in this document's folder are shown.
image-unsupported = Unsupported image format.
image-too-large = Image is too large to show.
image-unreadable = Image could not be read.
settings-theme-high-contrast = High contrast
view-fold = Fold
view-unfold = Unfold
view-unfold-all = Unfold All
settings-shortcuts = Keyboard shortcuts
settings-keymap-preset = Preset
settings-keymap-default = Text³
settings-keymap-notepadpp = Notepad++
settings-shortcut-change = Change
settings-shortcut-reset = Reset
settings-shortcut-press = Press a shortcut… (Esc cancels)
settings-shortcut-moved = Shortcut moved from:
edit-find-in-folder = Find in Folder…
folder-search-placeholder = Find in folder
folder-search-run = Search
folder-search-stop = Stop
folder-search-no-folder = Choose folder
folder-search-summary = { $count ->
    [one] 1 match
   *[other] { $count } matches
    } in { $files ->
    [one] 1 file
   *[other] { $files } files
    }
app-check-updates = Check for Updates…
update-checking = Checking for updates…
update-up-to-date = Text³ { $version } is the newest version.
update-no-releases = No Text³ releases have been published yet.
update-failed = Could not check for updates: { $reason }
update-invalid = unexpected response
update-available-title = Text³ { $version } is available
update-available-body = Open the download page to get it. Text³ does not download or install anything by itself.
update-open-page = Open Download Page
update-later = Later
settings-check-updates = Check for updates once a day
settings-check-updates-detail = Asks github.com for the latest release. Nothing else is sent.
file-local-history = Local History…
file-clear-history = Clear Local History…
history-title = Local history of { $name }
history-open = Open
history-restore = Restore
history-empty = No saved versions yet. A copy is kept each time you save.
history-unreadable = This version could not be read.
history-just-now = Just now
history-minutes-ago = { $count ->
    [one] 1 minute ago
   *[other] { $count } minutes ago
    }
history-hours-ago = { $count ->
    [one] 1 hour ago
   *[other] { $count } hours ago
    }
history-days-ago = { $count ->
    [one] 1 day ago
   *[other] { $count } days ago
    }
history-purge-title = Delete all local history?
history-purge-body = Saved copies of every file are removed from this computer. Your files themselves are not changed.
history-purge-confirm = Delete History
history-purged = Local history deleted.
settings-local-history = Keep local history of saved files
settings-local-history-detail = Keeps up to 50 versions per file for 30 days, on this computer only.
file-open-log = Open in Log Viewer…
log-lines = { $count ->
    [one] 1 line
   *[other] { $count } lines
    }
log-indexing = Reading… { $percent } %
log-matches = { $count ->
    [one] 1 matching line
   *[other] { $count } matching lines
    }
log-filter-hint = Filter lines
log-bad-pattern = Invalid pattern
log-follow = Follow
log-close = Close Viewer
log-opened-large = { $name } is large, so it opened read-only in the log viewer.
tab-pin = Pin Tab
tab-unpin = Unpin Tab
edit-add-cursor-above = Add Cursor Above
edit-add-cursor-below = Add Cursor Below
view-word-wrap = Word Wrap
status-wrap-on = Long lines now wrap to the window width.
status-wrap-off = Long lines no longer wrap.
settings-word-wrap = Wrap long lines
edit-go-to-line = Go to Line…
edit-go-to-heading = Go to Heading…
goto-line-hint = Line or line:column
goto-line-note = { $count ->
    [one] The document has 1 line.
   *[other] The document has { $count } lines.
    }
goto-line-invalid = Enter a line number, optionally followed by :column.
goto-heading-hint = Type to filter headings
goto-heading-none = This document has no headings.
format-xml = XML
format-yaml = YAML
tools-tree-error = Not valid { $format } at line { $line }, column { $column }: { $reason }
edit-select-next-occurrence = Select Next Occurrence
tools-record-macro = Start or Stop Macro Recording
tools-play-macro = Play Macro
macro-recording = Recording macro
macro-recorded = { $count ->
    [one] Macro recorded: 1 step.
   *[other] Macro recorded: { $count } steps.
    }
tools-compare-saved = Compare with Saved Version
tools-compare-clipboard = Compare with Clipboard
compare-saved-label = { $name } (on disk)
compare-clipboard-label = Clipboard
compare-saved-failed = Could not read the saved file: { $reason }
compare-clipboard-empty = The clipboard has no text to compare.
error-save-disk-full = { $name } was not saved: the disk is full. The file on disk is unchanged and your text is still here. Free some space, or use Save As to save it elsewhere.
error-save-permission = { $name } was not saved: you do not have permission to write it there. The file on disk is unchanged. Use Save As to save it elsewhere.
error-save-read-only-volume = { $name } was not saved: the disk is read-only. The file on disk is unchanged. Use Save As to save it elsewhere.
replace-preview-title = { $count ->
    [one] Replace 1 match?
   *[other] Replace { $count } matches?
    }
replace-preview-confirm = { $count ->
    [one] Replace 1
   *[other] Replace { $count }
    }
replace-preview-more = Only the first 200 changed lines are shown.
replace-preview-stale = The document changed; nothing was replaced. Run Replace All again.
encoding-detected = Opened as { $encoding } (detected from the file's contents).
encoding-suggested = Suggested from the file's contents: { $encoding }. Check the preview before opening.
app-about = About Text³
menu-help = Help
about-version = Version { $version }
about-license = Free and open source under the Mozilla Public License 2.0.
about-ai = Developed with AI assistance (Claude by Anthropic) under the direction of its maintainer.
about-notices = Third-party components and their licenses are listed in THIRD_PARTY_NOTICES, included with every release.
about-source = Source code and issues on GitHub
tools-xml-validate = Validate XML
tools-yaml-validate = Validate YAML
tools-markup-valid = Valid { $format }.
file-open-recent = Open Recent…
file-save-all = Save All
file-revert = Revert to Saved
recent-hint = Type to filter recent files
recent-none = No recent files yet.
revert-title = Discard changes to { $name }?
revert-body = The file is reloaded from disk. Unsaved changes are lost and cannot be undone.
revert-confirm = Discard and Reload
revert-done = Reloaded { $name } from disk.
status-saved-all = { $count ->
    [0] Nothing to save.
    [one] Saved 1 file.
   *[other] Saved { $count } files.
    }
edit-toggle-comment = Toggle Line Comment
edit-indent = Indent Lines
edit-outdent = Outdent Lines
comment-unsupported = This file type has no line comments.
tab-close-others = Close Other Tabs
tab-close-all = Close All Tabs
file-copy-path = Copy File Path
file-reveal-macos = Show in Finder
file-reveal-windows = Show in Explorer
file-reveal-other = Show in File Manager
status-path-copied = File path copied.
tabs-kept-unsaved = { $count ->
    [one] 1 tab with unsaved changes was kept open.
   *[other] { $count } tabs with unsaved changes were kept open.
    }
status-indent-tabs = Tabs
status-indent-spaces = Spaces: { $count }
find-searching = Searching…
compare-ignore-whitespace = Ignore whitespace
tools-compare-file = Compare with File…
compare-file-failed = Could not compare with { $name }: { $reason }
cli-help =
    Usage:
      text3 [FILE…]               open files
      text3 --diff LEFT RIGHT     compare two files side by side
      text3 --version             show the version
      text3 --help                show this help
    Use -- before file names that start with a dash.
cli-unknown-option = Unknown option { $option }.
cli-diff-needs-two = --diff needs two files: text3 --diff LEFT RIGHT
window-move-tab-left = Move Tab Left
window-move-tab-right = Move Tab Right
edit-skip-occurrence = Skip to Next Occurrence
tools-play-macro-times = Play Macro Multiple Times…
macro-times-hint = Number of times
macro-times-note = From 1 to { $max }
macro-times-invalid = Enter a number from 1 to { $max }.
view-outline = Outline
outline-empty = No headings in this document.
outline-close = Close Outline
folder-replace-placeholder = Replace with
folder-replace-run = Replace in Files…
replace-files-title = { $count ->
    [one] Replace 1 match in { $files ->
        [one] 1 file
       *[other] { $files } files
    }?
   *[other] Replace { $count } matches in { $files ->
        [one] 1 file
       *[other] { $files } files
    }?
}
replace-files-body = Files open in a tab are changed there and can be undone; save them yourself. Other files are saved now, and their previous contents are kept in local history when it is turned on. A file that changed on disk in the meantime is skipped.
replace-files-confirm = Replace
replace-files-done = { $count ->
    [one] Replaced 1 match
   *[other] Replaced { $count } matches
} in { $files ->
    [one] 1 file
   *[other] { $files } files
} ({ $open } open in tabs, not saved).
replace-files-failed = { $failed ->
    [one] 1 file was not changed:
   *[other] { $failed } files were not changed; first:
} { $first }
image-outside = This image is outside the document's folder.
image-allow = Show images from this folder
cli-help-integration =
    Integration with the file manager (current user only):
      text3 --register            add Text³ to "Open With" (Linux, Windows)
      text3 --unregister          remove it again
cli-registered = Text³ was added to "Open With". { $place }
cli-unregistered = Text³ was removed from "Open With".
cli-register-failed = The file manager integration could not be changed: { $reason }
cli-register-macos = On macOS nothing needs to be registered: once Text³.app is in the Applications folder, Finder offers it in Open With.
settings-tab-general = General
settings-tab-editor = Editor
settings-tab-appearance = Appearance
settings-shortcut-search = Search commands or keys
settings-font = Font
settings-font-default = Built-in monospace
settings-font-loading = Looking for installed fonts…
settings-font-missing = The font { $font } was not found; using the built-in font.
settings-theme-more = More themes
settings-theme-user-note = Your own themes: .theme files in { $folder } (see docs/THEMES.md).
language-auto = Detect automatically
language-choose = Choose the language of this tab
tools-xml-format = Format XML
tools-save-macro = Save Macro…
tools-play-saved-macro = Play Saved Macro…
tools-delete-saved-macro = Delete Saved Macro…
macro-save-hint = Macro name
macro-save-note = A saved macro with the same name is replaced.
macro-save-empty = Nothing to save: record a macro first.
macro-saved = Macro saved: { $name }
macro-deleted = Macro deleted: { $name }
macro-name-invalid = Enter a name of 1 to { $max } characters on one line.
macro-save-too-large = This macro is too long to save.
macro-save-full = Up to { $max } macros can be saved. Delete one first.
macro-save-failed = Could not save the macros: { $error }
macro-saved-hint = Type to filter saved macros
macro-saved-none = No saved macros yet.
settings-shortcut-import-npp = Import from Notepad++…
settings-shortcut-import-npp-hint = Choose a Notepad++ shortcuts.xml. On Windows it is in %AppData%\Notepad++\shortcuts.xml.
settings-shortcut-import-npp-done = Imported from Notepad++: { $imported }. Skipped: { $skipped }. Conflicts left: { $conflicts }.
settings-shortcut-import-npp-failed = Could not import { $name }: { $reason }
settings-shortcut-import-npp-too-large = The file is larger than 1 MiB.
settings-shortcut-import-npp-invalid = The file is not a valid Notepad++ shortcuts.xml.
view-split-right = Split Right
view-split-down = Split Down
view-close-pane = Close Pane
view-focus-next-pane = Focus Next Pane
view-focus-prev-pane = Focus Previous Pane
view-focus-next-region = Focus Next Region
view-focus-prev-region = Focus Previous Region
status-pane-limit = Pane limit reached (at most { $max }).
a11y-csv-grid = CSV table
a11y-grid-cell = Row { $row }, column { $column }: { $name }
a11y-grid-cell-plain = Row { $row }, column { $column }
a11y-grid-column-header = Column { $column } header
a11y-grid-row-header = Row { $row }
a11y-edit-field = Editing { $item }
a11y-json-tree = JSON tree
a11y-xml-tree = XML tree
a11y-yaml-tree = YAML tree
a11y-tree-item = Level { $level }
a11y-tree-item-expanded = Level { $level }, expanded
a11y-tree-item-collapsed = Level { $level }, collapsed
a11y-tree-object = Object, entries: { $count }
a11y-tree-list = List, items: { $count }
a11y-tree-element = Element { $name }
a11y-tree-attribute = Attribute { $name }
a11y-tree-text = Text
a11y-pane = Pane { $pane } of { $count }: { $name }
a11y-pane-active = Pane { $pane } of { $count }: { $name }, active
a11y-pane-editor = Text editor, pane { $pane } of { $count }: { $name }
a11y-mermaid-flowchart = Flowchart. Nodes: { $nodes }, links: { $links }.
a11y-mermaid-sequence = Sequence diagram. Participants: { $participants }, messages: { $messages }.
a11y-mermaid-state = State diagram. States: { $states }, transitions: { $transitions }.
a11y-mermaid-class = Class diagram. Classes: { $classes }, relationships: { $relations }.
a11y-shortcut-change = Change shortcut for { $command }
a11y-shortcut-change-current = Change shortcut for { $command }, now { $keys }
a11y-shortcut-reset = Reset shortcut for { $command }
status-line-ending-choose = Change the line endings of this file
status-encoding-choose = Reopen or save with another encoding
status-indent-choose = Indentation settings
a11y-settings-sections = Settings sections
a11y-tab-strip = Open files
a11y-tab-unsaved = { $name }, unsaved changes
migration-copied = Settings and data from Sanain were copied to Text³. The Sanain folders were left unchanged.

# Character and word count (status bar)
# How one thousand is written with digit grouping: the characters between
# 1 and 000 separate groups of three digits.
number-thousand = 1{ "," }000
count-status = Characters: { $chars } · Words: { $words }
count-status-selection = Characters: { $chars } of { $total-chars } · Words: { $words } of { $total-words }
count-choose = Character, word, line and byte counts
count-a11y-pending = Characters and words: not counted yet. Open for the counts.
count-heading-selection = Selection
count-heading-document = Document
count-characters = Characters
count-characters-no-spaces = Characters (no spaces)
count-words = Words
count-lines = Lines
count-bytes = Bytes ({ $encoding })
count-unencodable = Some characters cannot be saved as { $encoding }.
count-counting = Counting…
settings-show-counts = Show the character and word count in the status bar
