#!/usr/bin/env python3
"""Compare Text³ with other editors on this Mac (docs/PERFORMANCE.md, #179).

Every app is measured the same way, Text³ included:
  window_ms   from `open -n -a <app> <file>` (a new instance, as Finder
              would start it) to its first window on screen; measured by
              scripts/bench/window-wait.swift, which polls the window
              server every millisecond (no in-app probe)
  idle_rss    resident memory of all the app's new processes, summed
              (Electron apps run several), after the window has been idle
              for --settle seconds with the file open
  app_size    size of the .app bundle on disk

Each app gets a scratch profile where it allows one (Text³: TEXT3_* folders;
VS Code: --user-data-dir/--extensions-dir) so nothing of the user's is read
or changed, and one discarded warm-up launch. Rounds alternate the order of
the apps. Apps that are not installed are skipped and listed as such.

Usage: scripts/bench/compare.py [--runs 10] [--settle 5]
       [--text3 target/release/Text³.app] [--out bench/results/x.json]
macOS only. It opens and closes windows on the screen while it runs.
"""

import argparse
import datetime
import json
import os
import platform
import shutil
import signal
import statistics
import subprocess
import sys
import tempfile
import time
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]


def sh(*cmd):
    return subprocess.run(cmd, capture_output=True, text=True, check=True).stdout


def app_pids(bundle):
    """PIDs of processes whose executable is inside `bundle`."""
    prefix = str(Path(bundle).resolve()) + "/"
    out = sh("/bin/ps", "-A", "-o", "pid=,comm=")
    pids = []
    for line in out.splitlines():
        pid, _, comm = line.strip().partition(" ")
        if comm.strip().startswith(prefix):
            pids.append(int(pid))
    return set(pids)


def rss_kib(pids):
    if not pids:
        return 0
    out = sh("/bin/ps", "-o", "rss=", "-p", ",".join(map(str, pids)))
    return sum(int(x) for x in out.split())


def du_bytes(path):
    total = 0
    for root, _, files in os.walk(path):
        for f in files:
            p = os.path.join(root, f)
            if not os.path.islink(p):
                total += os.path.getsize(p)
    return total


def stop(bundle, before):
    """End every process of the app started since `before`."""
    for sig in (signal.SIGTERM, signal.SIGKILL):
        pids = app_pids(bundle) - before
        for pid in pids:
            try:
                os.kill(pid, sig)
            except ProcessLookupError:
                pass
        deadline = time.time() + 5
        while time.time() < deadline and app_pids(bundle) - before:
            time.sleep(0.05)


