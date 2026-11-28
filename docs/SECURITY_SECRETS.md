# Secrets: what never goes to GitHub

Owner rule (2026-11-28): nothing that could compromise the owner's Apple,
GitHub or any other account may ever end up on GitHub: not in the code,
the history, issues, pull requests, release notes, workflow logs or
artifacts.

## Where the signing material lives
- **Owner's machine only:** `~/.config/text3-signing` (folder mode 700,
  files 600): the Developer ID private key, the certificate, the .p12 and
  its password, and the App Store Connect API key. Back the folder up in a
  password manager; it is not in any repository.
- **GitHub, encrypted, write-only:** the environment `signing` holds
  `MACOS_CERT_P12`, `MACOS_CERT_PASSWORD`, `APPLE_API_KEY_P8`,
  `APPLE_API_KEY_ID`, `APPLE_API_ISSUER`, `APPLE_TEAM_ID`. Only jobs that run
  for `main` or a `v*` tag can use it (deployment branch policy); pull
  requests, forks and other branches cannot.
- Account passwords, two-factor codes, payment and tax details are never
  handled by Claude or stored anywhere in the project. The owner signs in.

## Rules for workflows and scripts
1. Secrets are used only in the dedicated signing job, never in jobs that
   run for pull requests; no `pull_request_target`.
2. Secrets are written to files under `$RUNNER_TEMP` with mode 600 and a
   temporary keychain, never echoed, and removed at the end of the job
   (also on failure).
3. Signed artifacts contain only public material: the signature names the
   certificate holder and team ID, as every signed Mac app does.
4. The API key has the Developer role (notarization and uploads), not
   Admin: it cannot change the account, users, agreements or payments.
5. `scripts/check-secrets.py` runs in CI on every change (and can run as a
   pre-commit hook with `--staged`); `.gitignore` excludes key, certificate,
   profile and keychain files. GitHub secret scanning with push protection
   is on for the repository.
6. Personal details from the developer account (address, phone, email) are
   never copied into the repository, issues or logs.

## If something leaks
1. Revoke at once: the API key in App Store Connect › Users and Access ›
   Integrations; the certificate in Certificates, Identifiers & Profiles
   (apps already notarized keep working).
2. Create new ones, replace the GitHub secrets, delete the leaked commit
   or artifact, and record what happened in STATUS.
