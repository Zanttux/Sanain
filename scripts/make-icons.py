#!/usr/bin/env python3
"""Regenerate every derived Text³ brand raster from the SVG masters in
assets/brand (text3-mascot.svg, text3-lockup.svg, text3-document.svg).

Needs resvg on PATH (`cargo install resvg`, MPL-2.0) and, for the macOS
.icns, BMP and TIFF outputs, the macOS tools iconutil, sips and tiffutil.
No Python packages beyond the standard library.

Outputs:
  packaging/icons/text3-N.png            app icon (Windows, Linux, window)
  packaging/icons/text3.ico              Windows executable/installer icon
  packaging/icons/text3.icns             macOS app icon (white plate)
  packaging/icons/text3-document-N.png   document glyph
  packaging/icons/text3-document.icns    macOS document icon
  packaging/windows/wizard-*.bmp         Inno Setup wizard artwork
  packaging/macos/dmg-background.tiff    DMG window background (1x + 2x)
  assets/brand/about-lockup.png          About dialog artwork (embedded)
  assets/brand/text3-lockup-plate.svg    lockup on a white plate (dark surfaces)
  assets/brand/text3-lockup-mono.svg     one-colour lockup (ink on white)
  assets/brand/social-preview.png        GitHub social preview (1280x640)
  website/favicon-32.png, icon-192.png, apple-touch-icon.png

Small sizes are hinted, not just scaled: 16 and 24 px show the face only,
32 px the head with ear tufts, each with a 1 px white outline so the dark
contour stays visible on dark taskbars and panels (docs/BRAND.md).

Usage: python3 scripts/make-icons.py
"""

import os
import re
import shutil
import struct
import subprocess
import sys
import tempfile

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
BRAND = os.path.join(ROOT, "assets", "brand")
ICONS = os.path.join(ROOT, "packaging", "icons")
WHITE, GRAY, BORDER = "#FFFFFF", "#F4F4F4", "#D4D4D4"
ARROW = "#9A9A9A"  # neutral; the DMG arrow must read on white
# The "Text³" wordmark inside the lockup (viewBox units), without the mascot.
WORDMARK = (795, 515, 690, 315)

# Regions of the mascot master (reference pixel coordinates), square.
HEAD = (228, 52, 520, 520)   # ears, tufts, face: 32 px
FACE = (282, 150, 420, 420)  # ears without tuft tips, face: 16 and 24 px


def master(name):
    with open(os.path.join(BRAND, name), encoding="utf-8") as f:
        svg = f.read()
    vb = tuple(float(v) for v in re.search(r'viewBox="([^"]+)"', svg).group(1).split())
    body = svg[svg.index("<path"):svg.rindex("</svg>")]
    return vb, body


def nested(art, x, y, w, h, view=None):
    vb, body = art
    vx, vy, vw, vh = view or vb
    return (f'<svg x="{x}" y="{y}" width="{w}" height="{h}" viewBox="{vx} {vy} {vw} {vh}" '
            f'preserveAspectRatio="xMidYMid meet">{body}</svg>')


def doc(w, h, inner, defs=""):
    return (f'<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" '
            f'viewBox="0 0 {w} {h}"><defs>{defs}</defs>{inner}</svg>\n')


HALO = ('<filter id="halo" x="-10%" y="-10%" width="120%" height="120%">'
        '<feMorphology in="SourceAlpha" operator="dilate" radius="{r}" result="d"/>'
        '<feFlood flood-color="#FFFFFF"/><feComposite in2="d" operator="in"/>'
        '<feMerge><feMergeNode/><feMergeNode in="SourceGraphic"/></feMerge></filter>')


def render(svg, out, tmp, width=None):
    src = os.path.join(tmp, "in.svg")
    with open(src, "w", encoding="utf-8") as f:
        f.write(svg)
    cmd = ["resvg", src, out]
    if width:
        cmd[1:1] = ["-w", str(width)]
    subprocess.run(cmd, check=True)


