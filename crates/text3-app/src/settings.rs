//! User settings, stored as a small versioned text file.
//!
//! Format: first line `text3-settings 1` (`sanain-settings 1`, written
//! before the rename, is read the same way), then `key=value` lines. Unknown
//! keys and invalid values are ignored (the default is kept), so files from
//! newer versions or hand edits never prevent startup.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const HEADER: &str = "text3-settings 1";
/// The header before the rename to Text³ (docs/REBRAND.md).
const LEGACY_HEADER: &str = "sanain-settings 1";

pub const FONT_SIZE_MIN: f32 = 8.0;
pub const FONT_SIZE_MAX: f32 = 40.0;
pub const FONT_SIZE_DEFAULT: f32 = 14.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeChoice {
    System,
    Light,
    Dark,
    HighContrast,
}

/// Name of the marker file that turns on portable mode.
pub const PORTABLE_MARKER: &str = "text3-portable";
/// The marker's name before the rename; still honoured, so a portable
/// Sanain folder updated with Text³ keeps its data folder.
pub const LEGACY_PORTABLE_MARKER: &str = "sanain-portable";

/// Portable mode: with a `text3-portable` (or `sanain-portable`) file next
/// to the executable (or next to Text³.app on macOS), settings, recovery
/// and history live in a `data` folder beside it instead of the user
/// profile.
pub fn portable_root() -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    portable_root_for(&exe)
}

/// [`portable_root`] for a given executable path.
pub fn portable_root_for(exe: &std::path::Path) -> Option<PathBuf> {
    let dir = exe.parent()?;
    // Text³.app/Contents/MacOS/text3 → the folder holding Text³.app.
    let bundle_parent = app_bundle_of(exe).and_then(|app| app.parent());
    [Some(dir), bundle_parent]
        .into_iter()
        .flatten()
        .find(|d| d.join(PORTABLE_MARKER).is_file() || d.join(LEGACY_PORTABLE_MARKER).is_file())
        .map(|d| d.join("data"))
}

/// The macOS app bundle (`….app`) holding `exe`, if it runs from one
/// (`….app/Contents/MacOS/<exe>`). Only the path is looked at.
pub fn app_bundle_of(exe: &std::path::Path) -> Option<&std::path::Path> {
    exe.parent()
        .filter(|d| d.ends_with("MacOS"))
        .and_then(|d| d.parent())
        .filter(|c| c.ends_with("Contents"))
        .and_then(|c| c.parent())
        .filter(|app| app.extension().is_some_and(|e| e == "app"))
}

/// The per-user configuration folder named `name` on this platform.
fn platform_config_dir(name: &str) -> Option<PathBuf> {
    let env = |k: &str| {
        std::env::var_os(k)
            .filter(|v| !v.is_empty())
            .map(PathBuf::from)
    };
    if cfg!(target_os = "macos") {
        env("HOME").map(|h| h.join("Library/Application Support").join(name))
    } else if cfg!(windows) {
        env("APPDATA").map(|d| d.join(name))
    } else {
        env("XDG_CONFIG_HOME")
            .or_else(|| env("HOME").map(|h| h.join(".config")))
            .map(|d| d.join(name))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Settings {
    /// Editor font size in points.
    pub font_size: f32,
    /// Columns per indentation level (1–16).
    pub tab_width: u8,
    /// Tab key inserts spaces instead of a tab character.
    pub insert_spaces: bool,
    pub theme: ThemeChoice,
    /// Reopen the previous session's files when started without arguments.
    pub restore_session: bool,
    /// Hot exit: quitting keeps unsaved and untitled tabs (in the recovery
    /// folder) instead of asking about each; they come back at the next
    /// start even when `restore_session` is off.
    pub hot_exit: bool,
    /// UI language as a BCP 47 tag; `None` follows the system.
    pub locale: Option<String>,
    pub keymap: crate::keymap::Keymap,
    /// Check GitHub for a newer release once a day (off by default: no
    /// network use without the user's choice).
    pub check_updates: bool,
    /// Unix time of the last automatic update check.
    pub last_update_check: u64,
    /// Keep a local copy of each saved version (docs/PRIVACY).
    pub local_history: bool,
    /// Soft-wrap long lines to the window width.
    pub word_wrap: bool,
    /// Editor font family; `None` uses the built-in monospace font.
    pub editor_font: Option<String>,
    /// A bundled or user colour theme (its id) replacing `theme`.
    pub color_theme: Option<String>,
    /// Show the character and word count in the status bar.
    pub show_counts: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            font_size: FONT_SIZE_DEFAULT,
            tab_width: 4,
            insert_spaces: false,
            theme: ThemeChoice::System,
            restore_session: true,
            hot_exit: true,
            locale: None,
            keymap: crate::keymap::Keymap::default(),
            check_updates: false,
            last_update_check: 0,
            local_history: true,
            word_wrap: true,
            editor_font: None,
            color_theme: None,
            show_counts: true,
        }
    }
}

