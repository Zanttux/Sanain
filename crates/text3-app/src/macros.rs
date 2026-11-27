//! Saved, named keyboard macros, kept in the config folder (`macros`).
//!
//! Format (version 1), one entry per line:
//!
//! ```text
//! sanain-macros 1
//! macro Wrap in quotes
//! insert "
//! line-end
//! insert "
//! end
//! ```
//!
//! A macro is `macro <name>`, its steps, then `end`. Each step is one
//! recordable [`Command`]: a keyword, optionally followed by a space and an
//! argument (`left extend`, `outdent 4`, `insert <text>`). Text arguments
//! escape `\` as `\\`, line breaks and tabs as `\n`, `\r`, `\t`, other
//! control characters as `\u{hex}` and a leading or trailing space as `\s`,
//! so every step stays on one line and survives editors that trim lines.
//!
//! Reading is defensive: a file with another header is ignored; a macro
//! with an unknown or malformed step, an invalid or duplicate name, too
//! many steps, or no `end` is skipped; other lines outside a macro are
//! ignored. Files over [`MAX_BYTES`] are not read at all.

use std::fs;
use std::io::{self, Read};
use std::path::Path;

use crate::editor::Command;

const HEADER: &str = "sanain-macros 1";
const FILE: &str = "macros";

/// Most macros kept.
pub const MAX_MACROS: usize = 100;
/// Most steps in one macro.
pub const MAX_STEPS: usize = 10_000;
/// Largest `macros` file read or written.
pub const MAX_BYTES: usize = 1024 * 1024;
/// Longest macro name, in characters.
pub const NAME_MAX: usize = 80;
/// Largest outdent width accepted from the file.
const OUTDENT_MAX: usize = 64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SavedMacro {
    pub name: String,
    pub steps: Vec<Command>,
}

/// Why a macro could not be added to the store.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MacroError {
    /// Nothing recorded.
    Empty,
    /// Empty, too long, or contains control characters.
    InvalidName,
    /// Too many steps, or the file would exceed [`MAX_BYTES`].
    TooLarge,
    /// [`MAX_MACROS`] are saved already.
    Full,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MacroStore {
    macros: Vec<SavedMacro>,
}

/// A valid macro name: trimmed, 1–[`NAME_MAX`] characters, no control
/// characters.
pub fn clean_name(name: &str) -> Option<String> {
    let name = name.trim();
    let n = name.chars().count();
    ((1..=NAME_MAX).contains(&n) && !name.chars().any(char::is_control)).then(|| name.to_owned())
}

impl MacroStore {
    /// Load from `dir/macros`; empty if missing, unreadable or too large.
    pub fn load(dir: &Path) -> Self {
        let Ok(file) = fs::File::open(dir.join(FILE)) else {
            return Self::default();
        };
        let mut bytes = Vec::new();
        if file
            .take(MAX_BYTES as u64 + 1)
            .read_to_end(&mut bytes)
            .is_err()
            || bytes.len() > MAX_BYTES
        {
            return Self::default();
        }
        String::from_utf8(bytes)
            .map(|t| Self::decode(&t))
            .unwrap_or_default()
    }

    /// Write `dir/macros`. If the file there holds anything this version
    /// could not read (a newer format, a damaged or hand-edited macro, bad
    /// bytes), it is first copied to `macros.N.bak` so saving never loses
    /// macros silently.
    pub fn save(&self, dir: &Path) -> io::Result<()> {
        fs::create_dir_all(dir)?;
        let path = dir.join(FILE);
        if path.exists() && !Self::fully_readable(&path) {
            let backup = (1..=1000)
                .map(|n| dir.join(format!("{FILE}.{n}.bak")))
                .find(|p| !p.exists())
                .ok_or_else(|| io::Error::other("too many macro backups"))?;
            // Via a temporary file, so a copy cut short (e.g. by a full
            // disk) never leaves a half-written backup behind.
            text3_io::write_atomic(&backup, &fs::read(&path)?, None)?;
        }
        text3_io::write_atomic(&dir.join(FILE), self.encode().as_bytes(), None).map(|_| ())
    }

