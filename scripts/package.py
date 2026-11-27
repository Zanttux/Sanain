#!/usr/bin/env python3
"""Build unsigned release packages for the current platform: a portable
archive (with the `text3-portable` marker) and an installer (Windows:
Inno Setup per-user setup.exe; macOS: .dmg; Linux: .deb).

Writes target/dist/text3-<version>-<os>-<arch>.(zip|tar.gz) and a
.sha256 file next to it. The archive holds the program (Text³.app on
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
    """Add a folder to a zip, keeping the executable bit (for Text³.app)."""
    for dirpath, _, files in os.walk(src):
        for name in sorted(files):
            path = os.path.join(dirpath, name)
            rel = os.path.join(arc, os.path.relpath(path, src)).replace(os.sep, "/")
            info = zipfile.ZipInfo.from_file(path, rel)
            info.compress_type = zipfile.ZIP_DEFLATED
            with open(path, "rb") as f:
                zf.writestr(info, f.read())


def checksum(path):
    digest = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            digest.update(chunk)
    base = os.path.basename(path)
    with open(path + ".sha256", "w", encoding="utf-8") as f:
        f.write(f"{digest.hexdigest()}  {base}\n")
    print(path)


def find_iscc():
    """Inno Setup's compiler: on PATH, in its default folder, or installed
    with Chocolatey on CI runners."""
    for c in [
        shutil.which("iscc"),
        r"C:\Program Files (x86)\Inno Setup 6\ISCC.exe",
        r"C:\Program Files\Inno Setup 6\ISCC.exe",
    ]:
        if c and os.path.isfile(c):
            return c
    if os.environ.get("CI") and shutil.which("choco"):
        subprocess.run(["choco", "install", "innosetup", "-y", "--no-progress"], check=True)
        return find_iscc_installed()
    return None


def find_iscc_installed():
    for c in [r"C:\Program Files (x86)\Inno Setup 6\ISCC.exe", r"C:\Program Files\Inno Setup 6\ISCC.exe"]:
        if os.path.isfile(c):
            return c
    return None


def windows_installer(ver, stage, dist, name):
    iscc = find_iscc()
    if not iscc:
        print("Inno Setup not found; skipping the Windows installer", file=sys.stderr)
        return None
    out = name + "-setup"
    run(
        iscc,
        f"/DAppVersion={ver}",
        f"/DSourceDir={stage}",
        f"/DOutputDir={dist}",
        f"/DOutputName={out}",
        f"/DRepoRoot={ROOT}",
        os.path.join(ROOT, "packaging", "windows", "text3.iss"),
    )
    return os.path.join(dist, out + ".exe")


def macos_dmg(stage_app, dist, name, notices):
    tmp = os.path.join(dist, name + "-dmg")
    shutil.rmtree(tmp, ignore_errors=True)
    os.makedirs(tmp)
    shutil.copytree(stage_app, os.path.join(tmp, "Text³.app"), symlinks=True)
    os.symlink("/Applications", os.path.join(tmp, "Applications"))
    for doc in DOCS:
        shutil.copy2(os.path.join(ROOT, doc), os.path.join(tmp, doc))
    shutil.copy2(notices, os.path.join(tmp, "THIRD_PARTY_NOTICES.md"))
    dmg = os.path.join(dist, name + ".dmg")
    if os.path.exists(dmg):
        os.remove(dmg)
    run("hdiutil", "create", "-volname", "Text³", "-srcfolder", tmp, "-ov", "-format", "UDZO", dmg)
    shutil.rmtree(tmp)
    return dmg


def linux_deb(ver, arch, dist, notices):
    deb_arch = {"x86_64": "amd64", "aarch64": "arm64"}.get(arch, arch)
    root = os.path.join(dist, "deb-root")
    shutil.rmtree(root, ignore_errors=True)
    def p(*parts):
        path = os.path.join(root, *parts)
        os.makedirs(os.path.dirname(path), exist_ok=True)
        return path
    shutil.copy2(os.path.join(ROOT, "target", "release", "text3"), p("usr", "bin", "text3"))
    os.chmod(p("usr", "bin", "text3"), 0o755)
    shutil.copy2(
        os.path.join(ROOT, "packaging", "icons", "text3-256.png"),
        p("usr", "share", "icons", "hicolor", "256x256", "apps", "io.github.zanttux.text3.png"),
    )
    shutil.copy2(
        os.path.join(ROOT, "packaging", "linux", "io.github.zanttux.text3.desktop"),
        p("usr", "share", "applications", "io.github.zanttux.text3.desktop"),
    )
    shutil.copy2(os.path.join(ROOT, "LICENSE"), p("usr", "share", "doc", "text3", "copyright"))
    shutil.copy2(notices, p("usr", "share", "doc", "text3", "THIRD_PARTY_NOTICES.md"))
    depends = "libc6, libgcc-s1"
    if shutil.which("dpkg-shlibdeps"):
        # dpkg-shlibdeps wants a debian/control next to it.
        os.makedirs(os.path.join(root, "debian"), exist_ok=True)
        with open(os.path.join(root, "debian", "control"), "w") as f:
            f.write("Source: text3\n\nPackage: text3\nArchitecture: any\n")
        r = subprocess.run(
            ["dpkg-shlibdeps", "-O", "usr/bin/text3"], cwd=root, capture_output=True, text=True
        )
        shutil.rmtree(os.path.join(root, "debian"))
        if r.returncode == 0 and "shlibs:Depends=" in r.stdout:
            depends = r.stdout.split("shlibs:Depends=", 1)[1].strip()
    size_kb = sum(
        os.path.getsize(os.path.join(d, f)) for d, _, fs in os.walk(root) for f in fs
    ) // 1024
    with open(p("DEBIAN", "control"), "w") as f:
        f.write(
            "Package: text3\n"
            f"Version: {ver}\n"
            f"Architecture: {deb_arch}\n"
            "Maintainer: Text³ project <https://github.com/Zanttux/text3>\n"
            f"Installed-Size: {size_kb}\n"
            f"Depends: {depends}\n"
            "Section: editors\n"
            "Priority: optional\n"
            "Homepage: https://github.com/Zanttux/text3\n"
            "Description: Native text editor for text, code, Markdown and data\n"
            " Text³ is a fast, offline text editor with tabs, Markdown preview,\n"
            " data views, comparison and a log viewer for very large files.\n"
        )
    deb = os.path.join(dist, f"text3_{ver}_{deb_arch}.deb")
    run("dpkg-deb", "--root-owner-group", "--build", root, deb)
    shutil.rmtree(root)
    return deb


def main():
    build = "--no-build" not in sys.argv[1:]
    ver = version()
    system, arch = target_name()
    name = f"text3-{ver}-{system}-{arch}"
    dist = os.path.join(ROOT, "target", "dist")
    stage = os.path.join(dist, name)
    shutil.rmtree(stage, ignore_errors=True)
    os.makedirs(stage)

    if system == "macos":
        # Builds the release binary and wraps it in Text³.app.
        run("sh", "scripts/macos-bundle.sh", "release")
        shutil.copytree(
            os.path.join(ROOT, "target", "release", "Text³.app"),
            os.path.join(stage, "Text³.app"),
            symlinks=True,
        )
    else:
        if build:
            run("cargo", "build", "--release", "--locked", "-p", "text3-app")
        exe = "text3.exe" if system == "windows" else "text3"
        shutil.copy2(os.path.join(ROOT, "target", "release", exe), os.path.join(stage, exe))

    for doc in DOCS:
        shutil.copy2(os.path.join(ROOT, doc), os.path.join(stage, doc))
    notices = os.path.join(stage, "THIRD_PARTY_NOTICES.md")
    run(sys.executable, "scripts/third-party-notices.py", notices)

    # Installer first, from the stage without the portable marker.
    installer = None
    if system == "windows":
        installer = windows_installer(ver, stage, dist, name)
    elif system == "macos":
        installer = macos_dmg(os.path.join(stage, "Text³.app"), dist, name, notices)
    elif system == "linux" and shutil.which("dpkg-deb"):
        installer = linux_deb(ver, arch, dist, notices)
    if installer:
        checksum(installer)

    # Portable archive: the marker keeps settings and data next to the app.
    with open(os.path.join(stage, "text3-portable"), "w", encoding="utf-8") as f:
        f.write(
            "This file makes Text³ portable: settings, crash recovery and local\n"
            "history are kept in the data folder next to it. Delete it to use\n"
            "your user profile instead.\n"
        )
    portable = name + "-portable"
    if system == "linux":
        archive = os.path.join(dist, portable + ".tar.gz")
        with tarfile.open(archive, "w:gz") as tf:
            tf.add(stage, arcname=portable)
    else:
        archive = os.path.join(dist, portable + ".zip")
        with zipfile.ZipFile(archive, "w") as zf:
            add_tree(zf, stage, portable)
    checksum(archive)
    shutil.rmtree(stage)
    return 0


if __name__ == "__main__":
    sys.exit(main())
