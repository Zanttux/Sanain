# Workflows

`ci.yml` runs fmt, clippy and tests on Linux, Windows and macOS for every PR
and push to `main`. It has a read-only token, no secrets and runs only on
GitHub-hosted runners.

Rules for all workflows:
- Pin third-party actions to a full commit SHA with the tag in a comment.
- Default `permissions: contents: read`; widen per job only when required.
- Never use `pull_request_target` or `workflow_run` to execute PR code.
- Never interpolate untrusted input (titles, branch names) into `run:`.
- Release workflows run only from protected tags in an owner-approved
  environment, and are added only after the release policy is approved.

Planned: cargo-deny (licenses, advisories), localization checks beyond the
test suite, fuzz smoke, release packaging.
