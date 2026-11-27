# Benchmarks

## Start-up and memory (2026-11-27)
Method: `scripts/measure-startup.py --baseline <old build>` (TESTING.md),
release profile (thin LTO), Rust 1.100.0, macOS 27.2.1, Apple M5 MacBook
Pro, 24 GB, internal SSD. Branch perf/startup-measure on dc101bf.
Scratch profile (TEXT3_CONFIG_DIR/TEXT3_RECOVERY_DIR, never the real
one), TEXT3_LOCALE=en-US, 960×680 pt window on a 2× display. 20 timed
runs per case (5 for 100 MB) plus 3 idle runs; baseline and candidate
launches alternate. The machine was busy with unrelated work (load
average 15–24), so absolute times run 10–30 % above an idle machine;
on a quieter run the same build showed warm median 148 ms. Datasets:
4 KB generated Rust, 1 MiB and 100 MiB generated ASCII log lines (fixed
generator in the script). Times: from spawning the process to the first
frame on screen (with a file: the first frame showing its text). Peak
RSS from `/usr/bin/time -l` up to that frame; idle RSS from `ps` 5 s
later, and the largest RSS sampled every 50 ms in between.

Baseline = dc101bf plus the probe only; candidate = this branch.
Median / p95:

| Case | Baseline | Candidate | Budget |
|---|---|---|---|
| Empty window, warm | 145.8 / 154.7 ms | 134.9 / 150.2 ms | warm ≤ 250 ms: met |
| Empty window, cold-ish | 1021 / 1087 ms | 986 / 1053 ms | cold p95 ≤ 500 ms: missed (see below) |
| Empty window, cold-ish, in-process only | 141 / 169 ms | 134 / 157 ms | — |
| Editor font set (Menlo), warm | 165.0 / 210.0 ms | 133.6 / 140.2 ms | met |
| 4 KB Rust file, warm | 178.6 / 208.1 ms | 165.3 / 183.7 ms | met |
| 1 MB text file, warm | 210.5 / 372.6 ms | 202.9 / 253.8 ms | — |
| 100 MB text file, warm | 586.6 / 607.1 ms | 561.7 / 604.2 ms | ≤ 2 s: met |
| Empty window, idle RSS | 109.1 MiB | 108.0 MiB | ≤ 100 MiB: missed by 8 MiB |
| Editor font set, idle RSS (largest seen) | 533.1 MiB (533.5) | 110.2 MiB (111.0) | missed by 10 MiB |
| 1 MB file, idle RSS | 111.8 MiB | 110.4 MiB | — |
| 100 MB file, peak / idle RSS | 413 / 322 MiB | 411 / 222–322 MiB | — |
| Empty window, peak footprint | 60.4 MiB | 59.5 MiB | — |

Where the time goes (candidate, warm, from `main`): settings 0.05 ms;
localization, recovery folder and command-line files 3–4 ms (1 MB file
7 ms, 100 MB 222 ms to read and decode); eframe creating the window,
NSApplication and the wgpu/Metal device 85–100 ms; app state < 0.1 ms;
first layout 6 ms; paint and present 30 ms. About 9 ms pass before
`main` (exec, dyld). A Time Profiler trace of the main thread
(`xctrace`) agrees: AppKit window creation, wgpu set-up and Core
Animation commit dominate; Text³'s own code is a few ms.

Fixed on this branch:
- The editor font setting (Settings › Editor › Font) read every system
  font file in full to find the family: about 600 MB on macOS (Apple
  Color Emoji alone is 192 MB). With a font set, RSS went to about
  530 MB on every start and stayed there for several seconds; this is
  where the earlier "empty window ~139 MB" (and worse) came from. fontdb
  now memory-maps each file just long enough to read its names
  (`memmap` feature; memmap2 was already in the tree via egui's system
  font fallback): 533 → 110 MiB, and start-up with a font set no longer
  competes with the scan (p95 210 → 140 ms).
- The first frame waited 20–35 ms for egui's system font list: the "+"
  button's tooltip text was formatted every frame, and formatting a
  shortcut checks the font has ⌥, which the built-in fonts lack. The
  tooltip is now formatted only when shown (the accessible description
  is formatted without the glyph check). Window → first frame 76 → 48 ms
  median in a 30-pair alternating run.
- The macOS menu bar is built at the start of the second frame instead
  of before the first (muda takes about 12 ms; ~5 ms off the first
  frame, the rest overlapped with waiting for the display).
- Inside Text³.app the Dock icon is left to the bundle instead of being
  replaced by the decoded 256 px PNG (saves the decode and a 2 MB image;
  not visible in these numbers, which run the bare binary).

