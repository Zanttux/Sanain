# ADR 0001: GUI toolkit

- Date: 2026-11-25
- Status: Accepted — owner approved egui/eframe on 2026-11-25
- Decision ID: D07

## Context
The editor needs fast text rendering, IME, Unicode fallback, light/dark
themes and a working screen-reader tree on Windows, macOS and Linux (E07,
E12). Accessibility is a P0 release gate and is the hardest property to
retrofit into a toolkit.

## Evidence
Spike: one window, 100,000 virtualised lines with Latin, CJK and emoji, plus
a single-line input. Release builds, macOS 27.2.1, Apple M5, 24 GB, Rust
1.100.0. Single run each; not a benchmark.

| | GPUI 0.2.2 | egui/eframe 0.37 (wgpu, AccessKit, system fonts) |
|---|---|---|
| Publisher | Zed Industries (crates.io team) | emilk / Rerun |
| License | Apache-2.0 | MIT OR Apache-2.0 |
| Clean release build | not measured cleanly (first run failed at shader step) | 47 s |
| Binary (stripped) | 4.2 MB | 11 MB |
| First frame | 354 ms | 363 ms |
| RSS after 4 s | 113 MB | 137 MB |
| CJK / emoji fallback | yes | yes (`system_fonts` feature; tofu without it) |
| macOS accessibility tree | window buttons only | app content exposed (AccessKit) |
| Build prerequisite | Metal toolchain, or `runtime_shaders` feature | none |

Static checks (crates.io dependency lists): AccessKit is a dependency of
egui 0.37; it is not a dependency of GPUI 0.2.2 or Iced 0.14.
`gpui-component` depends on `gpui-pre`, a third-party republish of a Zed
snapshot; avoid it.

Not tested: IME composition, typing latency, Windows, Linux, VoiceOver
navigation depth, NVDA/Narrator, high-DPI on other platforms. Neither
built-in text widget is suitable for large documents; Text³ needs its own
editor widget over `text3-core` with either toolkit.

## Options
1. **egui/eframe** — screen-reader support exists today; mature, widely
   used; immediate mode makes a custom editor widget straightforward. Looks
   less native by default; needs careful theming and a custom editor view.
2. **GPUI** — best text rendering and smallest footprint; proven by Zed.
   No accessibility tree today, pre-1.0 API, Zed-driven roadmap.
3. **Iced** — not spiked; no AccessKit in 0.14.

## Recommendation
egui/eframe for M1, with the editor view written against a small internal
rendering boundary so the toolkit stays replaceable. Revisit GPUI if it
gains a screen-reader tree.

## Consequences and reversal
Core, IO and i18n crates are toolkit-independent and unaffected. Switching
later costs the UI crate only. Before M3: verify IME, VoiceOver, NVDA and
Orca with the real editor widget; if egui fails those, reopen this ADR.
