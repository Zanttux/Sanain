#!/usr/bin/env python3
"""Generate website/index.html (English only, owner decision #105).
Run: python3 website/build.py"""
import html
import os

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = "https://github.com/Zanttux/Sanain"
NIGHTLY = REPO + "/releases/tag/nightly"

T = {
    "en": {
        "lang": "en", "root": "",
        "title": "Sanain — the power of words",
        "desc": "Sanain is a free, open-source text editor for text, code, Markdown and data. Native, offline, no account.",
        "nav": ["Download", "Features", "Privacy", "How it's made"],
        "tab": "welcome.md",
        "hero_l1": "The power", "hero_l2": "of words.",
        "hero_sub": "A free, open-source text editor for everyday text, code, configuration files, Markdown and data. Native on Windows, macOS and Linux.",
        "status": ["Ln 2, Col 10", "Markdown", "LF", "UTF-8"],
        "cta": "Download the nightly build", "src": "Source on GitHub",
        "cta_note": "Pre-alpha, version 0.2.1. Free for everyone, every feature included.",
        "intact_h": "Your files stay intact",
        "intact_p": "Open a file, change nothing, save it: the bytes are the same. Encodings and line endings are kept as they were, saves are atomic, and Sanain refuses to overwrite a file another program changed in the meantime. Unsaved text survives a crash.",
        "bytes_label": ["before ", "after  "],
        "feat_h": "What it does",
        "feats": [
            ("Editing", "Tabs and a split view, multiple cursors, macros, word wrap, folding, a command palette, and keyboard shortcuts you can change, with a Notepad++ preset."),
            ("Find and replace", "Regular expressions, a preview before Replace All, search across a folder and Replace in Files, with the previous version of each file kept in local history."),
            ("Markdown", "Source, split and preview with tables, task lists, coloured code blocks, local images and an outline. Nothing in a document runs or loads from the network."),
            ("Data and logs", "JSON, XML and YAML trees and validation, a CSV grid, side-by-side comparison, and a log viewer that opens files of any size."),
            ("Languages", "Highlighting for 22 languages, including languages inside others (JavaScript and CSS in HTML, HTML around PHP, code blocks in Markdown). The interface is in 26 languages."),
            ("Comfort", "Light, dark, high-contrast and bundled terminal themes, your own font, session restore, portable mode for a USB stick."),
        ],
        "shots_h": "Screenshots",
        "shots": [("editor.png", "Editor with tabs and syntax highlighting"), ("markdown.png", "Markdown source and preview"), ("split.png", "Split editor"), ("log.png", "Log viewer with a filter")],
        "dl_h": "Download",
        "dl_p": "Builds are made automatically from every change and published as the nightly pre-release.",
        "dl_head": ["System", "Installer", "Portable"],
        "dl_rows": [
            ("Windows 10/11 (x64)", "sanain-…-windows-x86_64-setup.exe", "sanain-…-windows-x86_64-portable.zip"),
            ("macOS 11+ (Apple Silicon)", "sanain-…-macos-aarch64.dmg", "sanain-…-macos-aarch64-portable.zip"),
            ("Linux (x86-64)", "sanain_…_amd64.deb", "sanain-…-linux-x86_64-portable.tar.gz"),
        ],
        "dl_warn": "These builds are not signed yet. macOS asks for confirmation the first time (right-click Sanain, then Open); Windows SmartScreen may warn. Each file has a SHA-256 checksum next to it. Sanain is tested by hand on macOS; Windows and Linux are built and tested automatically.",
        "priv_h": "Privacy",
        "priv_p": "Offline by default. No account, no ads, no telemetry. File contents never leave your computer. The only network use is an update check you can turn on, which asks GitHub whether a newer version exists.",
        "made_h": "How it's made",
        "made_p": "AI-built. Human-led. Community-driven. Sanain is written by Claude, an AI model by Anthropic, working as the developer under the direction of the project's founder, who decides what it becomes. Every change goes through automated tests on three operating systems before it is merged, and the work is public on GitHub.",
        "support_h": "Support",
        "support_p": "Sanain is free, and stays free with every feature for everyone. Bug reports, ideas and translations are welcome on GitHub.",
        "issues": "Report a problem or suggest an idea",
        "footer": "Sanain is free software under the Mozilla Public License 2.0.",
    }

}

def e(s):
    return html.escape(s, quote=True)

