# Releases and versioning

## Versioning
Semantic Versioning, starting at `0.0.1`. The single source of truth is
`[workspace.package] version` in `Cargo.toml`; all crates inherit it.

- `0.0.x`: pre-alpha development builds. Anything may change.
- `0.y.0`: bump the minor version at milestone exits (M1 → 0.1.0, …).
- Patch releases for fixes within a milestone.
- `1.0.0` only after M7 gates: all P0 acceptance tests, no known data-loss
  or security blockers, verified artifacts and owner approval.

Tag format `vX.Y.Z`. CHANGELOG.md, tag and manifest versions must match.

## Distribution
Candidate targets: Windows x64/ARM64, macOS Apple Silicon/Intel, Linux x64.
Advertise only tested targets. Windows installer and portable build; macOS
signed and notarized bundle; Linux archive or AppImage first. Verify signing
costs, store requirements and licenses before committing to a channel.

Binary releases must ship a third-party licence notices file (BSD-2-Clause,
BSD-3-Clause and MIT dependencies require it); generate it in CI (e.g. cargo-about).

Generate it with `scripts/third-party-notices.py THIRD_PARTY_NOTICES.md`
(crates linked into the app, all platforms, with their license texts;
MIT notices generated from crate authors where the package omits the
file; Apache-2.0 and BSL-1.0 full texts appended once). CI runs it on
every pull request and fails if any crate lacks license text. License,
advisory and source policy: `deny.toml`, checked by cargo-deny in CI.

Packaging: `scripts/package.py` builds, for the current platform, an
unsigned installer (Windows: per-user Inno Setup `-setup.exe`, no admin
rights, optional Open With registration; macOS: `.dmg`; Linux: `.deb`)
and a portable archive (`-portable`, with a `text3-portable` marker that
keeps settings and data in a `data` folder next to the program) in
`target/dist/` (macOS: `Text³.app` in a zip;
Windows: zip; Linux: tar.gz) with LICENSE, README, CHANGELOG, the
third-party notices and a `.sha256` file. The `Package` workflow runs it
on Linux, Windows and macOS for `v*` tags and on manual dispatch, and
keeps the archives as workflow artifacts for 14 days. Since 2026-11-26
(D23) it also runs on every push to main and publishes them: a `publish`
job (the only one with `contents: write`, pushes only) replaces the
`nightly` pre-release, or creates a pre-release for a `v*` tag, with the
archives and `SHA256SUMS`. Still missing for a stable release: signing
and notarization, an SBOM, and a Linux desktop entry and icon.

CI builds releases from tagged commits with the lockfile and produces
checksums, SBOM, notices and release notes. The owner approves publishing.
Updates: Text³ can check GitHub for a newer release (manual, or once a
day if the user opts in) and links to the release page. It never
downloads or installs anything itself. Automatic installation waits for
signed releases, a signed update manifest, rollback and a threat model;
those need signing keys and owner approval (docs/AUTONOMY.md).

CLI: `text3 [FILE…]`, `text3 --diff LEFT RIGHT`, `--version`, `--help`,
`--register`/`--unregister` (file manager Open With, per user)
(`--` ends options). Proposed: `--new`; `--tail` later. No shell interpolation of paths. Importing Notepad++
settings requires consent and never modifies the originals.
