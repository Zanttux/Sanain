# Benchmarks
No rigorous measurements yet (no repeated runs, no p95).

Informal observations (single runs, not baselines):
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
  (`cargo test --release -p sanain-app --lib -- --ignored --nocapture
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
