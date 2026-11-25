# Internationalization from M0
English is the canonical language for code, docs, issue templates and identifiers. User-facing UI must be localizable from the first screen. Proposed initial locales en-US and fi-FI; expand based on community demand. Use BCP 47 tags, OS locale detection, explicit override and deterministic fallback exact → language → en-US. Persist selection; never infer language from geography.

Evaluate Fluent (`fluent-bundle`, `unic-langid`) in M0. Keep UI-independent localization service; typed stable message IDs, named parameters, plural/select rules, translator comments. No string concatenation for sentences; do not translate file contents, CLI flags, paths, code or user data. Locale-aware display of numbers/dates only; machine formats remain invariant. Native menus, errors, accessibility names, dialogs, settings and update messages included. Load embedded resources offline; missing translations fall back to English and log only key IDs. No network translation at runtime.

Design for 30–50% expansion, Unicode graphemes, CJK IME, bidirectional text and RTL layout; do not claim RTL support until verified. Test font fallback and keyboard shortcuts independently of locale. Preserve Markdown source and codepoints. Pseudolocale en-XA exercises expansion and accents; ar-XB exercises directionality. Never publish machine translations as human-verified. Translation contributions use PRs initially; evaluate Weblate when volume warrants it. Keep translation memory free of user files and secrets.

CI: parse all FTL, reject duplicate IDs and incompatible variables, report missing keys, require complete en-US and reviewed launch locales, smoke-test fallback and locale switch. Document plural cases and screenshot checks. New UI strings without localization keys fail review. M0 spike demonstrates one menu, pluralized status, translated error, fallback and long label. M1 includes localization crate and test harness; M6 verifies launch locales on all platforms.

## Shipped languages (2026-11-25)

| Status | Languages |
|---|---|
| Canonical | en-US |
| Draft by Claude, native review pending | fi-FI |
| Machine translation by Claude, native review pending | cs-CZ, da-DK, de-DE, el-GR, es-ES, fr-FR, hu-HU, id-ID, it-IT, ja-JP, ko-KR, nb-NO, nl-NL, pl-PL, pt-BR, pt-PT, ro-RO, ru-RU, sv-SE, tr-TR, uk-UA, vi-VN, zh-CN, zh-TW |
| Not shipped yet | Right-to-left languages (Arabic, Hebrew, Persian, Urdu): the editor's RTL layout and bidirectional caret movement are not verified |

Every non-English file starts with a comment stating it is not yet
reviewed. Corrections are welcome as pull requests; a language counts as
reviewed when a fluent speaker has checked every message in the running
app. CI checks that every file parses, uses only en-US message IDs and the
same variables.
