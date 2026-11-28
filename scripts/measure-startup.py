#!/usr/bin/env python3
"""Measure Text³ start-up time and memory (docs/BENCHMARKS.md).

Builds the release binary, then launches it repeatedly with
TEXT3_MEASURE_STARTUP (see crates/text3-app/src/startprobe.rs) and a
scratch profile, never the user's own settings or recovery folder:

  empty-cold   empty window, fresh copy of the binary and fresh profile
               per run ("cold-ish": the OS file cache stays warm, purging
               it needs root)
  empty-warm   empty window, same binary and profile every run
  small-warm   a 4 KB Rust file opened from the command line
  font-warm    empty window with an editor font set (Settings › Editor ›
               Font), which scans the system fonts in the background
  1mb, 100mb   synthetic plain-text files (fixed generator, no randomness)

Per run it reports launch-to-first-frame (from just before the process is
spawned to the start of the frame after the first one, i.e. the first
frame is on screen), the in-process marks, and from `/usr/bin/time -l`
the peak resident set size and (macOS) peak memory footprint. Separate
runs without TEXT3_MEASURE_EXIT sample the resident size with `ps` after
the window has been idle for --settle seconds, and the largest resident
size sampled every 50 ms until then.

Usage: scripts/measure-startup.py [--runs 20] [--big-runs 5]
       [--scratch DIR] [--no-build | --binary PATH] [--json results.json]
macOS and Linux (`/usr/bin/time` output differs; on Linux only the max
RSS is read). Not for Windows.
"""

import argparse
import json
import math
import os
import platform
import shutil
import signal
import statistics
import subprocess
import sys
import time
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
DEFAULT_SCRATCH = Path(
    os.environ.get("TEXT3_MEASURE_SCRATCH")
    or (Path(os.environ.get("TMPDIR", "/tmp")) / "text3-measure-startup")
)


def p95(values):
    """Nearest-rank 95th percentile."""
    s = sorted(values)
    return s[max(0, math.ceil(0.95 * len(s)) - 1)]


def parse_line(line):
    out = {}
    for part in line.split():
        k, _, v = part.partition("=")
        out[k] = float(v)
    return out


def parse_time_l(stderr):
    """Peak RSS and footprint (bytes) from /usr/bin/time -l (macOS) or -v (Linux)."""
    rss = footprint = None
    for line in stderr.splitlines():
        line = line.strip()
        if line.endswith("maximum resident set size"):
            rss = int(line.split()[0])
        elif line.endswith("peak memory footprint"):
            footprint = int(line.split()[0])
        elif line.startswith("Maximum resident set size (kbytes):"):
            rss = int(line.split(":")[1]) * 1024
    return rss, footprint


def time_flag():
    return "-l" if sys.platform == "darwin" else "-v"


def make_env(profile, measure_file, exit_after):
    env = dict(os.environ)
    for k in list(env):
        if k.startswith(("TEXT3_", "SANAIN_")):
            del env[k]
    env["TEXT3_CONFIG_DIR"] = str(profile / "config")
    env["TEXT3_RECOVERY_DIR"] = str(profile / "data" / "recovery")
    env["TEXT3_NO_SESSION"] = "1"
    env["TEXT3_LOCALE"] = "en-US"
    env["TEXT3_MEASURE_STARTUP"] = str(measure_file)
    if exit_after:
        env["TEXT3_MEASURE_EXIT"] = "1"
    return env


def fresh_profile(path, editor_font=None):
    shutil.rmtree(path, ignore_errors=True)
    (path / "config").mkdir(parents=True)
    (path / "data").mkdir(parents=True)
    if editor_font:
        # Settings format: crates/text3-app/src/settings.rs.
        (path / "config" / "settings").write_text(
            f"text3-settings 1\neditor_font={editor_font}\n"
        )


def launch(binary, args, profile, scratch, timeout):
    """One run that exits after the first full frame."""
    measure = scratch / "measure.txt"
    measure.unlink(missing_ok=True)
    env = make_env(profile, measure, exit_after=True)
    spawn_us = time.time_ns() / 1000
    proc = subprocess.run(
        ["/usr/bin/time", time_flag(), str(binary), *map(str, args)],
        env=env,
        capture_output=True,
        text=True,
        timeout=timeout,
        check=False,
    )
    wall_ms = (time.time_ns() / 1000 - spawn_us) / 1000
    if proc.returncode != 0 or not measure.exists():
        raise RuntimeError(f"run failed ({proc.returncode}): {proc.stderr[-2000:]}")
    marks = parse_line(measure.read_text().strip().splitlines()[-1])
    rss, footprint = parse_time_l(proc.stderr)
    pre_main_ms = (marks["main_epoch_us"] - spawn_us) / 1000
    result = {
        "pre_main_ms": pre_main_ms,
        "launch_to_first_frame_ms": pre_main_ms + marks["first_frame_ms"],
        "wall_ms": wall_ms,
        "peak_rss": rss,
        "peak_footprint": footprint,
    }
    if "text_frame_ms" in marks:
        result["launch_to_text_frame_ms"] = pre_main_ms + marks["text_frame_ms"]
    for k, v in marks.items():
        if k != "main_epoch_us":
            result[k] = v
    return result


