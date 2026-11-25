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
error-open-encoding = { $name } is not UTF-8 or UTF-16 text. It was not opened, so its contents are unchanged. Other encodings are not supported yet.
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