    /// Whether every macro in the file at `path` is understood, so
    /// rewriting it from a loaded store loses nothing.
    fn fully_readable(path: &Path) -> bool {
        let Ok(bytes) = fs::read(path) else {
            return false;
        };
        let Ok(text) = String::from_utf8(bytes) else {
            return false;
        };
        if text.len() > MAX_BYTES {
            return false;
        }
        let mut lines = text.lines();
        if lines.next().map(str::trim) != Some(HEADER) {
            return false;
        }
        let blocks = lines.filter(|l| l.starts_with("macro ")).count();
        Self::decode(&text).macros.len() == blocks
    }

    pub fn decode(text: &str) -> Self {
        let mut store = Self::default();
        if text.len() > MAX_BYTES {
            return store;
        }
        let mut lines = text.lines();
        if lines.next().map(str::trim) != Some(HEADER) {
            return store;
        }
        // The macro being read: its name (None if invalid) and steps (None
        // once a step was rejected).
        let mut current: Option<(Option<String>, Option<Vec<Command>>)> = None;
        for line in lines {
            match &mut current {
                None => {
                    if let Some(name) = line.strip_prefix("macro ") {
                        current = Some((clean_name(name), Some(Vec::new())));
                    }
                }
                Some((name, steps)) => {
                    if line == "end" {
                        if let (Some(name), Some(steps)) = (name.take(), steps.take())
                            && !steps.is_empty()
                            && store.macros.len() < MAX_MACROS
                            && store.get(&name).is_none()
                        {
                            store.macros.push(SavedMacro { name, steps });
                        }
                        current = None;
                    } else if let Some(list) = steps {
                        match decode_step(line) {
                            Some(c) if list.len() < MAX_STEPS => list.push(c),
                            _ => *steps = None,
                        }
                    }
                }
            }
        }
        store
    }

    pub fn encode(&self) -> String {
        let mut out = format!("{HEADER}\n");
        for m in &self.macros {
            out.push_str("macro ");
            out.push_str(&m.name);
            out.push('\n');
            for c in &m.steps {
                if let Some(s) = encode_step(c) {
                    out.push_str(&s);
                    out.push('\n');
                }
            }
            out.push_str("end\n");
        }
        out
    }

    /// Add a macro; one with the same name is replaced.
    pub fn insert(&mut self, name: &str, steps: &[Command]) -> Result<String, MacroError> {
        let name = clean_name(name).ok_or(MacroError::InvalidName)?;
        let steps: Vec<Command> = steps
            .iter()
            .filter(|c| c.is_recordable())
            .cloned()
            .collect();
        if steps.is_empty() {
            return Err(MacroError::Empty);
        }
        if steps.len() > MAX_STEPS {
            return Err(MacroError::TooLarge);
        }
        let mut next = self.clone();
        let m = SavedMacro {
            name: name.clone(),
            steps,
        };
        let full = next.macros.len() >= MAX_MACROS;
        match next.macros.iter_mut().find(|x| x.name == name) {
            Some(x) => *x = m,
            None if full => return Err(MacroError::Full),
            None => next.macros.push(m),
        }
        if next.encode().len() > MAX_BYTES {
            return Err(MacroError::TooLarge);
        }
        *self = next;
        Ok(name)
    }

    /// Remove the macro called `name`; whether one was removed.
    pub fn remove(&mut self, name: &str) -> bool {
        let before = self.macros.len();
        self.macros.retain(|m| m.name != name);
        self.macros.len() != before
    }

    pub fn get(&self, name: &str) -> Option<&SavedMacro> {
        self.macros.iter().find(|m| m.name == name)
    }

    /// Saved macros sorted by name (case-insensitively).
    pub fn sorted(&self) -> Vec<&SavedMacro> {
        let mut v: Vec<&SavedMacro> = self.macros.iter().collect();
        v.sort_by_cached_key(|m| (m.name.to_lowercase(), m.name.clone()));
        v
    }

    pub fn is_empty(&self) -> bool {
        self.macros.is_empty()
    }

    pub fn len(&self) -> usize {
        self.macros.len()
    }
}

fn flag(extend: bool) -> &'static str {
    if extend { " extend" } else { "" }
}

