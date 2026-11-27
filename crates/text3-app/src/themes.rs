//! Colour themes: the bundled ones and user themes from
//! `<config>/themes/*.theme`.
//!
//! Bundled themes re-create the published colour palettes of popular
//! terminal and editor schemes (all MIT licensed, credited in
//! docs/THEMES.md); a few colours are adjusted so text reaches WCAG AA
//! (4.5:1) and syntax colours at least 3:1 on the background, which the
//! tests check. "Classic" is Text³'s own light scheme in the style of
//! Notepad++'s default colours (no Notepad++ files are used).
//!
//! User theme format, one `key = value` per line (`#` starts a comment):
//! `name`, `dark` (true/false), and colours as `#RRGGBB` for `canvas`,
//! `surface`, `text`, `muted`, `accent`, `border`, `error`, `warning`,
//! `keyword`, `string`, `escape`, `comment`, `number`, `function`, `type`,
//! `tag`, `attribute`. Missing colours come from the Text³ light or dark
//! theme.

use std::path::Path;

use eframe::egui::Color32;

use crate::theme::{DARK, LIGHT, Palette, Syntax};

/// A selectable theme.
#[derive(Debug, Clone)]
pub struct ThemeEntry {
    /// Stable identifier saved in the settings (`user:<file stem>` for
    /// user themes).
    pub id: String,
    pub name: String,
    pub palette: &'static Palette,
}

const fn hex(v: u32) -> Color32 {
    Color32::from_rgb((v >> 16) as u8, (v >> 8) as u8, v as u8)
}

#[allow(clippy::too_many_arguments)]
const fn theme(
    dark: bool,
    canvas: u32,
    surface: u32,
    text: u32,
    muted: u32,
    accent: u32,
    border: u32,
    error: u32,
    warning: u32,
    syn: [u32; 9],
) -> Palette {
    Palette {
        dark,
        canvas: hex(canvas),
        surface: hex(surface),
        text: hex(text),
        muted: hex(muted),
        accent: hex(accent),
        border: hex(border),
        error: hex(error),
        warning: hex(warning),
        syntax: Syntax {
            keyword: hex(syn[0]),
            string: hex(syn[1]),
            escape: hex(syn[2]),
            comment: hex(syn[3]),
            number: hex(syn[4]),
            function: hex(syn[5]),
            type_: hex(syn[6]),
            tag: hex(syn[7]),
            attribute: hex(syn[8]),
        },
    }
}

// syn: keyword, string, escape, comment, number, function, type, tag, attribute
static SOLARIZED_DARK: Palette = theme(
    true,
    0x002b36,
    0x073642,
    0x93a1a1,
    0x839496,
    0x268bd2,
    0x0d4b5c,
    0xdc322f,
    0xb58900,
    [
        0x859900, 0x2aa198, 0xcb4b16, 0x6c8791, 0xd33682, 0x268bd2, 0xb58900, 0x268bd2, 0x6c71c4,
    ],
);
static SOLARIZED_LIGHT: Palette = theme(
    false,
    0xfdf6e3,
    0xeee8d5,
    0x586e75,
    0x6f8187,
    0x1f6fb0,
    0xd9d2c0,
    0xc42a24,
    0x8a6800,
    [
        0x6b7a00, 0x1f7a73, 0xb0410f, 0x76878d, 0xc42a72, 0x1f6fb0, 0x8a6800, 0x1f6fb0, 0x5a5fb0,
    ],
);
static DRACULA: Palette = theme(
    true,
    0x282a36,
    0x21222c,
    0xf8f8f2,
    0x7a88b8,
    0xbd93f9,
    0x44475a,
    0xff5555,
    0xf1fa8c,
    [
        0xff79c6, 0xf1fa8c, 0xff79c6, 0x7a88b8, 0xbd93f9, 0x50fa7b, 0x8be9fd, 0xff79c6, 0x50fa7b,
    ],
);
static NORD: Palette = theme(
    true,
    0x2e3440,
    0x3b4252,
    0xd8dee9,
    0x8a96ae,
    0x88c0d0,
    0x4c566a,
    0xbf616a,
    0xebcb8b,
    [
        0x81a1c1, 0xa3be8c, 0xebcb8b, 0x8a96ae, 0xb48ead, 0x88c0d0, 0x8fbcbb, 0x81a1c1, 0x8fbcbb,
    ],
);
static GRUVBOX_DARK: Palette = theme(
    true,
    0x282828,
    0x3c3836,
    0xebdbb2,
    0xa89984,
    0xfe8019,
    0x504945,
    0xfb4934,
    0xfabd2f,
    [
        0xfb4934, 0xb8bb26, 0xfe8019, 0x928374, 0xd3869b, 0x8ec07c, 0xfabd2f, 0x83a598, 0x8ec07c,
    ],
);
static ONE_DARK: Palette = theme(
    true,
    0x282c34,
    0x21252b,
    0xabb2bf,
    0x848b98,
    0x61afef,
    0x3e4451,
    0xe06c75,
    0xe5c07b,
    [
        0xc678dd, 0x98c379, 0x56b6c2, 0x848b98, 0xd19a66, 0x61afef, 0xe5c07b, 0xe06c75, 0xd19a66,
    ],
);
static TOMORROW_NIGHT: Palette = theme(
    true,
    0x1d1f21,
    0x282a2e,
    0xc5c8c6,
    0x969896,
    0x81a2be,
    0x373b41,
    0xcc6666,
    0xf0c674,
    [
        0xb294bb, 0xb5bd68, 0xde935f, 0x969896, 0xde935f, 0x81a2be, 0xf0c674, 0xcc6666, 0x8abeb7,
    ],
);
static CLASSIC: Palette = theme(
    false,
    0xffffff,
    0xf0f0f0,
    0x000000,
    0x767676,
    0x0078d7,
    0xc8c8c8,
    0xc00000,
    0x8a5200,
    [
        0x0000ff, 0x767676, 0xb05000, 0x008000, 0xb05000, 0x000080, 0x8000ff, 0x0000ff, 0xd00000,
    ],
);

