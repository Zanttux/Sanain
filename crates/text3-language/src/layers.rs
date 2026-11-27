//! Language layers: a document's syntax tree plus one tree per injected
//! language (JavaScript in `<script>`, the inline grammar and fenced code
//! in Markdown, the HTML around PHP), kept between edits.
//!
//! The layers and their highlights follow `tree_sitter_highlight`: the
//! same injection rules (`injection.combined`, `injection.include-children`,
//! the included ranges of each layer) and the same merging of highlight
//! events from all layers, so inner layers override outer ones within
//! their ranges. Unlike `tree_sitter_highlight`, the layers are kept: after
//! an edit, a layer is parsed again only when the edit or a change of its
//! ranges reaches it, injections are looked for again only where the
//! enclosing layer changed, and the highlights can be computed for one
//! region of the document.

use std::borrow::Cow;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::ops::Range;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use tree_sitter::{
    InputEdit, Node, Point, Query, QueryCursor, Range as TsRange, StreamingIterator, Tree,
};
use tree_sitter_highlight::HighlightConfiguration;

use crate::{HighlightError, Highlighter, Lang, MARKDOWN_INLINE, NAMES, STYLES, Span, Style};

/// How many ancestors of a changed node are highlighted again with it, as
/// a margin: highlight patterns also look at a node's parent (a
/// function's name, a key in a pair). Changes of the parent itself show
/// up in Tree-sitter's changed ranges.
const CONTEXT: usize = 1;

/// Check the cancellation flag every this many query captures or matches.
const CANCEL_INTERVAL: usize = 100;

/// The grammar a layer is parsed with.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Grammar {
    Lang(Lang),
    /// Markdown's inline grammar, injected into its block grammar.
    MdInline,
}

/// A grammar with its highlight configuration and what layered analysis
/// needs from its queries.
pub(crate) struct LangConfig {
    /// The highlight query (injection patterns first, combined ones
    /// disabled), as `tree_sitter_highlight` builds it.
    pub(crate) hc: HighlightConfiguration,
    styles: Vec<Option<Style>>,
    /// Patterns of `hc.query` before this index are injections.
    injection_patterns: usize,
    injections: Option<Injections>,
}

/// The injection query of a grammar.
struct Injections {
    query: Query,
    content: Option<u32>,
    language: Option<u32>,
    patterns: Vec<InjectionPattern>,
}

struct InjectionPattern {
    combined: bool,
    include_children: bool,
    /// The language when no node names it.
    fallback: Fallback,
    /// How far the content node is below the pattern's root node (`None`:
    /// unknown, the whole tree counts as the root).
    depth: Option<usize>,
}

enum Fallback {
    None,
    Name(String),
    /// `injection.self`
    Own,
    /// `injection.parent`
    Parent,
}