def page(t):
    r = t["root"]
    ids = ["download", "features", "privacy", "made"]
    nav = "".join(f'<a href="#{i}">{e(n)}</a>' for i, n in zip(ids, t["nav"]))
    feats = "".join(f"<div><h3>{e(h)}</h3><p>{e(p)}</p></div>" for h, p in t["feats"])
    shots = "".join(
        f'<figure><img src="{r}screenshots/{f}" alt="{e(c)}" loading="lazy" width="1200" height="800"><figcaption>{e(c)}</figcaption></figure>'
        for f, c in t["shots"]
    )
    rows = "".join(f"<tr><td>{e(a)}</td><td><code>{e(b)}</code></td><td><code>{e(c)}</code></td></tr>" for a, b, c in t["dl_rows"])
    st = t["status"]
    b0, b1 = t["bytes_label"]
    return f"""<!doctype html>
<html lang="{t['lang']}">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{e(t['title'])}</title>
<meta name="description" content="{e(t['desc'])}">
<link rel="icon" href="{r}icon.svg" type="image/svg+xml">
<link rel="stylesheet" href="{r}style.css">
</head>
<body>
<div class="wrap">
<header class="top">
  <a class="brand" href="./"><img src="{r}icon.svg" alt="">Sanain</a>
  <nav class="top">{nav}</nav>
</header>

<div class="buffer" role="img" aria-label="{e(t['hero_l1'] + ' ' + t['hero_l2'] + ' ' + t['hero_sub'])}">
  <div class="tabs"><span class="tab on">{e(t['tab'])}</span><span class="tab">notes.txt</span><span class="tab">config.toml</span></div>
  <pre><span class="gutter">1<br>2<br><br>3</span><code><span class="kw">#</span> {e(t['hero_l1'])}
{e(t['hero_l2'])}<span class="caret"></span>

<span class="dim">{e(t['hero_sub'])}</span></code></pre>
  <div class="status"><span><b>{e(st[0])}</b></span><span>{e(st[1])}</span><span>{e(st[2])}</span><span>{e(st[3])}</span></div>
</div>
<div class="cta">
  <a class="btn" href="{NIGHTLY}">{e(t['cta'])}</a>
  <a class="btn ghost" href="{REPO}">{e(t['src'])}</a>
  <span class="note">{e(t['cta_note'])}</span>
</div>

<section id="intact">
  <h2>{e(t['intact_h'])}</h2>
  <p class="lede">{e(t['intact_p'])}</p>
  <div class="bytes" aria-hidden="true">{e(b0)}  63 61 66 c3 a9 0d 0a   <span class="same">c a f é \\r \\n</span>   UTF-8 · CRLF
{e(b1)}  63 61 66 c3 a9 0d 0a   <span class="same">c a f é \\r \\n</span>   UTF-8 · CRLF</div>
</section>

<section id="features">
  <h2>{e(t['feat_h'])}</h2>
  <div class="grid">{feats}</div>
</section>

<section id="screens">
  <h2>{e(t['shots_h'])}</h2>
  <div class="shots">{shots}</div>
</section>

<section id="download">
  <h2>{e(t['dl_h'])}</h2>
  <p>{e(t['dl_p'])} <a href="{NIGHTLY}">{NIGHTLY.replace('https://', '')}</a></p>
  <div class="table-scroll"><table class="dl"><thead><tr><th>{e(t['dl_head'][0])}</th><th>{e(t['dl_head'][1])}</th><th>{e(t['dl_head'][2])}</th></tr></thead><tbody>{rows}</tbody></table></div>
  <p class="warn">{e(t['dl_warn'])}</p>
</section>

<section id="privacy">
  <h2>{e(t['priv_h'])}</h2>
  <p>{e(t['priv_p'])}</p>
</section>

<section id="made">
  <h2>{e(t['made_h'])}</h2>
  <p>{e(t['made_p'])}</p>
</section>

<section id="support">
  <h2>{e(t['support_h'])}</h2>
  <p>{e(t['support_p'])} <a href="{REPO}/issues">{e(t['issues'])}</a></p>
</section>
</div>
<footer><div class="wrap">{e(t['footer'])} <a href="{REPO}">GitHub</a> · <a href="{REPO}/blob/main/PRIVACY.md">PRIVACY.md</a></div></footer>
</body>
</html>
"""

for key, out in (("en", "index.html"),):
    path = os.path.join(HERE, out)
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        f.write(page(T[key]))
    print(path)