/// Bundled themes (id, display name, palette); names are proper names
/// and not translated.
pub const BUNDLED: [(&str, &str, &Palette); 8] = [
    ("classic", "Classic (Notepad++ style)", &CLASSIC),
    ("solarized-light", "Solarized Light", &SOLARIZED_LIGHT),
    ("solarized-dark", "Solarized Dark", &SOLARIZED_DARK),
    ("dracula", "Dracula", &DRACULA),
    ("nord", "Nord", &NORD),
    ("gruvbox-dark", "Gruvbox Dark", &GRUVBOX_DARK),
    ("one-dark", "One Dark", &ONE_DARK),
    ("tomorrow-night", "Tomorrow Night", &TOMORROW_NIGHT),
];

fn parse_hex(v: &str) -> Option<Color32> {
    let v = v.strip_prefix('#')?;
    (v.len() == 6 && v.is_ascii())
        .then(|| u32::from_str_radix(v, 16).ok())
        .flatten()
        .map(hex)
}

/// Parse a user theme. Errors name the first bad line.
pub fn parse(text: &str) -> Result<(String, Palette), String> {
    let mut name = None;
    let mut dark = None;
    let mut colours: Vec<(String, Color32)> = Vec::new();
    for (n, raw) in text.lines().enumerate() {
        // `#` starts a comment at the start of a line or after a space;
        // colour values (`#RRGGBB`) follow `=` directly or after spaces.
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((k, v)) = line.split_once('=') else {
            return Err(format!("line {}: expected key = value", n + 1));
        };
        let (k, v) = (k.trim(), v.trim());
        let v = v.split(" #").next().unwrap_or(v).trim();
        match k {
            "name" if !v.is_empty() => name = Some(v.chars().take(60).collect::<String>()),
            "dark" => dark = Some(v == "true"),
            _ => match parse_hex(v) {
                Some(c) => colours.push((k.to_owned(), c)),
                None => return Err(format!("line {}: {k} is not a #RRGGBB colour", n + 1)),
            },
        }
    }
    let name = name.ok_or_else(|| "missing name".to_owned())?;
    let dark = dark.unwrap_or(true);
    let mut p = if dark { DARK } else { LIGHT };
    for (k, c) in colours {
        let slot = match k.as_str() {
            "canvas" => &mut p.canvas,
            "surface" => &mut p.surface,
            "text" => &mut p.text,
            "muted" => &mut p.muted,
            "accent" => &mut p.accent,
            "border" => &mut p.border,
            "error" => &mut p.error,
            "warning" => &mut p.warning,
            "keyword" => &mut p.syntax.keyword,
            "string" => &mut p.syntax.string,
            "escape" => &mut p.syntax.escape,
            "comment" => &mut p.syntax.comment,
            "number" => &mut p.syntax.number,
            "function" => &mut p.syntax.function,
            "type" => &mut p.syntax.type_,
            "tag" => &mut p.syntax.tag,
            "attribute" => &mut p.syntax.attribute,
            other => return Err(format!("unknown key {other}")),
        };
        *slot = c;
    }
    p.dark = dark;
    Ok((name, p))
}

