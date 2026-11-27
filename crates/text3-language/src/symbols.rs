//! Symbols of a file (functions, types, modules) for Goto Anything's
//! `@` mode, from the Tree-sitter tree. A small per-language list of node
//! kinds, not a query: names come from the node's `name` field (or the
//! declarator chain in C and C++).

use std::sync::atomic::AtomicUsize;

use tree_sitter::{Node, Parser};

use crate::{HighlightError, Lang};

/// At most this many symbols are returned.
pub const MAX_SYMBOLS: usize = 10_000;
/// Longer names are shortened.
const MAX_NAME_CHARS: usize = 120;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolKind {
    Function,
    Type,
    Module,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    /// 0-based line of the symbol's start.
    pub line: usize,
    /// How many symbols enclose this one (a method in a class: 1).
    pub depth: usize,
}

fn kind_of(lang: Lang, node: Node<'_>) -> Option<SymbolKind> {
    use SymbolKind::{Function, Module, Type};
    let k = node.kind();
    let has_body = || node.child_by_field_name("body").is_some();
    match lang {
        Lang::Rust => match k {
            "function_item" | "function_signature_item" | "macro_definition" => Some(Function),
            "struct_item" | "enum_item" | "trait_item" | "type_item" | "union_item"
            | "impl_item" => Some(Type),
            "mod_item" => Some(Module),
            _ => None,
        },
        Lang::Python => match k {
            "function_definition" => Some(Function),
            "class_definition" => Some(Type),
            _ => None,
        },
        Lang::JavaScript | Lang::TypeScript | Lang::Tsx => match k {
            "function_declaration"
            | "generator_function_declaration"
            | "method_definition"
            | "function_signature" => Some(Function),
            "variable_declarator" => node
                .child_by_field_name("value")
                .filter(|v| {
                    matches!(
                        v.kind(),
                        "arrow_function" | "function_expression" | "function"
                    )
                })
                .map(|_| Function),
            "class_declaration"
            | "abstract_class_declaration"
            | "interface_declaration"
            | "type_alias_declaration"
            | "enum_declaration" => Some(Type),
            "internal_module" => Some(Module),
            _ => None,
        },
        Lang::Go => match k {
            "function_declaration" | "method_declaration" => Some(Function),
            "type_spec" => Some(Type),
            _ => None,
        },
        Lang::C | Lang::Cpp => match k {
            "function_definition" => Some(Function),
            "struct_specifier" | "enum_specifier" | "union_specifier" | "class_specifier"
                if has_body() =>
            {
                Some(Type)
            }
            "namespace_definition" => Some(Module),
            _ => None,
        },
        Lang::Java => match k {
            "method_declaration" | "constructor_declaration" => Some(Function),
            "class_declaration"
            | "interface_declaration"
            | "enum_declaration"
            | "record_declaration" => Some(Type),
            _ => None,
        },
        Lang::CSharp => match k {
            "method_declaration" | "constructor_declaration" => Some(Function),
            "class_declaration"
            | "struct_declaration"
            | "interface_declaration"
            | "enum_declaration"
            | "record_declaration" => Some(Type),
            "namespace_declaration" | "file_scoped_namespace_declaration" => Some(Module),
            _ => None,
        },
        Lang::Ruby => match k {
            "method" | "singleton_method" => Some(Function),
            "class" => Some(Type),
            "module" => Some(Module),
            _ => None,
        },
        Lang::Php => match k {
            "function_definition" | "method_declaration" => Some(Function),
            "class_declaration"
            | "interface_declaration"
            | "trait_declaration"
            | "enum_declaration" => Some(Type),
            _ => None,
        },
        Lang::Lua => (k == "function_declaration").then_some(Function),
        Lang::Bash => (k == "function_definition").then_some(Function),
        Lang::PowerShell => match k {
            "function_statement" | "class_method_definition" => Some(Function),
            "class_statement" => Some(Type),
            _ => None,
        },
        // Data and markup: no symbols (Markdown headings come from the
        // Markdown outline).
        Lang::Css
        | Lang::Html
        | Lang::Json
        | Lang::Markdown
        | Lang::Toml
        | Lang::Xml
        | Lang::Yaml => None,
    }
}

fn text<'a>(node: Node<'_>, source: &'a [u8]) -> &'a str {
    std::str::from_utf8(source.get(node.byte_range()).unwrap_or_default()).unwrap_or_default()
}

