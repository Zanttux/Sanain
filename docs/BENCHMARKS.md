# Benchmarks
No rigorous measurements yet (no repeated runs, no p95).

Informal observations (single runs, not baselines):
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
