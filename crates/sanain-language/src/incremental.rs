//! Incremental syntax analysis: the syntax tree, highlight spans and
//! folds of one document, updated after an edit instead of recomputed.
//!
//! The previous [`Syntax`] keeps its text and tree. A new snapshot is
//! compared with the old text (common prefix and suffix), which gives one
//! [`InputEdit`] covering every change; the old tree is edited and parsed
//! again, reusing the unchanged parts. No edit history is needed, so the
//! result cannot drift from the text whatever happened in between (undo,
//! reload, several edits).
//!
//! Highlights of languages without injections come from running the
//! highlight query over the tree (the same algorithm as
//! `tree_sitter_highlight` for a single layer), and after an edit only
//! over the region that may have changed: the edit and the ranges whose
//! structure changed, widened to the enclosing syntax nodes and whole
//! lines. Spans outside it are kept and shifted. Languages with
//! injections (HTML, JavaScript, Markdown, PHP) are highlighted from
//! scratch by `tree_sitter_highlight`; their folds still use the
//! incremental tree. Folds are collected from the whole tree.
//!
//! An incremental result is exact for the tree it has (tested for trees
//! without syntax errors; inside error nodes Tree-sitter's changed ranges
//! can miss a change). The tree itself can differ from a fresh parse in
//! broken or ambiguous code, so [`Syntax::is_fresh`] tells callers when
//! to analyse again from scratch to get the canonical result.

use std::collections::HashSet;
use std::ops::{ControlFlow, Range};
use std::sync::atomic::{AtomicUsize, Ordering};

use tree_sitter::{
    InputEdit, Node, ParseOptions, Parser, Point, Query, QueryCursor, StreamingIterator, Tree,
};

use crate::{HighlightError, Highlighter, Lang, NAMES, STYLES, Span, Style, folds};

/// How many ancestors of an edited node are highlighted again with it, as
/// a margin: highlight patterns also look at a node's parent (a
/// function's name, a key in a pair). Changes of the parent itself show
/// up in Tree-sitter's changed ranges.
const CONTEXT: usize = 1;

/// When the region to highlight again exceeds this share of the document
/// (1/n), the whole document is highlighted instead.
const FULL_FRACTION: usize = 2;

/// Check the cancellation flag every this many query captures.
const CANCEL_INTERVAL: usize = 100;

/// Syntax state of one document: its text, tree, highlight spans and
/// folds. Pass it back to [`Highlighter::analyze`] with the next snapshot
/// to update it incrementally.
pub struct Syntax {
    lang: Lang,
    text: String,
    tree: Tree,
    spans: Vec<Span>,
    folds: Vec<Range<usize>>,
    rehighlighted: Range<usize>,
    /// Parsed by reusing an earlier tree.
    incremental: bool,
}

impl std::fmt::Debug for Syntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Syntax")
            .field("lang", &self.lang)
            .field("len", &self.text.len())
            .field("spans", &self.spans.len())
            .field("rehighlighted", &self.rehighlighted)
            .finish_non_exhaustive()
    }
}

impl Syntax {
    pub fn lang(&self) -> Lang {
        self.lang
    }

    /// The analysed text.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Highlight spans, as from [`Highlighter::highlight`].
    pub fn spans(&self) -> &[Span] {
        &self.spans
    }

    /// Fold ranges, as from [`Highlighter::folds`].
    pub fn folds(&self) -> &[Range<usize>] {
        &self.folds
    }

    /// The byte range whose highlights were computed by the last update
    /// (the whole text after a full analysis; empty when the text did not
    /// change).
    pub fn rehighlighted(&self) -> Range<usize> {
        self.rehighlighted.clone()
    }

    /// Whether the tree was parsed from scratch. An incrementally updated
    /// tree is a valid parse of the text but not always the same one: in
    /// ambiguous or broken code, Tree-sitter can reuse old subtrees where
    /// a fresh parse would recover or decide differently. Callers that
    /// need the canonical result analyse the text again without `prev`
    /// (the syntax worker does when typing pauses).
    pub fn is_fresh(&self) -> bool {
        !self.incremental
    }
}