/// One line, whitespace collapsed, at most [`MAX_NAME_CHARS`].
fn tidy(s: &str) -> String {
    let joined = s.split_whitespace().collect::<Vec<_>>().join(" ");
    if joined.chars().count() > MAX_NAME_CHARS {
        let mut cut: String = joined.chars().take(MAX_NAME_CHARS - 1).collect();
        cut.push('…');
        cut
    } else {
        joined
    }
}

fn name_of(lang: Lang, node: Node<'_>, source: &[u8]) -> Option<String> {
    // A Rust impl is named by its header: `impl Display for Point`.
    if node.kind() == "impl_item" {
        let header = text(node, source);
        let header = header.split('{').next().unwrap_or(header);
        return Some(tidy(header));
    }
    if matches!(lang, Lang::C | Lang::Cpp) && node.kind() == "function_definition" {
        // int *f(void) → pointer_declarator → function_declarator → f
        let mut d = node.child_by_field_name("declarator")?;
        while let Some(inner) = d.child_by_field_name("declarator") {
            d = inner;
        }
        let name = text(d, source);
        let name = name.split('(').next().unwrap_or(name);
        return Some(tidy(name)).filter(|n| !n.is_empty());
    }
    if let Some(n) = node.child_by_field_name("name") {
        return Some(tidy(text(n, source))).filter(|n| !n.is_empty());
    }
    // PowerShell names have no field: the first name-like child.
    let mut cursor = node.walk();
    let name = node.named_children(&mut cursor).find(|c| {
        matches!(
            c.kind(),
            "function_name" | "simple_name" | "identifier" | "constant" | "word"
        )
    })?;
    Some(tidy(text(name, source))).filter(|n| !n.is_empty())
}

