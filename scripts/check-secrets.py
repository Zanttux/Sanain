#!/usr/bin/env python3
"""Fail if a tracked file looks like a secret (docs/SECURITY_SECRETS.md).

Runs in CI on every pull request and push, and can run as a pre-commit
hook (`scripts/check-secrets.py --staged`). Checks file names (signing keys,
certificates, provisioning profiles, keychains, .env) and contents (private
key blocks, App Store Connect API keys, Apple app-specific passwords next to
a password word, GitHub, AWS, Google, Slack and Stripe tokens). It prints
only the file and line number, never the match.
"""
import re
import subprocess
import sys

NAME = re.compile(
    r"(\.(p8|p12|pfx|key|pem|cer|mobileprovision|provisionprofile|keychain|keychain-db)$)"
    r"|(^|/)AuthKey_[A-Z0-9]+|(^|/)\.env(\.|$)",
    re.I,
)
CONTENT = [
    ("private key", re.compile(r"-----BEGIN (RSA |EC |DSA |OPENSSH |ENCRYPTED )?PRIVATE KEY-----")),
    ("certificate", re.compile(r"-----BEGIN CERTIFICATE-----")),
    ("GitHub token", re.compile(r"\b(ghp|gho|ghu|ghs|ghr)_[A-Za-z0-9]{36}\b|\bgithub_pat_[A-Za-z0-9_]{60,}")),
    ("AWS key", re.compile(r"\bAKIA[0-9A-Z]{16}\b")),
    ("Google API key", re.compile(r"\bAIza[0-9A-Za-z_\-]{35}\b")),
    ("Slack token", re.compile(r"\bxox[abpr]-[0-9A-Za-z-]{10,}")),
    ("Stripe key", re.compile(r"\b(sk|rk)_live_[0-9A-Za-z]{20,}")),
    ("Apple app-specific password",
     re.compile(r"(?i)pass(word)?\W{0,20}[a-z]{4}-[a-z]{4}-[a-z]{4}-[a-z]{4}\b")),
]
# This file names the patterns it looks for.
SELF = "scripts/check-secrets.py"


def files(staged):
    cmd = ["git", "diff", "--cached", "--name-only", "--diff-filter=ACMR"] if staged \
        else ["git", "ls-files"]
    return [f for f in subprocess.run(cmd, capture_output=True, text=True, check=True)
            .stdout.splitlines() if f]


def main():
    staged = "--staged" in sys.argv[1:]
    bad = []
    for f in files(staged):
        if NAME.search(f):
            bad.append(f"{f}: file name of a key, certificate or credentials file")
            continue
        if f == SELF:
            continue
        try:
            data = open(f, "rb").read()
        except OSError:
            continue
        if b"\0" in data[:8192]:
            continue  # binary (fonts, images): not scanned for text secrets
        text = data.decode("utf-8", "replace")
        for label, pat in CONTENT:
            for m in pat.finditer(text):
                line = text.count("\n", 0, m.start()) + 1
                bad.append(f"{f}:{line}: looks like a {label}")
    if bad:
        print("Possible secrets found; remove them (and rotate them if they were real):")
        print("\n".join(f"  {b}" for b in bad))
        sys.exit(1)
    print(f"No secrets found in {'staged' if staged else 'tracked'} files.")


if __name__ == "__main__":
    main()
