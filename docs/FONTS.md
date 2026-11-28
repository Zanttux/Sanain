# Editor fonts (#176)

Owner request (2026-11-28): the font choice was poor; offer the fonts that
Notepad++ and Sublime Text users like most.

## Survey (2026-11-28)
No neutral, methodical survey of editor fonts exists (the Stack Overflow
survey covers editors, not fonts). Roundups and forums agree on:

- **JetBrains Mono** and **Fira Code**: the most widely adopted free coding
  fonts; JetBrains Mono is the default of JetBrains IDEs.
- **Cascadia Code**: Microsoft's open font, default of Windows Terminal;
  Cascadia Mono ships with Windows 11.
- **Consolas**: Sublime Text's default on Windows and the usual Notepad++
  choice (Notepad++'s own default is Courier New). Windows only, not
  redistributable.
- **Menlo** (Sublime's macOS default), **Monaco**, **SF Mono** (macOS).
- Often named next: **Source Code Pro**, **Inconsolata**, **IBM Plex
  Mono**, **Hack**, **DejaVu Sans Mono**, **Ubuntu Mono**, **Lucida
  Console**.

Sources: [SitePoint, best free monospace fonts](https://www.sitepoint.com/best-monospace-fonts-for-developers/),
[Dracula, best free fonts for programming](https://draculatheme.com/blog/best-free-fonts-for-programming),
[Programming fonts in 2026 survey](https://github.fleetingtimer.workers.dev/cboone/reports/blob/main/fonts/programming-fonts-survey.md),
[Sublime forum: default font](https://forum.sublimetext.com/t/what-is-sublime-texts-default-font-face/9202),
[Sublime forum: Consolas](https://forum.sublimetext.com/t/strange-thing-about-the-default-consolas-font/33859).

## What Text³ does
- Ships six of them (regular weight, SIL Open Font License 1.1, licence
  file beside each): JetBrains Mono, Fira Code, Cascadia Code, Source Code
  Pro, IBM Plex Mono, Inconsolata. Hack is already egui's built-in
  monospace font (the default).
- They are separate files (`fonts/` next to the program,
  `Contents/Resources/fonts` in Text³.app, `/usr/share/text3/fonts` in the
  deb), not compiled in: start-up time and memory are unchanged until one
  is chosen; downloads grow by 1.8 MB.
- Settings › Editor › Font lists the recommended fonts that are available
  first (`text3_app::fonts::RECOMMENDED`), then every other installed
  monospaced font.
- Not bundled: Consolas, Menlo, Monaco, SF Mono, Lucida Console (system
  fonts whose licences forbid redistribution); they are listed when
  installed. SF Mono lives inside Terminal.app, not in the system font
  folders, so it is usually not found.

## Sources of the files
| File | Source | SHA-256 |
|---|---|---|
| JetBrainsMono-Regular.ttf | github.com/JetBrains/JetBrainsMono `fonts/ttf` (master) | e6fd0d7e91550b3ed2b735d4312474362c4716edc4fc0577a0f61ed782d5aed1 |
| FiraCode-Regular.ttf | github.com/tonsky/FiraCode release 6.2 `ttf/` | 5992ab9640e2df491b2f609467b1de60e8bc39b2c28db184342a0592d98f6117 |
| CascadiaCode-Regular.ttf | github.com/microsoft/cascadia-code release v2407.24 `ttf/static/` | c33ef522cdfeff99907fb54f3e97152bb18bfb9b56ec2fef4d4ceec51c8974a4 |
| SourceCodePro-Regular.ttf | github.com/adobe-fonts/source-code-pro branch `release`, `TTF/` | 74bd80d3e42a08517cd7e1108ba3d86f2da29ac0f3065be95e0357956ab9db37 |
| IBMPlexMono-Regular.ttf | github.com/google/fonts `ofl/ibmplexmono` | 6a3412f058c7d8dfd9170c41e85ade48e5156ecb89356110ca57a0a27734af46 |
| Inconsolata-Regular.ttf | github.com/google/fonts `ofl/inconsolata/Inconsolata[wdth,wght].ttf` (variable; default instance is Regular) | 23ded25b447074d00659392bf9b1123d89df55cb07b0ad9bfef3366d199b5fcb |
