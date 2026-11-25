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

CI builds releases from tagged commits with the lockfile and produces
checksums, SBOM, notices and release notes. The owner approves publishing.
No auto-updater until a signed manifest, rollback and threat model exist.

CLI (proposed): `sanain <path>`, `sanain --diff a b`, `sanain --new`;
`--tail` later. No shell interpolation of paths. Importing Notepad++
settings requires consent and never modifies the originals.