/// One step as a line; `None` for commands macros never record.
fn encode_step(c: &Command) -> Option<String> {
    let s = match c {
        Command::Insert(t) if t.is_empty() => "insert".into(),
        Command::Insert(t) => format!("insert {}", escape(t)),
        Command::NewLine => "newline".into(),
        Command::Backspace => "backspace".into(),
        Command::Delete => "delete".into(),
        Command::Left { extend } => format!("left{}", flag(*extend)),
        Command::Right { extend } => format!("right{}", flag(*extend)),
        Command::Up { extend } => format!("up{}", flag(*extend)),
        Command::Down { extend } => format!("down{}", flag(*extend)),
        Command::WordLeft { extend } => format!("word-left{}", flag(*extend)),
        Command::WordRight { extend } => format!("word-right{}", flag(*extend)),
        Command::LineStart { extend } => format!("line-start{}", flag(*extend)),
        Command::LineEnd { extend } => format!("line-end{}", flag(*extend)),
        Command::DeleteWordBackward => "delete-word-backward".into(),
        Command::DeleteWordForward => "delete-word-forward".into(),
        Command::DeleteLine => "delete-line".into(),
        Command::DuplicateLine => "duplicate-line".into(),
        Command::MoveLineUp => "move-line-up".into(),
        Command::MoveLineDown => "move-line-down".into(),
        Command::SelectAll => "select-all".into(),
        Command::AddCaretVertical { down: false } => "caret-above".into(),
        Command::AddCaretVertical { down: true } => "caret-below".into(),
        Command::SingleSelection => "single-selection".into(),
        Command::IndentLines(t) => format!("indent {}", escape(t)),
        Command::OutdentLines(n) => format!("outdent {n}"),
        Command::ToggleComment(t) => format!("toggle-comment {}", escape(t)),
        Command::SelectNextOccurrence => "select-next-occurrence".into(),
        Command::SkipOccurrence => "skip-occurrence".into(),
        Command::Cut => "cut".into(),
        Command::PlaceCaret { .. }
        | Command::AddCaret { .. }
        | Command::SetSelections(_)
        | Command::Undo
        | Command::Redo => return None,
    };
    Some(s)
}

fn decode_step(line: &str) -> Option<Command> {
    let (word, arg) = match line.split_once(' ') {
        Some((w, a)) => (w, Some(a)),
        None => (line, None),
    };
    let extend = match arg {
        None => Some(false),
        Some("extend") => Some(true),
        Some(_) => None,
    };
    let plain = |c: Command| arg.is_none().then_some(c);
    let text = || unescape(arg.unwrap_or(""));
    match word {
        "insert" => text().map(Command::Insert),
        "indent" => text().filter(|t| !t.is_empty()).map(Command::IndentLines),
        "toggle-comment" => text().filter(|t| !t.is_empty()).map(Command::ToggleComment),
        "outdent" => arg?
            .parse::<usize>()
            .ok()
            .filter(|n| (1..=OUTDENT_MAX).contains(n))
            .map(Command::OutdentLines),
        "left" => extend.map(|extend| Command::Left { extend }),
        "right" => extend.map(|extend| Command::Right { extend }),
        "up" => extend.map(|extend| Command::Up { extend }),
        "down" => extend.map(|extend| Command::Down { extend }),
        "word-left" => extend.map(|extend| Command::WordLeft { extend }),
        "word-right" => extend.map(|extend| Command::WordRight { extend }),
        "line-start" => extend.map(|extend| Command::LineStart { extend }),
        "line-end" => extend.map(|extend| Command::LineEnd { extend }),
        "newline" => plain(Command::NewLine),
        "backspace" => plain(Command::Backspace),
        "delete" => plain(Command::Delete),
        "delete-word-backward" => plain(Command::DeleteWordBackward),
        "delete-word-forward" => plain(Command::DeleteWordForward),
        "delete-line" => plain(Command::DeleteLine),
        "duplicate-line" => plain(Command::DuplicateLine),
        "move-line-up" => plain(Command::MoveLineUp),
        "move-line-down" => plain(Command::MoveLineDown),
        "select-all" => plain(Command::SelectAll),
        "caret-above" => plain(Command::AddCaretVertical { down: false }),
        "caret-below" => plain(Command::AddCaretVertical { down: true }),
        "single-selection" => plain(Command::SingleSelection),
        "select-next-occurrence" => plain(Command::SelectNextOccurrence),
        "skip-occurrence" => plain(Command::SkipOccurrence),
        "cut" => plain(Command::Cut),
        _ => None,
    }
}