impl LangConfig {
    pub(crate) fn load(grammar: Grammar) -> Option<Self> {
        let (language, name, highlights, injections): (_, _, Cow<'static, str>, &str) =
            match grammar {
                Grammar::Lang(lang) => {
                    let (language, query) = lang.grammar();
                    (language, lang.name(), query, lang.injections_query())
                }
                Grammar::MdInline => (
                    tree_sitter_md::INLINE_LANGUAGE.into(),
                    MARKDOWN_INLINE,
                    tree_sitter_md::HIGHLIGHT_QUERY_INLINE.into(),
                    crate::MARKDOWN_INLINE_INJECTIONS.as_str(),
                ),
            };
        let mut hc =
            HighlightConfiguration::new(language.clone(), name, &highlights, injections, "")
                .ok()?;
        hc.configure(&NAMES);
        let styles = style_map(hc.query.capture_names());
        let injection_patterns = (0..hc.query.pattern_count())
            .filter(|&i| hc.query.start_byte_for_pattern(i) < injections.len())
            .count();
        let injections = if injections.trim().is_empty() {
            None
        } else {
            let query = Query::new(&language, injections).ok()?;
            let patterns = (0..query.pattern_count())
                .map(|i| {
                    let source = injections
                        .get(query.start_byte_for_pattern(i)..query.end_byte_for_pattern(i))
                        .unwrap_or_default();
                    let mut p = InjectionPattern {
                        combined: false,
                        include_children: false,
                        fallback: Fallback::None,
                        depth: content_depth(source),
                    };
                    for prop in query.property_settings(i) {
                        let unset = matches!(p.fallback, Fallback::None);
                        match prop.key.as_ref() {
                            "injection.combined" => p.combined = true,
                            "injection.include-children" => p.include_children = true,
                            "injection.language" if unset => {
                                if let Some(v) = &prop.value {
                                    p.fallback = Fallback::Name(v.to_string());
                                }
                            }
                            "injection.self" if unset => p.fallback = Fallback::Own,
                            "injection.parent" if unset => p.fallback = Fallback::Parent,
                            _ => {}
                        }
                    }
                    p
                })
                .collect();
            Some(Injections {
                content: query.capture_index_for_name("injection.content"),
                language: query.capture_index_for_name("injection.language"),
                query,
                patterns,
            })
        };
        Some(Self {
            hc,
            styles,
            injection_patterns,
            injections,
        })
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

/// How many nodes deep the `@injection.content` capture of one query
/// pattern is below the pattern's root node, from the pattern's source
/// (`((inline) @injection.content …)` is 0, `(fenced_code_block …
/// (code_fence_content) @injection.content)` is 1). `None` when the
/// pattern is not understood.
fn content_depth(pattern: &str) -> Option<usize> {
    #[derive(PartialEq)]
    enum Paren {
        Node,
        Group,
        Predicate,
    }
    let b = pattern.as_bytes();
    let mut stack: Vec<Paren> = Vec::new();
    // The depth of a node whose `)` was the previous token.
    let mut closed: Option<usize> = None;
    let mut found = None;
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            b';' => {
                while i < b.len() && b[i] != b'\n' {
                    i += 1;
                }
            }
            b'"' => {
                i += 1;
                while i < b.len() && b[i] != b'"' {
                    if b[i] == b'\\' {
                        i += 1;
                    }
                    i += 1;
                }
                closed = None;
            }
            b'(' => {
                let next = b[i + 1..].iter().find(|c| !c.is_ascii_whitespace());
                stack.push(match next {
                    Some(b'#') => Paren::Predicate,
                    Some(c) if c.is_ascii_alphanumeric() || *c == b'_' => Paren::Node,
                    _ => Paren::Group,
                });
                closed = None;
            }
            b')' => {
                let kind = stack.pop()?;
                let depth = stack.iter().filter(|p| **p == Paren::Node).count();
                closed = (kind == Paren::Node).then_some(depth);
            }
            b'@' => {
                let start = i + 1;
                let mut end = start;
                while end < b.len()
                    && (b[end].is_ascii_alphanumeric() || matches!(b[end], b'.' | b'_' | b'-'))
                {
                    end += 1;
                }
                let in_predicate = stack.contains(&Paren::Predicate);
                if !in_predicate && &pattern[start..end] == "injection.content" {
                    found = Some(closed?);
                }
                i = end;
                continue;
            }
            c if c.is_ascii_whitespace() => {}
            _ => closed = None,
        }
        i += 1;
    }
    found
}

/// A layer: one grammar parsed over some ranges of the document.
#[derive(Clone)]
pub(crate) struct Layer {
    config: Arc<LangConfig>,
    grammar: Grammar,
    /// Included ranges, in document coordinates; the whole document for
    /// the top layer.
    ranges: Vec<TsRange>,
    pub(crate) tree: Tree,
    /// Where `ranges[0]` starts in the tree's coordinates. Layers after an
    /// edit are shifted without editing their trees; a tree is brought up
    /// to date when it is used again.
    anchor: (usize, Point),
    /// Injections that are not combined, sorted by content.
    injections: Vec<Injection>,
    /// One per combined injection pattern.
    combined: Vec<Combined>,
}

#[derive(Clone)]
struct Injection {
    pattern: usize,
    /// The pattern's root node: a change inside it can change the match.
    root: Range<usize>,
    content: Range<usize>,
    layer: Box<Layer>,
}

/// The matches of one combined pattern, parsed together as one layer.
#[derive(Clone)]
struct Combined {
    pattern: usize,
    /// Sorted by content node.
    nodes: Vec<CombinedNode>,
    layer: Option<Box<Layer>>,
}

#[derive(Clone)]
struct CombinedNode {
    root: Range<usize>,
    language: Option<String>,
    node: TsRange,
    /// The node's children, left out of the layer unless the pattern
    /// includes them.
    children: Vec<TsRange>,
}

/// The whole document, as the top layer's range.
const FULL: TsRange = TsRange {
    start_byte: 0,
    end_byte: usize::MAX,
    start_point: Point { row: 0, column: 0 },
    end_point: Point {
        row: usize::MAX,
        column: usize::MAX,
    },
};

fn hull(ranges: &[TsRange]) -> Range<usize> {
    match (ranges.first(), ranges.last()) {
        (Some(a), Some(b)) => a.start_byte..b.end_byte,
        _ => 0..0,
    }
}

/// Whether two ranges overlap or touch.
fn touches(a: &Range<usize>, b: &Range<usize>) -> bool {
    a.start <= b.end && b.start <= a.end
}

/// A position of the text before `e` in the text after it. Positions
/// inside the replaced text go to its end (or, with `start`, its start).
fn map_byte(b: usize, e: &InputEdit, start: bool) -> usize {
    if b == usize::MAX {
        b
    } else if b >= e.old_end_byte {
        b - e.old_end_byte + e.new_end_byte
    } else if b > e.start_byte {
        if start { e.start_byte } else { e.new_end_byte }
    } else {
        b
    }
}

/// `r` of the text before `e` in the text after it, covering whatever
/// replaced the part of `r` that `e` touched.
fn map_range(r: &Range<usize>, e: &InputEdit) -> Range<usize> {
    map_byte(r.start, e, true)..map_byte(r.end, e, false)
}

fn shift_point(b: usize, p: Point, e: &InputEdit) -> (usize, Point) {
    if b == usize::MAX {
        (b, p)
    } else if b >= e.old_end_byte {
        let (old, new) = (e.old_end_position, e.new_end_position);
        let row = (p.row + new.row).saturating_sub(old.row);
        let column = if p.row == old.row {
            (p.column + new.column).saturating_sub(old.column)
        } else {
            p.column
        };
        (b - e.old_end_byte + e.new_end_byte, Point::new(row, column))
    } else if b > e.start_byte {
        (e.new_end_byte, e.new_end_position)
    } else {
        (b, p)
    }
}

fn shift_ts_range(r: &TsRange, e: &InputEdit) -> TsRange {
    let (start_byte, start_point) = shift_point(r.start_byte, r.start_point, e);
    let (end_byte, end_point) = shift_point(r.end_byte, r.end_point, e);
    TsRange {
        start_byte,
        end_byte,
        start_point,
        end_point,
    }
}

/// Shift a layer that `e` does not touch (it is before or after it).
fn shift_layer(layer: &mut Layer, e: &InputEdit) {
    if hull(&layer.ranges).end <= e.start_byte {
        return;
    }
    for r in &mut layer.ranges {
        *r = shift_ts_range(r, e);
    }
    for i in &mut layer.injections {
        shift_injection(i, e);
    }
    for c in &mut layer.combined {
        for n in &mut c.nodes {
            shift_node(n, e);
        }
        if let Some(l) = &mut c.layer {
            shift_layer(l, e);
        }
    }
}

fn shift_injection(i: &mut Injection, e: &InputEdit) {
    if i.root.end <= e.start_byte {
        return;
    }
    i.root = map_range(&i.root, e);
    i.content = map_range(&i.content, e);
    shift_layer(&mut i.layer, e);
}

fn shift_node(n: &mut CombinedNode, e: &InputEdit) {
    if n.root.end <= e.start_byte {
        return;
    }
    n.root = map_range(&n.root, e);
    n.node = shift_ts_range(&n.node, e);
    for c in &mut n.children {
        *c = shift_ts_range(c, e);
    }
}

/// Bring a layer's tree to the coordinates of its ranges.
fn normalize(layer: &mut Layer) {
    let Some(first) = layer.ranges.first() else {
        return;
    };
    let now = (first.start_byte, first.start_point);
    if layer.anchor != now {
        // Everything before the layer was replaced: only the padding
        // before the tree's first token changes.
        layer.tree.edit(&InputEdit {
            start_byte: 0,
            old_end_byte: layer.anchor.0,
            new_end_byte: now.0,
            start_position: Point::new(0, 0),
            old_end_position: layer.anchor.1,
            new_end_position: now.1,
        });
        layer.anchor = now;
    }
}

/// The ranges `nodes` cover within `parent`, leaving out each node's
/// `children` (a port of `tree_sitter_highlight`'s `intersect_ranges`).
fn intersect_ranges(parent: &[TsRange], nodes: &[(TsRange, &[TsRange])]) -> Vec<TsRange> {
    let mut result = Vec::new();
    let mut parents = parent.iter();
    let Some(mut parent_range) = parents.next() else {
        return result;
    };
    for (node, children) in nodes {
        let mut preceding = TsRange {
            start_byte: 0,
            start_point: Point::new(0, 0),
            end_byte: node.start_byte,
            end_point: node.start_point,
        };
        let following = TsRange {
            start_byte: node.end_byte,
            start_point: node.end_point,
            end_byte: usize::MAX,
            end_point: Point::new(usize::MAX, usize::MAX),
        };
        for excluded in children.iter().copied().chain(std::iter::once(following)) {
            let mut range = TsRange {
                start_byte: preceding.end_byte,
                start_point: preceding.end_point,
                end_byte: excluded.start_byte,
                end_point: excluded.start_point,
            };
            preceding = excluded;
            if range.end_byte < parent_range.start_byte {
                continue;
            }
            while parent_range.start_byte <= range.end_byte {
                if parent_range.end_byte > range.start_byte {
                    if range.start_byte < parent_range.start_byte {
                        range.start_byte = parent_range.start_byte;
                        range.start_point = parent_range.start_point;
                    }
                    if parent_range.end_byte < range.end_byte {
                        if range.start_byte < parent_range.end_byte {
                            result.push(TsRange {
                                start_byte: range.start_byte,
                                start_point: range.start_point,
                                end_byte: parent_range.end_byte,
                                end_point: parent_range.end_point,
                            });
                        }
                        range.start_byte = parent_range.end_byte;
                        range.start_point = parent_range.end_point;
                    } else {
                        if range.start_byte < range.end_byte {
                            result.push(range);
                        }
                        break;
                    }
                }
                match parents.next() {
                    Some(next) => parent_range = next,
                    None => return result,
                }
            }
        }
    }
    result
}

/// Where two range lists differ: from the first differing range to the
/// last.
fn ranges_diff(a: &[TsRange], b: &[TsRange]) -> Option<Range<usize>> {
    let first = a.iter().zip(b).position(|(x, y)| x != y);
    let first = match first {
        Some(i) => i,
        None if a.len() == b.len() => return None,
        None => a.len().min(b.len()),
    };
    let tail = a
        .iter()
        .rev()
        .zip(b.iter().rev())
        .take(a.len().min(b.len()) - first)
        .take_while(|(x, y)| x == y)
        .count();
    let (a, b) = (&a[first..a.len() - tail], &b[first..b.len() - tail]);
    let start = a
        .first()
        .map_or(usize::MAX, |r| r.start_byte)
        .min(b.first().map_or(usize::MAX, |r| r.start_byte));
    let end = a
        .last()
        .map_or(0, |r| r.end_byte)
        .max(b.last().map_or(0, |r| r.end_byte));
    (start <= end).then_some(start..end)
}

/// Line-aligned `range` of `text`: from the start of its first line
/// through the end of its last (including the line break).
pub(crate) fn whole_lines(text: &[u8], range: Range<usize>) -> Range<usize> {
    let end = range.end.min(text.len());
    let start = range.start.min(end);
    let start = text[..start]
        .iter()
        .rposition(|&b| b == b'\n')
        .map_or(0, |i| i + 1);
    let end = text[end..]
        .iter()
        .position(|&b| b == b'\n')
        .map_or(text.len(), |i| end + i + 1);
    start..end
}

/// `spot` widened to the node there and [`CONTEXT`] ancestors (not the
/// whole tree).
fn widen(tree: &Tree, spot: Range<usize>, len: usize) -> Range<usize> {
    let root = tree.root_node();
    let end = spot.end.min(len);
    let start = spot.start.min(end);
    let Some(mut node) = root.descendant_for_byte_range(start, end) else {
        return start..end;
    };
    if node == root {
        return start..end;
    }
    for _ in 0..CONTEXT {
        match node.parent() {
            Some(p) if p != root => node = p,
            _ => break,
        }
    }
    start.min(node.start_byte())..end.max(node.end_byte().min(len))
}

fn cancelled(cancel: Option<&AtomicUsize>) -> bool {
    cancel.is_some_and(|c| c.load(Ordering::Relaxed) != 0)
}

/// The ancestor `depth` levels above `node`, or the root when unknown.
fn ancestor(node: Node<'_>, depth: Option<usize>) -> Node<'_> {
    let Some(depth) = depth else {
        let mut n = node;
        while let Some(p) = n.parent() {
            n = p;
        }
        return n;
    };
    let mut n = node;
    for _ in 0..depth {
        match n.parent() {
            Some(p) => n = p,
            None => break,
        }
    }
    n
}

/// A non-combined injection found by the query, before its layer exists.
struct Found {
    pattern: usize,
    root: Range<usize>,
    content: Range<usize>,
    language: String,
    ranges: Vec<TsRange>,
}

/// Building and updating layers for one document version.
pub(crate) struct Ctx<'a> {
    pub(crate) hl: &'a mut Highlighter,
    pub(crate) source: &'a [u8],
    pub(crate) cancel: Option<&'a AtomicUsize>,
    /// The top layer's language name (`injection.parent`).
    pub(crate) host_name: &'static str,
    /// Byte ranges (of the new text) whose highlights may have changed.
    pub(crate) spots: Vec<Range<usize>>,
}

impl Ctx<'_> {
    fn parse(
        &mut self,
        config: &LangConfig,
        ranges: &[TsRange],
        old: Option<&Tree>,
    ) -> Result<Option<Tree>, HighlightError> {
        if cancelled(self.cancel) {
            return Err(HighlightError::Cancelled);
        }
        crate::incremental::parse(
            &mut self.hl.parser,
            &config.hc.language,
            self.source,
            ranges,
            old,
            self.cancel,
        )
    }

