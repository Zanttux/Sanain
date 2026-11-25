# Privacy commitments
Sanain is designed for offline use. No advertising, analytics, mandatory accounts or telemetry. File contents remain local unless a user explicitly invokes a future network feature with informed consent. Remote Markdown resources are never loaded (images from the web are shown as a note; local images only from the document's folder); do not execute embedded scripts. Unsaved text is checkpointed locally for crash recovery: macOS
`~/Library/Application Support/Sanain/Recovery`, Windows
`%LOCALAPPDATA%\Sanain\Recovery`, Linux `$XDG_DATA_HOME/sanain/recovery`
(set `SANAIN_RECOVERY_DIR` to change it). Files are plain text, readable
only by your user account on macOS and Linux, and deleted when the document
is saved or its changes are discarded. They are not encrypted. Settings are stored in `settings` in the same per-user folder (Windows:
`%APPDATA%\Sanain`, Linux: `$XDG_CONFIG_HOME/sanain`). The list of open file paths is stored next to
that folder (`session`) to reopen them on the next launch. Crash reports stay local and require explicit review before sharing. Local history: when you save, Sanain keeps a copy of the saved file in
its data folder (next to crash recovery), readable only by your user
account on macOS and Linux, unencrypted. Up to 50 versions per file are
kept for 30 days; files over 10 MB are skipped. Turn it off in Settings;
File > Clear Local History deletes all copies.

Update checks: Sanain contacts the network only when you choose "Check
for Updates" or turn on "Check for updates once a day" in Settings (off by
default). It then sends one HTTPS request to api.github.com for the latest
Sanain release, identifying itself as `Sanain/<version>`; nothing about you
or your files is sent. GitHub's own privacy policy applies to that request. GitHub and sponsor platforms have their own privacy policies. This is a design commitment, not a claim about unimplemented software.
