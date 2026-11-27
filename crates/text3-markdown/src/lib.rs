//! Markdown (CommonMark + GFM) parsed into a small, render-ready model.
//!
//! The model is deliberately inert: raw HTML is kept as literal text,
//! nothing is fetched or executed, and link targets are classified so the
//! UI can refuse unsafe schemes. Nesting deeper than [`MAX_DEPTH`] is
//! flattened to plain text so hostile documents cannot exhaust the stack.
//! The source document is never modified.

use comrak::nodes::{AstNode, ListType, NodeValue, TableAlignment};
use comrak::{Arena, Options, parse_document};

pub mod mermaid;

pub const MAX_DEPTH: usize = 48;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Inline {
    Text(String),
    Code(String),
    Strong(Vec<Inline>),
    Emph(Vec<Inline>),
    Strike(Vec<Inline>),
    Link {
        url: String,
        children: Vec<Inline>,
    },
    /// Images are not loaded yet; the alt text is shown.
    Image {
        url: String,
        alt: String,
    },
    /// Raw HTML, shown as literal text, never interpreted.
    Html(String),
    SoftBreak,
    LineBreak,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Align {
    None,
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
    /// `Some(checked)` for task list items.
    pub task: Option<bool>,
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockKind {
    Heading {
        level: u8,
        inlines: Vec<Inline>,
    },
    Paragraph(Vec<Inline>),
    Code {
        lang: String,
        text: String,
    },
    Quote(Vec<Block>),
    List {
        start: Option<usize>,
        items: Vec<Item>,
    },
    Table {
        aligns: Vec<Align>,
        header: Vec<Vec<Inline>>,
        rows: Vec<Vec<Vec<Inline>>>,
    },
    Rule,
    /// Raw HTML block, shown as literal text.
    Html(String),
    FrontMatter(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub kind: BlockKind,
    /// 1-based source line where the block starts (for scroll sync).
    pub line: usize,
}

/// What a link's target is, for deciding how the UI may open it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkKind {
    /// http(s) or mailto: may be opened after confirmation.
    External,
    /// `#anchor` within the document.
    Anchor,
    /// Relative path to a local file.
    Relative,
    /// javascript:, data:, file:, vbscript:, network paths (`\\host\share`)
    /// or anything unknown: never opened.
    Blocked,
}

/// Whether `path` names another computer or a device rather than a file
/// here: it starts with two slashes or backslashes in any mix
/// (`\\host\share`, `//host/share`, `\\?\UNC\…`, `\\.\pipe\…`).
/// On Windows merely looking at such a path (does the file exist?) makes
/// an SMB connection that hands the user's NTLM credentials to that host,
/// so a document must never point the editor at one (Notepad++
/// GHSA-rv8g-c295-pwcg, GHSA-7w2m-5769-p4q3).
pub fn is_network_path(path: &str) -> bool {
    let mut it = path.trim_start().chars();
    matches!((it.next(), it.next()), (Some('/' | '\\'), Some('/' | '\\')))
}

pub fn classify_link(url: &str) -> LinkKind {
    let u = url.trim();
    if u.starts_with('#') {
        return LinkKind::Anchor;
    }
    if is_network_path(u) {
        return LinkKind::Blocked;
    }
    let lower = u.to_ascii_lowercase();
    match lower.split_once(':') {
        Some((scheme, _))
            if scheme
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || "+-.".contains(c)) =>
        {
            if matches!(scheme, "http" | "https" | "mailto") {
                LinkKind::External
            } else if scheme.len() == 1 {
                // Windows drive letter such as C:\ — a local path.
                LinkKind::Relative
            } else {
                LinkKind::Blocked
            }
        }
        _ => LinkKind::Relative,
    }
}

fn options() -> Options<'static> {
    let mut o = Options::default();
    o.extension.strikethrough = true;
    o.extension.table = true;
    o.extension.autolink = true;
    o.extension.tasklist = true;
    o.extension.front_matter_delimiter = Some("---".into());
    o
}

/// Parse `source` into blocks.
pub fn parse(source: &str) -> Vec<Block> {
    let arena = Arena::new();
    let root = parse_document(&arena, source, &options());
    blocks(root, 0)
}

fn blocks<'a>(node: &'a AstNode<'a>, depth: usize) -> Vec<Block> {
    node.children()
        .filter_map(|c| block(c, depth + 1))
        .collect()
}