def app_icon(mascot, n):
    """Transparent app icon of n px (hinted crops below 48 px)."""
    if n >= 48:
        m = n / 16  # 1/16 clear space on each side
        return doc(n, n, nested(mascot, m, m, n - 2 * m, n - 2 * m))
    view = FACE if n <= 24 else HEAD
    r = 1.0
    return doc(n, n, f'<g filter="url(#halo)">{nested(mascot, r, r, n - 2 * r, n - 2 * r, view)}</g>',
               HALO.format(r=r))


def mac_icon(mascot, n):
    """macOS icon: white plate on Apple's grid (824/1024), mascot inside."""
    k = n / 1024
    x, s, rad = 100 * k, 824 * k, 185.4 * k
    stroke = max(1.0, 2 * k)
    plate = (f'<rect x="{x + stroke / 2}" y="{x + stroke / 2}" width="{s - stroke}" height="{s - stroke}" '
             f'rx="{rad}" fill="{WHITE}" stroke="{BORDER}" stroke-width="{stroke}"/>')
    if n <= 32:
        art = nested(mascot, x + s * 0.08, x + s * 0.08, s * 0.84, s * 0.84, FACE if n <= 16 else HEAD)
    else:
        art = nested(mascot, x + s / 8, x + s / 8, s * 0.75, s * 0.75)
    return doc(n, n, plate + art)


def write_ico(pngs, out):
    blobs = [open(p, "rb").read() for _, p in pngs]
    head = struct.pack("<HHH", 0, 1, len(pngs))
    offset = 6 + 16 * len(pngs)
    dirs = b""
    for (n, _), b in zip(pngs, blobs):
        dirs += struct.pack("<BBBBHHII", n % 256, n % 256, 0, 0, 1, 32, len(b), offset)
        offset += len(b)
    with open(out, "wb") as f:
        f.write(head + dirs + b"".join(blobs))


def iconset(make, out, tmp, sizes=(16, 32, 128, 256, 512)):
    s = os.path.join(tmp, "x.iconset")
    shutil.rmtree(s, ignore_errors=True)
    os.makedirs(s)
    for n in sizes:
        render(make(n), os.path.join(s, f"icon_{n}x{n}.png"), tmp)
        render(make(2 * n), os.path.join(s, f"icon_{n}x{n}@2x.png"), tmp)
    subprocess.run(["iconutil", "-c", "icns", s, "-o", out], check=True)


