//! User settings, stored as a small versioned text file.
//!
//! Format: first line `sanain-settings 1`, then `key=value` lines. Unknown
//! keys and invalid values are ignored (the default is kept), so files from
//! newer versions or hand edits never prevent startup.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const HEADER: &str = "sanain-settings 1";

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
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            font_size: FONT_SIZE_DEFAULT,
            tab_width: 4,
            insert_spaces: false,
            theme: ThemeChoice::System,
            restore_session: true,
            locale: None,
            keymap: crate::keymap::Keymap::default(),
            check_updates: false,
            last_update_check: 0,
            local_history: true,
        }
    }
}

impl Settings {
    /// Per-user configuration directory, e.g.
    /// `~/Library/Application Support/Sanain` on macOS.
    pub fn default_dir() -> Option<PathBuf> {
        let env = |k: &str| {
            std::env::var_os(k)
                .filter(|v| !v.is_empty())
                .map(PathBuf::from)
        };
        if let Some(d) = env("SANAIN_CONFIG_DIR") {
            return Some(d);
        }
        if cfg!(target_os = "macos") {
            env("HOME").map(|h| h.join("Library/Application Support/Sanain"))
        } else if cfg!(windows) {
            env("APPDATA").map(|d| d.join("Sanain"))
        } else {
            env("XDG_CONFIG_HOME")
                .or_else(|| env("HOME").map(|h| h.join(".config")))
                .map(|d| d.join("sanain"))
        }
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
            "check_updates={}\nlast_update_check={}\nlocal_history={}\n",
            self.check_updates, self.last_update_check, self.local_history
        ));
        out.push_str(&self.keymap.encode());
        out
    }

    /// Parse settings; returns defaults for anything missing or invalid.
    /// `None` only if the header is wrong (a different or unknown format).
    pub fn decode(text: &str) -> Option<Self> {
        let mut lines = text.lines();
        if lines.next()?.trim() != HEADER {
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
        sanain_io::write_atomic(&dir.join("settings"), self.encode().as_bytes(), None).map(|_| ())
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
    use super::*;

    #[test]
    fn round_trip() {
        let s = Settings {
            font_size: 16.5,
            tab_width: 2,
            insert_spaces: true,
            theme: ThemeChoice::Light,
            restore_session: false,
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
