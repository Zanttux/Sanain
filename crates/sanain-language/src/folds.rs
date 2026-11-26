//! Syntax-aware folding: foldable line ranges from the Tree-sitter tree.

use std::ops::{ControlFlow, Range};
use std::sync::atomic::{AtomicUsize, Ordering};

use tree_sitter::{Node, ParseOptions, Parser};

use crate::{HighlightError, Lang};

/// Node kinds that fold when they contain one of these.
const FOLDABLE: [&str; 26] = [
    "block",
    "body",
    "object",
    "array",
    "list",
    "class",
    "function",
    "method",
    "impl",
    "struct",
    "enum",
    "mod",
    "table",
    "element",
    "declaration_list",
    "field_declaration_list",
    "switch",
    "match",
    "statement_block",
    "compound_statement",
    "mapping",
    "sequence",
    "dictionary",
    "interface",
    "namespace",
    "comment",
];

fn foldable(lang: Lang, kind: &str) -> bool {
    // YAML block containers start on the line after their key; the
    // enclosing pair or item folds instead, on the key's line.
    if lang == Lang::Yaml && matches!(kind, "block_node" | "block_mapping" | "block_sequence") {
        return false;
    }
    // PowerShell wraps bodies in nodes that start after the header line
    // (`statement_list`, `script_block_body`, `hash_literal_body`); only
    // the constructs that start on their own line fold.
    if lang == Lang::PowerShell {
        return matches!(
            kind,
            "function_statement"
                | "class_statement"
                | "class_method_definition"
                | "enum_statement"
                | "statement_block"
                | "script_block_expression"
                | "hash_literal_expression"
                | "array_expression"
                | "param_block"
                | "named_block"
                | "switch_body"
                | "comment"
        );
    }
    // Markdown folds whole heading sections.
    if lang == Lang::Markdown && kind == "section" {
        return true;
    }
    FOLDABLE.iter().any(|k| kind.contains(k))
}

/// Fold ranges for a parsed `tree` of `source`. Each range is a fold:
/// `range.start` is the line the fold starts on (it stays visible) and
/// the hidden lines are `range.start + 1..range.end`. Sorted by start,
/// at most one fold per start line (the outermost).
fn collect(lang: Lang, root: Node<'_>, source: &[u8]) -> Vec<Range<usize>> {
    let starts: Vec<usize> = std::iter::once(0)
        .chain(
            source
                .iter()
                .enumerate()
                .filter(|&(_, &b)| b == b'\n')
                .map(|(i, _)| i + 1),
        )
        .collect();
    let line = |l: usize| -> &[u8] {
        let s = starts.get(l).copied().unwrap_or(source.len());
        let e = starts.get(l + 1).map_or(source.len(), |&e| e - 1);
        source.get(s..e).unwrap_or_default()
    };
    let trimmed = |l: usize| line(l).trim_ascii();

    let mut out: Vec<Range<usize>> = Vec::new();
    let mut cursor = root.walk();
    'walk: loop {
        let node = cursor.node();
        let (start, end) = (node.start_position(), node.end_position());
        // A node ending right after a newline ends on the previous line.
        let last = if end.column == 0 && end.row > start.row {
            end.row - 1
        } else {
            end.row
        };
        // A Python or Lua block starts at its first statement; fold from
        // the header line (`def f():`, `if x then`, `else`) instead.
        let first = if matches!(lang, Lang::Python | Lang::Lua) && node.kind() == "block" {
            node.parent().map_or(start.row, |p| p.start_position().row)
        } else {
            start.row
        };
        // The root (a whole module or document) does not fold.
        let nested = cursor.depth() > 0;
        if last > first && nested && node.is_named() && foldable(lang, node.kind()) {
            let closing = {
                let t = trimmed(last);
                matches!(t.first(), Some(b'}' | b']' | b')'))
                    || t.starts_with(b"</")
                    || (lang == Lang::Markdown && (t.starts_with(b"```") || t.starts_with(b"~~~")))
                    || (matches!(lang, Lang::Ruby | Lang::Lua)
                        && t.starts_with(b"end")
                        && !t
                            .get(3)
                            .is_some_and(|c| c.is_ascii_alphanumeric() || *c == b'_'))
            };
            let mut end_hidden = if closing { last } else { last + 1 };
            if !closing {
                // Trailing blank lines stay visible, as with indentation.
                while end_hidden > first + 1 && trimmed(end_hidden - 1).is_empty() {
                    end_hidden -= 1;
                }
            }
            if end_hidden > first + 1 {
                out.push(first..end_hidden);
            }
        }
        // Single-line nodes cannot contain multi-line ones.
        if last > start.row && cursor.goto_first_child() {
            continue;
        }
        loop {
            if cursor.goto_next_sibling() {
                continue 'walk;
            }
            if !cursor.goto_parent() {
                break 'walk;
            }
        }
    }
    // Outermost first for each start line, then keep one per line.
    out.sort_by(|a, b| a.start.cmp(&b.start).then(b.end.cmp(&a.end)));
    out.dedup_by_key(|r| r.start);
    out
}

