# Risk register
| Risk | Mitigation |
|---|---|
| Data loss | Atomic save, recovery, conflict detection, fault injection; release blocker |
| GUI immaturity | Comparative spike, accessibility and IME gate |
| Scope creep | Milestone gates, source-first UX, defer AI/plugins |
| Name conflict | Search EUIPO/WIPO/USPTO, package registries, stores and domains; legal review |
| License incompatibility | Inventory SPDX, review transitive deps and borrowed code before distribution |
| AI provenance/copyright | Review and record origin; avoid unverified copying |
| Low funding | Keep costs bounded; no revenue assumptions in roadmap |
| Platform fragmentation | CI matrix plus real hardware testing; advertise verified support only |
| Malicious files | Offline safe rendering, bounded parsing, fuzzing |
| Maintainer burnout | Small releases, documented decisions, no support SLA |

## Notepad++ security history compared (issue #157, 2026-11-27)
Public Notepad++ advisories (github.com/notepad-plus-plus/notepad-plus-plus/security/advisories,
47 listed on 2026-11-27) and security commits in its git history, grouped by
class. Status: **N/A** (Text³ has no such feature), **Handled** (where),
**Fixed** (in #157), **Open**.

| Notepad++ class | Reference | Text³ | Evidence |
|---|---|---|---|
| Buffer overflows in charset detection / UTF-16 conversion | GHSA-jg7c-f532-25h8, GHSA-67mm-g35x-jv47 (uchardet, GHSL-2023-092/102/103/112), GHSA-cfcv-789h-3pg3, GHSA-8jjh-4rq6-r2h8 | Handled | Safe Rust `chardetng` + `encoding_rs` (text3-io/Cargo.toml); odd-length UTF-16 is an error, not a read past the end (text3-io/src/encoding.rs:73); `unsafe_code = "forbid"` (Cargo.toml:25) |
| Stack/heap overflows in lexers, function list, session/path strings, column editor, UDL, `wcscpy_s` fast-fail | GHSA-5h8p-vvcf-xq8p, GHSA-9rr8-6vjg-gj52, GHSA-pxgg-96p2-c3hx, GHSA-gv94-327x-2gc5, GHSA-h2wq-6x75-h8q2, GHSA-3hx4-x297-6752, EU-FOSSA commits | Handled / Open | Bounds-checked Rust everywhere except `text3-platform` (audited `SAFETY:` blocks). Open: tree-sitter grammars are C parsing untrusted text in-process; mitigated by the 8 MB cap (text3-app/src/syntax.rs:31) and a worker thread, not fuzzed |
| Heap disclosure via clipboard length | GHSA-h5cv-65m7-x97v | Handled | Clipboard only as `String` via `arboard` (text3-app/src/shell.rs:805) |
| Crafted-file DoS (nesting, size, regex) | NPP crash fixes in XML parsing, macro playback | Handled | Markdown depth cap (text3-markdown/src/lib.rs:14), Mermaid caps (mermaid.rs:34-35), image size/dimension/alloc limits (text3-app/src/images.rs:6-8), linear-time `regex` with 1 MiB compiled-size limit (text3-core/src/search.rs:19,70), XML/YAML/JSON node cap (text3-tools/src/markup.rs:13), log viewer from 256 MB (shell.rs:918) |
| XML entity expansion | config/XML parsing fixes | Handled | Entities never expanded, DTDs ignored (text3-tools/src/markup.rs:4, xml.rs:13); YAML aliases not expanded (yaml.rs:29); shortcuts import capped at 1 MB (text3-app/src/npp_shortcuts.rs:21,27) |
| Code execution from config: `commandLineInterpreter`, `shortcuts.xml` user commands, macro HMAC bypasses/TOCTOU, panel restore | CVE-2026-48778, CVE-2026-48800, CVE-2026-52884, CVE-2026-52885, CVE-2026-71858, CVE-2026-85288 | N/A | No Run command or external tools: the only processes started are `reg`, `explorer`/`open`/`xdg-open` and `update-desktop-database`. Importing a Notepad++ `shortcuts.xml` skips macros, run and plugin commands (npp_shortcuts.rs:20) |
| Untrusted search path: bare program names, DLL planting next to installer/uninstaller/exe | CVE-2025-49144, CVE-2026-25926, CVE-2026-46710, EU-FOSSA gup.exe/Config.dll | **Fixed** | `reg.exe` and `explorer.exe` were started by bare name; Rust's `Command` searches the folder of `text3.exe` first (e.g. Downloads). Now full System32/Windows paths, no fallback (text3-platform/src/lib.rs:66, text3-app/src/shellreg.rs:226, shell.rs:827-838). Single static exe, no `LoadLibrary`/plugins. Linux `xdg-open`/`update-desktop-database` stay on PATH (Unix never searches the program folder) |
| Command-line injection ("Open containing folder", "Run by system" target confusion, installer PowerShell path injection) | EU-FOSSA 0f93670, CVE-2026-77605, CVE-2026-73250, GHSA-97rx-3h53-4cfj | Handled | Arguments passed as argv, no shell (shell.rs:827-843); installer runs only `text3.exe --register/--unregister` (packaging/windows/text3.iss:53,57), no PowerShell; registry command strings quoted (shellreg.rs:188) |
| UNC/`file://` paths leaking NTLM credentials (links, session files, workspace/toolbar paths) | GHSA-rv8g-c295-pwcg, GHSA-7w2m-5769-p4q3, GHSA-fw3m-3qpq-pvr8, GHSA-qmmp-22w3-mcwm, GHSA-gr8w-74qx-pc2v | **Fixed** | `file:` links were already refused, but `\\host\share` (and `//host`) links and images reached `is_file`/`canonicalize`, i.e. an SMB connection when the preview drew or the link was clicked. Now refused (text3-markdown/src/lib.rs:108, text3-app/src/images.rs:60); macOS `file://host/…` no longer becomes a path (text3-platform/src/lib.rs:154). Session and recent files are only Text³'s own, never imported |
| Link schemes | (same) | Handled | Only http(s)/mailto, after a confirmation showing the URL (shell.rs:4387, 5625); `javascript:`/`data:`/`file:` refused (text3-markdown/src/lib.rs:113); remote images never fetched (PRIVACY.md) |
| Updater hijack / supply chain: unverified installer download, TOCTOU, Authenticode bypass, Zip Slip in plugin extraction, forced update domain | commits bcf2aa6, 3daa593, 6f87bb7 (8.8.9 updater hardening), GHSA-hrvm-8rjc-v44x, CVE-2026-85995, CVE-2026-57233 | Handled | Update check never downloads or runs anything; HTTPS to api.github.com only, 1 MB cap, release page must start with the project's github.com prefix (text3-app/src/update.rs:15,23,151) |
| Release integrity | (same) | Open | Pinned action SHAs, `contents: read` by default, `cargo deny` with advisories, SHA256SUMS (.github/workflows/package.yml:83). Releases are unsigned (docs/RELEASE.md); CI installs Inno Setup from Chocolatey without a pinned version (scripts/package.py:85) |
| Backup/session path traversal (delete/overwrite outside backup dir) | CVE-2026-134011, CVE-2026-52886 | Handled | Recovery ids are our own file stems (text3-io/src/recovery.rs:157); recovery restores into a tab, never to a path from the file |
| Save races: UAC save broker TOCTOU, temp file swap | GHSA-w7wv-vg94-7pvp, GHSA-phqq-x4c2-7c2q, ee71355 | Handled / N/A | No elevation. Temp file `create_new` in the target folder (text3-io/src/file.rs:317), symlink target kept (file.rs:357), hard links overwritten in place behind a never-replaced backup (file.rs:206,227); recovery/history 0700/0600 (recovery.rs:64,133, history.rs:65,76) |
| IPC: WM_COPYDATA/WM_COMMAND from lower-integrity processes, NPPM plugin messages | CVE-2026-48770, GHSA-grw6-cw7j-3qfg, GHSA-pg39-63h5-cx35, GHSA-5jjv-2h6g-63p5, GHSA-j327-595g-jg4c, CVE-2026-86056, GHSA-8wmf-7r8c-wf2w | N/A | No single-instance IPC, window messages API or plugin interface |
| Plugin Admin / plugin DLL loading | EU-FOSSA c314ed1, 9fecbae, CVE-2026-57233 | N/A | No plugins (ROADMAP defers them) |
| Shell registration | | Handled | Per-user HKCU only, quoted `"exe" "%1"` (shellreg.rs:180-220); Linux Exec quoted per spec (shellreg.rs:62) |
