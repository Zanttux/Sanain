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

mod folds;
mod incremental;

pub use incremental::Syntax;

/// PHP's own injections plus the HTML around `<?php … ?>`, highlighted as
/// one combined HTML document.
static PHP_INJECTIONS: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    format!(
        "{}\n((text) @injection.content\n (#set! injection.language \"html\")\n (#set! injection.combined))\n",
        tree_sitter_php::INJECTIONS_QUERY
    )
});

/// The injection name the Markdown block grammar gives inline content.
const MARKDOWN_INLINE: &str = "markdown_inline";

/// Markdown's block injections. Inline content must include the block
/// node's anonymous children (its text), which the upstream query leaves
/// to the editor (Helix: `include-unnamed-children`).
static MARKDOWN_INJECTIONS: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    tree_sitter_md::INJECTION_QUERY_BLOCK.replace(
        "(#set! injection.language \"markdown_inline\"))",
        "(#set! injection.language \"markdown_inline\")\n  (#set! injection.include-children))",
    )
});

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
    Lua,
    Markdown,
    Php,
    PowerShell,
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
    pub const ALL: [Lang; 22] = [
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
        Lang::Lua,
        Lang::Markdown,
        Lang::Php,
        Lang::PowerShell,
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
            ".luacheckrc" => return Some(Lang::Lua),
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
            "lua" | "luau" | "rockspec" => Lang::Lua,
            "md" | "markdown" | "mdown" | "mkd" => Lang::Markdown,
            "php" | "phtml" | "php3" | "php4" | "php5" | "php7" | "php8" | "phps" => Lang::Php,
            "ps1" | "psm1" | "psd1" => Lang::PowerShell,
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

    /// Tree-sitter injection query: where other languages are embedded.
    fn injections_query(self) -> &'static str {
        match self {
            Lang::Html => tree_sitter_html::INJECTIONS_QUERY,
            Lang::JavaScript => tree_sitter_javascript::INJECTIONS_QUERY,
            Lang::Markdown => MARKDOWN_INJECTIONS.as_str(),
            Lang::Php => PHP_INJECTIONS.as_str(),
            _ => "",
        }
    }

    /// Languages this one can embed (see [`Self::injections_query`]).
    fn embedded(self) -> &'static [Lang] {
        match self {
            Lang::Html => &[Lang::JavaScript, Lang::Css],
            Lang::JavaScript => &[Lang::Css, Lang::Html],
            // Fenced code blocks are found by `fence_langs`.
            Lang::Markdown => &[Lang::Html, Lang::Yaml, Lang::Toml],
            // Heredocs may name their language (`<<<HTML`).
            Lang::Php => &[Lang::Html, Lang::JavaScript, Lang::Css],
            _ => &[],
        }
    }

    /// Detect from the start of a file whose name says nothing: a shebang
    /// (`#!/usr/bin/env python3`, `#!/bin/bash`), an XML declaration or an
    /// HTML doctype. `head` is the first few hundred characters.
    pub fn from_content(head: &str) -> Option<Self> {
        let head = head.trim_start_matches('\u{feff}');
        let first = head.lines().next().unwrap_or("").trim();
        if let Some(cmd) = first.strip_prefix("#!") {
            let mut words = cmd.split_whitespace();
            let mut prog = words.next()?.rsplit('/').next()?;
            if prog == "env" {
                prog = words.find(|w| !w.starts_with('-'))?;
            }
            let prog = prog.trim_end_matches(|c: char| c.is_ascii_digit() || c == '.');
            return Some(match prog {
                "sh" | "bash" | "zsh" | "dash" | "ksh" | "ash" => Lang::Bash,
                "python" | "pypy" => Lang::Python,
                "node" | "nodejs" | "deno" | "bun" => Lang::JavaScript,
                "ruby" | "jruby" => Lang::Ruby,
                "php" => Lang::Php,
                "lua" | "luajit" => Lang::Lua,
                "pwsh" | "powershell" => Lang::PowerShell,
                _ => return None,
            });
        }
        let lower = first.to_ascii_lowercase();
        if lower.starts_with("<?xml") {
            return Some(Lang::Xml);
        }
        if lower.starts_with("<!doctype html") || lower.starts_with("<html") {
            return Some(Lang::Html);
        }
        if lower.starts_with("<?php") {
            return Some(Lang::Php);
        }
        None
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
            "lua" | "luau" => Lang::Lua,
            "md" | "markdown" => Lang::Markdown,
            "php" => Lang::Php,
            "ps1" | "powershell" | "pwsh" | "ps" => Lang::PowerShell,
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
            | Lang::Php
            | Lang::Rust
            | Lang::Tsx
            | Lang::TypeScript => Some("//"),
            Lang::Bash | Lang::PowerShell | Lang::Python | Lang::Ruby | Lang::Toml | Lang::Yaml => {
                Some("#")
            }
            Lang::Lua => Some("--"),
            Lang::Css | Lang::Html | Lang::Json | Lang::Markdown | Lang::Xml => None,
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
            Lang::Lua => "Lua",
            Lang::Markdown => "Markdown",
            Lang::Php => "PHP",
            Lang::PowerShell => "PowerShell",
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
            Lang::Lua => (
                tree_sitter_lua::LANGUAGE.into(),
                tree_sitter_lua::HIGHLIGHTS_QUERY,
            ),
            Lang::Markdown => (
                tree_sitter_md::LANGUAGE.into(),
                tree_sitter_md::HIGHLIGHT_QUERY_BLOCK,
            ),
            Lang::Php => (
                tree_sitter_php::LANGUAGE_PHP.into(),
                tree_sitter_php::HIGHLIGHTS_QUERY,
            ),
            Lang::PowerShell => (
                tree_sitter_powershell::LANGUAGE.into(),
                tree_sitter_powershell::HIGHLIGHTS_QUERY,
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
const NAMES: [&str; 35] = [
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
    // Older (nvim-treesitter style) names used by some bundled queries.
    "conditional",
    "repeat",
    "storageclass",
    "include",
    "exception",
    "preproc",
    "field",
    "parameter",
    "method",
    "float",
    "delimiter",
    // Markdown.
    "text.title",
    "text.literal",
    "text.uri",
];

const STYLES: [Style; 35] = [
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
    Style::Keyword,
    Style::Keyword,
    Style::Keyword,
    Style::Keyword,
    Style::Keyword,
    Style::Attribute,
    Style::Property,
    Style::Variable,
    Style::Function,
    Style::Number,
    Style::Punctuation,
    Style::Keyword,
    Style::String,
    Style::Constant,
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
    /// Markdown's inline grammar, injected into its block grammar.
    md_inline: Option<Option<HighlightConfiguration>>,
    inner: tree_sitter_highlight::Highlighter,
    /// Parser for folding and [`Self::analyze`] (the highlighter keeps
    /// its own).
    parser: tree_sitter::Parser,
    /// Query cursor and capture styles for [`Self::analyze`].
    cursor: tree_sitter::QueryCursor,
    styles: HashMap<Lang, Vec<Option<Style>>>,
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
            md_inline: None,
            inner: tree_sitter_highlight::Highlighter::new(),
            parser: tree_sitter::Parser::new(),
            cursor: tree_sitter::QueryCursor::new(),
            styles: HashMap::new(),
        }
    }

    fn config(&mut self, lang: Lang) -> Option<&HighlightConfiguration> {
        self.configs
            .entry(lang)
            .or_insert_with(|| {
                let (language, query) = lang.grammar();
                let mut c = HighlightConfiguration::new(
                    language,
                    lang.name(),
                    &query,
                    lang.injections_query(),
                    "",
                )
                .ok()?;
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
        // Languages that may be embedded (e.g. JavaScript in HTML) must be
        // ready before highlighting borrows the configurations.
        for l in lang.embedded() {
            let _ = self.config(*l);
        }
        if lang == Lang::Markdown {
            for l in fence_langs(source) {
                let _ = self.config(l);
            }
            self.md_inline.get_or_insert_with(|| {
                let mut c = HighlightConfiguration::new(
                    tree_sitter_md::INLINE_LANGUAGE.into(),
                    MARKDOWN_INLINE,
                    tree_sitter_md::HIGHLIGHT_QUERY_INLINE,
                    tree_sitter_md::INJECTION_QUERY_INLINE,
                    "",
                )
                .ok()?;
                c.configure(&NAMES);
                Some(c)
            });
        }
        let Self {
            configs,
            md_inline,
            inner,
            ..
        } = self;
        let Some(Some(config)) = configs.get(&lang) else {
            return Ok(Vec::new());
        };
        let events = inner
            .highlight(config, source, None, cancel, |name| {
                if name == MARKDOWN_INLINE {
                    return md_inline.as_ref().and_then(Option::as_ref);
                }
                Lang::from_fence(name)
                    .and_then(|l| configs.get(&l))
                    .and_then(Option::as_ref)
            })
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

    /// Foldable line ranges of `source`, from its syntax tree. In each
    /// range, `start` is the line the fold starts on (it stays visible)
    /// and `start + 1..end` are the lines it hides. A closing bracket or
    /// tag on the node's last line stays visible. Sorted by start, at most
    /// one (the outermost) per start line. Lines are counted by `\n`.
    /// Cancellable like [`Self::highlight`].
    pub fn folds(
        &mut self,
        lang: Lang,
        source: &[u8],
        cancel: Option<&AtomicUsize>,
    ) -> Result<Vec<Range<usize>>, HighlightError> {
        let Some(config) = self.config(lang) else {
            return Err(HighlightError::Failed(format!(
                "{} query did not compile",
                lang.name()
            )));
        };
        let language = config.language.clone();
        folds::folds(&mut self.parser, &language, lang, source, cancel)
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

/// Languages named by the code fences (` ```rust `, `~~~ py`) of a
/// Markdown document, so their configurations are ready before
/// highlighting.
fn fence_langs(source: &[u8]) -> Vec<Lang> {
    let mut out: Vec<Lang> = Vec::new();
    for line in source.split(|&b| b == b'\n') {
        let t = line.trim_ascii_start();
        let Some(rest) = t.strip_prefix(b"```").or_else(|| t.strip_prefix(b"~~~")) else {
            continue;
        };
        let skip = rest.iter().take_while(|&&b| b == b'`' || b == b'~').count();
        let info = String::from_utf8_lossy(rest.get(skip..).unwrap_or_default());
        let info = info.trim_start();
        if let Some(l) = Lang::from_fence(info)
            && !out.contains(&l)
        {
            out.push(l);
        }
    }
    out
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
    fn embedded_languages_in_html() {
        let src = "<p>x</p>\n<script>const n = 42;</script>\n<style>a { color: red; }</style>\n";
        let mut h = Highlighter::new();
        let spans = h.highlight(Lang::Html, src.as_bytes(), None).unwrap();
        let at = |needle: &str| {
            let i = src.find(needle).unwrap();
            spans.iter().find(|s| s.range.contains(&i)).map(|s| s.style)
        };
        assert_eq!(
            at("const"),
            Some(Style::Keyword),
            "JavaScript inside <script>"
        );
        assert_eq!(at("42"), Some(Style::Number));
        assert!(at("color").is_some(), "CSS inside <style>");
    }

    #[test]
    fn php_highlights_the_html_around_it() {
        let src = "<p class=\"a\">hi</p>\n<?php echo strlen('x'); ?>\n<b>b</b>\n";
        let mut h = Highlighter::new();
        let spans = h.highlight(Lang::Php, src.as_bytes(), None).unwrap();
        let at = |needle: &str| {
            let i = src.find(needle).unwrap();
            spans.iter().find(|s| s.range.contains(&i)).map(|s| s.style)
        };
        assert_eq!(at("p class"), Some(Style::Tag), "HTML before <?php");
        assert_eq!(at("class"), Some(Style::Attribute));
        assert_eq!(at("echo"), Some(Style::Keyword));
        assert_eq!(at("'x'"), Some(Style::String));
        assert_eq!(at("b>b"), Some(Style::Tag), "HTML after ?>");
    }

    #[test]
    fn markdown_headings_inline_code_and_fenced_code() {
        let src = "# Title\n\nA `span` here.\n\n```rust\nfn f() {}\n```\n";
        let mut h = Highlighter::new();
        let spans = h.highlight(Lang::Markdown, src.as_bytes(), None).unwrap();
        let at = |needle: &str| {
            let i = src.find(needle).unwrap();
            spans.iter().find(|s| s.range.contains(&i)).map(|s| s.style)
        };
        assert_eq!(at("Title"), Some(Style::Keyword), "heading");
        assert_eq!(at("`span"), Some(Style::Punctuation), "code delimiter");
        assert_eq!(at("span`"), Some(Style::String), "inline code");
        assert_eq!(at("fn f"), Some(Style::Keyword), "Rust inside the fence");
        assert_eq!(fence_langs(src.as_bytes()), vec![Lang::Rust]);
        assert_eq!(
            fence_langs(b"  ~~~~ py title=x\n````js\n```\n```nope\n"),
            vec![Lang::Python, Lang::JavaScript]
        );
    }

    #[test]
    fn content_detection() {
        assert_eq!(Lang::from_content("#!/bin/bash\necho"), Some(Lang::Bash));
        assert_eq!(
            Lang::from_content("#!/usr/bin/env python3\n"),
            Some(Lang::Python)
        );
        assert_eq!(
            Lang::from_content("#!/usr/bin/env -S node --flag\n"),
            Some(Lang::JavaScript)
        );
        assert_eq!(
            Lang::from_content("#!/usr/local/bin/ruby2.7"),
            Some(Lang::Ruby)
        );
        assert_eq!(Lang::from_content("#!/usr/bin/perl"), None);
        assert_eq!(Lang::from_content("#!/usr/bin/env php\n"), Some(Lang::Php));
        assert_eq!(Lang::from_content("#!/usr/bin/env lua5.4"), Some(Lang::Lua));
        assert_eq!(
            Lang::from_content("#!/usr/bin/env pwsh"),
            Some(Lang::PowerShell)
        );
        assert_eq!(Lang::from_content("<?php\necho 1;"), Some(Lang::Php));
        assert_eq!(
            Lang::from_content("\u{feff}<?xml version=\"1.0\"?>"),
            Some(Lang::Xml)
        );
        assert_eq!(
            Lang::from_content("<!DOCTYPE html>\n<html>"),
            Some(Lang::Html)
        );
        assert_eq!(Lang::from_content("hello"), None);
    }

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
        assert_eq!(Lang::from_fence("php"), Some(Lang::Php));
        assert_eq!(Lang::from_fence("lua"), Some(Lang::Lua));
        assert_eq!(Lang::from_fence("pwsh"), Some(Lang::PowerShell));
        assert_eq!(Lang::from_fence("markdown"), Some(Lang::Markdown));
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
        assert_eq!(Lang::from_path(Path::new("index.php")), Some(Lang::Php));
        assert_eq!(Lang::from_path(Path::new("view.phtml")), Some(Lang::Php));
        assert_eq!(Lang::from_path(Path::new("init.lua")), Some(Lang::Lua));
        assert_eq!(Lang::from_path(Path::new(".luacheckrc")), Some(Lang::Lua));
        assert_eq!(
            Lang::from_path(Path::new("Build.PS1")),
            Some(Lang::PowerShell)
        );
        assert_eq!(
            Lang::from_path(Path::new("Mod.psm1")),
            Some(Lang::PowerShell)
        );
        assert_eq!(
            Lang::from_path(Path::new("README.md")),
            Some(Lang::Markdown)
        );
        assert_eq!(
            Lang::from_path(Path::new("notes.markdown")),
            Some(Lang::Markdown)
        );
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
            (Lang::Php, "<?php\nfunction f($a) { return 'x'; } // c\n"),
            (Lang::Lua, "local x = 'y' -- c\nfunction f() return 1 end"),
            (
                Lang::PowerShell,
                "function Get-X { param($a) Write-Host \"hi\" } # c",
            ),
            (Lang::Markdown, "# Title\n\nSome *text* and `code`.\n"),
        ];
        assert_eq!(samples.len(), Lang::ALL.len(), "a sample per language");
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