fn block<'a>(node: &'a AstNode<'a>, depth: usize) -> Option<Block> {
    let line = node.data().sourcepos.start.line;
    if depth > MAX_DEPTH {
        return Some(Block {
            kind: BlockKind::Paragraph(vec![Inline::Text(plain_text(node))]),
            line,
        });
    }
    let kind = match &node.data().value {
        NodeValue::Heading(h) => BlockKind::Heading {
            level: h.level,
            inlines: inlines(node, depth),
        },
        NodeValue::Paragraph => BlockKind::Paragraph(inlines(node, depth)),
        NodeValue::CodeBlock(c) => BlockKind::Code {
            lang: c.info.split_whitespace().next().unwrap_or("").to_owned(),
            text: c.literal.clone(),
        },
        NodeValue::BlockQuote | NodeValue::MultilineBlockQuote(_) => {
            BlockKind::Quote(blocks(node, depth))
        }
        NodeValue::List(l) => BlockKind::List {
            start: (l.list_type == ListType::Ordered).then_some(l.start),
            items: node
                .children()
                .map(|item| Item {
                    task: match &item.data().value {
                        NodeValue::TaskItem(t) => Some(t.symbol.is_some()),
                        _ => None,
                    },
                    blocks: blocks(item, depth + 1),
                })
                .collect(),
        },
        NodeValue::Table(t) => {
            let aligns = t
                .alignments
                .iter()
                .map(|a| match a {
                    TableAlignment::Left => Align::Left,
                    TableAlignment::Center => Align::Center,
                    TableAlignment::Right => Align::Right,
                    _ => Align::None,
                })
                .collect();
            let mut header = Vec::new();
            let mut rows = Vec::new();
            for row in node.children() {
                let is_header = matches!(&row.data().value, NodeValue::TableRow(r) if r.header);
                let cells: Vec<Vec<Inline>> = row.children().map(|c| inlines(c, depth)).collect();
                if is_header {
                    header = cells;
                } else {
                    rows.push(cells);
                }
            }
            BlockKind::Table {
                aligns,
                header,
                rows,
            }
        }
        NodeValue::ThematicBreak(_) => BlockKind::Rule,
        NodeValue::HtmlBlock(h) => BlockKind::Html(h.literal.clone()),
        NodeValue::FrontMatter(f) => BlockKind::FrontMatter(strip_delimiters(f)),
        // Anything else (footnotes, math, extensions not enabled) is shown
        // as its plain text rather than dropped.
        _ => {
            let text = plain_text(node);
            if text.trim().is_empty() {
                return None;
            }
            BlockKind::Paragraph(vec![Inline::Text(text)])
        }
    };
    Some(Block { kind, line })
}

fn inlines<'a>(node: &'a AstNode<'a>, depth: usize) -> Vec<Inline> {
    let mut out = Vec::new();
    for c in node.children() {
        if depth > MAX_DEPTH {
            out.push(Inline::Text(plain_text(c)));
            continue;
        }
        let v = match &c.data().value {
            NodeValue::Text(t) => Inline::Text(t.to_string()),
            NodeValue::Code(code) => Inline::Code(code.literal.clone()),
            NodeValue::Strong(_) => Inline::Strong(inlines(c, depth + 1)),
            NodeValue::Emph(_) => Inline::Emph(inlines(c, depth + 1)),
            NodeValue::Strikethrough(_) => Inline::Strike(inlines(c, depth + 1)),
            NodeValue::Link(l) => Inline::Link {
                url: l.url.clone(),
                children: inlines(c, depth + 1),
            },
            NodeValue::Image(l) => Inline::Image {
                url: l.url.clone(),
                alt: plain_text(c),
            },
            NodeValue::HtmlInline(h) => Inline::Html(h.clone()),
            NodeValue::SoftBreak => Inline::SoftBreak,
            NodeValue::LineBreak => Inline::LineBreak,
            NodeValue::Paragraph => {
                // Table cells and tight list items may wrap a paragraph.
                out.extend(inlines(c, depth + 1));
                continue;
            }
            _ => Inline::Text(plain_text(c)),
        };
        out.push(v);
    }
    out
}

/// Front matter without its `---` fence lines.
fn strip_delimiters(raw: &str) -> String {
    let lines: Vec<&str> = raw.trim().lines().collect();
    let inner = match lines.as_slice() {
        [first, mid @ .., last] if first.trim() == "---" && last.trim() == "---" => mid,
        all => all,
    };
    inner.join("\n")
}

fn plain_text<'a>(node: &'a AstNode<'a>) -> String {
    let mut s = String::new();
    // Iterative walk: no recursion, so depth cannot overflow the stack.
    for d in node.descendants() {
        match &d.data().value {
            NodeValue::Text(t) => s.push_str(t),
            NodeValue::Code(c) => s.push_str(&c.literal),
            NodeValue::SoftBreak | NodeValue::LineBreak => s.push(' '),
            NodeValue::HtmlInline(h) => s.push_str(h),
            _ => {}
        }
    }
    s
}

/// Headings as (level, text, source line) for an outline.
pub fn outline(blocks: &[Block]) -> Vec<(u8, String, usize)> {
    blocks
        .iter()
        .filter_map(|b| match &b.kind {
            BlockKind::Heading { level, inlines } => Some((*level, inline_text(inlines), b.line)),
            _ => None,
        })
        .collect()
}