impl Settings {
    /// Per-user configuration directory, e.g.
    /// `~/Library/Application Support/text3` on macOS. `TEXT3_CONFIG_DIR`
    /// (or the old `SANAIN_CONFIG_DIR`) and portable mode override it.
    pub fn default_dir() -> Option<PathBuf> {
        if let Some(d) = crate::migrate::var_os("CONFIG_DIR") {
            return Some(PathBuf::from(d));
        }
        if let Some(root) = portable_root() {
            return Some(root.join("settings"));
        }
        platform_config_dir("text3")
    }

    /// Where Sanain kept its settings before the rename (copied once by
    /// [`crate::migrate`]).
    pub fn legacy_default_dir() -> Option<PathBuf> {
        platform_config_dir(if cfg!(any(target_os = "macos", windows)) {
            "Sanain"
        } else {
            "sanain"
        })
    }

    pub fn encode(&self) -> String {
        let theme = match self.theme {
            ThemeChoice::System => "system",
            ThemeChoice::Light => "light",
            ThemeChoice::Dark => "dark",
            ThemeChoice::HighContrast => "high-contrast",
        };
        let mut out = format!(
            "{HEADER}\nfont_size={}\ntab_width={}\ninsert_spaces={}\ntheme={theme}\nrestore_session={}\n",
            self.font_size, self.tab_width, self.insert_spaces, self.restore_session
        );
        if let Some(l) = &self.locale {
            out.push_str(&format!("locale={l}\n"));
        }
        out.push_str(&format!(
            "check_updates={}\nlast_update_check={}\nlocal_history={}\nword_wrap={}\nhot_exit={}\nshow_counts={}\n",
            self.check_updates,
            self.last_update_check,
            self.local_history,
            self.word_wrap,
            self.hot_exit,
            self.show_counts
        ));
        if let Some(f) = &self.editor_font {
            out.push_str(&format!("editor_font={f}\n"));
        }
        if let Some(t) = &self.color_theme {
            out.push_str(&format!("color_theme={t}\n"));
        }
        out.push_str(&self.keymap.encode());
        out
    }

    /// Parse settings; returns defaults for anything missing or invalid.
    /// `None` only if the header is wrong (a different or unknown format).
    pub fn decode(text: &str) -> Option<Self> {
        let mut lines = text.lines();
        if !matches!(lines.next()?.trim(), HEADER | LEGACY_HEADER) {
            return None;
        }
        let mut s = Settings::default();
        for line in lines {
            let Some((k, v)) = line.split_once('=') else {
                continue;
            };
            let v = v.trim();
            match k.trim() {
                "font_size" => {
                    if let Ok(x) = v.parse::<f32>()
                        && x.is_finite()
                    {
                        s.font_size = x.clamp(FONT_SIZE_MIN, FONT_SIZE_MAX);
                    }
                }
                "tab_width" => {
                    if let Ok(x) = v.parse::<u8>() {
                        s.tab_width = x.clamp(1, 16);
                    }
                }
                "insert_spaces" => s.insert_spaces = v.parse().unwrap_or(s.insert_spaces),
                "restore_session" => s.restore_session = v.parse().unwrap_or(s.restore_session),
                "theme" => {
                    s.theme = match v {
                        "light" => ThemeChoice::Light,
                        "dark" => ThemeChoice::Dark,
                        "high-contrast" => ThemeChoice::HighContrast,
                        _ => ThemeChoice::System,
                    }
                }
                "locale" if !v.is_empty() && v.len() <= 35 => s.locale = Some(v.to_owned()),
                "check_updates" => s.check_updates = v.parse().unwrap_or(false),
                "last_update_check" => s.last_update_check = v.parse().unwrap_or(0),
                "local_history" => s.local_history = v.parse().unwrap_or(true),
                "word_wrap" => s.word_wrap = v.parse().unwrap_or(true),
                "hot_exit" => s.hot_exit = v.parse().unwrap_or(true),
                "show_counts" => s.show_counts = v.parse().unwrap_or(true),
                "editor_font" if !v.is_empty() && v.len() <= 200 => {
                    s.editor_font = Some(v.to_owned())
                }
                "color_theme" if !v.is_empty() && v.len() <= 200 => {
                    s.color_theme = Some(v.to_owned())
                }
                other => {
                    s.keymap.read_line(other, v);
                }
            }
        }
        Some(s)
    }

    /// Load from `dir/settings`; defaults if missing or unreadable.
    pub fn load(dir: &Path) -> Self {
        fs::read_to_string(dir.join("settings"))
            .ok()
            .and_then(|t| Self::decode(&t))
            .unwrap_or_default()
    }

    pub fn save(&self, dir: &Path) -> io::Result<()> {
        fs::create_dir_all(dir)?;
        text3_io::write_atomic(&dir.join("settings"), self.encode().as_bytes(), None).map(|_| ())
    }

    /// Text inserted by the Tab key.
    pub fn indent(&self) -> String {
        if self.insert_spaces {
            " ".repeat(usize::from(self.tab_width))
        } else {
            "\t".to_owned()
        }
    }