    /// Parse a layer and, recursively, everything injected into it.
    /// `None` when Tree-sitter refuses the ranges.
    pub(crate) fn build(
        &mut self,
        config: Arc<LangConfig>,
        grammar: Grammar,
        ranges: Vec<TsRange>,
    ) -> Result<Option<Layer>, HighlightError> {
        let top = ranges.first() == Some(&FULL);
        let Some(tree) = self.parse(&config, if top { &[] } else { &ranges }, None)? else {
            return Ok(None);
        };
        let combined = config
            .injections
            .iter()
            .flat_map(|i| i.patterns.iter().enumerate())
            .filter(|(_, p)| p.combined)
            .map(|(pattern, _)| Combined {
                pattern,
                nodes: Vec::new(),
                layer: None,
            })
            .collect();
        let anchor = ranges
            .first()
            .map_or((0, Point::new(0, 0)), |r| (r.start_byte, r.start_point));
        let mut layer = Layer {
            config,
            grammar,
            ranges,
            tree,
            anchor,
            injections: Vec::new(),
            combined,
        };
        self.refresh(&mut layer, None, None)?;
        Ok(Some(layer))
    }

    /// The top layer of `lang`, parsed from scratch.
    pub(crate) fn build_top(&mut self, lang: Lang) -> Result<Layer, HighlightError> {
        let grammar = Grammar::Lang(lang);
        let config = self.hl.config(grammar).ok_or_else(|| {
            HighlightError::Failed(format!("{} query did not compile", lang.name()))
        })?;
        self.build(config, grammar, vec![FULL])?
            .ok_or_else(|| HighlightError::Failed("parsing failed".into()))
    }

