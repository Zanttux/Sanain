#!/bin/sh
# Regenerate the platform icons from assets/brand/icon.svg (macOS tools:
# qlmanage, sips, iconutil; python3 for the .ico container).
# Output: packaging/icons/{sanain-N.png, Sanain.icns, sanain.ico}
set -eu
cd "$(dirname "$0")/.."
out=packaging/icons
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
mkdir -p "$out"
# Full-bleed master (Windows, Linux, window icon).
qlmanage -t -s 1024 -o "$tmp" assets/brand/icon.svg >/dev/null
mv "$tmp/icon.svg.png" "$tmp/full.png"
# macOS master: Apple's grid leaves a ~10 % margin around the body.
sed 's/viewBox="0 0 128 128"/viewBox="-15.5 -15.5 159 159"/' assets/brand/icon.svg > "$tmp/mac.svg"
qlmanage -t -s 1024 -o "$tmp" "$tmp/mac.svg" >/dev/null
for n in 16 24 32 48 64 128 256 512 1024; do
  sips -z "$n" "$n" "$tmp/full.png" --out "$out/sanain-$n.png" >/dev/null
done
set=$tmp/Sanain.iconset
mkdir "$set"
for n in 16 32 128 256 512; do
  sips -z "$n" "$n" "$tmp/mac.svg.png" --out "$set/icon_${n}x${n}.png" >/dev/null
  d=$((n * 2))
  sips -z "$d" "$d" "$tmp/mac.svg.png" --out "$set/icon_${n}x${n}@2x.png" >/dev/null
done
iconutil -c icns "$set" -o "$out/Sanain.icns"
python3 - "$out" <<'PY'
import struct, sys
out = sys.argv[1]
sizes = [16, 24, 32, 48, 64, 128, 256]
blobs = [open(f"{out}/sanain-{n}.png", "rb").read() for n in sizes]
head = struct.pack("<HHH", 0, 1, len(sizes))
offset = 6 + 16 * len(sizes)
dirs = b""
for n, b in zip(sizes, blobs):
    dirs += struct.pack("<BBBBHHII", n % 256, n % 256, 0, 0, 1, 32, len(b), offset)
    offset += len(b)
open(f"{out}/sanain.ico", "wb").write(head + dirs + b"".join(blobs))
PY
ls -l "$out"
