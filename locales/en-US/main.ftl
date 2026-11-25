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
file-new = New
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
