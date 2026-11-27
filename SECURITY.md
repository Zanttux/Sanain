# Security policy

Report vulnerabilities privately through GitHub:
**Security → Report a vulnerability** on github.com/Zanttux/text3.
Do not open public issues for security problems or attach real documents.

Text³ is pre-alpha; no version is supported for production use yet and
there is no response-time commitment. Reports are acknowledged and triaged
as capacity allows.

In scope: data loss or corruption, hostile files (parser crashes, resource
exhaustion), unsafe rendering, unexpected network access, recovery-file
leakage, supply-chain issues in the build.

Design rules: offline by default, no scripts or network from documents,
bounded parsing, `unsafe_code` forbidden in project crates, locked
dependencies.
