#!/bin/sh
# Build the Mac App Store package (#79, docs/APP_STORE.md):
# Text³.app with --features store, sandbox entitlements and the App Store
# provisioning profile, signed with an Apple Distribution identity, wrapped
# in a .pkg signed with a Mac Installer Distribution identity.
#
# Environment (nothing secret is stored in the repository):
#   TEXT3_MAS_APP_IDENTITY        "Apple Distribution: …" or its SHA-1
#   TEXT3_MAS_INSTALLER_IDENTITY  "3rd Party Mac Developer Installer: …"
#                                 or "Mac Installer Distribution: …"
#   TEXT3_MAS_PROFILE             path to the .provisionprofile
#   TEXT3_TEAM_ID                 Apple team ID
#   TEXT3_MAS_BUILD               build number, unique per upload (default 1)
# Output: target/dist/text3-<version>-mas.pkg
set -eu
cd "$(dirname "$0")/.."
: "${TEXT3_MAS_APP_IDENTITY:?}" "${TEXT3_MAS_INSTALLER_IDENTITY:?}" "${TEXT3_MAS_PROFILE:?}" "${TEXT3_TEAM_ID:?}"
version=$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -1)
# The ad-hoc seal from the bundle script is replaced below.
TEXT3_CARGO_FEATURES=store TEXT3_SIGN_IDENTITY= sh scripts/macos-bundle.sh release >/dev/null
app="target/release/Text³.app"
work="target/mas"
rm -rf "$work"
mkdir -p "$work" target/dist
cp -R "$app" "$work/"
app="$work/Text³.app"
cp "$TEXT3_MAS_PROFILE" "$app/Contents/embedded.provisionprofile"
# The store app has its own identity (io.github.zanttux.text3, docs/APP_STORE.md);
# the Developer ID download keeps Sanain's id so upgrades keep their settings.
plist="$app/Contents/Info.plist"
plutil -replace CFBundleIdentifier -string io.github.zanttux.text3 "$plist"
# Every upload needs a new build number: TEXT3_MAS_BUILD (CI: run number).
plutil -replace CFBundleVersion -string "${TEXT3_MAS_BUILD:-1}" "$plist"
# Apple Silicon only, which the store allows from macOS 12 (validation 90869).
plutil -replace LSMinimumSystemVersion -string 12.0 "$plist"
plutil -replace CFBundleIconFile -string text3.icns "$plist"
# Everything readable by everyone: productbuild installs as root, and a
# root-only file breaks signature checks (validation 90255).
chmod -R u+rwX,go+rX,go-w "$app"
sed "s/@TEAM@/$TEXT3_TEAM_ID/g" packaging/macos/store.entitlements > "$work/store.entitlements"
codesign --force --timestamp --options runtime \
  --entitlements "$work/store.entitlements" --sign "$TEXT3_MAS_APP_IDENTITY" "$app"
codesign --verify --strict "$app"
codesign -d --entitlements - "$app" | grep -q com.apple.security.app-sandbox
pkg="target/dist/text3-$version-mas.pkg"
productbuild --component "$app" /Applications --sign "$TEXT3_MAS_INSTALLER_IDENTITY" "$pkg"
pkgutil --check-signature "$pkg" | head -3
echo "$pkg"