    /// Update the top layer after `edit`: parse it again reusing the old
    /// tree, then update what is injected where it changed.
    pub(crate) fn update_top(
        &mut self,
        layer: &mut Layer,
        edit: &InputEdit,
    ) -> Result<(), HighlightError> {
        layer.tree.edit(edit);
        let config = layer.config.clone();
        let tree = self
            .parse(&config, &[], Some(&layer.tree))?
            .ok_or_else(|| HighlightError::Failed("parsing failed".into()))?;
        let len = self.source.len();
        let mut changed = edit.start_byte..edit.new_end_byte;
        self.spots.push(widen(&tree, changed.clone(), len));
        for r in layer.tree.changed_ranges(&tree) {
            changed.start = changed.start.min(r.start_byte);
            changed.end = changed.end.max(r.end_byte);
            self.spots.push(widen(&tree, r.start_byte..r.end_byte, len));
        }
        layer.tree = tree;
        let region = whole_lines(self.source, changed);
        self.refresh(layer, Some(region), Some(edit))
    }

    /// Update a layer that existed before `edit` for its new `ranges`:
    /// shift it when nothing reaches it, otherwise parse it again
    /// (incrementally) and update what is injected into it.
    fn reparse(
        &mut self,
        mut layer: Layer,
        ranges: Vec<TsRange>,
        edit: &InputEdit,
    ) -> Result<Option<Layer>, HighlightError> {
        let old_hull = hull(&layer.ranges);
        let touched = edit.start_byte <= old_hull.end && edit.old_end_byte >= old_hull.start;
        let shifted: Vec<TsRange> = layer
            .ranges
            .iter()
            .map(|r| shift_ts_range(r, edit))
            .collect();
        if !touched && shifted == ranges {
            shift_layer(&mut layer, edit);
            return Ok(Some(layer));
        }
        normalize(&mut layer);
        layer.tree.edit(edit);
        let config = layer.config.clone();
        let Some(tree) = self.parse(&config, &ranges, Some(&layer.tree))? else {
            self.spots.push(map_range(&old_hull, edit));
            return Ok(None);
        };
        let len = self.source.len();
        let edited = edit.start_byte..edit.new_end_byte;
        let mut region: Option<Range<usize>> = None;
        let mut add = |r: Range<usize>| {
            region = Some(match region.take() {
                Some(g) => g.start.min(r.start)..g.end.max(r.end),
                None => r,
            });
        };
        if touched {
            // The injections reaching over the edit change, even when it is
            // in a gap between the layer's ranges.
            add(edited.clone());
            if ranges
                .iter()
                .chain(&shifted)
                .any(|r| touches(&(r.start_byte..r.end_byte), &edited))
            {
                self.spots.push(widen(&tree, edited.clone(), len));
            }
        }
        for r in layer.tree.changed_ranges(&tree) {
            let r = r.start_byte..r.end_byte.min(len);
            add(r.clone());
            self.spots.push(widen(&tree, r, len));
        }
        if let Some(d) = ranges_diff(&shifted, &ranges) {
            let d = d.start..d.end.min(len);
            add(d.clone());
            self.spots.push(d);
        }
        layer.anchor = ranges
            .first()
            .map_or((0, Point::new(0, 0)), |r| (r.start_byte, r.start_point));
        layer.tree = tree;
        layer.ranges = ranges;
        let region = region.unwrap_or(edited.start..edited.start);
        let region = whole_lines(self.source, region);
        self.refresh(&mut layer, Some(region), Some(edit))?;
        Ok(Some(layer))
    }

