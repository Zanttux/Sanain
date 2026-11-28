# Mac App Store support build (#79)

Owner request (2026-11-28): a paid "support build" of Text³ in the Mac App
Store, prepared completely. The source stays MPL-2.0 and the free download
keeps every feature (#79); buying from the store is a convenient way to
support the project and get automatic updates.

## What differs from the free download
| | Free download (Developer ID) | Mac App Store |
|---|---|---|
| Bundle id | `io.github.zanttux.sanain` (kept from Sanain: upgrades keep settings) | `io.github.zanttux.text3` (new store app) |
| Build | default features | `--features store` |
| Updates | optional GitHub check (off by default) | the App Store; no update check, no network |
| Sandbox | no | yes: files the user opens (open panel, Finder, drag and drop) |
| Default apps, `--register` | offered | not offered (handled by macOS) |
| Signing | Developer ID + notarization | Apple Distribution + Mac Installer Distribution, App Store review |

The store app and the download are separate apps with separate settings.

## Done (2026-11-28)
- `store` Cargo feature and `text3_app::STORE`: hides the update check
  (menus, palette, Settings, start-up), the default-app offer and shell
  registration; CLI explains instead.
- `packaging/macos/store.entitlements`: app sandbox,
  user-selected read-write files, app-scoped bookmarks. No network.
- `scripts/package-mas.sh`: builds the store flavour, sets the bundle id
  and build number, embeds the provisioning profile, signs the app with
  entitlements and the hardened runtime, wraps it in a signed `.pkg`.
- Info.plist: `LSApplicationCategoryType` (Developer Tools) and
  `ITSAppUsesNonExemptEncryption = false` (HTTPS only).
- Apple side (owner signed in to Chrome): App ID
  `io.github.zanttux.text3`; Apple Distribution and Mac Installer
  Distribution certificates (keys generated locally, never uploaded);
  provisioning profile "Text3 Mac App Store"; App Store Connect app record
  **Text³** (macOS, English (U.S.), SKU `text3-macos`). The Paid Apps
  agreement, bank account, tax forms and EU trader (DSA) status were
  already active.
- Checked: Text³ starts and draws in the sandbox (325 ms to the first
  window launched through Launch Services).

## Still to do
Engineering (Claude):
1. ~~Security-scoped bookmarks for Open Recent, session restore and hot
   exit~~ (done: `text3_app::bookmarks`).
2. ~~Show in Finder through NSWorkspace~~ (done).
3. Find in Folder and Goto Anything on a folder the user chose (folder
   bookmark).
4. CI job for `v*` tags: build the store package and upload it to App Store
   Connect with the API key (secrets in the `signing` environment); review
   submission stays a manual step.
5. Store listing texts in English and Finnish, screenshots from a real
   build, privacy answers ("Data Not Collected"), age rating.

Owner decisions:
- **Price** (reserved: a financial decision). Proposal: one paid tier,
  for example 4.99 €, no in-app purchases, every feature identical to the
  free download.
- Submitting for review, once the build and listing are ready.

## Secrets
Keys and certificates live only in `~/.config/text3-signing` on the
owner's machine and in the GitHub environment `signing`
(docs/SECURITY_SECRETS.md). Nothing secret is in the repository.