    pub fn zoom(&mut self, step: f32) {
        self.font_size = (self.font_size + step).clamp(FONT_SIZE_MIN, FONT_SIZE_MAX);
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn portable_marker() {
        let d = tempfile::tempdir().unwrap();
        let exe = d.path().join("text3");
        assert_eq!(portable_root_for(&exe), None);
        std::fs::write(d.path().join(PORTABLE_MARKER), "").unwrap();
        assert_eq!(portable_root_for(&exe), Some(d.path().join("data")));
        let mac = d.path().join("Text³.app/Contents/MacOS/text3");
        std::fs::create_dir_all(mac.parent().unwrap()).unwrap();
        assert_eq!(
            portable_root_for(&mac),
            Some(d.path().join("data")),
            "marker next to Text³.app"
        );
    }

    #[test]
    fn app_bundle_is_recognised_from_the_path() {
        let exe = std::path::Path::new("/Applications/Text³.app/Contents/MacOS/text3");
        assert_eq!(
            app_bundle_of(exe),
            Some(std::path::Path::new("/Applications/Text³.app"))
        );
        for plain in [
            "/usr/local/bin/text3",
            "/x/target/release/text3",
            "/x/Contents/MacOS/text3",
            "/x/Text³.app/MacOS/text3",
        ] {
            assert_eq!(app_bundle_of(std::path::Path::new(plain)), None, "{plain}");
        }
    }

    /// A portable Sanain folder updated with Text³ keeps using its data.
    #[test]
    fn legacy_portable_marker() {
        let d = tempfile::tempdir().unwrap();
        let exe = d.path().join("text3");
        std::fs::write(d.path().join(LEGACY_PORTABLE_MARKER), "").unwrap();
        assert_eq!(portable_root_for(&exe), Some(d.path().join("data")));
    }

    #[test]
    fn new_and_legacy_folders_differ_only_in_name() {
        let (Some(new), Some(old)) = (platform_config_dir("text3"), Settings::legacy_default_dir())
        else {
            return; // no HOME/APPDATA in this environment
        };
        assert_eq!(new.parent(), old.parent());
        assert_eq!(new.file_name().unwrap(), "text3");
        assert!(old.ends_with("Sanain") || old.ends_with("sanain"));
    }
    use super::*;

    /// Written as Text³; files from Sanain are still read (docs/REBRAND.md).
    #[test]
    fn header_new_and_legacy() {
        let s = Settings {
            tab_width: 3,
            ..Settings::default()
        };
        assert!(s.encode().starts_with("text3-settings 1\n"));
        let old = Settings::decode("sanain-settings 1\ntab_width=3\n").unwrap();
        assert_eq!(old.tab_width, 3);
        assert_eq!(Settings::decode("other-settings 1\ntab_width=3\n"), None);
    }

    #[test]
    fn round_trip() {
        let s = Settings {
            font_size: 16.5,
            tab_width: 2,
            insert_spaces: true,
            theme: ThemeChoice::Light,
            restore_session: false,
            hot_exit: false,
            locale: Some("fi-FI".into()),
            keymap: {
                let mut k = crate::keymap::Keymap::default();
                k.read_line("keymap", "notepad++");
                k.read_line("key.file.save", "Mod+Alt+S");
                k
            },
            check_updates: true,
            last_update_check: 1_764_000_000,
            local_history: false,
            word_wrap: false,
            editor_font: Some("JetBrains Mono".into()),
            color_theme: Some("nord".into()),
            show_counts: false,
        };
        assert_eq!(Settings::decode(&s.encode()), Some(s.clone()));
        let hc = Settings {
            theme: ThemeChoice::HighContrast,
            ..Settings::default()
        };
        assert_eq!(Settings::decode(&hc.encode()), Some(hc));
        let dir = tempfile::tempdir().unwrap();
        s.save(dir.path()).unwrap();
        assert_eq!(Settings::load(dir.path()), s);
    }

    #[test]
    fn invalid_values_fall_back_and_clamp() {
        let s = Settings::decode(
            "sanain-settings 1\nfont_size=NaN\ntab_width=200\ninsert_spaces=maybe\ntheme=purple\nnew_key=1\nnoequals\n",
        )
        .unwrap();
        assert_eq!(s.font_size, FONT_SIZE_DEFAULT);
        assert_eq!(s.tab_width, 16);
        assert!(!s.insert_spaces);
        assert_eq!(s.theme, ThemeChoice::System);
        assert_eq!(Settings::decode("font_size=12"), None);
        assert_eq!(
            Settings::decode("sanain-settings 1\nfont_size=999")
                .unwrap()
                .font_size,
            FONT_SIZE_MAX
        );
    }

    #[test]
    fn missing_file_gives_defaults() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(Settings::load(dir.path()), Settings::default());
    }

    #[test]
    fn indent_and_zoom() {
        let mut s = Settings::default();
        assert_eq!(s.indent(), "\t");
        s.insert_spaces = true;
        s.tab_width = 2;
        assert_eq!(s.indent(), "  ");
        s.zoom(100.0);
        assert_eq!(s.font_size, FONT_SIZE_MAX);
    }
}
