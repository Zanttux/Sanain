//! `.editorconfig` support (https://editorconfig.org) for indentation:
//! `indent_style`, `indent_size` and `tab_width`. Other properties are
//! read but not applied (end of line, charset and whitespace trimming would
//! change file bytes; Text³ keeps files as they are).
//!
//! Files are searched from the document's folder upwards until one says
//! `root = true`; nearer files win, later sections win within a file.

use std::path::Path;

/// Indentation for a file as configured.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Indent {
    /// `Some(true)` for tabs, `Some(false)` for spaces.
    pub tabs: Option<bool>,
    /// Columns per indentation level.
    pub size: Option<usize>,
}

impl Indent {
    /// The text one indentation level inserts, falling back to the
    /// editor settings for anything not configured.
    pub fn text(self, fallback_tabs: bool, fallback_width: usize) -> String {
        if self.tabs.unwrap_or(fallback_tabs) {
            "\t".to_owned()
        } else {
            " ".repeat(self.size.unwrap_or(fallback_width).clamp(1, 16))
        }
    }

    pub fn is_empty(self) -> bool {
        self.tabs.is_none() && self.size.is_none()
    }
}

/// Match an EditorConfig section glob against a path relative to the
/// config file's folder (with `/` separators). Supports `*`, `**`, `?`,
/// `{a,b}` and `[abc]`/`[!abc]`. A glob without `/` matches the file name
/// in any folder.
pub fn glob_matches(glob: &str, rel_path: &str) -> bool {
    let glob = glob.trim();
    if glob.contains('/') {
        matches(glob.trim_start_matches('/').as_bytes(), rel_path.as_bytes())
    } else {
        let name = rel_path.rsplit('/').next().unwrap_or(rel_path);
        matches(glob.as_bytes(), name.as_bytes())
    }
}

fn matches(p: &[u8], s: &[u8]) -> bool {
    match p.first() {
        None => s.is_empty(),
        Some(b'*') if p.get(1) == Some(&b'*') => {
            let rest = &p[2..];
            (0..=s.len()).any(|i| matches(rest, &s[i..]))
        }
        Some(b'*') => {
            let rest = &p[1..];
            (0..=s.len())
                .take_while(|&i| i == 0 || s[i - 1] != b'/')
                .any(|i| matches(rest, &s[i..]))
        }
        Some(b'?') => !s.is_empty() && s[0] != b'/' && matches(&p[1..], &s[1..]),
        Some(b'{') => {
            let Some(close) = p.iter().position(|&c| c == b'}') else {
                return !s.is_empty() && s[0] == b'{' && matches(&p[1..], &s[1..]);
            };
            let rest = &p[close + 1..];
            p[1..close].split(|&c| c == b',').any(|alt| {
                let mut q = alt.to_vec();
                q.extend_from_slice(rest);
                matches(&q, s)
            })
        }
        Some(b'[') => {
            let Some(close) = p.iter().skip(1).position(|&c| c == b']').map(|i| i + 1) else {
                return !s.is_empty() && s[0] == b'[' && matches(&p[1..], &s[1..]);
            };
            let Some(&c) = s.first() else { return false };
            let (negate, set) = match p[1..close].strip_prefix(b"!") {
                Some(rest) => (true, rest),
                None => (false, &p[1..close]),
            };
            (set.contains(&c) != negate) && matches(&p[close + 1..], &s[1..])
        }
        Some(&c) => s.first() == Some(&c) && matches(&p[1..], &s[1..]),
    }
}

