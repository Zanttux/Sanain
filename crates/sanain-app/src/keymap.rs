//! Keyboard shortcuts as data: presets plus per-command overrides.
//!
//! Chords are written like `Mod+Shift+K`, where `Mod` is Cmd on macOS and
//! Ctrl elsewhere. Key names follow the logical key (the character or a
//! named key such as `F3`, `ArrowUp`, `Tab`). Parsing never panics; invalid
//! entries are ignored.

use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mods {
    /// Cmd on macOS, Ctrl elsewhere.
    pub command: bool,
    /// The physical Ctrl key on macOS (same as `command` elsewhere).
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Chord {
    pub mods: Mods,
    /// Key name, e.g. `K`, `F3`, `ArrowUp`, `Tab`, `Plus`.
    pub key: String,
}

const NAMED: &[&str] = &[
    "ArrowUp",
    "ArrowDown",
    "ArrowLeft",
    "ArrowRight",
    "Tab",
    "Enter",
    "Escape",
    "Space",
    "Backspace",
    "Delete",
    "Home",
    "End",
    "PageUp",
    "PageDown",
    "Plus",
    "Minus",
    "Equals",
    "Comma",
    "Period",
    "Slash",
    "Semicolon",
    "OpenBracket",
    "CloseBracket",
    "F1",
    "F2",
    "F3",
    "F4",
    "F5",
    "F6",
    "F7",
    "F8",
    "F9",
    "F10",
    "F11",
    "F12",
];

fn valid_key(k: &str) -> bool {
    (k.len() == 1 && k.chars().all(|c| c.is_ascii_alphanumeric())) || NAMED.contains(&k)
}

impl Chord {
    pub fn parse(s: &str) -> Option<Self> {
        let mut mods = Mods::default();
        let mut key = None;
        for part in s.split('+').map(str::trim) {
            match part.to_ascii_lowercase().as_str() {
                "mod" | "cmdorctrl" => mods.command = true,
                "ctrl" | "control" => mods.ctrl = true,
                "alt" | "option" => mods.alt = true,
                "shift" => mods.shift = true,
                "" => return None,
                _ => {
                    if key.is_some() {
                        return None;
                    }
                    let k = if part.len() == 1 {
                        part.to_ascii_uppercase()
                    } else {
                        NAMED
                            .iter()
                            .find(|n| n.eq_ignore_ascii_case(part))?
                            .to_string()
                    };
                    key = Some(k);
                }
            }
        }
        let key = key.filter(|k| valid_key(k))?;
        Some(Chord { mods, key })
    }

    pub fn format(&self) -> String {
        let mut parts: Vec<&str> = Vec::new();
        if self.mods.command {
            parts.push("Mod");
        }
        if self.mods.ctrl {
            parts.push("Ctrl");
        }
        if self.mods.alt {
            parts.push("Alt");
        }
        if self.mods.shift {
            parts.push("Shift");
        }
        parts.push(&self.key);
        parts.join("+")
    }
}

/// Named sets of shortcuts. Commands not listed keep the default.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Preset {
    Default,
    NotepadPlusPlus,
}

impl Preset {
    pub fn id(self) -> &'static str {
        match self {
            Preset::Default => "default",
            Preset::NotepadPlusPlus => "notepad++",
        }
    }

    pub fn from_id(s: &str) -> Self {
        if s == "notepad++" {
            Preset::NotepadPlusPlus
        } else {
            Preset::Default
        }
    }

    /// (command id, chord) pairs that differ from the default set. Command
    /// ids are the menu item ids (e.g. `edit.duplicate-line`).
    pub fn overrides(self) -> &'static [(&'static str, &'static str)] {
        match self {
            Preset::Default => &[],
            Preset::NotepadPlusPlus => &[
                ("edit.duplicate-line", "Mod+D"),
                // Mod+D duplicates the line in Notepad++.
                ("edit.select-next-occurrence", "none"),
                ("edit.delete-line", "Mod+Shift+L"),
                ("edit.move-line-up", "Mod+Shift+ArrowUp"),
                ("edit.move-line-down", "Mod+Shift+ArrowDown"),
                ("edit.find-next", "F3"),
                ("edit.find-prev", "Shift+F3"),
                ("edit.replace", "Mod+H"),
                ("tools.upper-case", "Mod+Shift+U"),
                ("tools.lower-case", "Mod+U"),
                ("view.unfold-all", "Alt+Shift+0"),
                ("view.fold", "Mod+Alt+F"),
                ("view.unfold", "Mod+Alt+Shift+F"),
            ],
        }
    }
}

