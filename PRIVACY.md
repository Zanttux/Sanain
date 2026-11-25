# Privacy commitments
Sanain is designed for offline use. No advertising, analytics, mandatory accounts or telemetry. File contents remain local unless a user explicitly invokes a future network feature with informed consent. Disable remote Markdown resources by default; do not execute embedded scripts. Unsaved text is checkpointed locally for crash recovery: macOS
`~/Library/Application Support/Sanain/Recovery`, Windows
`%LOCALAPPDATA%\Sanain\Recovery`, Linux `$XDG_DATA_HOME/sanain/recovery`
(set `SANAIN_RECOVERY_DIR` to change it). Files are plain text, readable
only by your user account on macOS and Linux, and deleted when the document
is saved or its changes are discarded. They are not encrypted. Settings are stored in `settings` in the same per-user folder (Windows:
`%APPDATA%\Sanain`, Linux: `$XDG_CONFIG_HOME/sanain`). The list of open file paths is stored next to
that folder (`session`) to reopen them on the next launch. Crash reports stay local and require explicit review before sharing. Optional update checks must be documented and configurable. GitHub and sponsor platforms have their own privacy policies. This is a design commitment, not a claim about unimplemented software.
