//! Language detection and syntax highlighting with Tree-sitter.
//!
//! Highlighting is a pure function of the source text, safe to run on a
//! background thread and cancellable. Output spans are byte ranges with a
//! small, theme-independent [`Style`] vocabulary; the UI maps styles to
//! colours.

use std::collections::HashMap;
use std::ops::Range;
use std::path::Path;
use std::sync::atomic::AtomicUsize;

use tree_sitter_highlight::{HighlightConfiguration, HighlightEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Lang {
    Bash,
    Css,
    JavaScript,
    Json,
    Python,
    Rust,
    Toml,
    Yaml,
}

impl Lang {
    pub const ALL: [Lang; 8] = [
        Lang::Bash,
        Lang::Css,
        Lang::JavaScript,
        Lang::Json,
        Lang::Python,
        Lang::Rust,
        Lang::Toml,
        Lang::Yaml,
    ];

    /// Detect from the file name. Unknown files are plain text.
    pub fn from_path(path: &Path) -> Option<Self> {
        let name = path.file_name()?.to_str()?.to_ascii_lowercase();
        match name.as_str() {
            ".bashrc" | ".bash_profile" | ".zshrc" | ".profile" => return Some(Lang::Bash),
            "cargo.lock" => return Some(Lang::Toml),
            _ => {}
        }
        let ext = name.rsplit_once('.')?.1;
        Some(match ext {
            "sh" | "bash" | "zsh" => Lang::Bash,
            "css" => Lang::Css,
            "js" | "mjs" | "cjs" | "jsx" => Lang::JavaScript,
            "json" | "jsonc" | "geojson" => Lang::Json,
            "py" | "pyi" | "pyw" => Lang::Python,
            "rs" => Lang::Rust,
            "toml" => Lang::Toml,
            "yaml" | "yml" => Lang::Yaml,
            _ => return None,
        })
    }

    /// Display name. Language names are proper nouns and not translated.
    pub fn name(self) -> &'static str {
        match self {
            Lang::Bash => "Shell",
            Lang::Css => "CSS",
            Lang::JavaScript => "JavaScript",
            Lang::Json => "JSON",
            Lang::Python => "Python",
            Lang::Rust => "Rust",
            Lang::Toml => "TOML",
            Lang::Yaml => "YAML",
        }
    }

    fn grammar(self) -> (tree_sitter::Language, &'static str) {
        match self {
            Lang::Bash => (
                tree_sitter_bash::LANGUAGE.into(),
                tree_sitter_bash::HIGHLIGHT_QUERY,
            ),
            Lang::Css => (
                tree_sitter_css::LANGUAGE.into(),
                tree_sitter_css::HIGHLIGHTS_QUERY,
            ),
            Lang::JavaScript => (
                tree_sitter_javascript::LANGUAGE.into(),
                tree_sitter_javascript::HIGHLIGHT_QUERY,
            ),
            Lang::Json => (
                tree_sitter_json::LANGUAGE.into(),
                tree_sitter_json::HIGHLIGHTS_QUERY,
            ),
            Lang::Python => (
                tree_sitter_python::LANGUAGE.into(),
                tree_sitter_python::HIGHLIGHTS_QUERY,
            ),
            Lang::Rust => (
                tree_sitter_rust::LANGUAGE.into(),
                tree_sitter_rust::HIGHLIGHTS_QUERY,
            ),
            Lang::Toml => (
                tree_sitter_toml_ng::LANGUAGE.into(),
                tree_sitter_toml_ng::HIGHLIGHTS_QUERY,
            ),
            Lang::Yaml => (
                tree_sitter_yaml::LANGUAGE.into(),
                tree_sitter_yaml::HIGHLIGHTS_QUERY,
            ),
        }
    }
}

/// Highlight categories. Capture names from grammar queries map onto these
/// by prefix (e.g. `function.method` → [`Style::Function`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Style {
    Keyword,
    String,
    Escape,
    Comment,
    Number,
    Constant,
    Function,
    Type,
    Property,
    Variable,
    Operator,
    Punctuation,
    Attribute,
    Tag,
}

/// Capture names we recognise, in the order of [`STYLES`].
const NAMES: [&str; 21] = [
    "keyword",
    "string",
    "string.special.key",
    "string.special",
    "escape",
    "comment",
    "number",
    "boolean",
    "constant",
    "function",
    "constructor",
    "type",
    "property",
    "variable.builtin",
    "variable.parameter",
    "operator",
    "punctuation",
    "attribute",
    "tag",
    "label",
    "embedded",
];

const STYLES: [Style; 21] = [
    Style::Keyword,
    Style::String,
    Style::Property,
    Style::String,
    Style::Escape,
    Style::Comment,
    Style::Number,
    Style::Constant,
    Style::Constant,
    Style::Function,
    Style::Type,
    Style::Type,
    Style::Property,
    Style::Variable,
    Style::Variable,
    Style::Operator,
    Style::Punctuation,
    Style::Attribute,
    Style::Tag,
    Style::Property,
    Style::Punctuation,
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    /// Byte range into the highlighted source.
    pub range: Range<usize>,
    pub style: Style,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum HighlightError {
    #[error("highlighting was cancelled")]
    Cancelled,
    #[error("highlighting failed: {0}")]
    Failed(String),
}

/// Reusable highlighter. Not `Sync`; keep one per worker thread.
pub struct Highlighter {
    configs: HashMap<Lang, Option<HighlightConfiguration>>,
    inner: tree_sitter_highlight::Highlighter,
}

impl std::fmt::Debug for Highlighter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Highlighter").finish_non_exhaustive()
    }
}

impl Default for Highlighter {
    fn default() -> Self {
        Self::new()
    }
}