fn escape(text: &str) -> String {
    let n = text.chars().count();
    let mut out = String::with_capacity(text.len());
    for (i, c) in text.chars().enumerate() {
        match c {
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            ' ' if i == 0 || i + 1 == n => out.push_str("\\s"),
            c if c.is_control() || c == '\u{2028}' || c == '\u{2029}' => {
                out.push_str(&format!("\\u{{{:x}}}", u32::from(c)))
            }
            c => out.push(c),
        }
    }
    out
}

fn unescape(text: &str) -> Option<String> {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next()? {
            '\\' => out.push('\\'),
            'n' => out.push('\n'),
            'r' => out.push('\r'),
            't' => out.push('\t'),
            's' => out.push(' '),
            'u' => {
                if chars.next()? != '{' {
                    return None;
                }
                let mut hex = String::new();
                loop {
                    match chars.next()? {
                        '}' => break,
                        c if c.is_ascii_hexdigit() && hex.len() < 6 => hex.push(c),
                        _ => return None,
                    }
                }
                out.push(char::from_u32(u32::from_str_radix(&hex, 16).ok()?)?);
            }
            _ => return None,
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Editor;

    fn every_step() -> Vec<Command> {
        let mut v = vec![
            Command::Insert(String::new()),
            Command::Insert(" lead, trail ".into()),
            Command::Insert("a\\b\nc\r\nd\te\u{1}f\u{2028}ä😀 \\s".into()),
            Command::NewLine,
            Command::Backspace,
            Command::Delete,
            Command::DeleteWordBackward,
            Command::DeleteWordForward,
            Command::DeleteLine,
            Command::DuplicateLine,
            Command::MoveLineUp,
            Command::MoveLineDown,
            Command::SelectAll,
            Command::AddCaretVertical { down: false },
            Command::AddCaretVertical { down: true },
            Command::SingleSelection,
            Command::IndentLines("    ".into()),
            Command::IndentLines("\t".into()),
            Command::OutdentLines(4),
            Command::ToggleComment("// ".into()),
            Command::SelectNextOccurrence,
            Command::SkipOccurrence,
            Command::Cut,
        ];
        for extend in [false, true] {
            v.extend([
                Command::Left { extend },
                Command::Right { extend },
                Command::Up { extend },
                Command::Down { extend },
                Command::WordLeft { extend },
                Command::WordRight { extend },
                Command::LineStart { extend },
                Command::LineEnd { extend },
            ]);
        }
        v
    }

    #[test]
    fn format_round_trips_every_recordable_step() {
        let mut s = MacroStore::default();
        s.insert("  All steps ", &every_step()).unwrap();
        s.insert("Ääkköset 😀", &[Command::NewLine]).unwrap();
        let text = s.encode();
        assert!(text.starts_with("sanain-macros 1\nmacro All steps\n"));
        assert!(text.contains("insert \\slead, trail\\s\n"));
        assert!(
            text.lines().all(|l| l == l.trim()),
            "no line has leading or trailing spaces"
        );
        let back = MacroStore::decode(&text);
        assert_eq!(back, s);
        assert_eq!(back.get("All steps").unwrap().steps, every_step());
        // CRLF line breaks from a hand edit read the same.
        assert_eq!(MacroStore::decode(&text.replace('\n', "\r\n")), s);
    }

    #[test]
    fn same_name_replaces_and_names_are_checked() {
        let mut s = MacroStore::default();
        s.insert("a", &[Command::NewLine]).unwrap();
        s.insert("a ", &[Command::Delete]).unwrap();
        assert_eq!(s.len(), 1);
        assert_eq!(s.get("a").unwrap().steps, vec![Command::Delete]);
        assert_eq!(
            s.insert("  ", &[Command::NewLine]),
            Err(MacroError::InvalidName)
        );
        assert_eq!(
            s.insert("a\nb", &[Command::NewLine]),
            Err(MacroError::InvalidName)
        );
        let long = "x".repeat(NAME_MAX + 1);
        assert_eq!(
            s.insert(&long, &[Command::NewLine]),
            Err(MacroError::InvalidName)
        );
        assert_eq!(s.insert("b", &[]), Err(MacroError::Empty));
        assert_eq!(
            s.insert(
                "b",
                &[
                    Command::Undo,
                    Command::PlaceCaret {
                        at: 0,
                        extend: false
                    }
                ]
            ),
            Err(MacroError::Empty),
            "unrecordable commands are dropped"
        );
        s.insert("B", &[Command::NewLine]).unwrap();
        s.insert("c", &[Command::NewLine]).unwrap();
        let names: Vec<&str> = s.sorted().iter().map(|m| m.name.as_str()).collect();
        assert_eq!(names, vec!["a", "B", "c"]);
        assert!(s.remove("B"));
        assert!(!s.remove("B"));
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn caps_on_macros_steps_and_size() {
        let mut s = MacroStore::default();
        for i in 0..MAX_MACROS {
            s.insert(&format!("m{i}"), &[Command::NewLine]).unwrap();
        }
        assert_eq!(
            s.insert("one more", &[Command::NewLine]),
            Err(MacroError::Full)
        );
        s.insert("m0", &[Command::Delete]).unwrap();
        let mut s = MacroStore::default();
        let many = vec![Command::NewLine; MAX_STEPS + 1];
        assert_eq!(s.insert("big", &many), Err(MacroError::TooLarge));
        let huge = vec![Command::Insert("x".repeat(MAX_BYTES))];
        assert_eq!(s.insert("huge", &huge), Err(MacroError::TooLarge));
        assert!(s.is_empty());
    }

    #[test]
    fn malformed_input_skips_only_the_bad_macro() {
        let text = "sanain-macros 1\n\
            # a comment outside a macro is ignored\n\
            macro good\n\
            insert hi\n\
            end\n\
            macro unknown step\n\
            insert x\n\
            fly-to-moon\n\
            end\n\
            macro bad escape\n\
            insert \\q\n\
            end\n\
            macro bad arg\n\
            left sideways\n\
            end\n\
            macro bad unicode\n\
            insert \\u{110000}\n\
            end\n\
            macro bad outdent\n\
            outdent 0\n\
            end\n\
            macro empty\n\
            end\n\
            macro good\n\
            delete\n\
            end\n\
            macro \n\
            newline\n\
            end\n\
            macro second\n\
            line-start extend\n\
            backspace\n\
            end\n\
            macro truncated\n\
            newline\n";
        let s = MacroStore::decode(text);
        let names: Vec<&str> = s.sorted().iter().map(|m| m.name.as_str()).collect();
        assert_eq!(names, vec!["good", "second"]);
        assert_eq!(
            s.get("good").unwrap().steps,
            vec![Command::Insert("hi".into())],
            "the first of two macros with one name wins"
        );
        assert_eq!(
            s.get("second").unwrap().steps,
            vec![Command::LineStart { extend: true }, Command::Backspace]
        );
        let s = MacroStore::decode("sanain-macros 1\nmacro a\nnewline extend\nend\n");
        assert!(s.is_empty(), "argument on a plain step");
        assert!(MacroStore::decode("sanain-macros 2\nmacro a\nnewline\nend\n").is_empty());
        assert!(MacroStore::decode("").is_empty());
        for junk in [
            "\\",
            "\\u",
            "\\u{",
            "\\u{}",
            "\\u{1234567}",
            "\\u{zz}",
            "\\u{d800}",
            "\\u{12",
        ] {
            assert_eq!(unescape(junk), None, "{junk}");
        }
    }

    #[test]
    fn oversized_input_is_rejected() {
        let mut text = String::from("sanain-macros 1\nmacro a\n");
        text.push_str(&"newline\n".repeat(MAX_STEPS + 1));
        text.push_str("end\nmacro b\nnewline\nend\n");
        let s = MacroStore::decode(&text);
        assert!(s.get("a").is_none(), "too many steps");
        assert!(s.get("b").is_some());

        let mut text = String::from("sanain-macros 1\n");
        for i in 0..MAX_MACROS + 5 {
            text.push_str(&format!("macro m{i}\nnewline\nend\n"));
        }
        assert_eq!(MacroStore::decode(&text).len(), MAX_MACROS);

        let mut big = String::from("sanain-macros 1\nmacro a\ninsert ");
        big.push_str(&"x".repeat(MAX_BYTES));
        big.push_str("\nend\n");
        assert!(MacroStore::decode(&big).is_empty());
        let d = tempfile::tempdir().unwrap();
        fs::write(d.path().join(FILE), &big).unwrap();
        assert!(MacroStore::load(d.path()).is_empty());
        fs::write(
            d.path().join(FILE),
            b"sanain-macros 1\nmacro \xff\nnewline\nend\n",
        )
        .unwrap();
        assert!(MacroStore::load(d.path()).is_empty(), "not UTF-8");
    }

    #[test]
    fn store_saves_and_loads_in_the_config_dir() {
        let d = tempfile::tempdir().unwrap();
        let dir = d.path().join("config");
        assert!(MacroStore::load(&dir).is_empty(), "missing file");
        let mut s = MacroStore::default();
        s.insert("x", &every_step()).unwrap();
        s.save(&dir).unwrap();
        assert!(dir.join("macros").is_file());
        assert_eq!(MacroStore::load(&dir), s);
        s.remove("x");
        s.save(&dir).unwrap();
        assert!(MacroStore::load(&dir).is_empty());
    }

    #[test]
    fn saving_over_an_unreadable_file_keeps_a_backup() {
        let d = tempfile::tempdir().unwrap();
        let dir = d.path();
        let mut s = MacroStore::default();
        s.insert("x", &every_step()).unwrap();
        // Readable file: rewritten without a backup.
        s.save(dir).unwrap();
        s.save(dir).unwrap();
        assert!(!dir.join("macros.1.bak").exists());
        // A damaged macro, then a newer format: each is kept in a backup.
        for (n, text) in [
            (
                1,
                "sanain-macros 1\nmacro good\ncut\nend\nmacro bad\nfly\nend\n",
            ),
            (2, "sanain-macros 2\nmacro future\nwarp\nend\n"),
        ] {
            std::fs::write(dir.join("macros"), text).unwrap();
            s.save(dir).unwrap();
            let bak = dir.join(format!("macros.{n}.bak"));
            assert_eq!(std::fs::read_to_string(&bak).unwrap(), text);
        }
        assert_eq!(MacroStore::load(dir), s);
    }

    #[test]
    fn a_loaded_macro_plays_like_the_recorded_one() {
        let start = |ed: &mut Editor| {
            ed.execute(Command::Insert("alpha\n  beta\ngamma\n".into()));
            ed.execute(Command::PlaceCaret {
                at: 0,
                extend: false,
            });
        };
        let mut rec = Editor::new();
        start(&mut rec);
        rec.start_recording();
        rec.execute(Command::Insert("- ".into()));
        rec.execute(Command::LineEnd { extend: false });
        rec.execute(Command::Insert(" \\ ;\t".into()));
        rec.execute(Command::WordLeft { extend: true });
        rec.execute(Command::Down { extend: false });
        rec.execute(Command::LineStart { extend: false });
        let recorded = rec.stop_recording();

        let d = tempfile::tempdir().unwrap();
        let mut s = MacroStore::default();
        s.insert("list item", &recorded).unwrap();
        s.save(d.path()).unwrap();
        let loaded = MacroStore::load(d.path())
            .get("list item")
            .unwrap()
            .steps
            .clone();
        assert_eq!(loaded, recorded);

        let (mut a, mut b) = (Editor::new(), Editor::new());
        start(&mut a);
        start(&mut b);
        a.play_times(&recorded, 3);
        b.play_times(&loaded, 3);
        assert_eq!(a.document().text(), b.document().text());
        assert_ne!(a.document().text(), "alpha\n  beta\ngamma\n");
        b.execute(Command::Undo);
        assert_eq!(
            b.document().text(),
            "alpha\n  beta\ngamma\n",
            "one undo step"
        );
    }
}
