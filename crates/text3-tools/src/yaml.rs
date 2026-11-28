//! An editable YAML tree over its source text.
//!
//! The parser ([`saphyr_parser`]) reports where each event starts and
//! ends; this module turns those into exact spans of mapping keys and
//! scalar values (a quoted scalar's end is found by scanning its quotes,
//! as the parser's end can reach past trailing spaces) and, for block
//! collections, into the lines each entry or item occupies. Every edit
//! replaces only those characters, then the edited text is parsed again
//! and refused unless it gives exactly the tree expected from the change
//! (same keys, values, types, nesting, anchors and tags elsewhere).
//! Comments, blank lines, indentation, quote styles and line endings
//! outside the edited characters stay byte for byte.
//!
//! Scalars are compared by type as YAML 1.1 and 1.2 readers would both
//! see them: a quoted or block scalar is a string; a plain one is a string
//! only when it cannot read as null, a boolean (`yes`, `no`, `on`, `off`,
//! `y`, `n`, `true`, `false` in their case forms), a number or a date (it
//! starts with a digit, or `.`, `+` or `-` and a digit), `.inf`, `.nan`,
//! `<<` or `=`. A string set to such text is written quoted, so it stays
//! a string.
//!
//! Read-only, with a [`Lock`] saying why: nodes inside flow collections
//! (`{a: 1}`, `[1, 2]`; the collection itself can still be renamed or
//! removed as an entry of its block parent), scalars with an anchor or a
//! tag, aliases, block scalars (`|`, `>`), complex and explicit (`? `)
//! keys and keys with an anchor or tag, every document after the first,
//! entries whose lines could not be told apart from their neighbours',
//! and the only entry of a collection (removing it would change the
//! collection's type). Aliases are never expanded; the builder is
//! iterative and stops at [`MAX_NODES`] events.

use std::ops::Range;

pub use crate::csv::Edit;
use crate::markup::{MAX_NODES, TreeError, too_large};

/// What a node of the tree is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// The stream (`items[0]`); its children are the documents' roots.
    Stream,
    Mapping,
    Sequence,
    Scalar,
    /// A reference to an anchor (`*name`), shown as written.
    Alias,
}

/// How a scalar is written.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Style {
    Plain,
    SingleQuoted,
    DoubleQuoted,
    /// `|`
    Literal,
    /// `>`
    Folded,
}

/// Why part of a node cannot be edited.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lock {
    /// Inside a flow collection (`[…]`, `{…}`), or adding inside one.
    Flow,
    /// The node has an anchor (`&name`).
    Anchor,
    /// The node has a tag (`!tag`, `!!str`).
    Tag,
    /// An alias (`*name`).
    Alias,
    /// A block scalar (`|` or `>`).
    BlockScalar,
    /// A complex or explicit (`? `) key.
    ComplexKey,
    /// Not in the first document of the stream.
    OtherDocument,
    /// The entry's lines are shared with other content.
    Layout,
    /// The only entry of its collection.
    OnlyChild,
}

/// Why an edit was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditError {
    /// The text cannot be written here (a key over 1024 characters).
    InvalidValue,
    /// Another entry of the same mapping already has this key.
    DuplicateKey,
    /// This part of the node is read-only.
    Locked(Lock),
    /// The edited text would not parse to the expected tree.
    NotPreserved,
    /// The operation does not apply to this node.
    NotApplicable,
}

/// A scalar as compared between trees.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Sc {
    /// Certainly a string (quoted, block, or plain that reads as text).
    Str(String),
    /// A plain scalar that may read as another type: compared as written.
    Plain(String),
}

/// One parse event, normalized for comparing trees.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Ev {
    DocStart(bool),
    DocEnd,
    /// Value, anchor (0 for none, else 1, 2, … in order), tag.
    Scalar(Sc, usize, Option<String>),
    SeqStart(usize, Option<String>),
    SeqEnd,
    MapStart(usize, Option<String>),
    MapEnd,
    Alias(usize),
}

/// A parse event with its style and char span.
struct Tok {
    ev: Ev,
    style: Option<Style>,
    span: Range<usize>,
}

/// Source ranges of a node. In [`Item::spans`] they count characters.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Spans {
    /// A mapping entry's key as written (quotes included).
    pub key: Option<Range<usize>>,
    /// A scalar or alias as written (quotes included; empty where an
    /// empty value would be typed), or a flow collection bracket to
    /// bracket.
    pub value: Option<Range<usize>>,
    /// What deleting the entry or item removes: its whole lines with the
    /// line break, including nested lines and a comment on its last line.
    pub removal: Option<Range<usize>>,
}

/// One node of an editable YAML tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
    pub parent: Option<usize>,
    pub kind: Kind,
    /// Entries or items in source order.
    pub children: Vec<usize>,
    /// Scalars only.
    pub style: Option<Style>,
    /// A flow collection, or inside one.
    pub flow: bool,
    /// Index of the document it belongs to.
    pub document: usize,
    /// Why the value cannot be edited (scalars).
    pub value_lock: Option<Lock>,
    /// Why the key cannot be renamed (mapping entries).
    pub key_lock: Option<Lock>,
    /// Why the entry cannot be deleted.
    pub delete_lock: Option<Lock>,
    /// Why nothing can be added after the entry.
    pub add_lock: Option<Lock>,
    /// Char ranges.
    pub spans: Spans,
    /// The same ranges in bytes.
    b: Spans,
    /// Events of the node (the value; without its key).
    ev: Range<usize>,
    /// The key's event, for mapping entries.
    key_ev: Option<usize>,
    /// Decoded key and how it compares; `None` for a complex key.
    key: Option<Sc>,
    /// Decoded scalar value.
    value: Option<Sc>,
    /// Byte where the entry starts (key or `-`).
    start: usize,
    /// Byte after the entry's last token.
    end: usize,
    /// Byte after the entry's last line and its break: where a new
    /// sibling's line goes. Whether that line has a break.
    after: usize,
    has_break: bool,
    /// Column of the key or `-`, in characters.
    indent: usize,
    /// For an empty value: insert a space before the typed value.
    space: bool,
}

impl Item {
    fn new(parent: Option<usize>, kind: Kind, document: usize) -> Self {
        Item {
            parent,
            kind,
            children: Vec::new(),
            style: None,
            flow: false,
            document,
            value_lock: None,
            key_lock: None,
            delete_lock: None,
            add_lock: None,
            spans: Spans::default(),
            b: Spans::default(),
            ev: 0..0,
            key_ev: None,
            key: None,
            value: None,
            start: 0,
            end: 0,
            after: 0,
            has_break: false,
            indent: 0,
            space: false,
        }
    }
}

/// An editable YAML tree. `items[0]` is the stream.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Doc {
    src: String,
    pub items: Vec<Item>,
    events: Vec<Ev>,
}

/// YAML 1.2 printable characters.
fn printable(c: char) -> bool {
    matches!(c, '\t' | '\n' | '\r' | ' '..='~' | '\u{85}' | '\u{a0}'..='\u{d7ff}' | '\u{e000}'..='\u{fffd}' | '\u{10000}'..)
}

fn line_break_char(c: char) -> bool {
    matches!(c, '\n' | '\r' | '\u{85}' | '\u{2028}' | '\u{2029}')
}

/// Whether a plain scalar reads as a string in both YAML 1.1 and 1.2 (see
/// the module documentation).
fn plain_is_string(s: &str) -> bool {
    if matches!(
        s,
        "" | "~"
            | "null"
            | "Null"
            | "NULL"
            | "y"
            | "Y"
            | "yes"
            | "Yes"
            | "YES"
            | "n"
            | "N"
            | "no"
            | "No"
            | "NO"
            | "true"
            | "True"
            | "TRUE"
            | "false"
            | "False"
            | "FALSE"
            | "on"
            | "On"
            | "ON"
            | "off"
            | "Off"
            | "OFF"
            | "<<"
            | "="
    ) {
        return false;
    }
    let lower = s.to_ascii_lowercase();
    let t = lower.trim_start_matches(['+', '-']);
    if matches!(t, ".inf" | ".nan") {
        return false;
    }
    let mut cs = t.chars();
    match cs.next() {
        Some(c) if c.is_ascii_digit() => false,
        Some('.') => !cs.next().is_some_and(|c| c.is_ascii_digit()),
        _ => true,
    }
}

fn classify(s: &str) -> Sc {
    if plain_is_string(s) {
        Sc::Str(s.to_owned())
    } else {
        Sc::Plain(s.to_owned())
    }
}

/// Whether `s` can be written as one plain scalar in block context and
/// reads back as exactly `s` (conservative).
fn plain_ok(s: &str) -> bool {
    let Some(first) = s.chars().next() else {
        return false;
    };
    if s.trim_matches([' ', '\t']) != s
        || s.chars()
            .any(|c| !printable(c) || c == '\t' || line_break_char(c) || c == '\u{feff}')
        || "#,[]{}&*!|>'\"%@`?:".contains(first)
        || s.starts_with("---")
        || s.starts_with("...")
        || s.contains(": ")
        || s.ends_with(':')
        || s.contains(" #")
    {
        return false;
    }
    if first == '-' {
        return s.chars().nth(1).is_some_and(|c| c != ' ');
    }
    true
}

/// Whether `s` can be written single-quoted on one line.
fn single_ok(s: &str) -> bool {
    s.chars()
        .all(|c| printable(c) && !line_break_char(c) && c != '\u{feff}')
}

