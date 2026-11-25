# Sanain
**The power of words.**

Sanain is a modern, native, open-source text editor in development built in Rust for Windows, macOS and Linux. Classic file-first editing meets first-class Markdown, structured file views and offline text transformations. Fast by design. Powerful when needed. Simple by default.

**AI-built. Human-led. Community-driven.** Claude is designated as the primary developer and maintainer, responsible for engineering, testing, documentation, releases and community operations under human leadership. Human contributions are welcome. We disclose actual AI involvement and do not claim that dependencies or community patches are AI-authored.

**Free. Forever. For everyone.** All features are intended to remain free; voluntary sponsorship supports development. No mandatory accounts, ads or telemetry.

## Status
Version 0.0.1, pre-alpha. A single-document editor window works: open,
type, select, undo, save, with byte-exact file handling and English and
Finnish UI. No tabs, search or session recovery yet. Tested by hand on
macOS only; Windows and Linux are covered by automated tests in CI. See docs/STATUS.md and docs/ROADMAP.md. 

## License
Mozilla Public License 2.0. See LICENSE.

## Building
Requires Rust via rustup; the toolchain version is pinned in
`rust-toolchain.toml`.

    cargo run --release -p sanain-app -- path/to/file.txt
    cargo test --workspace
    cargo clippy --workspace --all-targets -- -D warnings