def rss_of(pid):
    out = subprocess.run(
        ["ps", "-o", "rss=", "-p", str(pid)], capture_output=True, text=True, check=False
    )
    return int(out.stdout.strip()) * 1024 if out.stdout.strip() else None


def idle_rss(binary, args, profile, scratch, settle, timeout):
    """Resident size (bytes) after the first full frame plus `settle` s
    idle, and the largest size seen (sampled every 50 ms) until then."""
    measure = scratch / "measure-idle.txt"
    measure.unlink(missing_ok=True)
    env = make_env(profile, measure, exit_after=False)
    proc = subprocess.Popen(
        [str(binary), *map(str, args)],
        env=env,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )
    try:
        deadline = time.time() + timeout
        while not (measure.exists() and measure.read_text().strip()):
            if time.time() > deadline or proc.poll() is not None:
                raise RuntimeError("idle run did not reach the first frame")
            time.sleep(0.02)
        peak = 0
        end = time.time() + settle
        while time.time() < end:
            peak = max(peak, rss_of(proc.pid) or 0)
            time.sleep(0.05)
        rss = rss_of(proc.pid)
        return {"idle_rss": rss, "idle_peak_rss": max(peak, rss or 0)}
    finally:
        proc.send_signal(signal.SIGKILL)
        proc.wait()


def write_datasets(data):
    """Deterministic synthetic inputs (regenerated only when missing)."""
    data.mkdir(parents=True, exist_ok=True)
    small = data / "small.rs"
    if not small.exists():
        body = "".join(
            f"/// Item {i}.\npub fn item_{i}(x: u64) -> u64 {{\n    x * {i} + 1\n}}\n\n"
            for i in range(80)
        )
        small.write_text(body[:4096])
    sizes = {"1mb.txt": 1 << 20, "100mb.txt": 100 << 20}
    for name, size in sizes.items():
        path = data / name
        if path.exists() and path.stat().st_size == size:
            continue
        with open(path, "w", encoding="ascii", newline="\n") as f:
            written = 0
            n = 0
            while written < size:
                line = (
                    f"{n:08} 2026-01-01T00:00:{n % 60:02} INFO worker-{n % 16:02} "
                    f"request id={n * 7919 % 100003:06} took {n % 977} ms ok\n"
                )
                line = line[: size - written]
                f.write(line)
                written += len(line)
                n += 1
    return small, data / "1mb.txt", data / "100mb.txt"


def summarize(name, runs):
    def stat(key, scale=1.0, unit="ms"):
        vals = [r[key] / scale for r in runs if r.get(key) is not None]
        if not vals:
            return None
        return {
            "median": statistics.median(vals),
            "p95": p95(vals),
            "min": min(vals),
            "max": max(vals),
            "unit": unit,
        }

    mib = 1024 * 1024
    keys = [
        ("launch_to_first_frame_ms", 1.0, "ms"),
        ("launch_to_text_frame_ms", 1.0, "ms"),
        ("pre_main_ms", 1.0, "ms"),
        ("settings_ms", 1.0, "ms"),
        ("files_ms", 1.0, "ms"),
        ("window_ms", 1.0, "ms"),
        ("app_ms", 1.0, "ms"),
        ("first_ui_ms", 1.0, "ms"),
        ("first_frame_ms", 1.0, "ms"),
        ("text_frame_ms", 1.0, "ms"),
        ("peak_rss", mib, "MiB"),
        ("peak_footprint", mib, "MiB"),
        ("idle_rss", mib, "MiB"),
        ("idle_peak_rss", mib, "MiB"),
    ]
    out = {"runs": sum(1 for r in runs if "first_frame_ms" in r)}
    for key, scale, unit in keys:
        s = stat(key, scale, unit)
        if s:
            out[key] = s
    idle = sum(1 for r in runs if "idle_rss" in r)
    print(f"\n{name} ({out['runs']} timed runs, {idle} idle runs)")
    for key, s in out.items():
        if key == "runs":
            continue
        print(
            f"  {key:26} median {s['median']:8.1f}  p95 {s['p95']:8.1f}  "
            f"min {s['min']:8.1f}  max {s['max']:8.1f} {s['unit']}"
        )
    return out


