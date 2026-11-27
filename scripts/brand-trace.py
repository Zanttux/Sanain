#!/usr/bin/env python3
"""Trace the approved Text³ artwork into flat-fill SVG masters.

One-off/maintenance tool, not part of the build. Needs Pillow
(`python3 -m pip install pillow`) and vtracer (`cargo install vtracer`,
MIT). Inputs are the owner-supplied rasters in assets/brand (never
modified); outputs are assets/brand/text3-{lockup,mascot,document}.svg.

Steps: key out the page background (flood fill from the border), drop the
swoosh for the mascot-only master, trace with vtracer (stacked spline
paths), then snap the traced colours to the artwork's flat palette, which
removes the source's incidental shading (BRAND.md). No raster is embedded.

Usage: python3 scripts/brand-trace.py [--vtracer PATH]
"""

import os
import re
import shutil
import subprocess
import sys
import tempfile

from PIL import Image, ImageDraw

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
BRAND = os.path.join(ROOT, "assets", "brand")
REFERENCE = os.path.join(BRAND, "approved-reference.png")
DOCUMENT = os.path.join(BRAND, "drafts", "icons", "document-256.png")

# Flat palette sampled from the approved reference (cluster means of the
# traced fills). Not the proposed UI tokens: the artwork keeps its colours.
PALETTE = {
    "ink": (0x22, 0x1E, 0x13),
    "cream": (0xFB, 0xF1, 0xE1),
    "eye": (0xF6, 0xF5, 0xF3),
    "ochre": (0xEC, 0x9C, 0x31),
    "tan": (0xE6, 0xC5, 0x95),
    "line": (0x73, 0x7D, 0x51),
    "check": (0x72, 0x67, 0x4E),
    "green": (0x6F, 0x81, 0x1B),
}
MAGENTA = (255, 0, 255, 255)


def key_background(im, seeds_extra=(), barriers=()):
    """Make the page background (cream connected to the border) transparent."""
    im = im.convert("RGBA")
    w, h = im.size
    guide = im.copy()
    d = ImageDraw.Draw(guide)
    for line in barriers:
        d.line(line, fill=(20, 20, 20, 255), width=5)
    seeds = [(x, 0) for x in range(0, w, 16)] + [(x, h - 1) for x in range(0, w, 16)]
    seeds += [(0, y) for y in range(0, h, 16)] + [(w - 1, y) for y in range(0, h, 16)]
    seeds += list(seeds_extra)
    for s in seeds:
        r, g, b, a = guide.getpixel(s)
        if (r, g, b, a) != MAGENTA and a > 0 and r > 230 and g > 225 and b > 210:
            ImageDraw.floodfill(guide, s, MAGENTA, thresh=40)
    gp, px = guide.load(), im.load()
    for y in range(h):
        for x in range(w):
            if gp[x, y] == MAGENTA or px[x, y][3] == 0:
                px[x, y] = (0, 0, 0, 0)
    return im


def drop_swoosh(im, below):
    """Remove the saturated green swoosh under the mascot."""
    px = im.load()
    w, h = im.size
    for y in range(below, h):
        for x in range(w):
            r, g, b, a = px[x, y]
            if a and g > r and g > b + 15:
                px[x, y] = (0, 0, 0, 0)
    return im


