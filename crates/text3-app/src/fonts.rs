//! Fonts for the editor font setting: the installed monospaced families
//! plus the fonts that ship with Text³, and the bytes of a family's
//! regular face. Fonts are only read, never installed or changed. The scan
//! memory-maps each font file just long enough to read its names, so only
//! those pages are touched; only the chosen face's file is read in full.
//!
//! Bundled fonts (#176) are separate files in a `fonts` folder next to the
//! program, not part of the executable, so they cost nothing at start-up
//! or in memory until chosen: JetBrains Mono, Fira Code, Cascadia Code,
//! Source Code Pro, IBM Plex Mono and Inconsolata (SIL OFL 1.1, licences
//! beside them in assets/fonts). Hack is egui's built-in monospace font.

use std::path::{Path, PathBuf};

/// The most liked coding fonts among Notepad++, Sublime Text and other
/// editor users, best known first (survey in docs/FONTS.md). Shown at the
/// top of the list when available. Six of them ship with Text³; the
/// others come with the operating system or are commonly installed.
pub const RECOMMENDED: &[&str] = &[
    "JetBrains Mono",
    "Fira Code",
    "Cascadia Code",
    "Consolas",
    "Source Code Pro",
    "Menlo",
    "SF Mono",
    "Monaco",
    "Inconsolata",
    "IBM Plex Mono",
    "Cascadia Mono",
    "DejaVu Sans Mono",
    "Ubuntu Mono",
    "Hack",
    "Courier New",
    "Lucida Console",
];

/// The families that ship with Text³ (files in the `fonts` folder).
pub const BUNDLED: &[&str] = &[
    "JetBrains Mono",
    "Fira Code",
    "Cascadia Code",
    "Source Code Pro",
    "IBM Plex Mono",
    "Inconsolata",
];

/// The fonts available to choose, recommended ones first.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FontList {
    /// Available families from [`RECOMMENDED`], in that order.
    pub recommended: Vec<String>,
    /// Every other monospaced family, sorted.
    pub others: Vec<String>,
}