def bmp(svg, out, tmp):
    """Render `svg` to a 24-bit, bottom-up BMP on white. Inno Setup draws
    the top-down 32-bit BMPs that sips writes (negative height) upside
    down (#167), so the file is written here row by row instead."""
    png = os.path.join(tmp, "b.png")
    raw = os.path.join(tmp, "b.bmp")
    render(svg, png, tmp)
    subprocess.run(["sips", "-s", "format", "bmp", png, "--out", raw],
                   check=True, stdout=subprocess.DEVNULL)
    data = open(raw, "rb").read()
    offset, = struct.unpack_from("<I", data, 10)
    hsize, w, h = struct.unpack_from("<Iii", data, 14)
    bpp, comp = struct.unpack_from("<HI", data, 28)
    if bpp != 32 or comp not in (0, 3):
        sys.exit(f"unexpected BMP from sips: {bpp} bpp, compression {comp}")
    masks = struct.unpack_from("<IIII", data, 54) if hsize >= 56 else (
        0x00FF0000, 0x0000FF00, 0x000000FF, 0xFF000000)
    if comp == 0:
        masks = (0x00FF0000, 0x0000FF00, 0x000000FF, 0xFF000000)

    def chan(px, m):
        if not m:
            return 255
        shift = (m & -m).bit_length() - 1
        return (px & m) >> shift

    top_down = h < 0
    h = abs(h)
    rows = []
    for y in range(h):
        src = y if top_down else h - 1 - y
        row = bytearray()
        for x in range(w):
            px, = struct.unpack_from("<I", data, offset + 4 * (src * w + x))
            r, g, b, a = (chan(px, m) for m in masks)
            # Composite over white (sips keeps straight alpha).
            row += bytes((b * a + 255 * (255 - a)) // 255 for b in (b, g, r))
        row += b"\0" * ((4 - len(row) % 4) % 4)
        rows.append(bytes(row))
    pixels = b"".join(reversed(rows))  # bottom-up
    header = struct.pack("<2sIHHI", b"BM", 54 + len(pixels), 0, 0, 54)
    info = struct.pack("<IiiHHIIiiII", 40, w, h, 1, 24, 0, len(pixels), 2835, 2835, 0, 0)
    with open(out, "wb") as f:
        f.write(header + info + pixels)


def lockup_on(lockup, w, h, lw, x=None, y=None, bg=WHITE):
    vb = lockup[0]
    lh = lw * vb[3] / vb[2]
    x = (w - lw) / 2 if x is None else x
    y = (h - lh) / 2 if y is None else y
    back = f'<rect width="{w}" height="{h}" fill="{bg}"/>' if bg else ""
    return doc(w, h, back + nested(lockup, x, y, lw, lh))


def main():
    if not shutil.which("resvg"):
        sys.exit("resvg not found (cargo install resvg)")
    mascot = master("text3-mascot.svg")
    lockup = master("text3-lockup.svg")
    document = master("text3-document.svg")
    mac = sys.platform == "darwin"
    with tempfile.TemporaryDirectory() as tmp:
        os.makedirs(ICONS, exist_ok=True)
        sizes = [16, 24, 32, 48, 64, 128, 256, 512, 1024]
        for n in sizes:
            render(app_icon(mascot, n), os.path.join(ICONS, f"text3-{n}.png"), tmp)
        write_ico([(n, os.path.join(ICONS, f"text3-{n}.png")) for n in sizes if n <= 256],
                  os.path.join(ICONS, "text3.ico"))
        def document_icon(n):
            return doc(n, n, nested(document, 0, 0, n, n))
        for n in (16, 32, 48, 64, 128, 256):
            render(document_icon(n), os.path.join(ICONS, f"text3-document-{n}.png"), tmp)

        # Lockup variants (vector).
        vb = lockup[0]
        pad = vb[2] / 16
        pw, ph = vb[2] + 2 * pad, vb[3] + 2 * pad
        plate = doc(pw, ph, f'<rect width="{pw}" height="{ph}" rx="{pad}" fill="{WHITE}"/>'
                    + nested(lockup, pad, pad, vb[2], vb[3]))
        with open(os.path.join(BRAND, "text3-lockup-plate.svg"), "w", encoding="utf-8") as f:
            f.write(plate)
        ink = "#221E13"
        mono = re.sub(r'fill="#([0-9A-F]{6})"',
                      lambda m: 'fill="%s"' % (ink if m.group(1) in ("221E13", "6F811B") else WHITE),
                      open(os.path.join(BRAND, "text3-lockup.svg"), encoding="utf-8").read())
        mono = mono.replace("<title>Text³ lockup</title>", "<title>Text³ lockup, one colour</title>")
        with open(os.path.join(BRAND, "text3-lockup-mono.svg"), "w", encoding="utf-8") as f:
            f.write(mono)

        render(lockup_on(lockup, 640, 400, 640, 0, 0, bg=None).replace('height="400"', f'height="{640 * vb[3] / vb[2]:.0f}"').replace("0 0 640 400", f"0 0 640 {640 * vb[3] / vb[2]:.0f}"),
               os.path.join(BRAND, "about-lockup.png"), tmp)
        render(lockup_on(lockup, 1280, 640, 760), os.path.join(BRAND, "social-preview.png"), tmp)

        web = os.path.join(ROOT, "website")
        render(app_icon(mascot, 32), os.path.join(web, "favicon-32.png"), tmp)
        render(app_icon(mascot, 192), os.path.join(web, "icon-192.png"), tmp)
        render(doc(180, 180, f'<rect width="180" height="180" fill="{WHITE}"/>'
                   + nested(mascot, 18, 18, 144, 144)), os.path.join(web, "apple-touch-icon.png"), tmp)

        if not mac:
            print("not macOS: skipped .icns, BMP and TIFF outputs", file=sys.stderr)
            return 0
        iconset(lambda n: mac_icon(mascot, n), os.path.join(ICONS, "text3.icns"), tmp)
        iconset(document_icon, os.path.join(ICONS, "text3-document.icns"), tmp, (16, 32, 128))

        # Inno Setup wizard, laid out like the NSIS Modern UI installers
        # (Notepad++): a full-height panel on the Welcome and Finished pages
        # with the mascot above the wordmark, and the head in the top-right
        # corner of the inner pages. Neutral gray panel (docs/BRAND.md).
        win = os.path.join(ROOT, "packaging", "windows")
        for w, h, tag in ((164, 314, "100"), (328, 628, "200")):
            mw = w * 0.80
            mh = mw * mascot[0][3] / mascot[0][2]
            ww = w * 0.62
            wh = ww * WORDMARK[3] / WORDMARK[2]
            top = h * 0.24
            art = (f'<rect width="{w}" height="{h}" fill="{GRAY}"/>'
                   + nested(mascot, (w - mw) / 2, top, mw, mh)
                   + nested(lockup, (w - ww) / 2, top + mh + h * 0.04, ww, wh, WORDMARK))
            bmp(doc(w, h, art), os.path.join(win, f"wizard-image-{tag}.bmp"), tmp)
        for w, h, tag in ((55, 58, "100"), (110, 116, "200")):
            m = w * 0.04
            bmp(doc(w, h, f'<rect width="{w}" height="{h}" fill="{WHITE}"/>'
                    + nested(mascot, m, m, w - 2 * m, h - 2 * m, HEAD)),
                os.path.join(win, f"wizard-small-{tag}.bmp"), tmp)

        # DMG window, 660x400 content, in the same style as the Windows
        # wizard: a full-height gray panel on the left with the mascot above
        # the wordmark; on white, the app and the Applications link (placed
        # by scripts/package.py at x=330 and x=550, y=190) with an arrow
        # between them. No text, so the window needs no translation.
        one, two = os.path.join(tmp, "dmg.png"), os.path.join(tmp, "dmg@2x.png")
        pw, h = 220, 400
        mw = 170
        mh = mw * mascot[0][3] / mascot[0][2]
        ww = 128
        wh = ww * WORDMARK[3] / WORDMARK[2]
        top = (h - mh - 14 - wh) / 2
        panel = (f'<rect width="{pw}" height="{h}" fill="{GRAY}"/>'
                 f'<rect x="{pw}" width="1" height="{h}" fill="{BORDER}"/>'
                 + nested(mascot, (pw - mw) / 2, top, mw, mh)
                 + nested(lockup, (pw - ww) / 2, top + mh + 14, ww, wh, WORDMARK))
        arrow = (f'<g fill="none" stroke="{ARROW}" stroke-width="4" stroke-linecap="round" '
                 f'stroke-linejoin="round"><path d="M398 190 H480"/><path d="M464 173 L483 190 L464 207"/></g>')
        bg = doc(660, h, f'<rect width="660" height="{h}" fill="{WHITE}"/>' + panel + arrow)
        render(bg, one, tmp)
        render(bg, two, tmp, width=1320)
        subprocess.run(["tiffutil", "-cathidpicheck", one, two, "-out",
                        os.path.join(ROOT, "packaging", "macos", "dmg-background.tiff")],
                       check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    print("done")
    return 0


if __name__ == "__main__":
    sys.exit(main())
