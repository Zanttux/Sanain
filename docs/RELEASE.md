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

Packaging: `scripts/package.py` builds an unsigned archive for the
current platform in `target/dist/` (macOS: `Sanain.app` in a zip;
Windows: zip; Linux: tar.gz) with LICENSE, README, CHANGELOG, the
third-party notices and a `.sha256` file. The `Package` workflow runs it
on Linux, Windows and macOS for `v*` tags and on manual dispatch, and
keeps the archives as workflow artifacts for 14 days; it publishes
nothing. Still missing before a public release: signing and
notarization, an SBOM, and a Linux desktop entry and icon.

CI builds releases from tagged commits with the lockfile and produces
checksums, SBOM, notices and release notes. The owner approves publishing.
Updates: Sanain can check GitHub for a newer release (manual, or once a
day if the user opts in) and links to the release page. It never
downloads or installs anything itself. Automatic installation waits for
signed releases, a signed update manifest, rollback and a threat model;
those need signing keys and owner approval (docs/AUTONOMY.md).

CLI: `sanain [FILE…]`, `sanain --diff LEFT RIGHT`, `--version`, `--help`
(`--` ends options). Proposed: `--new`; `--tail` later. No shell interpolation of paths. Importing Notepad++
settings requires consent and never modifies the originals.