/// The user's keymap: a preset plus explicit per-command overrides. An
/// override of `None` removes the command's shortcut.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Keymap {
    pub preset: Preset,
    pub custom: BTreeMap<String, Option<Chord>>,
}

impl Default for Keymap {
    fn default() -> Self {
        Self {
            preset: Preset::Default,
            custom: BTreeMap::new(),
        }
    }
}

/// Resolution of a command's shortcut.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Binding {
    /// Use the built-in default.
    Default,
    /// Use this chord instead.
    Chord(Chord),
    /// No shortcut.
    None,
}

impl Keymap {
    pub fn binding(&self, command: &str) -> Binding {
        if let Some(c) = self.custom.get(command) {
            return c.clone().map_or(Binding::None, Binding::Chord);
        }
        match self
            .preset
            .overrides()
            .iter()
            .find(|(id, _)| *id == command)
        {
            // "none": the preset removes the default shortcut.
            Some((_, "none")) => Binding::None,
            Some((_, s)) => Chord::parse(s).map_or(Binding::Default, Binding::Chord),
            None => Binding::Default,
        }
    }

    /// Settings-file lines (`key.<command>=<chord>` or empty for none).
    pub fn encode(&self) -> String {
        let mut out = format!("keymap={}\n", self.preset.id());
        for (id, c) in &self.custom {
            out.push_str(&format!(
                "key.{id}={}\n",
                c.as_ref().map(Chord::format).unwrap_or_default()
            ));
        }
        out
    }

    /// Apply one settings line; returns whether it was a keymap line.
    pub fn read_line(&mut self, key: &str, value: &str) -> bool {
        if key == "keymap" {
            self.preset = Preset::from_id(value);
            return true;
        }
        let Some(id) = key.strip_prefix("key.") else {
            return false;
        };
        if id.is_empty() || id.len() > 64 {
            return true;
        }
        if value.is_empty() {
            self.custom.insert(id.to_owned(), None);
        } else if let Some(c) = Chord::parse(value) {
            self.custom.insert(id.to_owned(), Some(c));
        }
        true
    }

    /// Commands bound to `chord` (to warn about conflicts).
    pub fn conflicts<'a>(
        &self,
        chord: &Chord,
        commands: impl IntoIterator<Item = (&'a str, Option<Chord>)>,
    ) -> Vec<&'a str> {
        commands
            .into_iter()
            .filter(|(_, c)| c.as_ref() == Some(chord))
            .map(|(id, _)| id)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_and_format() {
        let c = Chord::parse("mod+shift+k").unwrap();
        assert_eq!(c.format(), "Mod+Shift+K");
        assert_eq!(Chord::parse("F3").unwrap().format(), "F3");
        assert_eq!(Chord::parse("Alt+arrowup").unwrap().key, "ArrowUp");
        for bad in ["", "Mod+", "Mod+K+J", "Shift", "Mod+Nope", "Mod++", "Ä"] {
            assert_eq!(Chord::parse(bad), None, "{bad}");
        }
    }

    #[test]
    fn preset_and_overrides() {
        let mut k = Keymap {
            preset: Preset::NotepadPlusPlus,
            ..Keymap::default()
        };
        assert_eq!(
            k.binding("edit.duplicate-line"),
            Binding::Chord(Chord::parse("Mod+D").unwrap())
        );
        assert_eq!(k.binding("file.save"), Binding::Default);
        k.read_line("key.file.save", "Mod+Alt+S");
        k.read_line("key.edit.duplicate-line", "");
        assert_eq!(
            k.binding("file.save"),
            Binding::Chord(Chord::parse("Mod+Alt+S").unwrap())
        );
        assert_eq!(k.binding("edit.duplicate-line"), Binding::None);
        let mut back = Keymap::default();
        for line in k.encode().lines() {
            let (a, b) = line.split_once('=').unwrap();
            assert!(back.read_line(a, b));
        }
        assert_eq!(back, k);
        assert!(!back.read_line("font_size", "12"));
    }

    #[test]
    fn every_preset_chord_parses() {
        for p in [Preset::Default, Preset::NotepadPlusPlus] {
            for (id, s) in p.overrides() {
                assert!(*s == "none" || Chord::parse(s).is_some(), "{id}: {s}");
            }
        }
    }
}