def drop_specks(im, min_size):
    """Clear opaque islands smaller than min_size pixels (swoosh and
    background remnants); the artwork itself is a few large islands."""
    w, h = im.size
    px = im.load()
    seen = bytearray(w * h)
    for start in range(w * h):
        if seen[start] or px[start % w, start // w][3] == 0:
            continue
        stack, island = [start], []
        seen[start] = 1
        while stack:
            i = stack.pop()
            island.append(i)
            x, y = i % w, i // w
            for nx, ny in ((x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)):
                j = ny * w + nx
                if 0 <= nx < w and 0 <= ny < h and not seen[j] and px[nx, ny][3]:
                    seen[j] = 1
                    stack.append(j)
        if len(island) < min_size:
            for i in island:
                px[i % w, i // w] = (0, 0, 0, 0)
    return im


def snap(svg):
    def nearest(m):
        c = tuple(int(m.group(1)[i:i + 2], 16) for i in (0, 2, 4))
        name = min(PALETTE, key=lambda k: sum((a - b) ** 2 for a, b in zip(PALETTE[k], c)))
        return 'fill="#%02X%02X%02X"' % PALETTE[name]
    return re.sub(r'fill="#([0-9A-Fa-f]{6})"', nearest, svg)


def trace(vtracer, png, out, title, box):
    with tempfile.TemporaryDirectory() as tmp:
        raw = os.path.join(tmp, "raw.svg")
        subprocess.run(
            [vtracer, "--input", png, "--output", raw, "--colormode", "color",
             "--mode", "spline", "--filter_speckle", "8", "--color_precision", "5",
             "--gradient_step", "24", "--corner_threshold", "60",
             "--segment_length", "4", "--splice_threshold", "45"],
            check=True, stdout=subprocess.DEVNULL,
        )
        with open(raw, encoding="utf-8") as f:
            svg = f.read()
    body = svg[svg.index("<path"):svg.rindex("</svg>")]
    x, y, w, h = box
    head = (
        '<?xml version="1.0" encoding="UTF-8"?>\n'
        f"<!-- {title}. Traced from assets/brand/approved-reference.png by "
        "scripts/brand-trace.py (vtracer, colours snapped to the flat artwork "
        "palette). Draft vector master: a designer should review it before it "
        "replaces the raster as the source. -->\n"
        f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="{x} {y} {w} {h}" '
        f'width="{w}" height="{h}">\n<title>{title}</title>\n'
    )
    with open(out, "w", encoding="utf-8") as f:
        f.write(head + snap(body).replace("/>", "/>\n").replace("\n\n", "\n") + "</svg>\n")
    print(out)


def padded_bbox(im, pad):
    x0, y0, x1, y1 = im.getbbox()
    return (x0 - pad, y0 - pad, x1 - x0 + 2 * pad, y1 - y0 + 2 * pad)


def main():
    vtracer = shutil.which("vtracer")
    if "--vtracer" in sys.argv:
        vtracer = sys.argv[sys.argv.index("--vtracer") + 1]
    if not vtracer:
        sys.exit("vtracer not found (cargo install vtracer)")
    ref = Image.open(REFERENCE)
    # The tail-tip curl is open towards the page; close its mouth so the
    # cream fur stays opaque. Also clear the counter of the "e".
    keyed = key_background(ref, seeds_extra=[(1000, 693)], barriers=[[(126, 904), (152, 858)]])
    keyed = drop_specks(keyed, 400)
    with tempfile.TemporaryDirectory() as tmp:
        lockup = os.path.join(tmp, "lockup.png")
        keyed.save(lockup)
        trace(vtracer, lockup, os.path.join(BRAND, "text3-lockup.svg"),
              "Text³ lockup", padded_bbox(keyed, 16))

        mascot = keyed.copy()
        ImageDraw.Draw(mascot).rectangle([800, 0, ref.width, ref.height], fill=(0, 0, 0, 0))
        mascot = drop_specks(drop_swoosh(mascot, 860), 5000)
        m = os.path.join(tmp, "mascot.png")
        mascot.save(m)
        trace(vtracer, m, os.path.join(BRAND, "text3-mascot.svg"),
              "Text³ mascot", padded_bbox(mascot, 8))

        doc = Image.open(DOCUMENT).convert("RGBA")
        d = os.path.join(tmp, "document.png")
        doc.save(d)
        trace(vtracer, d, os.path.join(BRAND, "text3-document.svg"),
              "Text³ document glyph", (0, 0, doc.width, doc.height))
    return 0


if __name__ == "__main__":
    sys.exit(main())