Budgets missed, and why:
- Cold-ish p95 (1053 ms): 860 ms of it is before `main`. macOS checks a
  never-run executable on its first launch (the 46 MB binary; 300–500 ms
  on a quiet machine, 860 ms under this load). That happens once per
  installed or updated binary; stripping symbols (46 → 43 MB) did not
  change it. In-process time is 134 / 157 ms. A true cold start (file
  cache dropped, after reboot) needs root (`purge`) and was not measured.
- Idle RSS 108 MiB: Text³'s own heap at idle is about 3 MB (font atlas
  1 MB; `malloc_history`). The rest is platform: two 10 MB Metal
  drawables for a 2× window (IOSurface 20 MB), AppKit, Core Animation,
  LaunchServices and Metal, plus shared system library pages that RSS
  counts. The private footprint (Activity Monitor's "Memory") is 59 MiB.
  Getting RSS under 100 MiB would mean dropping a swapchain buffer or the
  GPU renderer; not done.

Not measured locally: Windows and Linux (no machines in this session);
typing latency; 10 GB log mode.

## Earlier informal observations
Single runs, not baselines:
- 2026-11-27, incremental highlighting with injections (branch
  perf/incremental-injections on fb6fe9f), release profile, Rust
  1.100.0, macOS 27.2, Apple M5, warm, same method as below (30 edits
  in the middle of the file, two runs, the second shown), plus one
  keystroke after a pause once the worker has settled. Before → after,
  median / p95:
  Markdown 512 KB 272.4 / 289.7 ms → 14.6 / 25.4 ms (max 55 ms: the
  first keystroke after the initial full analysis, Tree-sitter's
  Markdown grammar re-parses a freshly parsed tree slowly; 11.9 ms
  after a pause);
  HTML 512 KB (a `<script>` and `<style>` per section) 65.7 / 66.8 ms
  → 2.7 / 2.8 ms; PHP 512 KB (HTML around PHP, inline `<?php echo`,
  `<script>`) 111.7 / 114.2 ms → 6.1 / 7.3 ms; Rust and JSON
  unchanged (11.3 / 13.0 ms, 3.0 / 3.4 ms). Most of the Markdown time
  is the incremental parse of the block grammar itself (8–15 ms raw);
  folds take about 4 ms. First full analysis: Markdown 355 → 270 ms,
  PHP 166 → 139 ms.
- 2026-11-26, incremental highlighting (branch perf/incremental-parse on
  f10de67), release profile, Rust 1.100.0, macOS 27.2, Apple M5, warm:
  time from submitting a one-character edit in the middle of the file
  to the syntax worker's result (spans and folds), 30 edits alternately
  inserting and deleting `x`, synthetic generated sources
  (`cargo test --release -p text3-app --lib -- --ignored --nocapture
  bench_`). Before → after, median / p95:
  Rust 1024 KB 173.9 / 177.7 ms → 12.2 / 13.4 ms;
  JSON 512 KB 81.1 / 83.1 ms → 3.3 / 3.4 ms;
  Markdown 512 KB (injections, still highlighted in full)
  330.0 / 335.7 ms → 278.0 / 287.1 ms. First full highlight of the Rust
  file 197 ms → 126 ms (one parse instead of two). The settling pass
  after a typing pause costs about one full highlight.
- 2026-11-26, 147 MB plain-text file (1.3 M lines of 113 chars) opened in
  the editor, release, Apple M5: resident memory 1.43 GB idle and 2.4 GB
  after one keystroke, because the status bar copied and scanned the
  whole text every frame to show the line ending. After the fix (chunked
  scan, cached per revision): 571 MB idle, 726 MB after a keystroke, 0 %
  CPU idle. Also checked without failure: 900 000-char line with word
  wrap, 3 MB of random bytes (encoding dialog), 100 000-deep JSON in the
  structured view.
- 2026-11-25, log viewer (branch m8/log-viewer on 1d37350), release
  profile, Rust 1.100.0, macOS 27.2.1, Apple M5, 24 GB, warm cache:
  synthetic 302 MB / 4,451,708-line ASCII log opened from the command
  line was fully indexed and displayed within 1.2 s; resident memory
  170 MB (an empty window is ~139 MB). A case-insensitive substring
  filter over the whole file finished within 3 s (68,844 matches, equal
  to `grep -ci`).

 See TESTING.md for budgets. Record commit, release profile, toolchain, OS, hardware, dataset generator seed, file size/encoding, run count, cold/warm method, median/p95, peak RSS, failures and raw results path. Workloads: startup empty, 1 MB text, 100 MB log, 100k lines typing, regex search, Markdown preview, 100 tabs restore, 10 GB log P1. Use synthetic data. Report baseline and regression threshold before optimization.