/// Where the bundled fonts are for the executable `exe`: `fonts` next to
/// it (Windows, portable archives), `Contents/Resources/fonts` in
/// Text³.app, `/usr/share/text3/fonts` for `/usr/bin/text3` (deb).
pub fn bundled_dir_for(exe: &Path) -> Option<PathBuf> {
    let dir = exe.parent()?;
    let mut candidates = vec![
        dir.join("fonts"),
        dir.join("../Resources/fonts"),
        dir.join("../share/text3/fonts"),
    ];
    // Development builds run from target/: use the repository's copies.
    if cfg!(debug_assertions) {
        candidates.push(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets/fonts"));
    }
    candidates.into_iter().find(|d| d.is_dir())
}

fn database() -> fontdb::Database {
    let mut db = fontdb::Database::new();
    db.load_system_fonts();
    if let Some(dir) = std::env::current_exe()
        .ok()
        .and_then(|e| e.canonicalize().ok())
        .and_then(|e| bundled_dir_for(&e))
    {
        db.load_fonts_dir(dir);
    }
    db
}

/// Sort `families` into recommended and other monospaced fonts.
/// `families` are (name, is monospaced) pairs; recommended fonts count
/// even when the font does not mark itself fixed-pitch.
pub fn arrange(families: impl IntoIterator<Item = (String, bool)>) -> FontList {
    let mut all: Vec<(String, bool)> = families
        .into_iter()
        .filter(|(n, _)| !n.starts_with('.'))
        .collect();
    all.sort_by_key(|(n, _)| n.to_lowercase());
    let recommended: Vec<String> = RECOMMENDED
        .iter()
        .filter_map(|r| {
            all.iter()
                .find(|(n, _)| n.eq_ignore_ascii_case(r))
                .map(|(n, _)| n.clone())
        })
        .collect();
    let mut others: Vec<String> = all
        .into_iter()
        .filter(|(n, mono)| *mono && !recommended.iter().any(|r| r.eq_ignore_ascii_case(n)))
        .map(|(n, _)| n)
        .collect();
    others.dedup();
    FontList {
        recommended,
        others,
    }
}

/// Installed and bundled fonts to choose from. Scans the system font
/// folders, so call it off the UI thread.
pub fn monospace_families() -> FontList {
    let db = database();
    arrange(
        db.faces()
            .filter_map(|f| f.families.first().map(|(n, _)| (n.clone(), f.monospaced))),
    )
}

/// The regular face of `family`: (file bytes, face index in the file).
pub fn load_family(family: &str) -> Option<(Vec<u8>, u32)> {
    let db = database();
    let query = fontdb::Query {
        families: &[fontdb::Family::Name(family)],
        ..Default::default()
    };
    let id = db.query(&query)?;
    let face = db.face(id)?;
    let bytes = match &face.source {
        fontdb::Source::File(path) => std::fs::read(path).ok()?,
        fontdb::Source::Binary(data) | fontdb::Source::SharedFile(_, data) => {
            data.as_ref().as_ref().to_vec()
        }
    };
    Some((bytes, face.index))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_family_is_none() {
        assert!(load_family("No Such Font Family 1234").is_none());
    }

    /// Every bundled family is in assets/fonts with its licence, is found
    /// by name, and is recommended.
    #[test]
    fn bundled_fonts_load_by_name() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets/fonts");
        let mut db = fontdb::Database::new();
        db.load_fonts_dir(&dir);
        for family in BUNDLED {
            assert!(RECOMMENDED.contains(family), "{family}");
            let query = fontdb::Query {
                families: &[fontdb::Family::Name(family)],
                ..Default::default()
            };
            let id = db
                .query(&query)
                .unwrap_or_else(|| panic!("{family} not found"));
            let face = db.face(id).unwrap();
            assert_eq!(face.weight, fontdb::Weight::NORMAL, "{family} regular");
            assert_eq!(face.style, fontdb::Style::Normal, "{family}");
            let fontdb::Source::File(path) = &face.source else {
                panic!("{family}: not a file")
            };
            let stem = path.file_stem().unwrap().to_string_lossy();
            let base = stem.trim_end_matches("-Regular");
            assert!(
                dir.join(format!("{base}-OFL.txt")).is_file(),
                "{family}: licence next to {}",
                path.display()
            );
        }
        // The real scan (debug build: the repository copies) finds them.
        let list = monospace_families();
        for family in BUNDLED {
            assert!(
                list.recommended.iter().any(|f| f == family),
                "{family}: {list:?}"
            );
        }
    }

    #[test]
    fn arrange_puts_recommended_first() {
        let list = arrange([
            ("Zed Mono".to_owned(), true),
            ("consolas".to_owned(), true),
            ("Arial".to_owned(), false),
            ("Fira Code".to_owned(), false),
            ("JetBrains Mono".to_owned(), true),
            ("JetBrains Mono".to_owned(), true),
            (".Hidden".to_owned(), true),
            ("Andale Mono".to_owned(), true),
        ]);
        assert_eq!(
            list.recommended,
            ["JetBrains Mono", "Fira Code", "consolas"]
        );
        assert_eq!(list.others, ["Andale Mono", "Zed Mono"]);
    }

    #[test]
    fn bundled_dir_layouts() {
        let d = tempfile::tempdir().unwrap();
        let app = d.path().join("Text³.app/Contents");
        std::fs::create_dir_all(app.join("MacOS")).unwrap();
        std::fs::create_dir_all(app.join("Resources/fonts")).unwrap();
        let found = bundled_dir_for(&app.join("MacOS/text3")).unwrap();
        assert!(found.ends_with("Resources/fonts"));
        let usr = d.path().join("usr");
        std::fs::create_dir_all(usr.join("bin")).unwrap();
        std::fs::create_dir_all(usr.join("share/text3/fonts")).unwrap();
        assert!(
            bundled_dir_for(&usr.join("bin/text3"))
                .unwrap()
                .ends_with("share/text3/fonts")
        );
        let win = d.path().join("Text3");
        std::fs::create_dir_all(win.join("fonts")).unwrap();
        assert_eq!(
            bundled_dir_for(&win.join("text3.exe")),
            Some(win.join("fonts"))
        );
    }
}