/// One edit turning `old` into `new`: bytes `start..old_end` of `old`
/// became `start..new_end` of `new`. `None` when the texts are equal.
fn diff(old: &str, new: &str) -> Option<(usize, usize, usize)> {
    let (a, b) = (old.as_bytes(), new.as_bytes());
    let max = a.len().min(b.len());
    // Compare in blocks first (memcmp), then byte by byte.
    const BLOCK: usize = 256;
    let mut p = 0;
    while p + BLOCK <= max && a[p..p + BLOCK] == b[p..p + BLOCK] {
        p += BLOCK;
    }
    while p < max && a[p] == b[p] {
        p += 1;
    }
    if p == a.len() && p == b.len() {
        return None;
    }
    let rest = max - p;
    let mut s = 0;
    while s + BLOCK <= rest
        && a[a.len() - s - BLOCK..a.len() - s] == b[b.len() - s - BLOCK..b.len() - s]
    {
        s += BLOCK;
    }
    while s < rest && a[a.len() - s - 1] == b[b.len() - s - 1] {
        s += 1;
    }
    // Keep the edit on character boundaries; the shared prefix and
    // suffix have the same boundaries in both texts.
    while !new.is_char_boundary(p) {
        p -= 1;
    }
    let (mut old_end, mut new_end) = (a.len() - s, b.len() - s);
    while !new.is_char_boundary(new_end) || !old.is_char_boundary(old_end) {
        old_end += 1;
        new_end += 1;
    }
    Some((p, old_end, new_end))
}

fn count_lines(bytes: &[u8]) -> usize {
    bytes.iter().filter(|&&b| b == b'\n').count()
}

/// The point at byte `at` of `text`, given the point at byte `from`.
fn advance(text: &[u8], from: usize, at: Point, to: usize) -> Point {
    let part = &text[from..to];
    match part.iter().rposition(|&b| b == b'\n') {
        Some(i) => Point::new(at.row + count_lines(part), part.len() - i - 1),
        None => Point::new(at.row, at.column + part.len()),
    }
}

fn input_edit(old: &str, new: &str, (start, old_end, new_end): (usize, usize, usize)) -> InputEdit {
    let start_position = advance(new.as_bytes(), 0, Point::new(0, 0), start);
    InputEdit {
        start_byte: start,
        old_end_byte: old_end,
        new_end_byte: new_end,
        start_position,
        old_end_position: advance(old.as_bytes(), start, start_position, old_end),
        new_end_position: advance(new.as_bytes(), start, start_position, new_end),
    }
}

/// Parse `source`, reusing `old` (already edited) when given. Cancellable.
pub(crate) fn parse(
    parser: &mut Parser,
    language: &tree_sitter::Language,
    source: &[u8],
    old: Option<&Tree>,
    cancel: Option<&AtomicUsize>,
) -> Result<Tree, HighlightError> {
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
        old,
        Some(ParseOptions::new().progress_callback(&mut progress)),
    );
    // The parser keeps partial state after an interruption.
    parser.reset();
    match tree {
        Some(tree) => Ok(tree),
        None if cancelled => Err(HighlightError::Cancelled),
        None => Err(HighlightError::Failed("parsing failed".into())),
    }
}

/// Styles for a query's capture names, chosen like
/// `HighlightConfiguration::configure` does for [`NAMES`]: the longest
/// recognised name whose every part is among the capture's parts.
pub(crate) fn style_map(capture_names: &[&str]) -> Vec<Option<Style>> {
    capture_names
        .iter()
        .map(|capture| {
            let parts: Vec<&str> = capture.split('.').collect();
            let mut best: Option<usize> = None;
            let mut best_len = 0;
            for (i, name) in NAMES.iter().enumerate() {
                let mut len = 0;
                let mut all = true;
                for part in name.split('.') {
                    len += 1;
                    if !parts.contains(&part) {
                        all = false;
                        break;
                    }
                }
                if all && len > best_len {
                    best = Some(i);
                    best_len = len;
                }
            }
            best.and_then(|i| STYLES.get(i).copied())
        })
        .collect()
}

