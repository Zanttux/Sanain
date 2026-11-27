#!/bin/sh
# Build Sanain.app (unsigned) into target/<profile>/Sanain.app.
# Usage: scripts/macos-bundle.sh [release|debug] [--open [file]]
set -eu
cd "$(dirname "$0")/.."
profile=${1:-release}
if [ "$profile" = release ]; then cargo build --release --locked -p text3-app; else cargo build --locked -p text3-app; fi
version=$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -1)
app="target/$profile/Sanain.app"
rm -rf "$app"
mkdir -p "$app/Contents/MacOS" "$app/Contents/Resources"
cp "target/$profile/sanain" "$app/Contents/MacOS/sanain"
cp packaging/icons/Sanain.icns "$app/Contents/Resources/Sanain.icns"
sed "s/@VERSION@/$version/g" packaging/macos/Info.plist > "$app/Contents/Info.plist"
echo "$app"
if [ "${2:-}" = --open ]; then
  shift 2
  open -n "$app" --args "$@"
fi
