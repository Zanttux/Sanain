# Brand system — Text³

Public name **Text³** (U+00B3 SUPERSCRIPT THREE), spoken "Text Cubed".
ASCII technical name `text3` for paths, the CLI, package identifiers and
URLs. Formerly Sanain (D04/D20, superseded by D27); historical release
names and changelog entries keep the old name. Positioning: a native,
free, open-source editor for everyday text and technical files. No
slogan, no three-part taglines, no generic AI marketing, no national
symbols. Never imply official Notepad++ affiliation. Avoid unverified
claims of speed, accessibility or AI provenance; AI involvement is
disclosed plainly (README, AI_POLICY, About).

## Artwork
The owner-approved reference is `assets/brand/approved-reference.png`
(handoff 2026-11-27, kept byte-identical; SHA-256
`905c37d205501f96fbc2759de4ebe07d3fd4f80e1e0ce2c174149efc84c500f6`). An
ochre lynx with a cream ruff, dark ear tufts, short tail and sparse spots
leans on a folded document; the wordmark "Text" with a green ³ and a green
underline swoosh. Keep the pose, silhouette and colours; no stretching,
rotation, recolouring or cropped ears.

| File | Use |
|---|---|
| `assets/brand/text3-lockup.svg` | Horizontal lockup (headers, About, installer welcome, social card) |
| `assets/brand/text3-lockup-plate.svg` | Lockup on a white plate for dark surfaces |
| `assets/brand/text3-lockup-mono.svg` | One-colour lockup (ink on white) |
| `assets/brand/text3-mascot.svg` | Mascot with document: application icon master |
| `assets/brand/text3-document.svg` | Document glyph: associated files (never the app identity) |
| `assets/brand/drafts/` | The handoff raster drafts, unchanged (provenance) |

The SVG masters are flat-fill vector traces of the approved reference
(scripts/brand-trace.py: vtracer, colours snapped to the artwork's own
palette, incidental shading flattened, no embedded raster). They are draft
masters until a designer reviews them (docs/REBRAND.md). Every raster in
`packaging/`, `website/` and `assets/brand/` is rendered from them by
`scripts/make-icons.py`.

Use the full lockup above 180 px width; below that the mascot icon alone.
Clear space at least 1/8 of the icon width around the lockup. On dark
surfaces put the artwork on a neutral white plate instead of recolouring
it.

Icons: 48 px and larger show the whole mascot with 1/16 clear space.
Small sizes are hinted rather than scaled: 16 and 24 px show the face
only, 32 px the head with ear tufts, each with a 1 px white outline so the
dark contour stays visible on dark taskbars. The macOS icon puts the
mascot on a white rounded plate on Apple's icon grid (824/1024).

## Colour and type
Artwork palette (sampled, used only inside the artwork): ink #221E13,
cream #FBF1E1, ochre #EC9C31, tan #E6C595, line #737D51, check #72674E,
green #6F811B. Implementation tokens (`assets/brand/tokens.json`): paper
#FFF7E5, ink #25251F, ochre #E5A13B, green #788B53, meadow #D4E4B4.

Website and UI surfaces are neutral white #FFFFFF or light gray #F4F4F4
with #D4D4D4 borders. Paper #FFF7E5 is restricted to the mascot and
document artwork: never a page background, section, card, navigation or
logo backing plate. Text is ink on white; green is an accent only (links
use a darker green that meets WCAG AA). System UI typography (Segoe UI,
system-ui, sans-serif); no bundled font files; the wordmark is artwork,
not live text. The editor font stays user-configurable.

The application's own colour themes (`crates/text3-app/src/theme.rs`)
are unchanged by the rebrand: every syntax foreground meets 4.5:1 on the
editor canvas in both themes. Never force brand colours into syntax
themes.

## Principles
- Every element has a job. No decorative gradients, glows or glossy
  effects, no generic SaaS illustrations.
- Follow platform conventions for menus, shortcuts, dialogs and focus.
- Design for long sessions: calm contrast, no motion without purpose.
- Marketing screenshots must come from real builds.
- Brand files are not cleared trademarks. The owner approves the final
  visual review and trademark/domain clearance before public cutover.
