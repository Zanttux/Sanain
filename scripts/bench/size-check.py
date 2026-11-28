#!/usr/bin/env python3
"""Fail when a package outgrows its budget (bench/budgets.json, #179).

Run after scripts/package.py: checks every installer and archive in
target/dist and the release binary, and prints their sizes so the trend is
visible in each CI log.
"""
import json
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]


def main():
    budget = json.loads((REPO / "bench" / "budgets.json").read_text(encoding="utf-8"))
    dist = REPO / "target" / "dist"
    over = []
    pkgs = [p for p in sorted(dist.iterdir()) if p.is_file()
            and p.suffix in {".dmg", ".zip", ".gz", ".exe", ".deb"}]
    if not pkgs:
        sys.exit("no packages in target/dist")
    for p in pkgs:
        size = p.stat().st_size
        print(f"{size / 1e6:8.2f} MB  {p.name}")
        if size > budget["download_max_bytes"]:
            over.append(p.name)
    for name in ("text3", "text3.exe"):
        b = REPO / "target" / "release" / name
        if b.is_file():
            size = b.stat().st_size
            print(f"{size / 1e6:8.2f} MB  {b.relative_to(REPO)}")
            if size > budget["unpacked_binary_max_bytes"]:
                over.append(name)
    if over:
        sys.exit(f"over the size budget (bench/budgets.json): {', '.join(over)}")
    print("within the size budget")


if __name__ == "__main__":
    main()