    /// Update the injections of `layer` (its tree already parsed): with a
    /// `region` and the `edit`, keep (shifted) those whose pattern root
    /// is outside the region and look for injections again inside it;
    /// without, look in the whole tree.
    fn refresh(
        &mut self,
        layer: &mut Layer,
        region: Option<Range<usize>>,
        edit: Option<&InputEdit>,
    ) -> Result<(), HighlightError> {
        let config = layer.config.clone();
        let Some(inj) = config.injections.as_ref() else {
            return Ok(());
        };
        let top = layer.ranges.first() == Some(&FULL);
        // Keep or drop what was injected before.
        let keep = |root: &Range<usize>| match (edit, &region) {
            (Some(e), Some(h)) => !touches(&map_range(root, e), h),
            _ => true,
        };
        let mut kept: Vec<Injection> = Vec::with_capacity(layer.injections.len());
        let mut dropped: Vec<Injection> = Vec::new();
        for mut i in std::mem::take(&mut layer.injections) {
            if keep(&i.root) {
                if let Some(e) = edit {
                    shift_injection(&mut i, e);
                }
                kept.push(i);
            } else {
                dropped.push(i);
            }
        }
        let mut combined_changed = vec![false; layer.combined.len()];
        for (c, changed) in layer.combined.iter_mut().zip(&mut combined_changed) {
            let nodes = std::mem::take(&mut c.nodes);
            for mut n in nodes {
                if keep(&n.root) {
                    if let Some(e) = edit {
                        shift_node(&mut n, e);
                    }
                    c.nodes.push(n);
                } else {
                    *changed = true;
                }
            }
        }
        // Look for injections.
        let mut found: Vec<Found> = Vec::new();
        let mut found_combined: Vec<(usize, CombinedNode)> = Vec::new();
        {
            let cursor = &mut self.hl.cursor;
            match &region {
                Some(h) => cursor.set_byte_range(h.start.saturating_sub(1)..h.end + 1),
                None => cursor.set_byte_range(0..usize::MAX),
            };
            let own = config.hc.language_name.as_str();
            let mut matches = cursor.matches(&inj.query, layer.tree.root_node(), self.source);
            let mut count = 0usize;
            while let Some(m) = matches.next() {
                count += 1;
                if count.is_multiple_of(CANCEL_INTERVAL) && cancelled(self.cancel) {
                    return Err(HighlightError::Cancelled);
                }
                let Some(p) = inj.patterns.get(m.pattern_index) else {
                    continue;
                };
                let mut language = None;
                let mut content = None;
                for c in m.captures() {
                    if Some(c.index) == inj.language {
                        language = c.node.utf8_text(self.source).ok();
                    } else if Some(c.index) == inj.content {
                        content = Some(c.node);
                    }
                }
                let parent = if p.combined && top {
                    None
                } else {
                    Some(self.host_name)
                };
                let language = language.or(match &p.fallback {
                    Fallback::None => None,
                    Fallback::Name(n) => Some(n.as_str()),
                    Fallback::Own => Some(own),
                    Fallback::Parent => parent,
                });
                let Some(content) = content else {
                    continue;
                };
                let root = ancestor(content, p.depth).byte_range();
                if let Some(h) = &region
                    && !touches(&root, h)
                {
                    continue;
                }
                let children: Vec<TsRange> = if p.include_children {
                    Vec::new()
                } else {
                    let mut walk = content.walk();
                    content.children(&mut walk).map(|c| c.range()).collect()
                };
                if p.combined {
                    found_combined.push((
                        m.pattern_index,
                        CombinedNode {
                            root,
                            language: language.map(str::to_owned),
                            node: content.range(),
                            children,
                        },
                    ));
                } else if let Some(language) = language {
                    let ranges = intersect_ranges(&layer.ranges, &[(content.range(), &children)]);
                    if !ranges.is_empty() {
                        found.push(Found {
                            pattern: m.pattern_index,
                            root,
                            content: content.byte_range(),
                            language: language.to_owned(),
                            ranges,
                        });
                    }
                }
            }
        }
        // Layers of the injections found: an old layer of the same
        // injection is updated, others are built.
        for f in found {
            let Some((grammar, sub)) = self.hl.resolve(&f.language) else {
                continue;
            };
            let reuse = edit.and_then(|e| {
                dropped.iter().position(|d| {
                    d.pattern == f.pattern
                        && d.layer.grammar == grammar
                        && touches(&map_range(&d.content, e), &f.content)
                })
            });
            let new = match (reuse, edit) {
                (Some(k), Some(e)) => {
                    let d = dropped.swap_remove(k);
                    self.reparse(*d.layer, f.ranges, e)?
                }
                _ => {
                    let l = self.build(sub, grammar, f.ranges)?;
                    if let Some(l) = &l {
                        self.spots.push(hull(&l.ranges));
                    }
                    l
                }
            };
            if let Some(new) = new {
                kept.push(Injection {
                    pattern: f.pattern,
                    root: f.root,
                    content: f.content,
                    layer: Box::new(new),
                });
            }
        }
        for d in dropped {
            let h = hull(&d.layer.ranges);
            self.spots.push(match edit {
                Some(e) => map_range(&h, e),
                None => h,
            });
        }
        kept.sort_by_key(|i| (i.content.start, i.content.end));
        layer.injections = kept;
        // Combined injections: one layer over all matches of a pattern.
        let Layer {
            ranges, combined, ..
        } = layer;
        for (c, changed) in combined.iter_mut().zip(combined_changed) {
            let mut changed = changed;
            for (_, node) in found_combined.iter().filter(|(p, _)| *p == c.pattern) {
                c.nodes.push(node.clone());
                changed = true;
            }
            if changed {
                c.nodes
                    .sort_by_key(|n| (n.node.start_byte, n.node.end_byte));
            }
            let language = c.nodes.iter().rev().find_map(|n| n.language.as_deref());
            let resolved = language.and_then(|l| self.hl.resolve(l));
            let new_ranges = if c.nodes.is_empty() {
                Vec::new()
            } else {
                let nodes: Vec<(TsRange, &[TsRange])> = c
                    .nodes
                    .iter()
                    .map(|n| (n.node, n.children.as_slice()))
                    .collect();
                intersect_ranges(ranges, &nodes)
            };
            let old = c.layer.take();
            c.layer = match (old, resolved, edit) {
                (Some(old), Some((grammar, _)), Some(e))
                    if old.grammar == grammar && !new_ranges.is_empty() =>
                {
                    self.reparse(*old, new_ranges, e)?
                }
                (old, resolved, _) => {
                    if let Some(old) = old {
                        let h = hull(&old.ranges);
                        self.spots.push(match edit {
                            Some(e) => map_range(&h, e),
                            None => h,
                        });
                    }
                    match resolved {
                        Some((grammar, sub)) if !new_ranges.is_empty() => {
                            let l = self.build(sub, grammar, new_ranges)?;
                            if let Some(l) = &l {
                                self.spots.push(hull(&l.ranges));
                            }
                            l
                        }
                        _ => None,
                    }
                }
            }
            .map(Box::new);
        }
        Ok(())
    }
}