impl Highlighter {
    pub fn new() -> Self {
        Self {
            configs: HashMap::new(),
            inner: tree_sitter_highlight::Highlighter::new(),
        }
    }

    fn config(&mut self, lang: Lang) -> Option<&HighlightConfiguration> {
        self.configs
            .entry(lang)
            .or_insert_with(|| {
                let (language, query) = lang.grammar();
                let mut c =
                    HighlightConfiguration::new(language, lang.name(), query, "", "").ok()?;
                c.configure(&NAMES);
                Some(c)
            })
            .as_ref()
    }

    /// Highlight `source`. Setting `cancel` to a non-zero value from another
    /// thread stops the work early with [`HighlightError::Cancelled`].
    pub fn highlight(
        &mut self,
        lang: Lang,
        source: &[u8],
        cancel: Option<&AtomicUsize>,
    ) -> Result<Vec<Span>, HighlightError> {
        // Split borrows: the configuration lives in `configs`, the parser
        // state in `inner`.
        if self.config(lang).is_none() {
            return Err(HighlightError::Failed(format!(
                "{} query did not compile",
                lang.name()
            )));
        }
        let Self { configs, inner } = self;
        let Some(Some(config)) = configs.get(&lang) else {
            return Ok(Vec::new());
        };
        let events = inner
            .highlight(config, source, None, cancel, |_| None)
            .map_err(map_err)?;
        let mut spans: Vec<Span> = Vec::new();
        let mut stack: Vec<Style> = Vec::new();
        for event in events {
            match event.map_err(map_err)? {
                HighlightEvent::HighlightStart(h) => {
                    stack.push(STYLES.get(h.0).copied().unwrap_or(Style::Variable));
                }
                HighlightEvent::HighlightEnd => {
                    stack.pop();
                }
                HighlightEvent::Source { start, end } => {
                    let Some(&style) = stack.last() else { continue };
                    match spans.last_mut() {
                        Some(last) if last.style == style && last.range.end == start => {
                            last.range.end = end;
                        }
                        _ => spans.push(Span {
                            range: start..end,
                            style,
                        }),
                    }
                }
            }
        }
        Ok(spans)
    }

    /// Whether every bundled grammar's query compiles (used by tests and
    /// diagnostics).
    pub fn check_all(&mut self) -> Vec<Lang> {
        Lang::ALL
            .into_iter()
            .filter(|&l| self.config(l).is_none())
            .collect()
    }
}

fn map_err(e: tree_sitter_highlight::Error) -> HighlightError {
    match e {
        tree_sitter_highlight::Error::Cancelled => HighlightError::Cancelled,
        other => HighlightError::Failed(other.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::Ordering;

    fn styles_of(lang: Lang, src: &str) -> Vec<(String, Style)> {
        Highlighter::new()
            .highlight(lang, src.as_bytes(), None)
            .unwrap()
            .into_iter()
            .map(|s| (src[s.range].to_owned(), s.style))
            .collect()
    }

    #[test]
    fn every_bundled_query_compiles() {
        assert_eq!(Highlighter::new().check_all(), Vec::<Lang>::new());
    }

    #[test]
    fn detects_by_extension_and_name() {
        assert_eq!(Lang::from_path(Path::new("/x/main.RS")), Some(Lang::Rust));
        assert_eq!(Lang::from_path(Path::new("a.yml")), Some(Lang::Yaml));
        assert_eq!(Lang::from_path(Path::new(".bashrc")), Some(Lang::Bash));
        assert_eq!(Lang::from_path(Path::new("Cargo.lock")), Some(Lang::Toml));
        assert_eq!(Lang::from_path(Path::new("notes.txt")), None);
        assert_eq!(Lang::from_path(Path::new("Makefile")), None);
    }

    #[test]
    fn rust_keywords_strings_comments() {
        let s = styles_of(Lang::Rust, "fn main() { let s = \"hi\"; // note\n}");
        assert!(s.contains(&("fn".into(), Style::Keyword)), "{s:?}");
        assert!(s.contains(&("\"hi\"".into(), Style::String)), "{s:?}");
        assert!(s.contains(&("// note".into(), Style::Comment)), "{s:?}");
    }

    #[test]
    fn every_language_highlights_something() {
        let samples = [
            (Lang::Bash, "echo \"hi\" # c"),
            (Lang::Css, "a { color: red; }"),
            (Lang::JavaScript, "const x = 'y';"),
            (Lang::Json, "{\"a\": 1}"),
            (Lang::Python, "def f():\n    return 'x'"),
            (Lang::Rust, "fn f() {}"),
            (Lang::Toml, "key = \"v\""),
            (Lang::Yaml, "key: value # c"),
        ];
        for (lang, src) in samples {
            assert!(!styles_of(lang, src).is_empty(), "{lang:?}");
        }
    }

    #[test]
    fn malformed_and_unicode_input_does_not_panic() {
        let mut h = Highlighter::new();
        for lang in Lang::ALL {
            for src in ["", "{{{{ \"unterminated", "ä🙂\u{0}\u{FEFF}", "\r\n\r\n"] {
                let spans = h.highlight(lang, src.as_bytes(), None).unwrap();
                for s in spans {
                    assert!(
                        src.is_char_boundary(s.range.start) && src.is_char_boundary(s.range.end)
                    );
                }
            }
        }
    }

    #[test]
    fn cancellation_stops_work() {
        let flag = AtomicUsize::new(0);
        flag.store(1, Ordering::Relaxed);
        let src = "fn f() {}\n".repeat(10_000);
        let r = Highlighter::new().highlight(Lang::Rust, src.as_bytes(), Some(&flag));
        assert_eq!(r, Err(HighlightError::Cancelled));
    }
}