/// Apply one `.editorconfig` file's text to `indent` for `rel_path`.
/// Returns whether the file declares `root = true`.
pub fn apply(text: &str, rel_path: &str, indent: &mut Indent) -> bool {
    let mut root = false;
    let mut active = false;
    let mut found = Indent::default();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
            continue;
        }
        if let Some(sec) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            active = glob_matches(sec, rel_path);
            continue;
        }
        let Some((k, v)) = line.split_once('=') else {
            continue;
        };
        let (k, v) = (k.trim().to_ascii_lowercase(), v.trim().to_ascii_lowercase());
        if k == "root" && !active {
            root = v == "true";
            continue;
        }
        if !active {
            continue;
        }
        match k.as_str() {
            "indent_style" => {
                found.tabs = match v.as_str() {
                    "tab" => Some(true),
                    "space" => Some(false),
                    _ => found.tabs,
                }
            }
            "indent_size" if v != "tab" => found.size = v.parse().ok().or(found.size),
            "tab_width" if found.size.is_none() => found.size = v.parse().ok(),
            _ => {}
        }
    }
    // Nearer files (applied first) win over farther ones.
    indent.tabs = indent.tabs.or(found.tabs);
    indent.size = indent.size.or(found.size);
    root
}

/// Indentation configured for `path` by `.editorconfig` files in its
/// folder and above.
pub fn indent_for(path: &Path) -> Indent {
    let mut indent = Indent::default();
    let Some(mut dir) = path.parent() else {
        return indent;
    };
    loop {
        let cfg = dir.join(".editorconfig");
        if let Ok(text) = std::fs::read_to_string(&cfg)
            && let Ok(rel) = path.strip_prefix(dir)
        {
            let rel = rel.to_string_lossy().replace('\\', "/");
            if apply(&text, &rel, &mut indent) {
                break;
            }
        }
        match dir.parent() {
            Some(p) => dir = p,
            None => break,
        }
    }
    indent
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn globs() {
        assert!(glob_matches("*", "src/main.rs"));
        assert!(glob_matches("*.rs", "src/main.rs"));
        assert!(!glob_matches("*.rs", "src/main.py"));
        assert!(glob_matches("*.{js,ts}", "a/b.ts"));
        assert!(glob_matches("Makefile", "sub/Makefile"));
        assert!(glob_matches("src/*.rs", "src/main.rs"));
        assert!(!glob_matches("src/*.rs", "src/x/main.rs"));
        assert!(glob_matches("src/**.rs", "src/x/main.rs"));
        assert!(glob_matches("file?.txt", "file1.txt"));
        assert!(glob_matches("[Mm]akefile", "makefile"));
        assert!(!glob_matches("[!M]akefile", "Makefile"));
    }

    #[test]
    fn later_sections_and_nearer_files_win() {
        let cfg = "root = true\n\n[*]\nindent_style = space\nindent_size = 4\n\n[Makefile]\nindent_style = tab\n\n[*.{yml,yaml}]\nindent_size = 2\n";
        let mut i = Indent::default();
        assert!(apply(cfg, "Makefile", &mut i));
        assert_eq!(
            i,
            Indent {
                tabs: Some(true),
                size: Some(4)
            }
        );
        let mut i = Indent::default();
        apply(cfg, "ci/x.yml", &mut i);
        assert_eq!(i.text(true, 8), "  ");
        // A nearer file set the size already: a farther one cannot change it.
        let mut i = Indent {
            tabs: None,
            size: Some(3),
        };
        apply(cfg, "a.rs", &mut i);
        assert_eq!(
            i,
            Indent {
                tabs: Some(false),
                size: Some(3)
            }
        );
    }

    #[test]
    fn walks_up_until_root() {
        let d = tempfile::tempdir().unwrap();
        let sub = d.path().join("proj/src");
        std::fs::create_dir_all(&sub).unwrap();
        std::fs::write(d.path().join(".editorconfig"), "[*]\nindent_style = tab\n").unwrap();
        std::fs::write(
            d.path().join("proj/.editorconfig"),
            "root = true\n[*.rs]\nindent_size = 2\n",
        )
        .unwrap();
        let i = indent_for(&sub.join("main.rs"));
        assert_eq!(
            i,
            Indent {
                tabs: None,
                size: Some(2)
            },
            "root stops the walk"
        );
        assert_eq!(i.text(false, 4), "  ");
        assert!(indent_for(&d.path().join("x.txt")).tabs == Some(true));
    }
}
