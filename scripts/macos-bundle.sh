#!/bin/sh
# Build Text³.app (unsigned) into target/<profile>/Text³.app.
# Usage: scripts/macos-bundle.sh [release|debug] [--open [file]]
set -eu
cd "$(dirname "$0")/.."
profile=${1:-release}
if [ "$profile" = release ]; then cargo build --release --locked -p text3-app; else cargo build --locked -p text3-app; fi
version=$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -1)
app="target/$profile/Text³.app"
rm -rf "$app"
mkdir -p "$app/Contents/MacOS" "$app/Contents/Resources"
cp "target/$profile/text3" "$app/Contents/MacOS/text3"
cp packaging/icons/text3.icns packaging/icons/text3-document.icns "$app/Contents/Resources/"
# Bundled editor fonts with their licences (#176), loaded only when chosen.
rm -rf "$app/Contents/Resources/fonts"
cp -R assets/fonts "$app/Contents/Resources/fonts"
sed "s/@VERSION@/$version/g" packaging/macos/Info.plist > "$app/Contents/Info.plist"
# Seal the whole bundle with an ad-hoc signature. The linker signs only the
# executable; a downloaded (quarantined) bundle whose signature does not
# cover Info.plist and Resources is reported as "damaged" by Gatekeeper
# and cannot be opened at all (#166). With a sealed ad-hoc signature macOS
# instead offers Privacy & Security › Open Anyway for the unsigned app.
# With TEXT3_SIGN_IDENTITY (a Developer ID Application identity in the
# keychain; set only by the signed release job, #174) the bundle is signed
# for real with the hardened runtime and a secure timestamp instead.
if [ -n "${TEXT3_SIGN_IDENTITY:-}" ]; then
  codesign --force --options runtime --timestamp --sign "$TEXT3_SIGN_IDENTITY" "$app"
else
  codesign --force --sign - --timestamp=none "$app"
fi
codesign --verify --strict "$app"
echo "$app"
if [ "${2:-}" = --open ]; then
  shift 2
  open -n "$app" --args "$@"
fi
