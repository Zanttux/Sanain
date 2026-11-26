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
    C,
    CSharp,
    Cpp,
    Css,
    Go,
    Html,
    Java,
    JavaScript,
    Json,
    Python,
    Ruby,
    Rust,
    Toml,
    Tsx,
    TypeScript,
    Xml,
    Yaml,
}

impl Lang {
    pub const ALL: [Lang; 18] = [
        Lang::Bash,
        Lang::C,
        Lang::CSharp,
        Lang::Cpp,
        Lang::Css,
        Lang::Go,
        Lang::Html,
        Lang::Java,
        Lang::JavaScript,
        Lang::Json,
        Lang::Python,
        Lang::Ruby,
        Lang::Rust,
        Lang::Toml,
        Lang::Tsx,
        Lang::TypeScript,
        Lang::Xml,
        Lang::Yaml,
    ];

    /// Detect from the file name. Unknown files are plain text.
    pub fn from_path(path: &Path) -> Option<Self> {
        let name = path.file_name()?.to_str()?.to_ascii_lowercase();
        match name.as_str() {
            ".bashrc" | ".bash_profile" | ".zshrc" | ".profile" => return Some(Lang::Bash),
            "cargo.lock" => return Some(Lang::Toml),
            "gemfile" | "rakefile" | "podfile" | "vagrantfile" => return Some(Lang::Ruby),
            _ => {}
        }
        let ext = name.rsplit_once('.')?.1;
        Some(match ext {
            "sh" | "bash" | "zsh" => Lang::Bash,
            "c" | "h" => Lang::C,
            "cc" | "cpp" | "cxx" | "c++" | "hh" | "hpp" | "hxx" | "h++" | "ino" => Lang::Cpp,
            "cs" | "csx" => Lang::CSharp,
            "go" => Lang::Go,
            "html" | "htm" | "xhtml" => Lang::Html,
            "java" => Lang::Java,
            "rb" | "rake" | "gemspec" => Lang::Ruby,
            "ts" | "mts" | "cts" => Lang::TypeScript,
            "tsx" => Lang::Tsx,
            "xml" | "xsd" | "xsl" | "xslt" | "svg" | "plist" | "xaml" | "csproj" | "fsproj"
            | "vbproj" | "props" | "targets" | "resx" | "rss" | "atom" | "gpx" | "kml" => Lang::Xml,
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

    /// Detect from a Markdown code fence info string (` ```rust `). Only the
    /// first word counts; names and common aliases are accepted.
    pub fn from_fence(info: &str) -> Option<Self> {
        let tag = info
            .split(|c: char| c.is_whitespace() || c == ',' || c == '{')
            .next()?
            .trim_start_matches('.')
            .to_ascii_lowercase();
        Some(match tag.as_str() {
            "sh" | "bash" | "shell" | "zsh" | "console" | "shellsession" => Lang::Bash,
            "c" | "h" => Lang::C,
            "cpp" | "c++" | "cxx" | "cc" | "hpp" => Lang::Cpp,
            "cs" | "csharp" | "c#" => Lang::CSharp,
            "css" => Lang::Css,
            "go" | "golang" => Lang::Go,
            "html" | "htm" | "xhtml" => Lang::Html,
            "java" => Lang::Java,
            "js" | "javascript" | "jsx" | "mjs" | "cjs" | "node" => Lang::JavaScript,
            "json" | "jsonc" | "json5" | "geojson" => Lang::Json,
            "py" | "python" | "python3" | "py3" => Lang::Python,
            "rb" | "ruby" => Lang::Ruby,
            "rs" | "rust" => Lang::Rust,
            "toml" => Lang::Toml,
            "tsx" => Lang::Tsx,
            "ts" | "typescript" => Lang::TypeScript,
            "xml" | "svg" | "plist" | "xsl" | "xslt" => Lang::Xml,
            "yaml" | "yml" => Lang::Yaml,
            _ => return None,
        })
    }

    /// The line comment prefix, if the language has one.
    pub fn line_comment(self) -> Option<&'static str> {
        match self {
            Lang::C
            | Lang::CSharp
            | Lang::Cpp
            | Lang::Go
            | Lang::Java
            | Lang::JavaScript
            | Lang::Rust
            | Lang::Tsx
            | Lang::TypeScript => Some("//"),
            Lang::Bash | Lang::Python | Lang::Ruby | Lang::Toml | Lang::Yaml => Some("#"),
            Lang::Css | Lang::Html | Lang::Json | Lang::Xml => None,
        }
    }

    /// Display name. Language names are proper nouns and not translated.
    pub fn name(self) -> &'static str {
        match self {
            Lang::Bash => "Shell",
            Lang::C => "C",
            Lang::CSharp => "C#",
            Lang::Cpp => "C++",
            Lang::Go => "Go",
            Lang::Html => "HTML",
            Lang::Java => "Java",
            Lang::Ruby => "Ruby",
            Lang::Tsx => "TSX",
            Lang::TypeScript => "TypeScript",
            Lang::Xml => "XML",
            Lang::Css => "CSS",
            Lang::JavaScript => "JavaScript",
            Lang::Json => "JSON",
            Lang::Python => "Python",
            Lang::Rust => "Rust",
            Lang::Toml => "TOML",
            Lang::Yaml => "YAML",
        }
    }

    /// The grammar and its highlight query. Some queries build on another
    /// language's (C++ on C, TypeScript on JavaScript).
    fn grammar(self) -> (tree_sitter::Language, std::borrow::Cow<'static, str>) {
        let (l, q): (tree_sitter::Language, &'static str) = match self {
            Lang::C => (
                tree_sitter_c::LANGUAGE.into(),
                tree_sitter_c::HIGHLIGHT_QUERY,
            ),
            Lang::Cpp => {
                let q = format!(
                    "{}\n{}",
                    tree_sitter_cpp::HIGHLIGHT_QUERY,
                    tree_sitter_c::HIGHLIGHT_QUERY
                );
                return (tree_sitter_cpp::LANGUAGE.into(), q.into());
            }
            Lang::CSharp => (
                tree_sitter_c_sharp::LANGUAGE.into(),
                tree_sitter_c_sharp::HIGHLIGHTS_QUERY,
            ),
            Lang::Go => (
                tree_sitter_go::LANGUAGE.into(),
                tree_sitter_go::HIGHLIGHTS_QUERY,
            ),
            Lang::Html => (
                tree_sitter_html::LANGUAGE.into(),
                tree_sitter_html::HIGHLIGHTS_QUERY,
            ),
            Lang::Java => (
                tree_sitter_java::LANGUAGE.into(),
                tree_sitter_java::HIGHLIGHTS_QUERY,
            ),
            Lang::Ruby => (
                tree_sitter_ruby::LANGUAGE.into(),
                tree_sitter_ruby::HIGHLIGHTS_QUERY,
            ),
            Lang::TypeScript | Lang::Tsx => {
                let q = format!(
                    "{}\n{}",
                    tree_sitter_typescript::HIGHLIGHTS_QUERY,
                    tree_sitter_javascript::HIGHLIGHT_QUERY
                );
                let l = if self == Lang::Tsx {
                    tree_sitter_typescript::LANGUAGE_TSX
                } else {
                    tree_sitter_typescript::LANGUAGE_TYPESCRIPT
                };
                return (l.into(), q.into());
            }
            Lang::Xml => (
                tree_sitter_xml::LANGUAGE_XML.into(),
                tree_sitter_xml::XML_HIGHLIGHT_QUERY,
            ),
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
        };
        (l, q.into())
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
                    HighlightConfiguration::new(language, lang.name(), &query, "", "").ok()?;
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

    #[test]
    fn fence_tags() {
        assert_eq!(Lang::from_fence("rust"), Some(Lang::Rust));
        assert_eq!(Lang::from_fence("Python title=\"x\""), Some(Lang::Python));
        assert_eq!(Lang::from_fence("js,linenos"), Some(Lang::JavaScript));
        assert_eq!(
            Lang::from_fence("{.yaml}"),
            None,
            "attribute syntax is not a tag"
        );
        assert_eq!(Lang::from_fence(".yaml"), Some(Lang::Yaml));
        assert_eq!(Lang::from_fence("c#"), Some(Lang::CSharp));
        assert_eq!(Lang::from_fence(""), None);
        assert_eq!(Lang::from_fence("mermaid"), None);
    }
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
        assert_eq!(Lang::from_path(Path::new("x.hpp")), Some(Lang::Cpp));
        assert_eq!(Lang::from_path(Path::new("App.tsx")), Some(Lang::Tsx));
        assert_eq!(Lang::from_path(Path::new("Gemfile")), Some(Lang::Ruby));
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
            (Lang::C, "int main(void) { return 0; } /* c */"),
            (Lang::Cpp, "class A { public: int x = 1; };"),
            (Lang::CSharp, "class A { string s = \"x\"; }"),
            (Lang::Go, "package main\nfunc main() { _ = \"x\" }"),
            (Lang::Html, "<p class=\"a\">hi</p><!-- c -->"),
            (Lang::Java, "class A { String s = \"x\"; }"),
            (Lang::Ruby, "def f\n  'x'\nend"),
            (Lang::TypeScript, "const x: number = 1;"),
            (Lang::Tsx, "const a = <div className=\"x\">hi</div>;"),
            (Lang::Xml, "<?xml version=\"1.0\"?><a b=\"c\">t</a>"),
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
