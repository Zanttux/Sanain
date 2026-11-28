# UX specification
Default window: menu, optional compact toolbar, tabs, editor, status bar (line/column, selection, encoding, newline, syntax). Sidebar hidden by default. Command palette fuzzy search (Cmd/Ctrl+Shift+P) and Goto Anything (Cmd/Ctrl+P: files, `:line`, `@symbol`, `#text`); discoverable menu equivalents. Windows familiar shortcuts Ctrl+N/O/S/F/H, macOS native Cmd equivalents; configurable conflicts. Keyboard-only operation, visible focus, accessible labels, high contrast, scaling 100–200%, screen reader and IME testing. No modal on startup, no sponsorship nag, no forced project. One exception, asked for by the owner (#164): on first start Text³ offers once, in a non-blocking window, to become the default app; any answer is remembered and it never appears again (Settings › General keeps the choice). First run opens scratch buffer; restore previous session by default with opt-out. Quitting keeps unsaved and untitled tabs for the next start without prompts (hot exit, on by default, opt-out in Settings); closing a single tab still asks. Several running instances keep their own unsaved tabs; the last one to quit sets the tab order for the next start, and the others' tabs follow. Sponsor link in About and website only. Error messages explain cause, preservation of edits and recovery path. Avoid mimicking Notepad++ branding or icons. Design tokens for spacing, typography, semantic colors; verify contrast. No claims of accessibility until tested.

## Design rules (quality gate G1, docs/QUALITY_GATES.md)
From the UI/UX audit of 2026-11-28 (#181). Every UI change is checked
against these before it is built.

1. **Tokens only.** Colours from `Palette`; spacing from 4/8/12/16/24;
   UI text in the system font at one body size (`strong` for titles,
   `muted` for secondary); monospace only for code, data and shortcuts. No
   colour, spacing or margin literals in feature code.
2. **Reuse components.** ToolBar, QuickPick, Modal with `dialog_buttons`,
   `setting_row`, status-bar menu items. A new pattern needs a reason and
   an update to this file.
3. **Where things go.** Every command is in a menu and therefore in the
   palette. Settings hold persistent preferences only. Toggles show a
   checkmark, exclusive modes are radio items. At most about 15 items per
   menu level; group the rest into submenus. No feature toolbars.
4. **Dialogs only for real decisions** (data could be lost, or leaving
   the app). The title names the object; buttons are verbs, never
   Yes/No; platform button order; Esc cancels; Enter is the safe default
   and never the destructive one; the body says what is kept and how to
   recover.
5. **Wording.** Title Case (en-US) for menu items, buttons and window
   titles; sentence case for labels, check boxes, tooltips and messages.
   "…" only when more input follows. Curly quotes. No shortcuts, file
   paths or repository paths inside strings. One term per concept.
6. **Keyboard and screen reader.** Everything reachable with Tab or F6
   with a visible focus ring; every control has a localized accessible
   name (icon-only controls too); toggled, selected and expanded states
   are exposed; Esc closes the innermost transient UI and returns focus to
   the editor; every new view has an AccessKit test.
7. **No interruptions.** Nothing appears unasked at start-up or while
   typing (the exception above aside). Offers live in Settings, About or
   menus. Status messages are temporary and announced politely.
8. **States.** Every list or panel has a localized empty state saying what
   fills it; progress for anything over about 300 ms; errors give the
   cause, what was kept and the next step.
9. **Density and targets.** Click targets in the chrome are at least
   24×24 px at 100 %; tab bar, tool bars and status bar share one row
   height; side panels hidden by default; the editor keeps the most space.
10. **Platform and verification.** macOS: native menu bar, Settings… on
    Cmd+,. Windows and Linux: in-window menus. Follow the system's light,
    dark and high-contrast settings. Before merging, check light, dark,
    high contrast, 200 % scale, the longest language (de or fi) and a CJK
    language. Claim nothing for an untested platform.

Keep: the command palette and Goto Anything as the model for every
chooser; message copy that states cause, what is kept and how to recover;
calm chrome (no toolbar or sidebar by default, one accent marker on the
active tab); the keyboard model (F6 regions, arrow keys in lists, shortcut
editor with presets); safety in the interface (preview before Replace
All, confirmation before links, read-only large logs, local history).