def machine():
    info = {"os": platform.platform(), "machine": platform.machine()}
    if sys.platform == "darwin":
        for key, cmd in {
            "cpu": ["sysctl", "-n", "machdep.cpu.brand_string"],
            "ram_bytes": ["sysctl", "-n", "hw.memsize"],
            "macos": ["sw_vers", "-productVersion"],
        }.items():
            try:
                info[key] = subprocess.run(
                    cmd, capture_output=True, text=True, check=True
                ).stdout.strip()
            except (OSError, subprocess.CalledProcessError):
                pass
    try:
        info["rustc"] = subprocess.run(
            ["rustc", "--version"], cwd=REPO, capture_output=True, text=True, check=True
        ).stdout.strip()
        info["commit"] = subprocess.run(
            ["git", "rev-parse", "--short", "HEAD"],
            cwd=REPO,
            capture_output=True,
            text=True,
            check=True,
        ).stdout.strip()
    except (OSError, subprocess.CalledProcessError):
        pass
    return info


def main():
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--runs", type=int, default=20)
    ap.add_argument("--big-runs", type=int, default=5, help="runs for the 100 MB file")
    ap.add_argument("--idle-runs", type=int, default=3)
    ap.add_argument("--settle", type=float, default=5.0)
    ap.add_argument("--scratch", type=Path, default=DEFAULT_SCRATCH)
    ap.add_argument("--no-build", action="store_true")
    ap.add_argument("--binary", type=Path, help="measure this build (implies --no-build)")
    ap.add_argument(
        "--baseline",
        type=Path,
        help="also measure this build, alternating launches with the other "
        "one, so both see the same machine load",
    )
    ap.add_argument(
        "--editor-font",
        default="Menlo" if sys.platform == "darwin" else "DejaVu Sans Mono",
        help="installed monospaced family for the editor-font run",
    )
    ap.add_argument("--json", type=Path)
    args = ap.parse_args()

    scratch = args.scratch.resolve()
    scratch.mkdir(parents=True, exist_ok=True)
    if not args.no_build and not args.binary:
        subprocess.run(
            ["cargo", "build", "--release", "--locked", "-p", "text3-app"],
            cwd=REPO,
            check=True,
        )
    builds = {"candidate": (args.binary or REPO / "target" / "release" / "text3").resolve()}
    if args.baseline:
        builds = {"baseline": args.baseline.resolve(), **builds}
    small, one_mb, hundred_mb = write_datasets(scratch / "data")
    results = {"machine": machine(), "load_before": os.getloadavg()}
    print(f"machine: {json.dumps(results['machine'])}")
    print(f"load average: {results['load_before']}")
    print(f"scratch: {scratch}")
    for k, b in builds.items():
        print(f"{k}: {b}")
    timeout = 120

    def alternate(runs, one):
        """`one(key, binary, i)` for every build, `runs` times, alternating."""
        out = {k: [] for k in builds}
        for i in range(runs):
            order = list(builds.items())
            if i % 2:
                order.reverse()
            for k, b in order:
                out[k].append(one(k, b, i))
        return out

    def record(name, label, per_build):
        for k, rs in per_build.items():
            tag = f" [{k}]" if len(builds) > 1 else ""
            results.setdefault(name, {})[k] = summarize(label + tag, rs)

    # Cold-ish: a new copy of the binary (new file, code signature checked
    # again, nothing of it in the dyld caches) and an empty profile.
    def cold_run(k, b, i):
        run_dir = scratch / "cold" / k
        shutil.rmtree(run_dir, ignore_errors=True)
        run_dir.mkdir(parents=True)
        copy = run_dir / f"text3-{i}"
        shutil.copy2(b, copy)
        fresh_profile(run_dir / "profile")
        return launch(copy, [], run_dir / "profile", scratch, timeout)

    record("empty-cold", "empty window, cold-ish", alternate(args.runs, cold_run))

    def profile_of(k, font=False):
        return scratch / ("profile-font" if font else "profile") / k

    for k, b in builds.items():
        fresh_profile(profile_of(k))
        fresh_profile(profile_of(k, True), editor_font=args.editor_font)
        launch(b, [], profile_of(k), scratch, timeout)  # warm-up

    def warm(name, label, file_args, runs, font=False):
        timed = alternate(
            runs, lambda k, b, _i: launch(b, file_args, profile_of(k, font), scratch, timeout)
        )
        idle = alternate(
            args.idle_runs,
            lambda k, b, _i: idle_rss(
                b, file_args, profile_of(k, font), scratch, args.settle, timeout
            ),
        )
        record(name, label, {k: timed[k] + idle[k] for k in builds})

    warm("empty-warm", "empty window, warm", [], args.runs)
    warm(
        "font-warm",
        f"empty window, editor font {args.editor_font}, warm",
        [],
        args.runs,
        font=True,
    )
    warm("small-warm", "4 KB Rust file, warm", [small], args.runs)
    warm("1mb-warm", "1 MB text file, warm", [one_mb], args.runs)
    warm("100mb-warm", "100 MB text file, warm", [hundred_mb], args.big_runs)

    results["load_after"] = os.getloadavg()
    print(f"\nload average: {results['load_after']}")
    if args.json:
        args.json.write_text(json.dumps(results, indent=2))
        print(f"results: {args.json}")


if __name__ == "__main__":
    main()
