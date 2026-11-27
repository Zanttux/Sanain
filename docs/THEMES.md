# Themes

Settings › Appearance offers the Text³ themes (light, dark, high
contrast, or following the system) and these bundled themes:

| Theme | Based on | License of the original |
|---|---|---|
| Classic (Notepad++ style) | Text³'s own light scheme in the style of Notepad++'s default colours; no Notepad++ files are used | — |
| Solarized Light / Dark | Solarized by Ethan Schoonover | MIT |
| Dracula | Dracula Theme by Zeno Rocha and contributors | MIT |
| Nord | Nord by Arctic Ice Studio / Sven Greb | MIT |
| Gruvbox Dark | gruvbox by Pavel Pertsev (morhetz) | MIT |
| One Dark | One Dark from the Atom editor (GitHub) | MIT |
| Tomorrow Night | Tomorrow Theme by Chris Kempson | MIT |

The bundled themes re-create the published colour palettes; no theme files
are copied. A few colours are adjusted so that text has at least 4.5:1
contrast with the background (WCAG 2.2 AA) and syntax colours, comments
and line numbers at least 3:1; a test checks every bundled theme.

## Your own themes

Put `.theme` files in the `themes` folder inside Text³'s settings folder
(Settings › Appearance shows the path). Each line is `key = value`; lines
starting with `#` are comments.

```
name = My theme
dark = true
canvas = #1e1e1e      # editor background
surface = #252526     # panels and menus
text = #d4d4d4
muted = #8a8a8a       # line numbers, hints
accent = #4fc1ff      # selection, links
border = #3c3c3c
error = #f48771
warning = #cca700
keyword = #569cd6
string = #ce9178
escape = #d7ba7d
comment = #6a9955
number = #b5cea8
function = #dcdcaa
type = #4ec9b0
tag = #569cd6
attribute = #9cdcfe
```

`name` is required; missing colours come from the Text³ dark theme (or
the light one with `dark = false`). Files over 64 KB, unknown keys and
colours that are not `#RRGGBB` are reported in Settings and the file is
skipped. Themes are read when Settings opens the Appearance page.