/// The symbols of a parsed tree, in document order.
pub(crate) fn collect(lang: Lang, root: Node<'_>, source: &[u8]) -> Vec<Symbol> {
    let mut out = Vec::new();
    // End bytes of the symbols enclosing the current node.
    let mut open: Vec<usize> = Vec::new();
    let mut cursor = root.walk();
    'walk: loop {
        let node = cursor.node();
        while open.last().is_some_and(|&end| end <= node.start_byte()) {
            open.pop();
        }
        if node.is_named()
            && let Some(kind) = kind_of(lang, node)
            && let Some(name) = name_of(lang, node, source)
        {
            out.push(Symbol {
                name,
                kind,
                line: node.start_position().row,
                depth: open.len(),
            });
            if out.len() >= MAX_SYMBOLS {
                break;
            }
            open.push(node.end_byte());
        }
        if cursor.goto_first_child() {
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
    out
}

/// Parse `source` and list its symbols (see [`crate::Highlighter::symbols`]).
pub(crate) fn symbols(
    parser: &mut Parser,
    language: &tree_sitter::Language,
    lang: Lang,
    source: &[u8],
    cancel: Option<&AtomicUsize>,
) -> Result<Vec<Symbol>, HighlightError> {
    let tree = crate::incremental::parse(parser, language, source, &[], None, cancel)?
        .ok_or_else(|| HighlightError::Failed("parsing failed".into()))?;
    Ok(collect(lang, tree.root_node(), source))
}

#[cfg(test)]
mod tests {
    use super::{Symbol, SymbolKind};
    use crate::{Highlighter, Lang};

    fn names(lang: Lang, src: &str) -> Vec<(String, usize, usize)> {
        Highlighter::new()
            .symbols(lang, src.as_bytes(), None)
            .unwrap()
            .into_iter()
            .map(|s| (s.name, s.line, s.depth))
            .collect()
    }

    fn n(name: &str, line: usize, depth: usize) -> (String, usize, usize) {
        (name.to_owned(), line, depth)
    }

    #[test]
    fn rust_items_impls_and_nesting() {
        let src = "\
mod m {
    pub struct Point { x: i32 }
    impl std::fmt::Display for Point {
        fn fmt(&self) {}
    }
}
enum E { A }
trait T { fn t(&self); }
macro_rules! mac { () => {} }
fn main() {}
";
        assert_eq!(
            names(Lang::Rust, src),
            vec![
                n("m", 0, 0),
                n("Point", 1, 1),
                n("impl std::fmt::Display for Point", 2, 1),
                n("fmt", 3, 2),
                n("E", 6, 0),
                n("T", 7, 0),
                n("t", 7, 1),
                n("mac", 8, 0),
                n("main", 9, 0),
            ]
        );
        let kinds: Vec<SymbolKind> = Highlighter::new()
            .symbols(Lang::Rust, src.as_bytes(), None)
            .unwrap()
            .iter()
            .map(|s: &Symbol| s.kind)
            .collect();
        assert_eq!(kinds[0], SymbolKind::Module);
        assert_eq!(kinds[1], SymbolKind::Type);
        assert_eq!(kinds[3], SymbolKind::Function);
    }

    #[test]
    fn python_classes_and_methods() {
        let src = "class A:\n    def f(self):\n        pass\n\n@dec\ndef g():\n    pass\n";
        assert_eq!(
            names(Lang::Python, src),
            vec![n("A", 0, 0), n("f", 1, 1), n("g", 5, 0)]
        );
    }

    #[test]
    fn javascript_and_typescript() {
        let js = "function a() {}\nclass B {\n  m() {}\n}\nconst c = () => 1;\nconst d = 5;\n";
        assert_eq!(
            names(Lang::JavaScript, js),
            vec![n("a", 0, 0), n("B", 1, 0), n("m", 2, 1), n("c", 4, 0)]
        );
        let ts = "interface I { x: number }\ntype U = string;\nenum E { A }\nnamespace N {\n  export function f(): void {}\n}\n";
        assert_eq!(
            names(Lang::TypeScript, ts),
            vec![
                n("I", 0, 0),
                n("U", 1, 0),
                n("E", 2, 0),
                n("N", 3, 0),
                n("f", 4, 1)
            ]
        );
        let tsx = "function App() { return <div/>; }\n";
        assert_eq!(names(Lang::Tsx, tsx), vec![n("App", 0, 0)]);
    }

    #[test]
    fn c_and_cpp_declarators() {
        let c = "struct S { int x; };\nstruct S s;\nstatic int *make(void) { return 0; }\nint main(void) { return 0; }\n";
        assert_eq!(
            names(Lang::C, c),
            vec![n("S", 0, 0), n("make", 2, 0), n("main", 3, 0)]
        );
        let cpp = "namespace ns {\nclass K {\n};\nvoid K::run() {}\n}\n";
        assert_eq!(
            names(Lang::Cpp, cpp),
            vec![n("ns", 0, 0), n("K", 1, 1), n("K::run", 3, 1)]
        );
    }

    #[test]
    fn go_java_csharp() {
        let go = "package p\ntype T struct{}\nfunc (t T) M() {}\nfunc F() {}\n";
        assert_eq!(
            names(Lang::Go, go),
            vec![n("T", 1, 0), n("M", 2, 0), n("F", 3, 0)]
        );
        let java = "class A {\n  A() {}\n  void m() {}\n}\ninterface I {}\n";
        assert_eq!(
            names(Lang::Java, java),
            vec![n("A", 0, 0), n("A", 1, 1), n("m", 2, 1), n("I", 4, 0)]
        );
        let cs = "namespace N.M {\n  class C {\n    void Run() {}\n  }\n}\n";
        assert_eq!(
            names(Lang::CSharp, cs),
            vec![n("N.M", 0, 0), n("C", 1, 1), n("Run", 2, 2)]
        );
    }

    #[test]
    fn ruby_php_lua_bash_powershell() {
        let rb = "module M\n  class C\n    def run; end\n  end\nend\n";
        assert_eq!(
            names(Lang::Ruby, rb),
            vec![n("M", 0, 0), n("C", 1, 1), n("run", 2, 2)]
        );
        let php = "<?php\nclass A {\n  function m() {}\n}\nfunction f() {}\n";
        assert_eq!(
            names(Lang::Php, php),
            vec![n("A", 1, 0), n("m", 2, 1), n("f", 4, 0)]
        );
        let lua = "function M.f(x)\nend\nlocal function g() end\n";
        assert_eq!(names(Lang::Lua, lua), vec![n("M.f", 0, 0), n("g", 2, 0)]);
        let sh = "build() {\n  make\n}\nfunction test { :; }\n";
        assert_eq!(
            names(Lang::Bash, sh),
            vec![n("build", 0, 0), n("test", 3, 0)]
        );
        let ps =
            "function Get-Thing {\n  param($a)\n}\nclass Box {\n  [int] Size() { return 1 }\n}\n";
        assert_eq!(
            names(Lang::PowerShell, ps),
            vec![n("Get-Thing", 0, 0), n("Box", 3, 0), n("Size", 4, 1)]
        );
    }

    #[test]
    fn data_languages_have_none_and_odd_input_survives() {
        assert!(names(Lang::Json, "{\"a\": 1}").is_empty());
        let mut h = Highlighter::new();
        for lang in Lang::ALL {
            for src in ["", "fn (((\n\"unterminated", "ä🙂\u{0}\n\u{FEFF}"] {
                assert!(h.symbols(lang, src.as_bytes(), None).is_ok(), "{lang:?}");
            }
        }
    }
}
