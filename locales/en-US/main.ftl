app-name = Sanain
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
    [yes] ● { $name } — Sanain
   *[no] { $name } — Sanain
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
app-quit = Quit Sanain
app-hide = Hide Sanain
app-hide-others = Hide Others
app-show-all = Show All
window-minimize = Minimize
window-zoom = Zoom
status-recovered = Restored unsaved changes from the last session
status-recovery-failed = Could not save a recovery copy: { $reason }
edit-duplicate-line = Duplicate Line
edit-delete-line = Delete Line
edit-move-line-up = Move Line Up
edit-move-line-down = Move Line Down
file-close-tab = Close Tab
file-reopen-tab = Reopen Closed Tab
window-next-tab = Next Tab
window-prev-tab = Previous Tab
tab-close = Close tab
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
settings-language = Language
settings-language-system = Match system
settings-language-note = Language changes apply after restarting Sanain.
settings-save-failed = Could not save settings: { $reason }
view-command-palette = Command Palette…
palette-placeholder = Type a command
view-markdown-source = Markdown Source
view-markdown-split = Markdown Source and Preview
view-markdown-preview = Markdown Preview
preview-too-large = This document is too large to preview.
preview-mermaid-unsupported = Mermaid diagrams are not rendered yet; showing the source.
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
view-structured = Structured View (CSV, JSON)
csv-delimiter = Delimiter
csv-rows = { $count ->
    [one] 1 row
   *[other] { $count } rows
    }
csv-unterminated = Unterminated quote starting on line { $line }
csv-truncated = Showing the first 200,000 rows
json-more = … { $count } more
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
settings-keymap-default = Sanain
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
update-up-to-date = Sanain { $version } is the newest version.
update-no-releases = No Sanain releases have been published yet.
update-failed = Could not check for updates: { $reason }
update-invalid = unexpected response
update-available-title = Sanain { $version } is available
update-available-body = Open the download page to get it. Sanain does not download or install anything by itself.
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
