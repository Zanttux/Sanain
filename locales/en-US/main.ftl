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
view-structured = Structured View (CSV, JSON, XML, YAML)
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
view-split-editor = Split Editor
encoding-detected = Opened as { $encoding } (detected from the file's contents).
encoding-suggested = Suggested from the file's contents: { $encoding }. Check the preview before opening.
app-about = About Sanain
menu-help = Help
about-tagline = The power of words.
about-version = Version { $version }
about-license = Free and open source under the Mozilla Public License 2.0.
about-ai = AI-built. Human-led. Community-driven.
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
      sanain [FILE…]              open files
      sanain --diff LEFT RIGHT    compare two files side by side
      sanain --version            show the version
      sanain --help               show this help
    Use -- before file names that start with a dash.
cli-unknown-option = Unknown option { $option }.
cli-diff-needs-two = --diff needs two files: sanain --diff LEFT RIGHT
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
      sanain --register           add Sanain to "Open With" (Linux, Windows)
      sanain --unregister         remove it again
cli-registered = Sanain was added to "Open With". { $place }
cli-unregistered = Sanain was removed from "Open With".
cli-register-failed = The file manager integration could not be changed: { $reason }
cli-register-macos = Finder integration on macOS is not available yet. Open files from Sanain (File › Open, drag and drop) or from the command line.
settings-tab-general = General
settings-tab-editor = Editor
settings-tab-appearance = Appearance
settings-shortcut-search = Search commands or keys
settings-font = Font
settings-font-default = Built-in monospace
settings-font-loading = Looking for installed fonts…
settings-font-missing = The font { $font } was not found; using the built-in font.
about-sponsor = Support Sanain on GitHub Sponsors