/// All themes: bundled, then user themes from `config_dir/themes`. Bad
/// user theme files are returned as (file name, error).
pub fn all(config_dir: Option<&Path>) -> (Vec<ThemeEntry>, Vec<(String, String)>) {
    let mut out: Vec<ThemeEntry> = BUNDLED
        .iter()
        .map(|(id, name, p)| ThemeEntry {
            id: (*id).to_owned(),
            name: (*name).to_owned(),
            palette: p,
        })
        .collect();
    let mut errors = Vec::new();
    let Some(dir) = config_dir.map(|d| d.join("themes")) else {
        return (out, errors);
    };
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return (out, errors);
    };
    let mut files: Vec<_> = entries
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|e| e == "theme"))
        .collect();
    files.sort();
    for path in files.into_iter().take(100) {
        let file = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        let stem = path
            .file_stem()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        let text = match std::fs::metadata(&path) {
            Ok(m) if m.len() <= 64 * 1024 => {
                std::fs::read_to_string(&path).map_err(|e| e.to_string())
            }
            Ok(_) => Err("file too large".to_owned()),
            Err(e) => Err(e.to_string()),
        };
        match text.and_then(|t| parse(&t)) {
            Ok((name, p)) => out.push(ThemeEntry {
                id: format!("user:{stem}"),
                name,
                // A handful of small palettes, loaded once per refresh.
                palette: Box::leak(Box::new(p)),
            }),
            Err(e) => errors.push((file, e)),
        }
    }
    (out, errors)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lum(c: Color32) -> f64 {
        let f = |v: u8| {
            let c = f64::from(v) / 255.0;
            if c <= 0.03928 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        };
        0.2126 * f(c.r()) + 0.7152 * f(c.g()) + 0.0722 * f(c.b())
    }

    fn contrast(a: Color32, b: Color32) -> f64 {
        let (x, y) = (lum(a), lum(b));
        (x.max(y) + 0.05) / (x.min(y) + 0.05)
    }

    #[test]
    fn bundled_themes_are_readable() {
        for (id, _, p) in BUNDLED {
            assert!(contrast(p.text, p.canvas) >= 4.5, "{id}: text");
            assert!(contrast(p.muted, p.canvas) >= 3.0, "{id}: muted");
            let s = p.syntax;
            for (k, c) in [
                ("keyword", s.keyword),
                ("string", s.string),
                ("escape", s.escape),
                ("comment", s.comment),
                ("number", s.number),
                ("function", s.function),
                ("type", s.type_),
                ("tag", s.tag),
                ("attribute", s.attribute),
            ] {
                assert!(contrast(c, p.canvas) >= 3.0, "{id}: {k}");
            }
            assert_eq!(p.dark, lum(p.canvas) < 0.2, "{id}: dark flag");
        }
    }

    #[test]
    fn user_themes() {
        let (name, p) = parse(
            "# my theme\nname = Ocean\ndark = true\ncanvas = #001122 # deep\nkeyword=#ffcc00\n",
        )
        .unwrap();
        assert_eq!(name, "Ocean");
        assert!(p.dark);
        assert_eq!(p.canvas, Color32::from_rgb(0x00, 0x11, 0x22));
        assert_eq!(p.syntax.keyword, Color32::from_rgb(0xff, 0xcc, 0x00));
        assert_eq!(p.text, DARK.text, "missing colours from the base");
        assert!(
            parse("name = X\ncanvas = red\n")
                .unwrap_err()
                .contains("line 2")
        );
        assert!(parse("canvas = #000000\n").is_err(), "name required");
        assert!(parse("name = X\nbogus = #000000\n").is_err());

        let d = tempfile::tempdir().unwrap();
        std::fs::create_dir(d.path().join("themes")).unwrap();
        std::fs::write(d.path().join("themes/ocean.theme"), "name = Ocean\n").unwrap();
        std::fs::write(d.path().join("themes/bad.theme"), "nonsense\n").unwrap();
        let (list, errors) = all(Some(d.path()));
        assert_eq!(list.len(), BUNDLED.len() + 1);
        assert_eq!(list.last().unwrap().id, "user:ocean");
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].0, "bad.theme");
    }
}