/// `s` as a double-quoted scalar on one line.
pub fn double_quoted(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\0' => out.push_str("\\0"),
            '\u{7}' => out.push_str("\\a"),
            '\u{8}' => out.push_str("\\b"),
            '\t' => out.push_str("\\t"),
            '\n' => out.push_str("\\n"),
            '\u{b}' => out.push_str("\\v"),
            '\u{c}' => out.push_str("\\f"),
            '\r' => out.push_str("\\r"),
            '\u{1b}' => out.push_str("\\e"),
            '\u{85}' => out.push_str("\\N"),
            '\u{2028}' => out.push_str("\\L"),
            '\u{2029}' => out.push_str("\\P"),
            c if !printable(c) || c == '\u{feff}' => {
                let n = c as u32;
                if n <= 0xffff {
                    out.push_str(&format!("\\u{n:04X}"));
                } else {
                    out.push_str(&format!("\\U{n:08X}"));
                }
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// How typed text is written in place of a scalar of `style` whose value
/// is `old`: the same style when it can hold the text; a plain string
/// stays a string (quoted if the text would read as another type); text
/// typed over a plain non-string is kept as written when it is one plain
/// scalar. Otherwise double-quoted.
fn render(style: Style, old: &Sc, new: &str) -> (String, Sc) {
    let quoted = || (double_quoted(new), Sc::Str(new.to_owned()));
    match style {
        Style::SingleQuoted if single_ok(new) => (
            format!("'{}'", new.replace('\'', "''")),
            Sc::Str(new.to_owned()),
        ),
        Style::Plain if plain_ok(new) => match old {
            Sc::Str(_) if !plain_is_string(new) => quoted(),
            _ => (new.to_owned(), classify(new)),
        },
        _ => quoted(),
    }
}

fn tag_text(t: &saphyr_parser::Tag) -> String {
    format!("{}{}", t.handle, t.suffix)
}

/// Parse into normalized events with char spans.
fn tokens(text: &str) -> Result<Vec<Tok>, TreeError> {
    use saphyr_parser::{Event, Parser, ScalarStyle};
    let mut out = Vec::new();
    for item in Parser::new_from_str(text) {
        let (event, span) = item.map_err(|e| TreeError {
            line: e.marker().line(),
            column: e.marker().col() + 1,
            message: e.info().to_owned(),
        })?;
        if out.len() >= MAX_NODES {
            let byte = text
                .char_indices()
                .nth(span.start.index())
                .map_or(text.len(), |(b, _)| b);
            return Err(too_large(text, byte));
        }
        let mut style = None;
        let ev = match event {
            Event::DocumentStart(explicit) => Ev::DocStart(explicit),
            Event::DocumentEnd => Ev::DocEnd,
            Event::Scalar(v, st, anchor, tag) => {
                let s = match st {
                    ScalarStyle::Plain => Style::Plain,
                    ScalarStyle::SingleQuoted => Style::SingleQuoted,
                    ScalarStyle::DoubleQuoted => Style::DoubleQuoted,
                    ScalarStyle::Literal => Style::Literal,
                    ScalarStyle::Folded => Style::Folded,
                };
                style = Some(s);
                let sc = match s {
                    Style::Plain => classify(&v),
                    _ => Sc::Str(v.into_owned()),
                };
                Ev::Scalar(sc, anchor, tag.map(|t| tag_text(&t)))
            }
            Event::SequenceStart(anchor, tag) => Ev::SeqStart(anchor, tag.map(|t| tag_text(&t))),
            Event::SequenceEnd => Ev::SeqEnd,
            Event::MappingStart(anchor, tag) => Ev::MapStart(anchor, tag.map(|t| tag_text(&t))),
            Event::MappingEnd => Ev::MapEnd,
            Event::Alias(id) => Ev::Alias(id),
            _ => continue,
        };
        out.push(Tok {
            ev,
            style,
            span: span.start.index()..span.end.index().max(span.start.index()),
        });
    }
    Ok(out)
}

/// Number anchors 1, 2, … in order of definition, aliases to match, so a
/// tree compares equal after an anchor before it was removed.
fn renumber(events: &mut [Ev]) {
    let mut map = std::collections::HashMap::new();
    let mut next = 0;
    for e in events.iter_mut() {
        let a = match e {
            Ev::Scalar(_, a, _) | Ev::SeqStart(a, _) | Ev::MapStart(a, _) => a,
            Ev::Alias(a) => {
                *a = map.get(a).copied().unwrap_or(usize::MAX);
                continue;
            }
            _ => continue,
        };
        if *a != 0 {
            next += 1;
            map.insert(*a, next);
            *a = next;
        }
    }
}

/// Start of the line holding byte `b` (after a leading BOM).
fn line_start(src: &str, b: usize) -> usize {
    match src[..b].rfind(['\n', '\r']) {
        Some(k) => k + 1,
        None if src.starts_with('\u{feff}') && b >= 3 => 3,
        None => 0,
    }
}

/// The end of the line holding byte `b` (before its break) and the byte
/// after its break (equal at the end of the text).
fn line_end(src: &str, b: usize) -> (usize, usize) {
    match src[b..].find(['\n', '\r']) {
        Some(k) => {
            let e = b + k;
            let len = if src[e..].starts_with("\r\n") { 2 } else { 1 };
            (e, e + len)
        }
        None => (src.len(), src.len()),
    }
}

/// The length of the line break ending just before byte `b`, if any.
fn break_before(src: &str, b: usize) -> usize {
    let head = &src[..b];
    if head.ends_with("\r\n") {
        2
    } else if head.ends_with(['\n', '\r']) {
        1
    } else {
        0
    }
}

fn is_blank(s: &str) -> bool {
    s.trim_matches([' ', '\t']).is_empty()
}

/// Where a scalar written at `start` ends: past its closing quote for
/// quoted styles (after any anchor or tag), else `fallback`.
fn scalar_end(src: &str, start: usize, style: Style, fallback: usize) -> usize {
    let q = match style {
        Style::SingleQuoted => '\'',
        Style::DoubleQuoted => '"',
        _ => return fallback,
    };
    let Some(open) = src[start..].find(q).map(|k| start + k) else {
        return fallback;
    };
    let mut it = src[open + 1..].char_indices();
    while let Some((k, c)) = it.next() {
        if q == '"' && c == '\\' {
            it.next();
        } else if c == q {
            if q == '\'' && src[open + 1 + k + 1..].starts_with('\'') {
                it.next();
                continue;
            }
            return open + 1 + k + 1;
        }
    }
    fallback
}

/// The `-` of a block sequence item whose content starts at byte `first`:
/// on the same line before it, or alone (with a comment) on an earlier
/// line with only blank and comment lines between.
fn find_dash(src: &str, first: usize) -> Option<usize> {
    let ls = line_start(src, first);
    let before = src[ls..first].trim_end_matches([' ', '\t']);
    if before.ends_with('-') {
        let d = ls + before.len() - 1;
        let prev = src[..d].chars().next_back();
        return (d == ls || prev.is_some_and(|c| c == ' ' || c == '\t')).then_some(d);
    }
    if !before.is_empty() {
        return None;
    }
    let mut ls = ls;
    while ls > 0 && ls > usize::from(src.starts_with('\u{feff}')) * 3 {
        let prev = line_start(src, ls - break_before(src, ls));
        let line = &src[prev..ls];
        let t = line.trim_start_matches([' ', '\t']);
        let t = t.trim_end_matches(['\n', '\r']);
        if !(t.is_empty() || t.starts_with('#')) {
            let rest = t.strip_prefix('-')?;
            let r = rest.trim_start_matches([' ', '\t']);
            let spaced = rest.is_empty() || rest.starts_with([' ', '\t']);
            return (spaced && (r.is_empty() || r.starts_with('#')))
                .then(|| prev + (line.len() - line.trim_start_matches([' ', '\t']).len()));
        }
        ls = prev;
    }
    None
}

/// A mapping or sequence being read.
struct Frame {
    item: usize,
    map: bool,
    /// A key read and waiting for its value.
    key: Option<Key>,
}

/// A mapping key: its event, byte span, value, and why it cannot be
/// renamed.
type Key = (usize, Option<Range<usize>>, Option<Sc>, Option<Lock>);

/// Parse `src` into an editable tree without recursion. Accepts what the
/// read-only [`crate::markup::yaml_tree`] accepts.
pub fn doc(src: &str) -> Result<Doc, TreeError> {
    let toks = tokens(src)?;
    // Char offsets to bytes, in one pass over the text.
    let mut offs: Vec<usize> = toks
        .iter()
        .flat_map(|t| [t.span.start, t.span.end])
        .collect();
    offs.sort_unstable();
    offs.dedup();
    let mut bytes = Vec::with_capacity(offs.len());
    {
        let mut it = src.char_indices().map(|(b, _)| b).chain([src.len()]);
        let mut ch = 0;
        let mut cur = it.next().unwrap_or(src.len());
        for &o in &offs {
            while ch < o {
                match it.next() {
                    Some(b) => cur = b,
                    None => {
                        cur = src.len();
                        break;
                    }
                }
                ch += 1;
            }
            bytes.push(cur);
        }
    }
    let byte = |c: usize| bytes[offs.binary_search(&c).unwrap_or(0)];

    let mut items = vec![Item::new(None, Kind::Stream, 0)];
    let mut stack: Vec<Frame> = Vec::new();
    let mut docs = 0usize;
    // Depth inside a complex key and the key's first event.
    let mut skip = (0usize, 0usize);
    for (k, t) in toks.iter().enumerate() {
        let doc_index = docs.saturating_sub(1);
        if skip.0 > 0 {
            match t.ev {
                Ev::SeqStart(..) | Ev::MapStart(..) => skip.0 += 1,
                Ev::SeqEnd | Ev::MapEnd => skip.0 -= 1,
                _ => {}
            }
            if skip.0 == 0
                && let Some(f) = stack.last_mut()
            {
                f.key = Some((skip.1, None, None, Some(Lock::ComplexKey)));
            }
            continue;
        }
        let (s, e) = (byte(t.span.start), byte(t.span.end));
        match &t.ev {
            Ev::DocStart(_) => {
                docs += 1;
                continue;
            }
            Ev::DocEnd => continue,
            Ev::SeqEnd | Ev::MapEnd => {
                let Some(f) = stack.pop() else { continue };
                let last = items[f.item].children.last().map(|&c| items[c].end);
                let it = &mut items[f.item];
                it.ev.end = k + 1;
                let end = if it.flow { Some(e) } else { last };
                it.end = it.end.max(end.unwrap_or(0));
                continue;
            }
            _ => {}
        }
        // A key, when the innermost mapping waits for one.
        if let Some(f) = stack.last_mut()
            && f.map
            && f.key.is_none()
        {
            match &t.ev {
                Ev::Scalar(sc, anchor, tag) => {
                    let style = t.style.unwrap_or(Style::Plain);
                    let end = scalar_end(src, s, style, e);
                    let explicit = src[line_start(src, s)..s]
                        .trim_end_matches([' ', '\t'])
                        .ends_with('?');
                    let lock = if explicit || matches!(style, Style::Literal | Style::Folded) {
                        Some(Lock::ComplexKey)
                    } else if *anchor != 0 {
                        Some(Lock::Anchor)
                    } else if tag.is_some() {
                        Some(Lock::Tag)
                    } else {
                        None
                    };
                    f.key = Some((k, Some(s..end), Some(sc.clone()), lock));
                }
                Ev::Alias(_) => f.key = Some((k, Some(s..e), None, Some(Lock::ComplexKey))),
                _ => skip = (1, k),
            }
            continue;
        }
        // A node: a document root, a mapping value or a sequence item.
        let parent = stack.last().map_or(0, |f| f.item);
        let pflow = items[parent].flow;
        let kind = match t.ev {
            Ev::Scalar(..) => Kind::Scalar,
            Ev::Alias(_) => Kind::Alias,
            Ev::SeqStart(..) => Kind::Sequence,
            _ => Kind::Mapping,
        };
        let mut it = Item::new(Some(parent), kind, doc_index);
        it.ev = k..k + 1;
        it.flow = pflow;
        it.start = s;
        if let Some(f) = stack.last_mut()
            && let Some((kev, kb, ksc, klock)) = f.key.take()
        {
            it.key_ev = Some(kev);
            it.b.key = kb.clone();
            it.key = ksc;
            it.key_lock = klock;
            if let Some(kb) = kb {
                it.start = kb.start;
                it.end = kb.end;
            }
            if klock == Some(Lock::ComplexKey) {
                it.delete_lock = Some(Lock::ComplexKey);
                it.add_lock = Some(Lock::ComplexKey);
            }
        }
        match &t.ev {
            Ev::Scalar(sc, anchor, tag) => {
                let style = t.style.unwrap_or(Style::Plain);
                let end = scalar_end(src, s, style, e);
                it.style = Some(style);
                it.value = Some(sc.clone());
                it.b.value = Some(s..end);
                it.end = it.end.max(end);
                it.value_lock = if matches!(style, Style::Literal | Style::Folded) {
                    Some(Lock::BlockScalar)
                } else if *anchor != 0 {
                    Some(Lock::Anchor)
                } else if tag.is_some() {
                    Some(Lock::Tag)
                } else {
                    None
                };
            }
            Ev::Alias(_) => {
                it.b.value = Some(s..e);
                it.end = it.end.max(e);
                it.value_lock = Some(Lock::Alias);
            }
            _ => {
                let flow = e > s && src[..e].ends_with(['[', '{']);
                it.flow = pflow || flow;
                if flow {
                    it.b.value = Some(s..e);
                }
            }
        }
        if it.flow && pflow {
            it.value_lock = Some(Lock::Flow);
            it.key_lock = Some(Lock::Flow);
            it.delete_lock = Some(Lock::Flow);
            it.add_lock = Some(Lock::Flow);
        }
        let container = matches!(kind, Kind::Mapping | Kind::Sequence);
        items.push(it);
        let i = items.len() - 1;
        items[parent].children.push(i);
        if container {
            stack.push(Frame {
                item: i,
                map: kind == Kind::Mapping,
                key: None,
            });
        }
    }
    // A flow collection's value span runs to its closing bracket.
    for it in items.iter_mut().skip(1) {
        if matches!(it.kind, Kind::Mapping | Kind::Sequence)
            && let Some(v) = &mut it.b.value
        {
            v.end = it.end.max(v.end);
        }
    }
    layout(src, &mut items);
    let mut events: Vec<Ev> = toks.into_iter().map(|t| t.ev).collect();
    renumber(&mut events);
    char_spans(src, &mut items);
    Ok(Doc {
        src: src.to_owned(),
        items,
        events,
    })
}

/// Lines, removal ranges, insertion points and locks of block entries.
fn layout(src: &str, items: &mut [Item]) {
    let n = items.len();
    // Where each node's first token starts: the dash of sequence items.
    for i in 1..n {
        let Some(p) = items[i].parent else { continue };
        let it = &items[i];
        if it.document > 0 {
            let it = &mut items[i];
            for l in [
                &mut it.value_lock,
                &mut it.key_lock,
                &mut it.delete_lock,
                &mut it.add_lock,
            ] {
                *l = Some(Lock::OtherDocument);
            }
            continue;
        }
        if p == 0 || items[p].flow {
            continue;
        }
        if items[p].kind == Kind::Sequence {
            match find_dash(src, items[i].start) {
                Some(d) => items[i].start = d,
                None => {
                    items[i].delete_lock = Some(Lock::Layout);
                    items[i].add_lock = Some(Lock::Layout);
                    continue;
                }
            }
        }
        let it = &items[i];
        let s = it.start;
        let ls = line_start(src, s);
        items[i].indent = src[ls..s].chars().count();
        // The last line of the entry.
        let e = items[i].end.max(s);
        let (after, has_break) = if e > s && break_before(src, e) > 0 {
            (e, true)
        } else {
            let (content_end, after) = line_end(src, e);
            let rest = src[e..content_end].trim_start_matches([' ', '\t']);
            if !(rest.is_empty() || rest.starts_with('#')) {
                items[i].delete_lock.get_or_insert(Lock::Layout);
                items[i].add_lock.get_or_insert(Lock::Layout);
                continue;
            }
            (after, after > content_end)
        };
        items[i].after = after;
        items[i].has_break = has_break;
        let own_line = is_blank(&src[ls..s]);
        let removal = if own_line {
            let mut start = ls;
            if !has_break {
                start -= break_before(src, ls);
            }
            Some(start..after)
        } else {
            // First entry on its parent's `- ` line: remove up to the next
            // sibling, which then takes its place, if only blank lines
            // stand between.
            let sib = &items[p].children;
            let pos = sib.iter().position(|&c| c == i).unwrap_or(0);
            sib.get(pos + 1).and_then(|&nx| {
                let ns = if items[p].kind == Kind::Sequence {
                    find_dash(src, items[nx].start)?
                } else {
                    items[nx].b.key.as_ref()?.start
                };
                let nls = line_start(src, ns);
                (nls >= after
                    && is_blank(&src[nls..ns])
                    && src[nls..ns].chars().count() == items[i].indent
                    && src[after..nls]
                        .chars()
                        .all(|c| matches!(c, ' ' | '\t' | '\n' | '\r')))
                .then_some(s..ns)
            })
        };
        match removal {
            Some(r) => items[i].b.removal = Some(r),
            None => {
                items[i].delete_lock.get_or_insert(Lock::Layout);
            }
        }
        if items[p].children.len() == 1 {
            items[i].delete_lock.get_or_insert(Lock::OnlyChild);
        }
    }
    // Empty plain values: where typed text goes.
    for i in 1..n {
        let it = &items[i];
        if it.kind != Kind::Scalar || it.b.value.as_ref().is_none_or(|v| !v.is_empty()) {
            continue;
        }
        let Some(p) = it.parent else { continue };
        let from = match (&it.b.key, items[p].kind) {
            (Some(k), _) => {
                let rest = &src[k.end..];
                let skip = rest.len() - rest.trim_start_matches([' ', '\t']).len();
                rest[skip..].starts_with(':').then_some(k.end + skip)
            }
            (None, Kind::Sequence) if !items[p].flow => Some(it.start),
            _ => None,
        };
        let Some(ind) = from.filter(|&f| src[f..].starts_with([':', '-'])) else {
            items[i].value_lock.get_or_insert(Lock::Layout);
            continue;
        };
        let at = ind + 1;
        let space = !src[at..].starts_with(' ');
        let at = if space { at } else { at + 1 };
        items[i].b.value = Some(at..at);
        items[i].space = space;
    }
}

/// Fill [`Item::spans`] from the byte spans with one pass over the text.
fn char_spans(src: &str, items: &mut [Item]) {
    let mut offs: Vec<usize> = Vec::new();
    for it in items.iter() {
        for r in [&it.b.key, &it.b.value, &it.b.removal]
            .into_iter()
            .flatten()
        {
            offs.extend([r.start, r.end]);
        }
    }
    offs.sort_unstable();
    offs.dedup();
    let mut chars = Vec::with_capacity(offs.len());
    let (mut byte, mut ch) = (0usize, 0usize);
    for &o in &offs {
        ch += src[byte..o].chars().count();
        byte = o;
        chars.push(ch);
    }
    let c = |b: usize| chars[offs.binary_search(&b).unwrap_or(0)];
    let cr = |r: &Range<usize>| c(r.start)..c(r.end);
    for it in items.iter_mut() {
        it.spans = Spans {
            key: it.b.key.as_ref().map(cr),
            value: it.b.value.as_ref().map(cr),
            removal: it.b.removal.as_ref().map(cr),
        };
    }
}

fn sc_text(s: &Sc) -> &str {
    match s {
        Sc::Str(v) | Sc::Plain(v) => v,
    }
}

impl Doc {
    /// The source this tree was parsed from.
    pub fn source(&self) -> &str {
        &self.src
    }

    /// Child positions from the stream down to `i` (the first is the
    /// document).
    pub fn path_of(&self, mut i: usize) -> Vec<usize> {
        let mut path = Vec::new();
        while let Some(p) = self.items.get(i).and_then(|it| it.parent) {
            path.push(
                self.items[p]
                    .children
                    .iter()
                    .position(|&c| c == i)
                    .unwrap_or(0),
            );
            i = p;
        }
        path.reverse();
        path
    }

    /// The node at a path of child positions (`[]` is the stream).
    pub fn find(&self, path: &[usize]) -> Option<usize> {
        let mut i = 0;
        self.items.first()?;
        for &k in path {
            i = *self.items[i].children.get(k)?;
        }
        Some(i)
    }

    /// A mapping entry's key as written (`None` for a complex key).
    pub fn key_text(&self, i: usize) -> Option<&str> {
        self.src.get(self.items.get(i)?.b.key.clone()?)
    }

    /// A mapping entry's key, decoded.
    pub fn key(&self, i: usize) -> Option<String> {
        self.items
            .get(i)?
            .key
            .as_ref()
            .map(|k| sc_text(k).to_owned())
    }

    /// A scalar, alias or flow collection as written.
    pub fn value_text(&self, i: usize) -> Option<&str> {
        self.src.get(self.items.get(i)?.b.value.clone()?)
    }

    /// A scalar's value, decoded.
    pub fn value(&self, i: usize) -> Option<String> {
        self.items
            .get(i)?
            .value
            .as_ref()
            .map(|v| sc_text(v).to_owned())
    }

    /// Whether the mapping entry `i` can be renamed.
    pub fn can_rename(&self, i: usize) -> bool {
        self.items
            .get(i)
            .is_some_and(|it| it.b.key.is_some() && it.key_lock.is_none())
    }

    /// Whether the scalar `i` can be edited.
    pub fn can_set_value(&self, i: usize) -> bool {
        self.items.get(i).is_some_and(|it| {
            it.kind == Kind::Scalar && it.value_lock.is_none() && it.b.value.is_some()
        })
    }

    /// Whether the entry or item `i` can be deleted.
    pub fn can_delete(&self, i: usize) -> bool {
        self.items.get(i).is_some_and(|it| {
            it.parent.is_some_and(|p| p != 0) && it.delete_lock.is_none() && it.b.removal.is_some()
        })
    }

    /// Whether an entry or item can be added after `i`.
    pub fn can_add_after(&self, i: usize) -> bool {
        self.items.get(i).is_some_and(|it| {
            it.parent.is_some_and(|p| p != 0) && it.add_lock.is_none() && it.after > 0
        })
    }

    /// Why nothing can be added inside the collection `i`, if so.
    pub fn add_inside_lock(&self, i: usize) -> Option<Lock> {
        let it = self.items.get(i)?;
        if !matches!(it.kind, Kind::Mapping | Kind::Sequence) {
            return None;
        }
        if it.flow {
            return Some(Lock::Flow);
        }
        if it.document > 0 {
            return Some(Lock::OtherDocument);
        }
        let &last = it.children.last()?;
        let l = &self.items[last];
        l.add_lock.or((l.after == 0).then_some(Lock::Layout))
    }

    /// Whether an entry or item can be added at the end of `i`.
    pub fn can_add_inside(&self, i: usize) -> bool {
        self.items.get(i).is_some_and(|it| {
            matches!(it.kind, Kind::Mapping | Kind::Sequence)
                && !it.children.is_empty()
                && self.add_inside_lock(i).is_none()
        })
    }

    /// The first reason part of node `i` is read-only, for showing.
    pub fn lock(&self, i: usize) -> Option<Lock> {
        let it = self.items.get(i)?;
        if it.kind == Kind::Stream {
            return None;
        }
        let scalar = matches!(it.kind, Kind::Scalar | Kind::Alias);
        let key = it.b.key.is_some() || it.key_lock.is_some();
        [
            scalar.then_some(it.value_lock).flatten(),
            key.then_some(it.key_lock).flatten(),
            it.delete_lock.filter(|&l| l != Lock::OnlyChild),
            self.add_inside_lock(i),
        ]
        .into_iter()
        .flatten()
        .next()
    }

    /// Whether replacing the scalar `i` with `new` changes anything.
    pub fn check_value(&self, i: usize, new: &str) -> Result<bool, EditError> {
        self.set_value(i, new).map(|e| e.is_some())
    }

    /// Set a scalar to `new` (plain text; quoted as needed, see
    /// [`render`]).
    pub fn set_value(&self, i: usize, new: &str) -> Result<Option<Vec<Edit>>, EditError> {
        let it = self.items.get(i).ok_or(EditError::NotApplicable)?;
        if it.kind != Kind::Scalar {
            return Err(it
                .value_lock
                .map_or(EditError::NotApplicable, EditError::Locked));
        }
        if let Some(l) = it.value_lock {
            return Err(EditError::Locked(l));
        }
        let (Some(style), Some(old), Some(at)) = (it.style, &it.value, &it.b.value) else {
            return Err(EditError::NotApplicable);
        };
        if sc_text(old) == new {
            return Ok(None);
        }
        let (mut text, sc) = render(style, old, new);
        if it.space {
            text.insert(0, ' ');
        }
        let mut expected = self.events.clone();
        expected[it.ev.start] = Ev::Scalar(sc, 0, None);
        self.checked(vec![(at.clone(), text)], expected).map(Some)
    }

    /// Whether renaming entry `i` to `name` changes anything.
    pub fn check_key(&self, i: usize, name: &str) -> Result<bool, EditError> {
        self.rename_key(i, name).map(|e| e.is_some())
    }

    /// The key text and how it compares, for a new key `name` in mapping
    /// `p` (other than entry `except`).
    fn new_key(
        &self,
        p: usize,
        except: Option<usize>,
        style: Style,
        old: &Sc,
        name: &str,
    ) -> Result<(String, Sc), EditError> {
        let (text, sc) = render(style, old, name);
        if text.chars().count() > 1024 {
            return Err(EditError::InvalidValue);
        }
        let taken = self.items[p]
            .children
            .iter()
            .any(|&c| Some(c) != except && self.items[c].key.as_ref() == Some(&sc));
        if taken {
            return Err(EditError::DuplicateKey);
        }
        Ok((text, sc))
    }

    /// Rename a mapping entry's key (quoted as for values).
    pub fn rename_key(&self, i: usize, name: &str) -> Result<Option<Vec<Edit>>, EditError> {
        let it = self.items.get(i).ok_or(EditError::NotApplicable)?;
        if let Some(l) = it.key_lock {
            return Err(EditError::Locked(l));
        }
        let (Some(kb), Some(old), Some(kev), Some(p)) = (&it.b.key, &it.key, it.key_ev, it.parent)
        else {
            return Err(EditError::NotApplicable);
        };
        if sc_text(old) == name {
            return Ok(None);
        }
        let style = match self.src[kb.start..].chars().next() {
            Some('\'') => Style::SingleQuoted,
            Some('"') => Style::DoubleQuoted,
            _ => Style::Plain,
        };
        let (text, sc) = self.new_key(p, Some(i), style, old, name)?;
        let mut expected = self.events.clone();
        expected[kev] = Ev::Scalar(sc, 0, None);
        self.checked(vec![(kb.clone(), text)], expected).map(Some)
    }

    /// Remove a block mapping entry or sequence item with its lines.
    pub fn delete(&self, i: usize) -> Result<Vec<Edit>, EditError> {
        let it = self.items.get(i).ok_or(EditError::NotApplicable)?;
        if it.parent.is_none_or(|p| p == 0) {
            return Err(EditError::NotApplicable);
        }
        if let Some(l) = it.delete_lock {
            return Err(EditError::Locked(l));
        }
        let r = it.b.removal.clone().ok_or(EditError::NotApplicable)?;
        let from = it.key_ev.unwrap_or(it.ev.start);
        let mut expected = self.events.clone();
        expected.drain(from..it.ev.end);
        renumber(&mut expected);
        self.checked(vec![(r, String::new())], expected)
    }

    /// A key no entry of mapping `p` has: `base`, `base2`, ….
    pub fn unique_key(&self, p: usize, base: &str) -> String {
        let used = |k: &str| {
            self.items.get(p).is_some_and(|it| {
                it.children
                    .iter()
                    .any(|&c| self.key(c).as_deref() == Some(k))
            })
        };
        let mut name = base.to_owned();
        let mut n = 1;
        while used(&name) {
            n += 1;
            name = format!("{base}{n}");
        }
        name
    }

    /// Add `key: null` (in a mapping; `key` is required) or `- null` (in a
    /// sequence) on a new line after entry `i`, at its indentation.
    pub fn add_after(&self, i: usize, key: Option<&str>) -> Result<Vec<Edit>, EditError> {
        let it = self.items.get(i).ok_or(EditError::NotApplicable)?;
        let p = it
            .parent
            .filter(|&p| p != 0)
            .ok_or(EditError::NotApplicable)?;
        if let Some(l) = it.add_lock {
            return Err(EditError::Locked(l));
        }
        if it.after == 0 {
            return Err(EditError::NotApplicable);
        }
        let null = Ev::Scalar(Sc::Plain("null".into()), 0, None);
        let (line, new) = match self.items[p].kind {
            Kind::Mapping => {
                let key = key.ok_or(EditError::NotApplicable)?;
                let (text, sc) =
                    self.new_key(p, None, Style::Plain, &Sc::Str(String::new()), key)?;
                (format!("{text}: null"), vec![Ev::Scalar(sc, 0, None), null])
            }
            Kind::Sequence => ("- null".to_owned(), vec![null]),
            _ => return Err(EditError::NotApplicable),
        };
        let pad = " ".repeat(it.indent);
        let text = if it.has_break {
            let brk = &self.src[it.after - break_before(&self.src, it.after)..it.after];
            format!("{pad}{line}{brk}")
        } else {
            format!("{}{pad}{line}", crate::dominant_break(&self.src))
        };
        let mut expected = self.events.clone();
        expected.splice(it.ev.end..it.ev.end, new);
        self.checked(vec![(it.after..it.after, text)], expected)
    }

    /// Add `key: null` or `- null` at the end of block collection `i`.
    pub fn add_child(&self, i: usize, key: Option<&str>) -> Result<Vec<Edit>, EditError> {
        let it = self.items.get(i).ok_or(EditError::NotApplicable)?;
        if let Some(l) = self.add_inside_lock(i) {
            return Err(EditError::Locked(l));
        }
        let &last = it.children.last().ok_or(EditError::NotApplicable)?;
        self.add_after(last, key)
    }

    /// The edits as char ranges, if the text they produce parses to the
    /// `expected` events.
    fn checked(
        &self,
        mut edits: Vec<(Range<usize>, String)>,
        expected: Vec<Ev>,
    ) -> Result<Vec<Edit>, EditError> {
        edits.sort_by_key(|e| e.0.start);
        let mut out = String::with_capacity(self.src.len() + 64);
        let mut at = 0;
        for (r, text) in &edits {
            let before = self.src.get(at..r.start).ok_or(EditError::NotApplicable)?;
            self.src.get(r.clone()).ok_or(EditError::NotApplicable)?;
            out.push_str(before);
            out.push_str(text);
            at = r.end;
        }
        out.push_str(&self.src[at..]);
        let mut got: Vec<Ev> = tokens(&out)
            .map_err(|_| EditError::NotPreserved)?
            .into_iter()
            .map(|t| t.ev)
            .collect();
        renumber(&mut got);
        if got != expected {
            return Err(EditError::NotPreserved);
        }
        let (mut byte, mut ch) = (0, 0);
        let mut to_char = |b: usize| {
            ch += self.src[byte..b].chars().count();
            byte = b;
            ch
        };
        Ok(edits
            .into_iter()
            .map(|(r, text)| {
                let s = to_char(r.start);
                let e = to_char(r.end);
                Edit { range: s..e, text }
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json::Node;

    /// Apply edits (char ranges, ascending) to `src`.
    fn apply(src: &str, edits: &[Edit]) -> String {
        let at = |c: usize| src.char_indices().nth(c).map_or(src.len(), |(b, _)| b);
        let mut out = String::new();
        let mut last = 0;
        for e in edits {
            let (s, t) = (at(e.range.start), at(e.range.end));
            assert!(s >= last, "edits ascending and apart");
            out.push_str(&src[last..s]);
            out.push_str(&e.text);
            last = t;
        }
        out.push_str(&src[last..]);
        out
    }

    fn at(d: &Doc, path: &[usize]) -> usize {
        d.find(path)
            .unwrap_or_else(|| panic!("no node at {path:?}"))
    }

    /// The result parses; returned for comparing.
    fn ok(src: &str, edits: &[Edit]) -> String {
        let out = apply(src, edits);
        assert!(doc(&out).is_ok(), "{out}");
        out
    }

    const SAMPLE: &str = "# Asetukset\r\nname: Text3   # nimi\r\nversion: 1.2.3\r\nquoted: 'it''s'\r\ndq: \"a\\tb\"  # tab\r\nempty:\r\nlist:\r\n  # first\r\n  - one\r\n  - two: 2\r\n    three: 3 # c3\r\n  - - x\r\n    - y\r\nflow: [1, 2]\r\nbase: &b {x: 1}\r\ncopy: *b\r\ntagged: !!str 5\r\nlit: |\r\n  line1\r\n  line2\r\nafter: z\r\n";

    fn chars(src: &str, r: &Range<usize>) -> String {
        src.chars().skip(r.start).take(r.end - r.start).collect()
    }

    #[test]
    fn spans_and_locks() {
        let d = doc(SAMPLE).unwrap();
        let root = at(&d, &[0]);
        assert_eq!(d.items[root].kind, Kind::Mapping);
        assert_eq!(d.items[root].children.len(), 12);
        let name = at(&d, &[0, 0]);
        assert_eq!(d.key_text(name), Some("name"));
        assert_eq!(d.value_text(name), Some("Text3"));
        let sp = &d.items[name].spans;
        assert_eq!(
            chars(SAMPLE, sp.removal.as_ref().unwrap()),
            "name: Text3   # nimi\r\n"
        );
        let q = at(&d, &[0, 2]);
        assert_eq!(d.value_text(q), Some("'it''s'"));
        assert_eq!(d.value(q).as_deref(), Some("it's"));
        let dq = at(&d, &[0, 3]);
        assert_eq!(d.value_text(dq), Some("\"a\\tb\""), "quote end, not spaces");
        assert_eq!(d.value(dq).as_deref(), Some("a\tb"));
        let empty = at(&d, &[0, 4]);
        assert_eq!(d.value_text(empty), Some(""));
        assert!(d.can_set_value(empty));
        // Sequence items start at their dash.
        let one = at(&d, &[0, 5, 0]);
        assert_eq!(
            chars(SAMPLE, d.items[one].spans.removal.as_ref().unwrap()),
            "  - one\r\n"
        );
        let item = at(&d, &[0, 5, 1]);
        assert_eq!(
            chars(SAMPLE, d.items[item].spans.removal.as_ref().unwrap()),
            "  - two: 2\r\n    three: 3 # c3\r\n"
        );
        // First entry on a `- ` line: up to the next sibling.
        let two = at(&d, &[0, 5, 1, 0]);
        assert_eq!(
            chars(SAMPLE, d.items[two].spans.removal.as_ref().unwrap()),
            "two: 2\r\n    "
        );
        let list = at(&d, &[0, 5]);
        assert_eq!(
            chars(SAMPLE, d.items[list].spans.removal.as_ref().unwrap()),
            "list:\r\n  # first\r\n  - one\r\n  - two: 2\r\n    three: 3 # c3\r\n  - - x\r\n    - y\r\n"
        );
        // Read-only parts.
        let flow = at(&d, &[0, 6]);
        assert!(d.can_delete(flow) && d.can_rename(flow));
        assert_eq!(d.add_inside_lock(flow), Some(Lock::Flow));
        assert_eq!(d.value_text(flow), Some("[1, 2]"));
        let f1 = at(&d, &[0, 6, 0]);
        assert_eq!(d.items[f1].value_lock, Some(Lock::Flow));
        assert!(!d.can_set_value(f1) && !d.can_delete(f1) && !d.can_add_after(f1));
        assert_eq!(d.lock(f1), Some(Lock::Flow));
        let base = at(&d, &[0, 7]);
        assert_eq!(d.value_text(base), Some("&b {x: 1}"));
        assert_eq!(d.items[at(&d, &[0, 8])].value_lock, Some(Lock::Alias));
        assert_eq!(d.items[at(&d, &[0, 9])].value_lock, Some(Lock::Tag));
        let lit = at(&d, &[0, 10]);
        assert_eq!(d.items[lit].value_lock, Some(Lock::BlockScalar));
        assert_eq!(
            chars(SAMPLE, d.items[lit].spans.removal.as_ref().unwrap()),
            "lit: |\r\n  line1\r\n  line2\r\n"
        );
        assert!(d.can_delete(lit));
        assert_eq!(d.path_of(two), vec![0, 5, 1, 0]);
        assert_eq!(d.find(&[0, 99]), None);
    }

    #[test]
    fn set_value_keeps_style_and_type() {
        let d = doc(SAMPLE).unwrap();
        let set = |path: &[usize], v: &str| {
            let e = d.set_value(at(&d, path), v).unwrap().unwrap();
            assert_eq!(e.len(), 1);
            ok(SAMPLE, &e)
        };
        let name = |to: &str| SAMPLE.replace("name: Text3 ", &format!("name: {to} "));
        // A plain string stays a string: the Norway problem and friends.
        assert_eq!(set(&[0, 0], "Text3 2"), name("Text3 2"));
        for (typed, written) in [
            ("no", "\"no\""),
            ("yes", "\"yes\""),
            ("Off", "\"Off\""),
            ("null", "\"null\""),
            ("~", "\"~\""),
            ("1.0", "\"1.0\""),
            ("12", "\"12\""),
            ("-5", "\"-5\""),
            (".5", "\".5\""),
            (".inf", "\".inf\""),
            ("2001-12-14", "\"2001-12-14\""),
            (":x", "\":x\""),
            ("#x", "\"#x\""),
            ("a: b", "\"a: b\""),
            ("a #b", "\"a #b\""),
            ("ends:", "\"ends:\""),
            (" lead", "\" lead\""),
            ("trail ", "\"trail \""),
            ("", "\"\""),
            ("line\nbreak\t", "\"line\\nbreak\\t\""),
            ("- x", "\"- x\""),
            ("---", "\"---\""),
            ("*a", "\"*a\""),
            ("&a", "\"&a\""),
            ("!a", "\"!a\""),
            ("[a]", "\"[a]\""),
            ("{a}", "\"{a}\""),
            ("|", "\"|\""),
            ("'q'", "\"'q'\""),
            ("%x", "\"%x\""),
            ("@x", "\"@x\""),
            ("\u{1}\u{feff}", "\"\\u0001\\uFEFF\""),
        ] {
            assert_eq!(set(&[0, 0], typed), name(written), "{typed:?}");
        }
        for plain in [
            "ünïcode ✓",
            "say \"hi\"\\",
            "a,b",
            "x-y",
            "-x",
            "a:b",
            "a#b",
            "hello world",
        ] {
            assert_eq!(set(&[0, 0], plain), name(plain), "{plain:?}");
        }
        // A plain non-string keeps what is typed when it is one plain scalar.
        let ver = |to: &str| SAMPLE.replace("version: 1.2.3", &format!("version: {to}"));
        assert_eq!(set(&[0, 1], "2.0"), ver("2.0"));
        assert_eq!(set(&[0, 1], "yes"), ver("yes"));
        assert_eq!(set(&[0, 1], "beta"), ver("beta"));
        assert_eq!(set(&[0, 1], "a: b"), ver("\"a: b\""));
        assert_eq!(set(&[0, 1], ""), ver("\"\""));
        // Quoted styles are kept.
        let quoted = |to: &str| SAMPLE.replace("'it''s'", to);
        assert_eq!(set(&[0, 2], "no"), quoted("'no'"));
        assert_eq!(set(&[0, 2], "a'b"), quoted("'a''b'"));
        assert_eq!(set(&[0, 2], ""), quoted("''"));
        assert_eq!(set(&[0, 2], "x\ny"), quoted("\"x\\ny\""));
        assert_eq!(
            set(&[0, 3], "1.0 #"),
            SAMPLE.replace("\"a\\tb\"  # tab", "\"1.0 #\"  # tab")
        );
        // An empty value gets a space.
        assert_eq!(
            set(&[0, 4], "x"),
            SAMPLE.replace("empty:\r\n", "empty: x\r\n")
        );
        assert_eq!(
            set(&[0, 4], "no"),
            SAMPLE.replace("empty:\r\n", "empty: no\r\n"),
            "empty is null, not a string: typed text kept"
        );
        assert_eq!(
            set(&[0, 5, 1, 1], "4"),
            SAMPLE.replace("three: 3 #", "three: 4 #")
        );
        assert_eq!(d.set_value(at(&d, &[0, 0]), "Text3"), Ok(None));
        // The new value reads back as typed and as a string.
        let out = set(&[0, 0], "no");
        let nd = doc(&out).unwrap();
        let n = at(&nd, &[0, 0]);
        assert_eq!(nd.value(n).as_deref(), Some("no"));
        assert_eq!(nd.items[n].value, Some(Sc::Str("no".into())));
    }

    #[test]
    fn empty_sequence_items_and_root_scalars() {
        let src = "a:\n  -\n  - \n  - x\n";
        let d = doc(src).unwrap();
        let e0 = d.set_value(at(&d, &[0, 0, 0]), "v").unwrap().unwrap();
        assert_eq!(ok(src, &e0), "a:\n  - v\n  - \n  - x\n");
        let e1 = d.set_value(at(&d, &[0, 0, 1]), "v").unwrap().unwrap();
        assert_eq!(ok(src, &e1), "a:\n  -\n  - v\n  - x\n");
        let src = "just text # c\n";
        let d = doc(src).unwrap();
        let e = d.set_value(at(&d, &[0]), "no").unwrap().unwrap();
        assert_eq!(ok(src, &e), "\"no\" # c\n");
        assert!(!d.can_delete(at(&d, &[0])));
        assert_eq!(d.delete(at(&d, &[0])), Err(EditError::NotApplicable));
    }

    #[test]
    fn rename_keys() {
        let d = doc(SAMPLE).unwrap();
        let rn = |path: &[usize], v: &str| d.rename_key(at(&d, path), v);
        let out = ok(SAMPLE, &rn(&[0, 0], "nimi").unwrap().unwrap());
        assert_eq!(out, SAMPLE.replace("name: Text3", "nimi: Text3"));
        let out = ok(SAMPLE, &rn(&[0, 0], "no").unwrap().unwrap());
        assert_eq!(out, SAMPLE.replace("name: Text3", "\"no\": Text3"));
        let out = ok(SAMPLE, &rn(&[0, 0], "a: b").unwrap().unwrap());
        assert_eq!(out, SAMPLE.replace("name: Text3", "\"a: b\": Text3"));
        let out = ok(SAMPLE, &rn(&[0, 5, 1, 0], "kaksi").unwrap().unwrap());
        assert_eq!(out, SAMPLE.replace("- two: 2", "- kaksi: 2"));
        assert_eq!(rn(&[0, 0], "version"), Err(EditError::DuplicateKey));
        assert_eq!(rn(&[0, 0], "name"), Ok(None));
        assert_eq!(
            rn(&[0, 5, 0], "x"),
            Err(EditError::NotApplicable),
            "sequence item"
        );
        assert_eq!(rn(&[0, 7, 0], "y"), Err(EditError::Locked(Lock::Flow)));
        let long = "k".repeat(1100);
        assert_eq!(rn(&[0, 0], &long), Err(EditError::InvalidValue));
        // Quoted keys keep their quotes.
        let src = "'a b': 1\n\"c\": 2\n1: x\n";
        let d = doc(src).unwrap();
        let out = ok(
            src,
            &d.rename_key(at(&d, &[0, 0]), "it's").unwrap().unwrap(),
        );
        assert_eq!(out, "'it''s': 1\n\"c\": 2\n1: x\n");
        let out = ok(src, &d.rename_key(at(&d, &[0, 1]), "d").unwrap().unwrap());
        assert_eq!(out, "'a b': 1\n\"d\": 2\n1: x\n");
        let out = ok(src, &d.rename_key(at(&d, &[0, 2]), "2").unwrap().unwrap());
        assert_eq!(
            out, "'a b': 1\n\"c\": 2\n2: x\n",
            "a number key stays as typed"
        );
        // `1` and `"1"` are different keys.
        let src = "1: a\nb: c\n";
        let d = doc(src).unwrap();
        assert_eq!(
            d.rename_key(at(&d, &[0, 1]), "1").map(|e| e.is_some()),
            Ok(true),
            "renamed to the string \"1\""
        );
        let src = "\"1\": a\n2: c\n";
        let d = doc(src).unwrap();
        let out = ok(src, &d.rename_key(at(&d, &[0, 1]), "1").unwrap().unwrap());
        assert_eq!(
            out, "\"1\": a\n1: c\n",
            "the number 1 next to the string \"1\""
        );
    }

    #[test]
    fn delete_entries_with_their_lines() {
        let d = doc(SAMPLE).unwrap();
        let del = |path: &[usize]| ok(SAMPLE, &d.delete(at(&d, path)).unwrap());
        assert_eq!(del(&[0, 0]), SAMPLE.replace("name: Text3   # nimi\r\n", ""));
        assert_eq!(
            del(&[0, 5, 0]),
            SAMPLE.replace("  - one\r\n", ""),
            "the comment above stays"
        );
        assert_eq!(
            del(&[0, 5, 1, 0]),
            SAMPLE.replace("  - two: 2\r\n    three", "  - three")
        );
        assert_eq!(
            del(&[0, 5, 1, 1]),
            SAMPLE.replace("    three: 3 # c3\r\n", "")
        );
        assert_eq!(
            del(&[0, 5, 2, 0]),
            SAMPLE.replace("  - - x\r\n    - y", "  - - y")
        );
        assert_eq!(
            del(&[0, 5]),
            SAMPLE.replace(
                "list:\r\n  # first\r\n  - one\r\n  - two: 2\r\n    three: 3 # c3\r\n  - - x\r\n    - y\r\n",
                ""
            )
        );
        assert_eq!(
            del(&[0, 10]),
            SAMPLE.replace("lit: |\r\n  line1\r\n  line2\r\n", "")
        );
        assert_eq!(del(&[0, 6]), SAMPLE.replace("flow: [1, 2]\r\n", ""));
        // The anchor `copy` refers to: refused, the alias would dangle.
        assert_eq!(d.delete(at(&d, &[0, 7])), Err(EditError::NotPreserved));
        // The alias itself can go.
        assert_eq!(del(&[0, 8]), SAMPLE.replace("copy: *b\r\n", ""));
        // The last line without a break takes the break before it.
        let src = "a: 1\nb: 2";
        let d = doc(src).unwrap();
        assert_eq!(ok(src, &d.delete(at(&d, &[0, 1])).unwrap()), "a: 1");
        assert_eq!(ok(src, &d.delete(at(&d, &[0, 0])).unwrap()), "b: 2");
        // An only child would turn the collection into null.
        let src = "a:\n  b: 1\nc:\n  - x\n";
        let d = doc(src).unwrap();
        assert_eq!(
            d.delete(at(&d, &[0, 0, 0])),
            Err(EditError::Locked(Lock::OnlyChild))
        );
        assert_eq!(
            d.delete(at(&d, &[0, 1, 0])),
            Err(EditError::Locked(Lock::OnlyChild))
        );
        // A comment between a `- key` line and the next entry: refused.
        let src = "- a: 1\n  # keep\n  b: 2\n- c\n";
        let d = doc(src).unwrap();
        assert_eq!(
            d.delete(at(&d, &[0, 0, 0])),
            Err(EditError::Locked(Lock::Layout))
        );
        let out = ok(src, &d.delete(at(&d, &[0, 0, 1])).unwrap());
        assert_eq!(out, "- a: 1\n  # keep\n- c\n");
    }

    #[test]
    fn add_entries_at_the_same_indentation() {
        let d = doc(SAMPLE).unwrap();
        let three = at(&d, &[0, 5, 1, 1]);
        let p = d.items[three].parent.unwrap();
        let key = d.unique_key(p, "three");
        assert_eq!(key, "three2");
        let out = ok(SAMPLE, &d.add_after(three, Some(&key)).unwrap());
        assert_eq!(
            out,
            SAMPLE.replace("three: 3 # c3\r\n", "three: 3 # c3\r\n    three2: null\r\n")
        );
        let out = ok(
            SAMPLE,
            &d.add_after(at(&d, &[0, 5, 1, 0]), Some("k")).unwrap(),
        );
        assert_eq!(
            out,
            SAMPLE.replace("  - two: 2\r\n", "  - two: 2\r\n    k: null\r\n")
        );
        let out = ok(SAMPLE, &d.add_after(at(&d, &[0, 5, 0]), None).unwrap());
        assert_eq!(
            out,
            SAMPLE.replace("  - one\r\n", "  - one\r\n  - null\r\n")
        );
        let out = ok(SAMPLE, &d.add_after(at(&d, &[0, 5, 2, 0]), None).unwrap());
        assert_eq!(
            out,
            SAMPLE.replace("  - - x\r\n", "  - - x\r\n    - null\r\n")
        );
        let out = ok(SAMPLE, &d.add_child(at(&d, &[0, 5]), None).unwrap());
        assert_eq!(
            out,
            SAMPLE.replace("    - y\r\n", "    - y\r\n  - null\r\n")
        );
        let out = ok(SAMPLE, &d.add_after(at(&d, &[0, 10]), Some("key")).unwrap());
        assert_eq!(
            out,
            SAMPLE.replace("  line2\r\n", "  line2\r\nkey: null\r\n")
        );
        let out = ok(SAMPLE, &d.add_child(at(&d, &[0]), Some("no")).unwrap());
        assert_eq!(out, format!("{SAMPLE}\"no\": null\r\n"));
        assert_eq!(
            d.add_after(at(&d, &[0, 0]), Some("version")),
            Err(EditError::DuplicateKey)
        );
        assert_eq!(
            d.add_after(at(&d, &[0, 0]), None),
            Err(EditError::NotApplicable)
        );
        assert_eq!(
            d.add_child(at(&d, &[0, 6]), None),
            Err(EditError::Locked(Lock::Flow))
        );
        assert_eq!(
            d.add_after(at(&d, &[0, 6, 0]), None),
            Err(EditError::Locked(Lock::Flow))
        );
        // At the end of a text without a final break.
        let src = "a: 1\r\nb: 2";
        let d = doc(src).unwrap();
        let out = ok(src, &d.add_after(at(&d, &[0, 1]), Some("c")).unwrap());
        assert_eq!(out, "a: 1\r\nb: 2\r\nc: null");
    }

    #[test]
    fn refused_and_read_only_cases() {
        let d = doc(SAMPLE).unwrap();
        let sv = |path: &[usize]| d.set_value(at(&d, path), "v");
        assert_eq!(sv(&[0, 6, 0]), Err(EditError::Locked(Lock::Flow)));
        assert_eq!(sv(&[0, 8]), Err(EditError::Locked(Lock::Alias)));
        assert_eq!(sv(&[0, 9]), Err(EditError::Locked(Lock::Tag)));
        assert_eq!(sv(&[0, 10]), Err(EditError::Locked(Lock::BlockScalar)));
        assert_eq!(sv(&[0, 5]), Err(EditError::NotApplicable));
        let src = "a: &x 1\n&k b: 2\n!t c: 3\n? d\n: 4\n[e]: 5\n---\nz: 1\n";
        let d = doc(src).unwrap();
        let (doc0, doc1) = (at(&d, &[0]), at(&d, &[1]));
        assert_eq!(d.items[0].children.len(), 2);
        let a = d.items[doc0].children[0];
        assert_eq!(d.set_value(a, "2"), Err(EditError::Locked(Lock::Anchor)));
        assert!(d.can_rename(a) && d.can_delete(a));
        let b = d.items[doc0].children[1];
        assert_eq!(d.rename_key(b, "x"), Err(EditError::Locked(Lock::Anchor)));
        let c = d.items[doc0].children[2];
        assert_eq!(d.rename_key(c, "x"), Err(EditError::Locked(Lock::Tag)));
        let q = d.items[doc0].children[3];
        assert_eq!(
            d.rename_key(q, "x"),
            Err(EditError::Locked(Lock::ComplexKey))
        );
        assert_eq!(d.delete(q), Err(EditError::Locked(Lock::ComplexKey)));
        assert!(d.can_set_value(q), "the value of an explicit entry");
        let fk = d.items[doc0].children[4];
        assert_eq!(d.key_text(fk), None);
        assert_eq!(
            d.rename_key(fk, "x"),
            Err(EditError::Locked(Lock::ComplexKey))
        );
        let z = d.items[doc1].children[0];
        assert_eq!(
            d.set_value(z, "2"),
            Err(EditError::Locked(Lock::OtherDocument))
        );
        assert_eq!(d.delete(z), Err(EditError::Locked(Lock::OtherDocument)));
        assert_eq!(d.lock(z), Some(Lock::OtherDocument));
        // Editing the first document keeps the second.
        let out = ok(src, &d.delete(a).unwrap());
        assert!(out.ends_with("---\nz: 1\n"));
        assert!(doc("a: [1\n").is_err());
    }

    #[test]
    fn crlf_bom_and_unicode_offsets() {
        let src = "\u{feff}ä: ö # ü\r\nb:\r\n  - å\r\n";
        let d = doc(src).unwrap();
        let a = at(&d, &[0, 0]);
        assert_eq!(d.items[a].spans.key, Some(1..2));
        assert_eq!(d.items[a].spans.value, Some(4..5));
        let out = ok(src, &d.set_value(a, "yes").unwrap().unwrap());
        assert_eq!(out, "\u{feff}ä: \"yes\" # ü\r\nb:\r\n  - å\r\n");
        let out = ok(src, &d.delete(a).unwrap());
        assert_eq!(out, "\u{feff}b:\r\n  - å\r\n");
        let out = ok(src, &d.add_after(at(&d, &[0, 1, 0]), None).unwrap());
        assert_eq!(out, "\u{feff}ä: ö # ü\r\nb:\r\n  - å\r\n  - null\r\n");
        let out = ok(src, &d.rename_key(a, "äö").unwrap().unwrap());
        assert_eq!(out, "\u{feff}äö: ö # ü\r\nb:\r\n  - å\r\n");
    }

    #[test]
    fn dash_alone_on_its_line() {
        let src = "l:\n  -   # c\n    b: 1\n    c: 2\n  - d\n";
        let d = doc(src).unwrap();
        let item = at(&d, &[0, 0, 0]);
        assert_eq!(
            chars(src, d.items[item].spans.removal.as_ref().unwrap()),
            "  -   # c\n    b: 1\n    c: 2\n"
        );
        let out = ok(src, &d.delete(item).unwrap());
        assert_eq!(out, "l:\n  - d\n");
        let out = ok(src, &d.add_after(at(&d, &[0, 0, 0, 1]), Some("e")).unwrap());
        assert_eq!(
            out,
            "l:\n  -   # c\n    b: 1\n    c: 2\n    e: null\n  - d\n"
        );
    }

    #[test]
    fn big_and_hostile_inputs_stay_bounded() {
        let mut src = String::from("a: &a [x, x, x, x, x, x, x, x, x]\n");
        let mut prev = 'a';
        for c in 'b'..='j' {
            src.push_str(&format!("{c}: &{c} [*{prev}, *{prev}, *{prev}, *{prev}, *{prev}, *{prev}, *{prev}, *{prev}, *{prev}]\n"));
            prev = c;
        }
        let d = doc(&src).unwrap();
        assert_eq!(d.items[at(&d, &[0])].children.len(), 10);
        let deep = format!("{}{}", "[".repeat(50_000), "]".repeat(50_000));
        let _ = doc(&deep);
        assert!(doc("").unwrap().items[0].children.is_empty());
    }

    // Random block-style documents and edits.

    #[derive(Debug, Clone)]
    enum M {
        Map(Vec<M>),
        Seq(Vec<M>),
        S(usize),
    }

    const KEYS: [&str; 5] = ["alpha", "'b k'", "\"c\"", "ä-1", "on"];
    const SCALARS: [&str; 10] = [
        "plain",
        "'it''s'",
        "\"a\\tb\"",
        "42",
        "yes",
        "~",
        "",
        "x y z",
        "\"#no\"",
        "ünï ✓",
    ];
    const VALUES: [&str; 22] = [
        "no",
        "yes",
        "1.0",
        ":x",
        "#x",
        " lead",
        "trail ",
        "",
        "ünï ✓",
        "a: b",
        "a #b",
        "plain",
        "it's",
        "say \"hi\"",
        "line\nbreak",
        "null",
        "- x",
        "[x]",
        "*a",
        "---",
        "x,y",
        "\u{85}nel",
    ];

    fn arb_m() -> impl proptest::strategy::Strategy<Value = M> {
        use proptest::prelude::*;
        let leaf = (0..SCALARS.len()).prop_map(M::S);
        leaf.prop_recursive(4, 40, 4, |inner| {
            prop_oneof![
                proptest::collection::vec(inner.clone(), 1..5).prop_map(M::Map),
                proptest::collection::vec(inner, 1..4).prop_map(M::Seq),
            ]
        })
    }

    struct Out<'a> {
        s: String,
        nl: &'a str,
        n: usize,
        seed: u64,
    }

    impl Out<'_> {
        fn coin(&mut self, k: u64) -> bool {
            self.seed = self
                .seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (self.seed >> 33).is_multiple_of(k)
        }
        fn comment_line(&mut self, indent: usize) {
            if self.coin(4) {
                self.n += 1;
                self.s
                    .push_str(&format!("{}# c{}{}", " ".repeat(indent), self.n, self.nl));
            }
        }
        fn end_line(&mut self) {
            if self.coin(3) {
                self.n += 1;
                self.s.push_str(&format!("  # c{}", self.n));
            }
            self.s.push_str(self.nl);
        }
        fn map(&mut self, entries: &[M], indent: usize, inline: bool) {
            for (k, v) in entries.iter().enumerate() {
                if !(k == 0 && inline) {
                    self.comment_line(indent);
                    self.s.push_str(&" ".repeat(indent));
                }
                self.s.push_str(KEYS[k]);
                self.s.push(':');
                self.value(v, indent);
            }
        }
        fn value(&mut self, v: &M, indent: usize) {
            match v {
                M::S(x) => {
                    if !SCALARS[*x].is_empty() {
                        self.s.push(' ');
                        self.s.push_str(SCALARS[*x]);
                    }
                    self.end_line();
                }
                M::Map(m) => {
                    self.end_line();
                    self.map(m, indent + 2, false);
                }
                M::Seq(items) => {
                    self.end_line();
                    let i = if self.coin(2) { indent } else { indent + 2 };
                    self.seq(items, i, false);
                }
            }
        }
        fn seq(&mut self, items: &[M], indent: usize, inline: bool) {
            for (k, v) in items.iter().enumerate() {
                if !(k == 0 && inline) {
                    self.comment_line(indent);
                    self.s.push_str(&" ".repeat(indent));
                }
                self.s.push('-');
                match v {
                    M::Map(m) => {
                        self.s.push(' ');
                        self.map(m, indent + 2, true);
                    }
                    M::Seq(inner) if self.coin(2) => {
                        self.s.push(' ');
                        self.seq(inner, indent + 2, true);
                    }
                    _ => self.value(v, indent),
                }
            }
        }
    }

    fn node_at<'a>(n: &'a mut Node, path: &[usize]) -> &'a mut Node {
        let mut n = n;
        for &k in path {
            n = match n {
                Node::Object(m) => &mut m[k].1,
                Node::Array(a) => &mut a[k],
                Node::Scalar(_) => unreachable!(),
            };
        }
        n
    }

    fn remove(n: &mut Node, path: &[usize]) {
        let (last, up) = path.split_last().unwrap();
        match node_at(n, up) {
            Node::Object(m) => {
                m.remove(*last);
            }
            Node::Array(a) => {
                a.remove(*last);
            }
            Node::Scalar(_) => unreachable!(),
        }
    }

    fn insert(n: &mut Node, path: &[usize], at: usize, key: Option<String>) {
        match node_at(n, path) {
            Node::Object(m) => m.insert(at, (key.unwrap(), Node::Scalar("null".into()))),
            Node::Array(a) => a.insert(at, Node::Scalar("null".into())),
            Node::Scalar(_) => unreachable!(),
        }
    }

    fn comments(s: &str) -> usize {
        s.matches("# c").count()
    }

    proptest::proptest! {
        #![proptest_config(proptest::prelude::ProptestConfig::with_cases(400))]
        #[test]
        fn random_edits_give_the_expected_tree(
            root in proptest::collection::vec(arb_m(), 1..5),
            crlf in proptest::bool::ANY,
            final_break in proptest::bool::ANY,
            seed in proptest::num::u64::ANY,
            ops in proptest::collection::vec((0..5usize, 0..200usize, 0..VALUES.len()), 1..5),
        ) {
            let nl = if crlf { "\r\n" } else { "\n" };
            let mut o = Out { s: String::new(), nl, n: 0, seed };
            o.map(&root, 0, false);
            let mut src = o.s;
            if !final_break {
                src.truncate(src.len() - nl.len());
            }
            let d = doc(&src).map_err(|e| proptest::test_runner::TestCaseError::fail(format!("{e:?}\n{src}")))?;
            for &(op, pick, v) in &ops {
                let i = 1 + pick % (d.items.len() - 1);
                let path = d.path_of(i);
                let value = VALUES[v];
                let mut want = crate::markup::yaml_tree(&src).unwrap();
                let tpath = &path[1..];
                let p = d.items[i].parent.unwrap();
                let pos = *path.last().unwrap();
                let result = match op {
                    0 => {
                        let r = d.set_value(i, value);
                        if d.can_set_value(i) {
                            *node_at(&mut want, tpath) = Node::Scalar(value.into());
                            proptest::prop_assert!(r.is_ok(), "{:?} {:?} {}", r, value, src);
                        } else {
                            proptest::prop_assert!(r.is_err());
                        }
                        r.ok().flatten()
                    }
                    1 => {
                        let r = d.rename_key(i, value);
                        if d.can_rename(i) {
                            let taken = d.items[p].children.iter().any(|&c| c != i && d.key(c).as_deref() == Some(value));
                            if let Node::Object(m) = node_at(&mut want, &tpath[..tpath.len() - 1]) {
                                m[pos].0 = value.into();
                            }
                            if taken {
                                proptest::prop_assert!(r.is_err());
                            } else {
                                proptest::prop_assert!(r.is_ok(), "{:?} {:?} {}", r, value, src);
                            }
                        } else {
                            proptest::prop_assert!(r.is_err());
                        }
                        r.ok().flatten()
                    }
                    2 => {
                        let r = d.delete(i);
                        if d.can_delete(i) {
                            remove(&mut want, tpath);
                            proptest::prop_assert!(r.is_ok(), "{:?} {}", r, src);
                        } else {
                            proptest::prop_assert!(r.is_err());
                        }
                        r.ok()
                    }
                    3 => {
                        let key = (d.items[p].kind == Kind::Mapping).then(|| d.unique_key(p, "key"));
                        let r = d.add_after(i, key.as_deref());
                        if d.can_add_after(i) {
                            insert(&mut want, &tpath[..tpath.len() - 1], pos + 1, key);
                            proptest::prop_assert!(r.is_ok(), "{:?} {}", r, src);
                        } else {
                            proptest::prop_assert!(r.is_err());
                        }
                        r.ok()
                    }
                    _ => {
                        let map = d.items[i].kind == Kind::Mapping;
                        let key = map.then(|| d.unique_key(i, "key"));
                        let r = d.add_child(i, key.as_deref());
                        if d.can_add_inside(i) {
                            let n = d.items[i].children.len();
                            insert(&mut want, tpath, n, key);
                            proptest::prop_assert!(r.is_ok(), "{:?} {}", r, src);
                        } else {
                            proptest::prop_assert!(r.is_err());
                        }
                        r.ok()
                    }
                };
                let Some(edits) = result else { continue };
                let out = apply(&src, &edits);
                // Bytes outside the edits are unchanged.
                let n_chars = src.chars().count();
                for e in &edits {
                    proptest::prop_assert!(e.range.start <= e.range.end && e.range.end <= n_chars);
                }
                let first = edits[0].range.start;
                let last = edits[edits.len() - 1].range.end;
                let pre: String = src.chars().take(first).collect();
                let post: String = src.chars().skip(last).collect();
                proptest::prop_assert!(out.starts_with(&pre) && out.ends_with(&post));
                let got = crate::markup::yaml_tree(&out);
                proptest::prop_assert!(got.is_ok(), "{} -> {}", src, out);
                proptest::prop_assert_eq!(got.unwrap(), want, "{} -> {}", src, out);
                let nd = doc(&out).unwrap();
                // Comments are kept, except on removed lines.
                let removed: String = src.chars().skip(first).take(last - first).collect();
                let lost = if op == 2 { comments(&removed) } else { 0 };
                proptest::prop_assert_eq!(comments(&out), comments(&src) - lost, "{} -> {}", src, out);
                if op == 0 && d.items[i].value.as_ref().is_some_and(|v| matches!(v, Sc::Str(_))) {
                    let j = nd.find(&path).unwrap();
                    proptest::prop_assert_eq!(&nd.items[j].value, &Some(Sc::Str(value.into())));
                }
            }
        }
    }
}