/// Parse `source` and return its fold ranges (see [`crate::Highlighter::folds`]).
pub(crate) fn folds(
    parser: &mut Parser,
    language: &tree_sitter::Language,
    lang: Lang,
    source: &[u8],
    cancel: Option<&AtomicUsize>,
) -> Result<Vec<Range<usize>>, HighlightError> {
    parser
        .set_language(language)
        .map_err(|e| HighlightError::Failed(e.to_string()))?;
    let mut cancelled = false;
    let mut progress = |_: &tree_sitter::ParseState| {
        if cancel.is_some_and(|c| c.load(Ordering::Relaxed) != 0) {
            cancelled = true;
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    };
    let len = source.len();
    let tree = parser.parse_with_options(
        &mut |i, _| if i < len { &source[i..] } else { &[] },
        None,
        Some(ParseOptions::new().progress_callback(&mut progress)),
    );
    // The parser keeps partial state after an interruption.
    parser.reset();
    match tree {
        Some(tree) => Ok(collect(lang, tree.root_node(), source)),
        None if cancelled => Err(HighlightError::Cancelled),
        None => Err(HighlightError::Failed("parsing failed".into())),
    }
}

#[cfg(test)]
mod tests {
    use crate::{Highlighter, Lang};

    fn folds(lang: Lang, src: &str) -> Vec<std::ops::Range<usize>> {
        Highlighter::new()
            .folds(lang, src.as_bytes(), None)
            .unwrap()
    }

    #[test]
    fn rust_functions_and_nested_impl() {
        let src = "\
impl A {
    fn f() {
        x();
        y();
    }

    fn g() {}
}
fn h() {
    1
}
";
        // `}` lines stay visible; `fn g() {}` does not fold.
        assert_eq!(folds(Lang::Rust, src), vec![0..7, 1..4, 8..10]);
    }

    #[test]
    fn json_objects_and_arrays() {
        let src = "{\n  \"a\": [\n    1,\n    2\n  ],\n  \"b\": {\"c\": 1}\n}\n";
        assert_eq!(folds(Lang::Json, src), vec![0..6, 1..4]);
    }

    #[test]
    fn python_folds_from_the_header_line() {
        let src = "\
class A:
    def f(self):
        if x:
            a()
        else:
            b()

    def g(self):
        pass


y = 1
";
        // No closing line: hidden through the last line of the body,
        // blank lines after it stay visible.
        assert_eq!(folds(Lang::Python, src), vec![0..9, 1..6, 2..4, 4..6, 7..9]);
    }

    #[test]
    fn c_with_irregular_indentation() {
        let src = "\
int main(void)
{
if (x) {
return 1;
}
  return 0;
    }
";
        // Indentation sees no structure here; the syntax tree does.
        assert_eq!(folds(Lang::C, src), vec![0..6, 1..6, 2..4]);
    }

    #[test]
    fn one_fold_per_start_line_keeps_the_outermost() {
        // The object and the array both start on line 0.
        let src = "[{\n  \"a\": 1,\n  \"b\": 2\n}]\n";
        assert_eq!(folds(Lang::Json, src), vec![0..3]);
        let r = "fn f() {\n    a();\n    b();\n}\n";
        assert_eq!(folds(Lang::Rust, r), vec![0..3], "fn item and its block");
    }

    #[test]
    fn markup_comments_and_yaml() {
        let html = "<div>\n  <p>a</p>\n  <p>b</p>\n</div>\n";
        assert_eq!(folds(Lang::Html, html), vec![0..3]);
        let c = "/* a\n * b\n * c\n */\nint x;\n";
        assert_eq!(folds(Lang::C, c), vec![0..4]);
        let yaml = "a:\n  b: 1\n  c:\n    - 1\n    - 2\nd: 2\n";
        assert_eq!(folds(Lang::Yaml, yaml), vec![0..5, 2..5]);
    }

    #[test]
    fn lua_blocks_fold_from_the_header_and_keep_end_visible() {
        let src = "\
local t = {
  a = 1,
}
function f(x)
  if x then
    return 1
  end
  return 2
end
";
        assert_eq!(folds(Lang::Lua, src), vec![0..2, 3..8, 4..6]);
    }

    #[test]
    fn powershell_function_folds_on_its_name_line() {
        let src = "\
function Get-X {
  param($a)
  $h = @{
    k = 1
    j = 2
  }
  if ($a) {
    $h = @{
      k = 1
    }
  }
}
";
        // Not on `$h = @{`'s next line (the hash body) nor on the first
        // statement of the function body.
        assert_eq!(folds(Lang::PowerShell, src), vec![0..11, 2..5, 6..10, 7..9]);
    }

    #[test]
    fn php_classes_functions_and_arrays() {
        let src = "<?php\nclass A {\n  function f() {\n    return [\n      1,\n    ];\n  }\n}\n";
        assert_eq!(folds(Lang::Php, src), vec![1..7, 2..6, 3..5]);
    }

    #[test]
    fn markdown_sections_lists_and_fences() {
        let src = "\
# A

text

## B

- x
- y

```rust
fn f() {}
```

# C
end
";
        // Sections hide through their last line; blank lines before the
        // next heading stay visible, and so does a closing fence.
        assert_eq!(
            folds(Lang::Markdown, src),
            vec![0..12, 4..12, 6..8, 9..11, 13..15]
        );
    }

    #[test]
    fn every_language_survives_odd_input() {
        let mut h = Highlighter::new();
        for lang in Lang::ALL {
            for src in ["", "{{{{\n\"unterminated", "ä🙂\u{0}\n\u{FEFF}", "\r\n\r\n"] {
                assert!(h.folds(lang, src.as_bytes(), None).is_ok(), "{lang:?}");
            }
        }
    }
}