/// Spans of `range` of `source` from the highlight query, with the
/// semantics of `tree_sitter_highlight` for one layer without locals or
/// injections: captures in document order, the last pattern capturing a
/// node wins (earlier matches of that node are dropped), and nested
/// highlights override the enclosing one until they end. Also returns the
/// extent of the highlighted nodes that intersect `range`.
fn query_spans(
    cursor: &mut QueryCursor,
    query: &Query,
    styles: &[Option<Style>],
    root: Node<'_>,
    source: &[u8],
    range: Range<usize>,
    cancel: Option<&AtomicUsize>,
) -> Result<(Vec<Span>, Range<usize>), HighlightError> {
    struct Out {
        spans: Vec<Span>,
        pos: usize,
        end: usize,
    }
    impl Out {
        /// Style the source from the current position up to `to`.
        fn emit(&mut self, to: usize, style: Option<Style>) {
            let to = to.min(self.end);
            if to <= self.pos {
                return;
            }
            if let Some(style) = style {
                match self.spans.last_mut() {
                    Some(last) if last.style == style && last.range.end == self.pos => {
                        last.range.end = to;
                    }
                    _ => self.spans.push(Span {
                        range: self.pos..to,
                        style,
                    }),
                }
            }
            self.pos = to;
        }
    }

    cursor.set_byte_range(range.clone());
    let mut captures = cursor.captures(query, root, source);
    let mut out = Out {
        spans: Vec::new(),
        pos: range.start,
        end: range.end,
    };
    let mut extent = range.clone();
    // Highlights in effect: where each ends, innermost last.
    let mut stack: Vec<(usize, Style)> = Vec::new();
    let mut removed: HashSet<u32> = HashSet::new();
    // The node being captured and the capture that currently wins for it.
    let mut group: Option<(Node<'_>, u32)> = None;
    let mut group_match = 0u32;
    let finish = |node: Node<'_>,
                  capture: u32,
                  out: &mut Out,
                  stack: &mut Vec<(usize, Style)>,
                  extent: &mut Range<usize>| {
        let (start, end) = (node.start_byte(), node.end_byte());
        while let Some(&(e, style)) = stack.last()
            && e <= start
        {
            out.emit(e, Some(style));
            stack.pop();
        }
        if let Some(style) = styles.get(capture as usize).copied().flatten() {
            out.emit(start, stack.last().map(|s| s.1));
            stack.push((end, style));
            extent.start = extent.start.min(start);
            extent.end = extent.end.max(end);
        }
    };
    let mut count = 0usize;
    while let Some((m, index)) = captures.next() {
        count += 1;
        if count.is_multiple_of(CANCEL_INTERVAL)
            && cancel.is_some_and(|c| c.load(Ordering::Relaxed) != 0)
        {
            return Err(HighlightError::Cancelled);
        }
        if removed.contains(&m.id()) {
            continue;
        }
        let Some(capture) = m.captures().get(*index) else {
            continue;
        };
        match group {
            Some((node, _)) if node == capture.node => {
                // A later pattern for the same node replaces the earlier
                // one, whose match is dropped.
                removed.insert(group_match);
            }
            Some((node, c)) => finish(node, c, &mut out, &mut stack, &mut extent),
            None => {}
        }
        group = Some((capture.node, capture.index));
        group_match = m.id();
    }
    if let Some((node, c)) = group {
        finish(node, c, &mut out, &mut stack, &mut extent);
    }
    while let Some((e, style)) = stack.pop() {
        out.emit(e, Some(style));
    }
    Ok((out.spans, extent))
}

/// Line-aligned `range` of `text`: from the start of its first line
/// through the end of its last (including the line break).
fn whole_lines(text: &[u8], range: Range<usize>) -> Range<usize> {
    let start = text[..range.start]
        .iter()
        .rposition(|&b| b == b'\n')
        .map_or(0, |i| i + 1);
    let end = text[range.end..]
        .iter()
        .position(|&b| b == b'\n')
        .map_or(text.len(), |i| range.end + i + 1);
    start..end
}

/// The region to highlight again after `edit`: the edited text and the
/// ranges whose structure changed, each widened to [`CONTEXT`] ancestors
/// of the node there and then to whole lines.
fn dirty(old: &Tree, new: &Tree, text: &[u8], edit: &InputEdit) -> Range<usize> {
    let root = new.root_node();
    let spots = std::iter::once(edit.start_byte..edit.new_end_byte)
        .chain(old.changed_ranges(new).map(|r| r.start_byte..r.end_byte));
    let mut region = edit.start_byte..edit.new_end_byte;
    for spot in spots {
        let end = spot.end.min(text.len());
        let start = spot.start.min(end);
        let mut node = root.descendant_for_byte_range(start, end).unwrap_or(root);
        for _ in 0..CONTEXT {
            match node.parent() {
                Some(p) if p != root => node = p,
                _ => break,
            }
        }
        region.start = region.start.min(start).min(node.start_byte());
        region.end = region.end.max(end).max(node.end_byte().min(text.len()));
    }
    whole_lines(text, region)
}

/// `old` spans (of the text before the edit) outside `region` (of the
/// text after it), shifted past the edit, with `fresh` spans for
/// `region` in between. Adjacent spans of one style are joined, as a
/// full highlight would produce them.
fn splice(old: &[Span], fresh: Vec<Span>, region: &Range<usize>, edit: &InputEdit) -> Vec<Span> {
    let delta = edit.new_end_byte as isize - edit.old_end_byte as isize;
    // The region in old coordinates; it contains the whole edit.
    let old_end = (region.end as isize - delta) as usize;
    let mut out: Vec<Span> = Vec::with_capacity(old.len() + fresh.len());
    let push = |out: &mut Vec<Span>, s: Span| {
        if s.range.is_empty() {
            return;
        }
        match out.last_mut() {
            Some(last) if last.style == s.style && last.range.end == s.range.start => {
                last.range.end = s.range.end;
            }
            _ => out.push(s),
        }
    };
    let before = old.partition_point(|s| s.range.start < region.start);
    for s in &old[..before] {
        push(
            &mut out,
            Span {
                range: s.range.start..s.range.end.min(region.start),
                style: s.style,
            },
        );
    }
    for s in fresh {
        push(&mut out, s);
    }
    let after = old.partition_point(|s| s.range.end <= old_end);
    for s in &old[after..] {
        let start = s.range.start.max(old_end);
        push(
            &mut out,
            Span {
                range: (start as isize + delta) as usize..(s.range.end as isize + delta) as usize,
                style: s.style,
            },
        );
    }
    out
}

impl Highlighter {
    /// Highlight and fold `text`. With `prev`, the analysis of an earlier
    /// version of the same document, the tree is updated incrementally
    /// and, for languages without injections, only the region that may
    /// have changed is highlighted again. Without `prev` the result
    /// equals [`Self::highlight`] and [`Self::folds`]; with it, see the
    /// module documentation and [`Syntax::is_fresh`]. Cancellable like
    /// [`Self::highlight`]; `prev` is left untouched either way.
    pub fn analyze(
        &mut self,
        lang: Lang,
        text: String,
        prev: Option<&Syntax>,
        cancel: Option<&AtomicUsize>,
    ) -> Result<Syntax, HighlightError> {
        let Some(config) = self.config(lang) else {
            return Err(HighlightError::Failed(format!(
                "{} query did not compile",
                lang.name()
            )));
        };
        let language = config.language.clone();
        let prev = prev.filter(|p| p.lang == lang);
        let change = match prev {
            Some(p) => match diff(&p.text, &text) {
                Some(d) => Some((p, input_edit(&p.text, &text, d))),
                None => {
                    return Ok(Syntax {
                        lang,
                        text,
                        tree: p.tree.clone(),
                        spans: p.spans.clone(),
                        folds: p.folds.clone(),
                        rehighlighted: 0..0,
                        incremental: p.incremental,
                    });
                }
            },
            None => None,
        };
        let source = text.as_bytes();
        let (tree, old_tree) = match &change {
            Some((p, edit)) => {
                let mut old = p.tree.clone();
                old.edit(edit);
                let tree = parse(&mut self.parser, &language, source, Some(&old), cancel)?;
                (tree, Some(old))
            }
            None => (
                parse(&mut self.parser, &language, source, None, cancel)?,
                None,
            ),
        };
        let incremental = change.is_some();
        let folds = folds::collect(lang, tree.root_node(), source);
        if !lang.injections_query().is_empty() {
            let spans = self.highlight(lang, source, cancel)?;
            return Ok(Syntax {
                lang,
                tree,
                spans,
                folds,
                rehighlighted: 0..text.len(),
                incremental,
                text,
            });
        }
        let Some(query) = self
            .configs
            .get(&lang)
            .and_then(Option::as_ref)
            .map(|c| &c.query)
        else {
            return Ok(Syntax {
                lang,
                tree,
                spans: Vec::new(),
                folds,
                rehighlighted: 0..text.len(),
                incremental,
                text,
            });
        };
        let styles = self
            .styles
            .entry(lang)
            .or_insert_with(|| style_map(query.capture_names()));
        let root = tree.root_node();
        let full = 0..source.len();
        let (spans, rehighlighted) = match (&change, &old_tree) {
            (Some((p, edit)), Some(old)) => {
                let mut region = dirty(old, &tree, source, edit);
                loop {
                    if region.len() * FULL_FRACTION >= source.len() {
                        let (spans, _) = query_spans(
                            &mut self.cursor,
                            query,
                            styles,
                            root,
                            source,
                            full.clone(),
                            cancel,
                        )?;
                        break (spans, full);
                    }
                    let (fresh, extent) = query_spans(
                        &mut self.cursor,
                        query,
                        styles,
                        root,
                        source,
                        region.clone(),
                        cancel,
                    )?;
                    // A highlighted node reaching outside the region may
                    // have changed outside it too: widen and repeat.
                    if extent.start < region.start || extent.end > region.end {
                        region = whole_lines(
                            source,
                            region.start.min(extent.start)
                                ..region.end.max(extent.end).min(source.len()),
                        );
                        continue;
                    }
                    break (splice(&p.spans, fresh, &region, edit), region);
                }
            }
            _ => {
                let (spans, _) = query_spans(
                    &mut self.cursor,
                    query,
                    styles,
                    root,
                    source,
                    full.clone(),
                    cancel,
                )?;
                (spans, full)
            }
        };
        Ok(Syntax {
            lang,
            text,
            tree,
            spans,
            folds,
            rehighlighted,
            incremental,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Deterministic pseudo-random numbers (xorshift).
    struct Rng(u64);
    impl Rng {
        fn next(&mut self) -> u64 {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            self.0
        }
        fn below(&mut self, n: usize) -> usize {
            (self.next() % n.max(1) as u64) as usize
        }
    }

    fn sample(lang: Lang) -> String {
        let unit = match lang {
            Lang::Rust => {
                "/// Doc {i}.\n#[derive(Debug)]\npub struct S{i} { a: u32, b: String }\n\nimpl S{i} {\n    fn f(&self, x: &str) -> Option<u32> {\n        // note\n        let v = vec![1, 2, 3];\n        if x == \"q{i}\" { return Some(self.a + 0x1f); }\n        println!(\"{}\", v.len()); /* block */ None\n    }\n}\n\n"
            }
            Lang::Json => {
                "  \"k{i}\": {\"a\": [1, 2.5, true, null], \"s\": \"text \\n {i}\", \"o\": {\"x\": false}},\n"
            }
            Lang::Python => {
                "class C{i}(Base):\n    \"\"\"Doc.\"\"\"\n    def f(self, x=1):\n        # c\n        if x > 2:\n            return f\"v{x}\"\n        return [i for i in range(3)]\n\n"
            }
            Lang::C => {
                "#include <stdio.h>\n/* c */\nstatic int f{i}(int x) {\n    if (x > 1) { return x * 2; }\n    printf(\"%d\\n\", x); // n\n    return 0;\n}\n\n"
            }
            Lang::Yaml => {
                "key{i}:\n  name: \"v\" # c\n  list:\n    - 1\n    - true\n  other: text\n"
            }
            Lang::Toml => {
                "[table{i}]\nkey = \"v\" # c\nnum = 1_000\narr = [1, 2]\ndate = 1979-05-27\n\n"
            }
            Lang::Css => {
                "/* c */\n.a{i} > b:hover { color: #fff; margin: 0 1px !important; }\n@media (max-width: 10px) { .x { top: 0; } }\n"
            }
            Lang::Go => {
                "// c\nfunc F{i}(x int) (string, error) {\n\tif x > 1 {\n\t\treturn \"s\", nil\n\t}\n\treturn fmt.Sprintf(\"%d\", x), nil\n}\n\n"
            }
            Lang::Bash => {
                "# c\nf{i}() {\n  local x=\"$1\"\n  if [ -n \"$x\" ]; then echo \"hi $x\"; fi\n}\n\n"
            }
            Lang::Lua => {
                "-- c\nlocal function f{i}(x)\n  if x then\n    return { a = 1, 'b' }\n  end\nend\n\n"
            }
            Lang::TypeScript => {
                "// c\nexport class A{i} implements B {\n  private x: number = 1;\n  f(y: string): string { return `t${y}`; }\n}\n\n"
            }
            Lang::Html => {
                "<div class=\"a{i}\">\n  <p>t &amp; u</p>\n  <script>const n = {i};</script>\n  <style>a { color: red; }</style>\n</div>\n"
            }
            Lang::Php => "<p>{i}</p>\n<?php\nfunction f{i}($a) {\n  return 'x' . $a; // c\n}\n?>\n",
            Lang::Markdown => {
                "# Title {i}\n\nSome *text* and `code`.\n\n```rust\nfn f() {}\n```\n\n- item\n\n"
            }
            _ => "x{i}\n",
        };
        let mut s = String::new();
        if lang == Lang::Json {
            s.push_str("{\n");
        }
        for i in 0..30 {
            s.push_str(&unit.replace("{i}", &i.to_string()));
        }
        if lang == Lang::Json {
            s.push_str("  \"end\": 0\n}\n");
        }
        s
    }

    const TOKENS: [&str; 30] = [
        "x", " ", "\n", "{", "}", "(", ")", "[", "]", "\"", "'", "`", "/*", "*/", "//", "#", "<",
        ">", "</p>", ":", ",", ";", "fn ", "if ", "= ", "-", "ä", "🙂", "\n    ", "end",
    ];

    fn char_boundary(text: &str, at: usize) -> usize {
        let mut at = at.min(text.len());
        while !text.is_char_boundary(at) {
            at -= 1;
        }
        at
    }

    /// Byte offsets where lines start.
    fn line_starts(text: &str) -> Vec<usize> {
        std::iter::once(0)
            .chain(text.match_indices('\n').map(|(i, _)| i + 1))
            .filter(|&i| i < text.len())
            .collect()
    }

    fn mutate(rng: &mut Rng, lang: Lang, text: &mut String) {
        let at = char_boundary(text, rng.below(text.len() + 1));
        let lines = line_starts(text);
        let line = |i: usize| {
            let start = lines.get(i).copied().unwrap_or(text.len());
            let end = lines.get(i + 1).copied().unwrap_or(text.len());
            start..end
        };
        match rng.below(36) {
            0 => {
                // Paste a copy of some text elsewhere.
                let from = char_boundary(text, rng.below(text.len() + 1));
                let to = char_boundary(text, from + rng.below(300));
                let chunk = text[from..to].to_owned();
                text.insert_str(at, &chunk);
            }
            1 => {
                // Delete a larger block.
                let to = char_boundary(text, at + rng.below(400));
                text.replace_range(at..to, "");
            }
            2..=7 => {
                let to = char_boundary(text, at + 1 + rng.below(4));
                text.replace_range(at..to, "");
            }
            8..=9 => {
                let to = char_boundary(text, at + 1 + rng.below(6));
                text.replace_range(at..to, TOKENS[rng.below(TOKENS.len())]);
            }
            10..=11 => {
                // Duplicate a line somewhere else.
                let copy = text[line(rng.below(lines.len()))].to_owned();
                let to = line(rng.below(lines.len())).start;
                text.insert_str(to, &copy);
            }
            12..=13 => {
                let r = line(rng.below(lines.len()));
                text.replace_range(r, "");
            }
            14..=15 => {
                // Comment a line out, or back in.
                let r = line(rng.below(lines.len()));
                let prefix = lang.line_comment().unwrap_or("#");
                let body = text[r.clone()].to_owned();
                let indent = body.len() - body.trim_start().len();
                if body.trim_start().starts_with(prefix) {
                    text.replace_range(r.start + indent..r.start + indent + prefix.len(), "");
                } else {
                    text.insert_str(r.start + indent, prefix);
                }
            }
            16..=25 => {
                // Type inside a word, as most typing does.
                let word = text[at..]
                    .find(|c: char| c.is_alphanumeric())
                    .map_or(at, |i| at + i);
                text.insert(word, if rng.below(2) == 0 { 'z' } else { '7' });
            }
            _ => text.insert_str(at, TOKENS[rng.below(TOKENS.len())]),
        }
    }

    const LANGS: [Lang; 14] = [
        Lang::Rust,
        Lang::Json,
        Lang::Python,
        Lang::C,
        Lang::Yaml,
        Lang::Toml,
        Lang::Css,
        Lang::Go,
        Lang::Bash,
        Lang::Lua,
        Lang::TypeScript,
        // With injections: highlighted in full, folds incremental.
        Lang::Html,
        Lang::Php,
        Lang::Markdown,
    ];

    /// Apply `steps` random edits per seed and language, analysing each
    /// version incrementally, and check after each:
    ///
    /// - the spans equal a full highlight query over the same tree, so
    ///   partial highlighting and splicing lose nothing (asserted for
    ///   trees without syntax errors, where Tree-sitter's changed ranges
    ///   are complete);
    /// - when the tree equals a fresh parse, spans and folds equal a fresh
    ///   [`Highlighter::highlight`] and [`Highlighter::folds`].
    ///
    /// Otherwise the chain sometimes goes on from the incremental result
    /// (more typing) and sometimes from a fresh analysis (the worker
    /// settling it). Every few steps the text goes back to its last
    /// version without syntax errors, as when a typo is fixed. Returns
    /// per language the number of partial updates and of updates whose
    /// tree differed from a fresh parse.
    fn check_random_edits(seeds: u64, steps: usize) -> Vec<(Lang, usize, usize)> {
        let mut h = Highlighter::new();
        let mut fresh = Highlighter::new();
        let mut parser = Parser::new();
        let mut counts = Vec::new();
        for (n, lang) in LANGS.into_iter().enumerate() {
            let (mut partial, mut diverged) = (0, 0);
            let language = h.config(lang).unwrap().language.clone();
            let query = h.config(lang).unwrap().query.capture_names().len();
            assert!(query > 0);
            for seed in 0..seeds {
                let mut rng = Rng(0x9e37_79b9_7f4a_7c15 ^ ((n as u64 + 1) * 0x1000_0001 + seed));
                let mut text = sample(lang);
                let mut good = text.clone();
                let mut syn = h.analyze(lang, text.clone(), None, None).unwrap();
                for step in 0..steps {
                    if rng.below(2) == 0 {
                        text.clone_from(&good);
                    }
                    mutate(&mut rng, lang, &mut text);
                    syn = h.analyze(lang, text.clone(), Some(&syn), None).unwrap();
                    let at = format!("{lang:?} seed {seed} step {step}");
                    let errors = syn.tree.root_node().has_error();
                    if lang.injections_query().is_empty() && !errors {
                        let config = h.configs[&lang].as_ref().unwrap();
                        let styles = style_map(config.query.capture_names());
                        let (full, _) = query_spans(
                            &mut QueryCursor::new(),
                            &config.query,
                            &styles,
                            syn.tree.root_node(),
                            text.as_bytes(),
                            0..text.len(),
                            None,
                        )
                        .unwrap();
                        assert!(
                            syn.spans() == full.as_slice(),
                            "{at}: spans differ from a full query of the tree\n{}",
                            first_difference(syn.spans(), &full, &text)
                        );
                    }
                    let tree = parse(&mut parser, &language, text.as_bytes(), None, None).unwrap();
                    if errors {
                        // Best effort until settled: Tree-sitter's changed
                        // ranges can miss changes inside error nodes.
                        if rng.below(2) == 0 {
                            syn = h.analyze(lang, text.clone(), None, None).unwrap();
                        }
                    } else if tree.root_node().to_sexp() == syn.tree.root_node().to_sexp() {
                        let spans = fresh.highlight(lang, text.as_bytes(), None).unwrap();
                        let folds = fresh.folds(lang, text.as_bytes(), None).unwrap();
                        assert!(
                            syn.spans() == spans.as_slice(),
                            "{at}: spans differ from fresh\n{}",
                            first_difference(syn.spans(), &spans, &text)
                        );
                        assert_eq!(syn.folds(), folds.as_slice(), "{at}: folds");
                        if syn.rehighlighted().len() < text.len() {
                            partial += 1;
                        }
                    } else {
                        diverged += 1;
                        if rng.below(2) == 0 {
                            syn = h.analyze(lang, text.clone(), None, None).unwrap();
                        }
                    }
                    if !errors {
                        good.clone_from(&text);
                    }
                }
            }
            counts.push((lang, partial, diverged));
        }
        counts
    }

    #[test]
    fn incremental_equals_fresh_after_random_edits() {
        for (lang, partial, _) in check_random_edits(2, 60) {
            // Not vacuous: many updates of these were partial.
            if lang.injections_query().is_empty() {
                assert!(partial >= 5, "{lang:?}: only {partial} partial updates");
            }
        }
    }

    /// The same with many more edits: `cargo test --release -p
    /// sanain-language -- --ignored stress --nocapture`.
    #[test]
    #[ignore = "slow; run in release"]
    fn stress_incremental_equals_fresh() {
        for (lang, partial, diverged) in check_random_edits(20, 300) {
            eprintln!("{lang:?}: {partial} partial updates, {diverged} trees unlike a fresh parse");
        }
    }

    fn first_difference(a: &[Span], b: &[Span], text: &str) -> String {
        let i = a
            .iter()
            .zip(b)
            .position(|(x, y)| x != y)
            .unwrap_or(a.len().min(b.len()));
        let show = |s: Option<&Span>| {
            s.map(|s| {
                format!(
                    "{:?} {:?} {:?}",
                    s.range,
                    s.style,
                    text.get(s.range.clone())
                )
            })
        };
        format!(
            "incremental {:?}\nfresh       {:?}",
            show(a.get(i)),
            show(b.get(i))
        )
    }

    #[test]
    fn fresh_analysis_matches_tree_sitter_highlight_for_every_language() {
        let mut h = Highlighter::new();
        for lang in Lang::ALL {
            let text = sample(lang);
            let syn = h.analyze(lang, text.clone(), None, None).unwrap();
            let spans = Highlighter::new()
                .highlight(lang, text.as_bytes(), None)
                .unwrap();
            assert_eq!(syn.spans(), spans.as_slice(), "{lang:?}");
        }
    }

    #[test]
    fn small_edit_rehighlights_only_nearby_lines() {
        let mut h = Highlighter::new();
        let mut text = sample(Lang::Rust);
        let syn = h.analyze(Lang::Rust, text.clone(), None, None).unwrap();
        let at = text.find("q5").unwrap();
        text.insert(at, 'z');
        let next = h
            .analyze(Lang::Rust, text.clone(), Some(&syn), None)
            .unwrap();
        let r = next.rehighlighted();
        assert!(r.contains(&at) && r.len() < 400, "{r:?}");
        // Unchanged text: nothing to do.
        let same = h
            .analyze(Lang::Rust, text.clone(), Some(&next), None)
            .unwrap();
        assert_eq!(same.rehighlighted(), 0..0);
        assert_eq!(same.spans(), next.spans());
        // Another language: analysed from scratch.
        let other = h.analyze(Lang::C, text.clone(), Some(&next), None).unwrap();
        assert_eq!(other.rehighlighted(), 0..text.len());
    }

    #[test]
    fn cancelled_update_leaves_previous_state_usable() {
        let mut h = Highlighter::new();
        let text = sample(Lang::Rust).repeat(20);
        let syn = h.analyze(Lang::Rust, text.clone(), None, None).unwrap();
        let flag = AtomicUsize::new(1);
        let edited = format!("{text}fn tail() {{}}\n");
        assert_eq!(
            h.analyze(Lang::Rust, edited.clone(), Some(&syn), Some(&flag))
                .unwrap_err(),
            HighlightError::Cancelled
        );
        let next = h
            .analyze(Lang::Rust, edited.clone(), Some(&syn), None)
            .unwrap();
        let spans = h.highlight(Lang::Rust, edited.as_bytes(), None).unwrap();
        assert_eq!(next.spans(), spans.as_slice());
    }

    #[test]
    fn diff_finds_one_edit_on_char_boundaries() {
        assert_eq!(diff("abc", "abc"), None);
        assert_eq!(diff("abc", "abxc"), Some((2, 2, 3)));
        assert_eq!(diff("abc", "ac"), Some((1, 2, 1)));
        assert_eq!(diff("", "x"), Some((0, 0, 1)));
        // "ä" and "ö" share their first byte; the edit covers whole chars.
        assert_eq!(diff("aä", "aö"), Some((1, 3, 3)));
        let long = "x".repeat(1000);
        assert_eq!(
            diff(&format!("{long}a{long}"), &format!("{long}{long}")),
            Some((1000, 1001, 1000))
        );
        let e = input_edit(
            "a\nb\nc",
            "a\nbX\n\nc",
            diff("a\nb\nc", "a\nbX\n\nc").unwrap(),
        );
        assert_eq!(e.start_position, Point::new(1, 1));
        assert_eq!(e.old_end_position, Point::new(1, 1));
        assert_eq!(e.new_end_position, Point::new(2, 0));
    }
}