pub fn inline_text(inlines: &[Inline]) -> String {
    let mut s = String::new();
    for i in inlines {
        match i {
            Inline::Text(t) | Inline::Code(t) | Inline::Html(t) => s.push_str(t),
            Inline::Strong(c) | Inline::Emph(c) | Inline::Strike(c) => s.push_str(&inline_text(c)),
            Inline::Link { children, .. } => s.push_str(&inline_text(children)),
            Inline::Image { alt, .. } => s.push_str(alt),
            Inline::SoftBreak | Inline::LineBreak => s.push(' '),
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gfm_elements() {
        let md = "---\ntitle: x\n---\n# Title\n\nSome *em* **strong** ~~gone~~ `code` [link](https://x.org).\n\n- [x] done\n- [ ] todo\n\n| a | b |\n|:--|--:|\n| 1 | 2 |\n\n```rust\nfn f() {}\n```\n\n> quote\n\n---\n";
        let b = parse(md);
        let kinds: Vec<&str> = b
            .iter()
            .map(|b| match &b.kind {
                BlockKind::FrontMatter(_) => "front",
                BlockKind::Heading { .. } => "h",
                BlockKind::Paragraph(_) => "p",
                BlockKind::List { .. } => "list",
                BlockKind::Table { .. } => "table",
                BlockKind::Code { .. } => "code",
                BlockKind::Quote(_) => "quote",
                BlockKind::Rule => "hr",
                BlockKind::Html(_) => "html",
            })
            .collect();
        assert_eq!(
            kinds,
            ["front", "h", "p", "list", "table", "code", "quote", "hr"]
        );
        let BlockKind::List { items, .. } = &b[3].kind else {
            unreachable!()
        };
        assert_eq!(
            items.iter().map(|i| i.task).collect::<Vec<_>>(),
            [Some(true), Some(false)]
        );
        let BlockKind::Table {
            aligns,
            header,
            rows,
        } = &b[4].kind
        else {
            unreachable!()
        };
        assert_eq!(aligns, &[Align::Left, Align::Right]);
        assert_eq!(inline_text(&header[0]), "a");
        assert_eq!(inline_text(&rows[0][1]), "2");
        let BlockKind::Code { lang, text } = &b[5].kind else {
            unreachable!()
        };
        assert_eq!((lang.as_str(), text.as_str()), ("rust", "fn f() {}\n"));
        assert_eq!(b[0].kind, BlockKind::FrontMatter("title: x".into()));
        assert_eq!(b[1].line, 4, "source line of the heading");
        assert_eq!(outline(&b), vec![(1, "Title".to_owned(), 4)]);
    }

    #[test]
    fn html_is_literal_never_markup() {
        let b = parse("<script>alert(1)</script>\n\nhi <b>x</b>");
        assert_eq!(
            b[0].kind,
            BlockKind::Html("<script>alert(1)</script>\n".into())
        );
        let BlockKind::Paragraph(p) = &b[1].kind else {
            unreachable!()
        };
        assert!(p.contains(&Inline::Html("<b>".into())));
    }

    #[test]
    fn link_classification() {
        assert_eq!(classify_link("https://example.org"), LinkKind::External);
        assert_eq!(classify_link("MAILTO:a@b.c"), LinkKind::External);
        assert_eq!(classify_link("#section"), LinkKind::Anchor);
        assert_eq!(classify_link("docs/x.md"), LinkKind::Relative);
        assert_eq!(classify_link("C:\\x.md"), LinkKind::Relative);
        for bad in [
            "javascript:alert(1)",
            " JavaScript:x",
            "data:text/html,x",
            "file:///etc/passwd",
            "vbscript:x",
            r"\\attacker\share\x.md",
            "//attacker/share/x.md",
            r"\\?\UNC\attacker\share\x.md",
            r"\\.\pipe\x",
            r" /\attacker\x",
        ] {
            assert_eq!(classify_link(bad), LinkKind::Blocked, "{bad}");
        }
        assert_eq!(classify_link(r"\docs\x.md"), LinkKind::Relative);
        assert_eq!(classify_link("/docs/x.md"), LinkKind::Relative);
    }

    #[test]
    fn deep_nesting_does_not_overflow() {
        let md = format!("{}deep", "> ".repeat(5_000));
        let b = parse(&md);
        assert!(!b.is_empty());
        let md = format!("{}x{}", "*".repeat(3_000), "*".repeat(3_000));
        let _ = parse(&md);
        let md = "- ".repeat(2_000) + "item";
        let _ = parse(&md);
    }

    #[test]
    fn malformed_and_unicode_inputs() {
        for md in [
            "",
            "\0",
            "| a |\n|---|\n| b | c | d |",
            "```\nunclosed",
            "[x](",
            "ä🙂\r\n\r\n# ü",
        ] {
            let _ = parse(md);
        }
    }
}
