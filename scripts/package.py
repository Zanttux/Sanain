#!/usr/bin/env python3
"""Build an unsigned release archive for the current platform.

Writes target/dist/sanain-<version>-<os>-<arch>.(zip|tar.gz) and a
.sha256 file next to it. The archive holds the program (Sanain.app on
macOS), LICENSE, README.md, CHANGELOG.md and THIRD_PARTY_NOTICES.md.

Nothing is signed or published: publishing a release needs the owner's
approval and, for installers, signing certificates (docs/RELEASE.md).

Usage: scripts/package.py [--no-build]
"""

import hashlib
import os
import platform
import re
import shutil
import subprocess
import sys
import tarfile
import zipfile

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
DOCS = ["LICENSE", "README.md", "CHANGELOG.md"]


def run(*cmd):
    subprocess.run(cmd, check=True, cwd=ROOT)


def version():
    with open(os.path.join(ROOT, "Cargo.toml"), encoding="utf-8") as f:
        m = re.search(r'^version = "([^"]+)"', f.read(), re.M)
    if not m:
        sys.exit("version not found in Cargo.toml")
    return m.group(1)


def target_name():
    system = {"Darwin": "macos", "Windows": "windows", "Linux": "linux"}.get(
        platform.system(), platform.system().lower()
    )
    machine = platform.machine().lower()
    arch = {"amd64": "x86_64", "x64": "x86_64", "arm64": "aarch64"}.get(machine, machine)
    return system, arch


def add_tree(zf, src, arc):
    """Add a folder to a zip, keeping the executable bit (for Sanain.app)."""
    for dirpath, _, files in os.walk(src):
        for name in sorted(files):
            path = os.path.join(dirpath, name)
            rel = os.path.join(arc, os.path.relpath(path, src)).replace(os.sep, "/")
            info = zipfile.ZipInfo.from_file(path, rel)
            info.compress_type = zipfile.ZIP_DEFLATED
            with open(path, "rb") as f:
                zf.writestr(info, f.read())


def main():
    build = "--no-build" not in sys.argv[1:]
    ver = version()
    system, arch = target_name()
    name = f"sanain-{ver}-{system}-{arch}"
    dist = os.path.join(ROOT, "target", "dist")
    stage = os.path.join(dist, name)
    shutil.rmtree(stage, ignore_errors=True)
    os.makedirs(stage)

    if system == "macos":
        # Builds the release binary and wraps it in Sanain.app.
        run("sh", "scripts/macos-bundle.sh", "release")
        shutil.copytree(
            os.path.join(ROOT, "target", "release", "Sanain.app"),
            os.path.join(stage, "Sanain.app"),
            symlinks=True,
        )
    else:
        if build:
            run("cargo", "build", "--release", "--locked", "-p", "sanain-app")
        exe = "sanain.exe" if system == "windows" else "sanain"
        shutil.copy2(os.path.join(ROOT, "target", "release", exe), os.path.join(stage, exe))

    for doc in DOCS:
        shutil.copy2(os.path.join(ROOT, doc), os.path.join(stage, doc))
    run(sys.executable, "scripts/third-party-notices.py", os.path.join(stage, "THIRD_PARTY_NOTICES.md"))

    if system == "linux":
        archive = os.path.join(dist, name + ".tar.gz")
        with tarfile.open(archive, "w:gz") as tf:
            tf.add(stage, arcname=name)
    else:
        archive = os.path.join(dist, name + ".zip")
        with zipfile.ZipFile(archive, "w") as zf:
            add_tree(zf, stage, name)

    digest = hashlib.sha256()
    with open(archive, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            digest.update(chunk)
    base = os.path.basename(archive)
    with open(archive + ".sha256", "w", encoding="utf-8") as f:
        f.write(f"{digest.hexdigest()}  {base}\n")
    shutil.rmtree(stage)
    print(archive)
    return 0


if __name__ == "__main__":
    sys.exit(main())
