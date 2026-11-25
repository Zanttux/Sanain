# Sanain
**The power of words.**

Sanain is a modern, native, open-source text editor in development built in Rust for Windows, macOS and Linux. Classic file-first editing meets first-class Markdown, structured file views and offline text transformations. Fast by design. Powerful when needed. Simple by default.

**AI-built. Human-led. Community-driven.** Claude is designated as the primary developer and maintainer, responsible for engineering, testing, documentation, releases and community operations under human leadership. Human contributions are welcome. We disclose actual AI involvement and do not claim that dependencies or community patches are AI-authored.

**Free. Forever. For everyone.** All features are intended to remain free; voluntary sponsorship supports development. No mandatory accounts, ads or telemetry.

## Status
Version 0.0.1, pre-alpha. There is no application window yet. The document
model, byte-exact file loading and atomic saving, and localization are in
place and tested. See docs/STATUS.md and docs/ROADMAP.md. The license is
not yet chosen; the code is not yet available under an open-source license.

## Building
Requires Rust via rustup; the toolchain version is pinned in
`rust-toolchain.toml`.

    cargo test --workspace
    cargo clippy --workspace --all-targets -- -D warnings