/// A highlight a layer would start: a captured node and its style.
struct Candidate {
    start: usize,
    end: usize,
    style: Style,
}

/// The highlights of `layer` intersecting `region`, in the order
/// `tree_sitter_highlight` processes them: captures in document order,
/// injection matches skipped, and of several patterns capturing one node
/// the last wins (the earlier matches are dropped).
fn candidates(
    cursor: &mut QueryCursor,
    layer: &Layer,
    source: &[u8],
    region: Range<usize>,
    cancel: Option<&AtomicUsize>,
) -> Result<Vec<Candidate>, HighlightError> {
    let config = &layer.config;
    cursor.set_byte_range(region);
    let mut captures = cursor.captures(&config.hc.query, layer.tree.root_node(), source);
    let mut out = Vec::new();
    let mut removed: HashSet<u32> = HashSet::new();
    // The node being captured and the capture that currently wins for it.
    let mut group: Option<(Node<'_>, u32)> = None;
    let mut group_match = 0u32;
    let mut finish = |group: &mut Option<(Node<'_>, u32)>| {
        if let Some((node, capture)) = group.take()
            && let Some(style) = config.styles.get(capture as usize).copied().flatten()
        {
            out.push(Candidate {
                start: node.start_byte(),
                end: node.end_byte(),
                style,
            });
        }
    };
    let mut count = 0usize;
    while let Some((m, index)) = captures.next() {
        count += 1;
        if count.is_multiple_of(CANCEL_INTERVAL) && cancelled(cancel) {
            return Err(HighlightError::Cancelled);
        }
        if m.pattern_index < config.injection_patterns {
            // Injections are layers of their own.
            m.remove();
            finish(&mut group);
            continue;
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
            Some(_) => finish(&mut group),
            None => {}
        }
        group = Some((capture.node, capture.index));
        group_match = m.id();
    }
    finish(&mut group);
    Ok(out)
}

/// Bring the trees of the layers reaching `region` up to date.
fn normalize_in(layer: &mut Layer, region: &Range<usize>) {
    normalize(layer);
    for i in &mut layer.injections {
        if touches(&hull(&i.layer.ranges), region) {
            normalize_in(&mut i.layer, region);
        }
    }
    for c in &mut layer.combined {
        if let Some(l) = &mut c.layer
            && touches(&hull(&l.ranges), region)
        {
            normalize_in(l, region);
        }
    }
}

/// The layers reaching `region` with their depth, outer layers first.
fn reaching<'l>(
    layer: &'l Layer,
    depth: usize,
    region: &Range<usize>,
    out: &mut Vec<(&'l Layer, usize)>,
) {
    out.push((layer, depth));
    for c in &layer.combined {
        if let Some(l) = &c.layer
            && touches(&hull(&l.ranges), region)
        {
            reaching(l, depth + 1, region, out);
        }
    }
    for i in &layer.injections {
        if touches(&hull(&i.layer.ranges), region) {
            reaching(&i.layer, depth + 1, region, out);
        }
    }
}

/// Highlight spans of `region` from all layers, merged like
/// `tree_sitter_highlight` merges their events, and the extent of the
/// highlighted nodes that intersect `region`.
pub(crate) fn highlight(
    top: &mut Layer,
    cursor: &mut QueryCursor,
    source: &[u8],
    region: Range<usize>,
    cancel: Option<&AtomicUsize>,
) -> Result<(Vec<Span>, Range<usize>), HighlightError> {
    normalize_in(top, &region);
    let mut layers = Vec::new();
    reaching(top, 0, &region, &mut layers);
    let mut streams = Vec::with_capacity(layers.len());
    for (layer, depth) in &layers {
        streams.push((
            *depth,
            candidates(cursor, layer, source, region.clone(), cancel)?,
        ));
    }
    Ok(merge(&streams, region))
}

/// Merge the highlights of several layers (each with its depth): at each
/// position, ends before starts and deeper layers first; a highlight of
/// the same node as the one just started in a deeper layer is skipped;
/// the text between events has the style of the most recently started
/// highlight still open.
fn merge(layers: &[(usize, Vec<Candidate>)], region: Range<usize>) -> (Vec<Span>, Range<usize>) {
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
    struct State {
        next: usize,
        ends: Vec<usize>,
    }
    // (position, end 0 / start 1, deeper first, layer order)
    type Key = (usize, u8, usize, usize);
    let key = |i: usize, s: &State| -> Option<Key> {
        let (depth, cands) = &layers[i];
        let rank = usize::MAX - depth;
        match (cands.get(s.next), s.ends.last()) {
            (Some(c), Some(&e)) if c.start < e => Some((c.start, 1, rank, i)),
            (_, Some(&e)) => Some((e, 0, rank, i)),
            (Some(c), None) => Some((c.start, 1, rank, i)),
            (None, None) => None,
        }
    };
    let mut states: Vec<State> = layers
        .iter()
        .map(|_| State {
            next: 0,
            ends: Vec::new(),
        })
        .collect();
    let mut heap: BinaryHeap<Reverse<Key>> = states
        .iter()
        .enumerate()
        .filter_map(|(i, s)| key(i, s).map(Reverse))
        .collect();
    let mut out = Out {
        spans: Vec::new(),
        pos: region.start,
        end: region.end,
    };
    let mut extent = region.clone();
    let mut stack: Vec<Style> = Vec::new();
    let mut last: Option<(usize, usize, usize)> = None;
    while let Some(Reverse((pos, kind, _, i))) = heap.pop() {
        let depth = layers[i].0;
        let s = &mut states[i];
        if kind == 0 {
            s.ends.pop();
            out.emit(pos, stack.last().copied());
            stack.pop();
        } else {
            let c = &layers[i].1[s.next];
            s.next += 1;
            let same = matches!(last, Some((a, b, d)) if a == c.start && b == c.end && depth < d);
            if !same {
                last = Some((c.start, c.end, depth));
                s.ends.push(c.end);
                out.emit(c.start, stack.last().copied());
                stack.push(c.style);
                extent.start = extent.start.min(c.start);
                extent.end = extent.end.max(c.end);
            }
        }
        if let Some(k) = key(i, &states[i]) {
            heap.push(Reverse(k));
        }
    }
    let end = out.end;
    out.emit(end, stack.last().copied());
    (out.spans, extent)
}

#[cfg(test)]
impl Layer {
    /// Each layer's grammar, ranges, tree and whether it has syntax
    /// errors, outer layers first.
    pub(crate) fn fingerprint(&self) -> Vec<(Grammar, Vec<Range<usize>>, String)> {
        let mut out = Vec::new();
        self.collect_fingerprint(&mut out);
        out
    }

    fn collect_fingerprint(&self, out: &mut Vec<(Grammar, Vec<Range<usize>>, String)>) {
        out.push((
            self.grammar,
            self.ranges
                .iter()
                .map(|r| r.start_byte..r.end_byte)
                .collect(),
            self.tree.root_node().to_sexp(),
        ));
        for c in &self.combined {
            if let Some(l) = &c.layer {
                l.collect_fingerprint(out);
            }
        }
        for i in &self.injections {
            i.layer.collect_fingerprint(out);
        }
    }

    /// Whether any layer has a syntax error.
    pub(crate) fn has_error(&self) -> bool {
        self.tree.root_node().has_error()
            || self.injections.iter().any(|i| i.layer.has_error())
            || self
                .combined
                .iter()
                .any(|c| c.layer.as_ref().is_some_and(|l| l.has_error()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_depth_of_injection_patterns() {
        assert_eq!(
            content_depth("((inline) @injection.content\n (#set! injection.language \"x\"))"),
            Some(0)
        );
        assert_eq!(
            content_depth(
                "(fenced_code_block\n  (info_string\n    (language) @injection.language)\n  (code_fence_content) @injection.content)"
            ),
            Some(1)
        );
        assert_eq!(
            content_depth(
                "(document\n  .\n  (section\n    .\n    (thematic_break)\n    (_) @injection.content\n    (thematic_break))\n  (#set! injection.language \"yaml\"))"
            ),
            Some(2)
        );
        assert_eq!(
            content_depth(
                "(call_expression\n  function: [\n    (identifier) @injection.language\n    (member_expression\n      property: (property_identifier) @injection.language)\n  ]\n  arguments: (template_string (string_fragment) @injection.content)\n  (#set! injection.combined))"
            ),
            Some(2)
        );
        assert_eq!(
            content_depth("(x (#eq? @injection.content \"a\"))"),
            None,
            "a predicate is not a capture"
        );
    }

    #[test]
    fn every_injection_pattern_is_understood() {
        let mut h = Highlighter::new();
        let mut grammars: Vec<Grammar> = Lang::ALL.into_iter().map(Grammar::Lang).collect();
        grammars.push(Grammar::MdInline);
        for g in grammars {
            let c = h.config(g).unwrap();
            if let Some(inj) = &c.injections {
                for (i, p) in inj.patterns.iter().enumerate() {
                    // The Glimmer pattern of JavaScript has no content.
                    let has_content = inj.query.capture_names().contains(&"injection.content") && {
                        let s = inj.query.start_byte_for_pattern(i);
                        let e = inj.query.end_byte_for_pattern(i);
                        let src = match g {
                            Grammar::Lang(l) => l.injections_query(),
                            Grammar::MdInline => crate::MARKDOWN_INLINE_INJECTIONS.as_str(),
                        };
                        src[s..e].contains("@injection.content")
                    };
                    if has_content {
                        assert!(p.depth.is_some(), "{g:?} pattern {i}");
                    }
                }
            }
        }
    }

    #[test]
    fn intersect_ranges_leaves_out_children_and_gaps() {
        let r = |a: usize, b: usize| TsRange {
            start_byte: a,
            end_byte: b,
            start_point: Point::new(0, a),
            end_point: Point::new(0, b),
        };
        let parent = [r(0, 10), r(20, 30)];
        assert_eq!(
            intersect_ranges(&parent, &[(r(5, 25), &[r(7, 8)])]),
            vec![r(5, 7), r(8, 10), r(20, 25)]
        );
        assert_eq!(
            ranges_diff(&[r(0, 1), r(2, 3)], &[r(0, 1), r(2, 4)]),
            Some(2..4)
        );
        assert_eq!(ranges_diff(&[r(0, 1)], &[r(0, 1)]), None);
    }
}