def version_of(bundle):
    try:
        return sh(
            "/usr/bin/defaults", "read", str(Path(bundle) / "Contents" / "Info"),
            "CFBundleShortVersionString",
        ).strip()
    except subprocess.CalledProcessError:
        return "?"


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--runs", type=int, default=10)
    ap.add_argument("--settle", type=float, default=5.0)
    ap.add_argument("--text3", default=str(REPO / "target" / "release" / "Text³.app"))
    ap.add_argument("--out")
    a = ap.parse_args()
    if sys.platform != "darwin":
        sys.exit("macOS only")

    scratch = Path(tempfile.mkdtemp(prefix="text3-compare-"))
    helper = scratch / "window-wait"
    subprocess.run(
        ["swiftc", "-O", "-o", str(helper), str(REPO / "scripts/bench/window-wait.swift")],
        check=True,
    )
    # A 4 KB plain-text file, the same bytes every time.
    doc = scratch / "sample.txt"
    line = "The quick brown fox jumps over the lazy dog. 0123456789\n"
    doc.write_text(line * (4096 // len(line)), encoding="utf-8")

    t3 = scratch / "text3"
    vs = scratch / "vscode"
    apps = [
        ("Text³", a.text3, [
            "--env", f"TEXT3_CONFIG_DIR={t3 / 'config'}",
            "--env", f"TEXT3_RECOVERY_DIR={t3 / 'recovery'}",
            "--env", "TEXT3_LOCALE=en-US",
            str(doc),
        ]),
        ("TextEdit", "/System/Applications/TextEdit.app", [str(doc)]),
        ("CotEditor", "/Applications/CotEditor.app", [str(doc)]),
        ("Sublime Text", "/Applications/Sublime Text.app", [str(doc)]),
        ("Zed", "/Applications/Zed.app", [str(doc)]),
        ("BBEdit", "/Applications/BBEdit.app", [str(doc)]),
        ("Visual Studio Code", "/Applications/Visual Studio Code.app", [
            str(doc), "--args", f"--user-data-dir={vs / 'data'}",
            f"--extensions-dir={vs / 'ext'}", "--disable-extensions",
            "--skip-welcome", "--skip-release-notes", "--disable-workspace-trust",
        ]),
    ]
    present = [x for x in apps if Path(x[1]).is_dir()]
    missing = [x[0] for x in apps if not Path(x[1]).is_dir()]
    if not any(n == "Text³" for n, _, _ in present):
        sys.exit(f"no Text³.app at {a.text3}: run scripts/macos-bundle.sh release")

    def launch(name, bundle, args):
        before = app_pids(bundle)
        out = sh(str(helper), "30", bundle, *args).strip()
        fields = dict(p.split("=", 1) for p in out.split())
        ms = None if fields["window_ms"] == "timeout" else float(fields["window_ms"])
        time.sleep(a.settle)
        rss = rss_kib(app_pids(bundle) - before)
        stop(bundle, before)
        time.sleep(1.0)
        return ms, rss

    results = {n: {"window_ms": [], "idle_rss_kib": []} for n, _, _ in present}
    for name, bundle, args in present:
        print(f"warm-up: {name}", flush=True)
        launch(name, bundle, args)
    for r in range(a.runs):
        order = present if r % 2 == 0 else list(reversed(present))
        for name, bundle, args in order:
            ms, rss = launch(name, bundle, args)
            results[name]["window_ms"].append(ms)
            results[name]["idle_rss_kib"].append(rss)
            print(f"run {r + 1}/{a.runs} {name}: window {ms} ms, idle RSS {rss / 1024:.1f} MiB",
                  flush=True)

    def summary(vals):
        v = [x for x in vals if x is not None]
        if not v:
            return None
        s = sorted(v)
        return {
            "median": statistics.median(s),
            "p95": s[max(0, -(-95 * len(s) // 100) - 1)],
            "n": len(s),
            "timeouts": len(vals) - len(v),
        }

    mac = platform.mac_ver()[0]
    chip = sh("/usr/sbin/sysctl", "-n", "machdep.cpu.brand_string").strip()
    mem = int(sh("/usr/sbin/sysctl", "-n", "hw.memsize")) // 2**30
    load = os.getloadavg()[0]
    commit = sh("git", "-C", str(REPO), "rev-parse", "--short", "HEAD").strip()
    report = {
        "kind": "compare",
        "date": datetime.date.today().isoformat(),
        "machine": {"os": f"macOS {mac}", "cpu": chip, "memory_gib": mem,
                    "load_average_end": round(load, 1)},
        "text3_commit": commit,
        "method": "scripts/bench/compare.py",
        "runs": a.runs,
        "settle_s": a.settle,
        "file": "4 KB plain text",
        "apps": {
            name: {
                "version": version_of(bundle),
                "app_size_bytes": du_bytes(bundle),
                "window_ms": summary(results[name]["window_ms"]),
                "idle_rss_mib": summary([x / 1024 for x in results[name]["idle_rss_kib"]]),
            }
            for name, bundle, _ in present
        },
        "not_installed": missing,
    }
    text = json.dumps(report, indent=2, ensure_ascii=False)
    print(text)
    if a.out:
        Path(a.out).parent.mkdir(parents=True, exist_ok=True)
        Path(a.out).write_text(text + "\n", encoding="utf-8")
    shutil.rmtree(scratch, ignore_errors=True)


if __name__ == "__main__":
    main()
