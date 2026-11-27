//! An editable XML tree over its source text.
//!
//! Every node knows the exact characters it came from: an element's name
//! in its start and end tag, each attribute (name, quoted value, and the
//! whole attribute with its leading whitespace), text nodes, and the whole
//! element. Each operation returns the smallest replacements that make the
//! change (one, or two for renaming an element's start and end tags), the
//! edited text is parsed again and refused if it is no longer well formed,
//! and nothing else is rewritten: indentation, line endings, quote styles,
//! entity and character references, CDATA sections, comments and the
//! declaration stay byte for byte.
//!
//! Safety is as in [`crate::markup`]: entities are never expanded, DTDs are
//! ignored and the builder is iterative with a node cap.
//!
//! Names are checked against the XML 1.0 `Name` production (all of its
//! Unicode ranges), with the namespace rule on top that a name holds at
//! most one colon and neither starts nor ends with one. Namespace prefixes
//! are not checked against declarations.

use std::collections::HashSet;
use std::ops::Range;

pub use crate::csv::Edit;
use crate::markup::{MAX_NODES, TreeError, position, too_large};

/// What a node of the tree is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// The whole document (`items[0]`); its children are the top-level
    /// elements.
    Document,
    Element,
    Attribute,
    /// Character data in an element that also holds markup (mixed
    /// content): a run of text, references and CDATA between tags.
    Text,
}

/// Why an edit was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditError {
    /// Not an XML name (see the module documentation).
    InvalidName,
    /// The element already has an attribute with this name.
    DuplicateName,
    /// The result would not be well-formed XML (or holds a character XML
    /// cannot represent).
    NotWellFormed,
    /// The operation does not apply to this node.
    NotApplicable,
}

/// Source ranges of a node. In [`Item::spans`] they count characters (what
/// the editor's rope uses).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Spans {
    /// Element: its name in the start tag. Attribute: its name.
    pub name: Option<Range<usize>>,
    /// Element: its name in the end tag; `None` for an empty element
    /// (`<a/>`).
    pub end_name: Option<Range<usize>>,
    /// Attribute: the value with its quotes. Element: the content between
    /// its tags (`None` for `<a/>`). Text node: the text.
    pub value: Option<Range<usize>>,
    /// Element: start tag to end tag. Attribute: with its leading
    /// whitespace. Text node: the text.
    pub whole: Range<usize>,
    /// What deleting the node removes: the attribute with its leading
    /// whitespace; the element, plus the line break and indentation before
    /// it (and spaces after it) when it sits on a line of its own.
    pub removal: Range<usize>,
}

/// One node of an editable XML tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
    pub parent: Option<usize>,
    pub kind: Kind,
    /// Attributes first, then child elements and text nodes in source order.
    pub children: Vec<usize>,
    /// Element without child markup (only text, references and CDATA, or
    /// nothing): its text is edited as a whole and it has no text nodes.
    pub text_only: bool,
    /// Char ranges.
    pub spans: Spans,
    /// The same ranges in bytes.
    b: Spans,
    /// Element: its attributes could be located in the start tag.
    attrs_ok: bool,
}

impl Item {
    fn new(parent: Option<usize>, kind: Kind, whole: Range<usize>) -> Self {
        Item {
            parent,
            kind,
            children: Vec::new(),
            text_only: false,
            spans: Spans::default(),
            b: Spans {
                removal: whole.clone(),
                whole,
                ..Spans::default()
            },
            attrs_ok: true,
        }
    }
}

/// An editable XML tree. `items[0]` is the document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Doc {
    src: String,
    pub items: Vec<Item>,
    /// The source passes the full checks (one root element, nothing but
    /// whitespace outside it, valid names, unique attributes): edits must
    /// keep it so. Otherwise they only must not break tag nesting.
    strict: bool,
    /// Names of entity references other than the predefined ones, as used
    /// in the source: typing `&name;` keeps such a reference.
    refs: HashSet<String>,
    /// One level of indentation, guessed from the source (`  ` if none).
    unit: String,
}

const XML_WS: [char; 4] = [' ', '\t', '\n', '\r'];

/// An element being read: its item, where its content starts, whether it
/// holds child markup and the text run not yet made a node.
struct Frame {
    item: usize,
    content_start: usize,
    markup: bool,
    run: Option<Range<usize>>,
}

struct Builder<'a> {
    src: &'a str,
    items: Vec<Item>,
    stack: Vec<Frame>,
}

impl Builder<'_> {
    fn add(&mut self, item: Item) -> usize {
        let parent = item.parent;
        self.items.push(item);
        let i = self.items.len() - 1;
        if let Some(p) = parent {
            self.items[p].children.push(i);
        }
        i
    }

    /// Make the pending text run of the innermost element a node, if it
    /// holds more than whitespace.
    fn flush(&mut self) {
        let Some(f) = self.stack.last_mut() else {
            return;
        };
        let Some(run) = f.run.take() else {
            return;
        };
        let item = f.item;
        if !self.src[run.clone()].trim_matches(XML_WS).is_empty() {
            let mut t = Item::new(Some(item), Kind::Text, run.clone());
            t.b.value = Some(run);
            self.add(t);
        }
    }

    /// Markup inside the innermost element ends its text run.
    fn markup(&mut self) {
        self.flush();
        if let Some(f) = self.stack.last_mut() {
            f.markup = true;
        }
    }

    fn element(&mut self, tag: Range<usize>, name_len: usize, empty: bool) {
        self.markup();
        let parent = self.stack.last().map_or(0, |f| f.item);
        let mut e = Item::new(Some(parent), Kind::Element, tag.clone());
        e.b.name = Some(tag.start + 1..tag.start + 1 + name_len);
        e.text_only = true;
        let attrs = scan_attrs(&self.src[tag.clone()], name_len);
        e.attrs_ok = attrs.is_some();
        let i = self.add(e);
        for a in attrs.unwrap_or_default() {
            let at = |r: Range<usize>| tag.start + r.start..tag.start + r.end;
            let mut it = Item::new(Some(i), Kind::Attribute, at(a.whole));
            it.b.name = Some(at(a.name));
            it.b.value = Some(at(a.value));
            self.add(it);
        }
        if !empty {
            self.stack.push(Frame {
                item: i,
                content_start: tag.end,
                markup: false,
                run: None,
            });
        }
    }
}

/// Parse `src` into an editable tree without recursion. Accepts what the
/// read-only [`crate::markup::xml_tree`] accepts.
pub fn doc(src: &str) -> Result<Doc, TreeError> {
    use quick_xml::events::Event;
    // quick-xml skips a leading BOM, which would shift every offset.
    let base = if src.starts_with('\u{feff}') { 3 } else { 0 };
    let mut reader = quick_xml::Reader::from_str(&src[base..]);
    reader.config_mut().check_end_names = true;
    let mut b = Builder {
        src,
        items: vec![Item::new(None, Kind::Document, 0..src.len())],
        stack: Vec::new(),
    };
    let mut refs = HashSet::new();
    let err = |byte: usize, message: String| {
        let (line, column) = position(src, byte);
        TreeError {
            line,
            column,
            message,
        }
    };
    let mut events = 0usize;
    loop {
        let start = reader.buffer_position() as usize + base;
        let event = reader
            .read_event()
            .map_err(|e| err(reader.error_position() as usize + base, e.to_string()))?;
        let end = reader.buffer_position() as usize + base;
        events += 1;
        if events > MAX_NODES || b.items.len() > MAX_NODES {
            return Err(too_large(src, start));
        }
        match event {
            Event::Start(e) => b.element(start..end, e.name().as_ref().len(), false),
            Event::Empty(e) => b.element(start..end, e.name().as_ref().len(), true),
            Event::End(e) => {
                b.flush_if_mixed();
                let Some(f) = b.stack.pop() else {
                    return Err(err(start, "unexpected closing tag".into()));
                };
                let it = &mut b.items[f.item];
                let n = e.name().as_ref().len();
                it.b.end_name = Some(start + 2..start + 2 + n);
                it.b.whole.end = end;
                it.b.value = Some(f.content_start..start);
                it.text_only = !f.markup;
            }
            Event::Text(_) | Event::CData(_) | Event::GeneralRef(_) => {
                if let Event::GeneralRef(r) = &event {
                    let name = r.xml10_content().into_owned();
                    if decode_ref(&name).is_none() {
                        refs.insert(name);
                    }
                }
                if let Some(f) = b.stack.last_mut() {
                    f.run = Some(f.run.take().map_or(start, |r| r.start)..end);
                }
            }
            Event::Eof => break,
            // Comments, processing instructions, declarations, DOCTYPE.
            _ => b.markup(),
        }
    }
    if let Some(f) = b.stack.last() {
        let name = b.items[f.item].b.name.clone().map_or("", |r| &src[r]);
        let (line, column) = position(src, src.len());
        return Err(TreeError {
            line,
            column,
            message: format!("unclosed element <{name}>"),
        });
    }
    let mut items = b.items;
    // Attribute values may hold references too.
    for it in &items {
        if it.kind == Kind::Attribute
            && let Some(v) = &it.b.value
        {
            for u in units(src, v.start + 1..v.end - 1, false).0 {
                if let Dec::Raw = u.dec {
                    refs.insert(src[u.raw.start + 1..u.raw.end - 1].to_owned());
                }
            }
        }
    }
    for it in &mut items {
        if it.kind == Kind::Element && it.parent != Some(0) {
            it.b.removal = element_removal(src, &it.b.whole);
        }
    }
    let unit = indent_unit(src, &items).unwrap_or_else(|| "  ".to_owned());
    char_spans(src, &mut items);
    Ok(Doc {
        strict: well_formed(src, true),
        src: src.to_owned(),
        items,
        refs,
        unit,
    })
}

impl Builder<'_> {
    /// At an end tag: the run becomes a node only in mixed content.
    fn flush_if_mixed(&mut self) {
        if self.stack.last().is_some_and(|f| f.markup) {
            self.flush();
        }
    }
}

/// The line break and indentation before byte `at`, when only spaces and
/// tabs stand between them; the break is empty at the start of the file.
fn line_start(src: &str, at: usize) -> Option<(Range<usize>, &str)> {
    let before = &src[..at];
    let ind = before.len() - before.trim_end_matches([' ', '\t']).len();
    let s = at - ind;
    let head = &src[..s];
    let brk = if head.ends_with("\r\n") {
        2
    } else if head.ends_with(['\n', '\r']) {
        1
    } else if head.is_empty() || head == "\u{feff}" {
        0
    } else {
        return None;
    };
    Some((s - brk..s, &src[s..at]))
}

fn element_removal(src: &str, whole: &Range<usize>) -> Range<usize> {
    let Some((brk, _)) = line_start(src, whole.start) else {
        return whole.clone();
    };
    if brk.is_empty() {
        return whole.clone();
    }
    let after = &src[whole.end..];
    let spaces = after.len() - after.trim_start_matches([' ', '\t']).len();
    let rest = &after[spaces..];
    if rest.is_empty() || rest.starts_with(['\n', '\r']) {
        brk.start..whole.end + spaces
    } else if rest.starts_with('<') {
        brk.start..whole.end
    } else {
        // Text follows on the same line: the break before is content.
        whole.clone()
    }
}

/// One level of indentation: the first child element on its own line whose
/// indentation extends its parent's.
fn indent_unit(src: &str, items: &[Item]) -> Option<String> {
    for it in items {
        let (Kind::Element, Some(p)) = (it.kind, it.parent) else {
            continue;
        };
        if items[p].kind != Kind::Element {
            continue;
        }
        let (Some((brk, ci)), Some((_, pi))) = (
            line_start(src, it.b.whole.start),
            line_start(src, items[p].b.whole.start),
        ) else {
            continue;
        };
        if !brk.is_empty() && ci.len() > pi.len() && ci.starts_with(pi) {
            return Some(ci[pi.len()..].to_owned());
        }
    }
    None
}

/// Fill [`Item::spans`] from the byte spans with one pass over the text.
fn char_spans(src: &str, items: &mut [Item]) {
    let mut offs: Vec<usize> = Vec::new();
    for it in items.iter() {
        let s = &it.b;
        for r in [&s.name, &s.end_name, &s.value].into_iter().flatten() {
            offs.extend([r.start, r.end]);
        }
        offs.extend([s.whole.start, s.whole.end, s.removal.start, s.removal.end]);
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
        let s = &it.b;
        it.spans = Spans {
            name: s.name.as_ref().map(cr),
            end_name: s.end_name.as_ref().map(cr),
            value: s.value.as_ref().map(cr),
            whole: cr(&s.whole),
            removal: cr(&s.removal),
        };
    }
}

/// Attribute positions in a start tag, relative to it.
#[derive(Debug, Clone, PartialEq, Eq)]
struct AttrSpan {
    whole: Range<usize>,
    name: Range<usize>,
    value: Range<usize>,
}

/// Locate the attributes of a start or empty-element tag (`<` to `>`).
/// `None` if the tag is not `<name (S attr)* S? /?>`.
fn scan_attrs(tag: &str, name_len: usize) -> Option<Vec<AttrSpan>> {
    let b = tag.as_bytes();
    let ws = |c: u8| matches!(c, b' ' | b'\t' | b'\n' | b'\r');
    let close = if tag.ends_with("/>") {
        tag.len() - 2
    } else {
        tag.len().checked_sub(1)?
    };
    let mut i = 1 + name_len;
    let mut out = Vec::new();
    loop {
        let s = i;
        while i < close && ws(b[i]) {
            i += 1;
        }
        if i >= close {
            return (i == close).then_some(out);
        }
        if i == s {
            return None;
        }
        let n = i;
        while i < close && !ws(b[i]) && b[i] != b'=' {
            i += 1;
        }
        if i == n {
            return None;
        }
        let name = n..i;
        while i < close && ws(b[i]) {
            i += 1;
        }
        if b.get(i) != Some(&b'=') {
            return None;
        }
        i += 1;
        while i < close && ws(b[i]) {
            i += 1;
        }
        let q = *b.get(i)?;
        if q != b'"' && q != b'\'' {
            return None;
        }
        let end = i + 1 + tag[i + 1..close].find(q as char)?;
        out.push(AttrSpan {
            whole: s..end + 1,
            name,
            value: i..end + 1,
        });
        i = end + 1;
    }
}

/// The character a predefined entity or character reference stands for.
fn decode_ref(name: &str) -> Option<char> {
    match name {
        "amp" => Some('&'),
        "lt" => Some('<'),
        "gt" => Some('>'),
        "quot" => Some('"'),
        "apos" => Some('\''),
        _ => {
            let n = name.strip_prefix('#')?;
            let v = match n.strip_prefix('x') {
                Some(h) => u32::from_str_radix(h, 16).ok()?,
                None => n.parse().ok()?,
            };
            char::from_u32(v).filter(|&c| is_xml_char(c))
        }
    }
}

/// Whether `name` (between `&` and `;`) is a well-formed reference.
fn is_ref(name: &str) -> bool {
    match name.strip_prefix('#') {
        Some(n) => decode_ref(name).is_some() && !n.is_empty(),
        None => is_name(name),
    }
}

/// XML 1.0 `Char`: what a document may hold, literally or as a reference.
fn is_xml_char(c: char) -> bool {
    matches!(c, '\t' | '\n' | '\r' | ' '..='\u{d7ff}' | '\u{e000}'..='\u{fffd}' | '\u{10000}'..)
}

fn is_name_start(c: char) -> bool {
    matches!(c,
        'A'..='Z' | '_' | 'a'..='z' | '\u{c0}'..='\u{d6}' | '\u{d8}'..='\u{f6}'
        | '\u{f8}'..='\u{2ff}' | '\u{370}'..='\u{37d}' | '\u{37f}'..='\u{1fff}'
        | '\u{200c}'..='\u{200d}' | '\u{2070}'..='\u{218f}' | '\u{2c00}'..='\u{2fef}'
        | '\u{3001}'..='\u{d7ff}' | '\u{f900}'..='\u{fdcf}' | '\u{fdf0}'..='\u{fffd}'
        | '\u{10000}'..='\u{effff}')
}

fn is_name_char(c: char) -> bool {
    is_name_start(c)
        || matches!(c, '-' | '.' | '0'..='9' | '\u{b7}' | '\u{300}'..='\u{36f}' | '\u{203f}'..='\u{2040}')
}

/// Whether `s` is an XML name with at most one colon, not at either end
/// (`a`, `x:a`, `ä-1.b`; not `1a`, `:a`, `a:b:c`).
pub fn is_name(s: &str) -> bool {
    let mut parts = s.split(':');
    let ok = |p: &str| {
        let mut cs = p.chars();
        cs.next().is_some_and(is_name_start) && cs.all(is_name_char)
    };
    let first = parts.next().is_some_and(ok);
    match (parts.next(), parts.next()) {
        (None, _) => first,
        (Some(local), None) => first && ok(local),
        _ => false,
    }
}

/// Whether a start tag passes the full checks: a valid name, attributes
/// that parse, are unique and hold only well-formed references.
fn tag_ok(tag: &str, name_len: usize) -> bool {
    if !tag.get(1..1 + name_len).is_some_and(is_name) {
        return false;
    }
    let Some(attrs) = scan_attrs(tag, name_len) else {
        return false;
    };
    let mut seen = HashSet::new();
    attrs.iter().all(|a| {
        let v = &tag[a.value.start + 1..a.value.end - 1];
        is_name(&tag[a.name.clone()])
            && seen.insert(&tag[a.name.clone()])
            && !v.contains('<')
            && v.split('&')
                .skip(1)
                .all(|r| r.find(';').is_some_and(|k| is_ref(&r[..k])))
    })
}

/// Whether `text` parses with matching tags. With `strict`, also one root
/// element, nothing but whitespace, comments and processing instructions
/// outside it, and [`tag_ok`] tags.
fn well_formed(text: &str, strict: bool) -> bool {
    use quick_xml::events::Event;
    let body = text.strip_prefix('\u{feff}').unwrap_or(text);
    let mut reader = quick_xml::Reader::from_str(body);
    reader.config_mut().check_end_names = true;
    let (mut depth, mut roots) = (0usize, 0usize);
    loop {
        let start = reader.buffer_position() as usize;
        let Ok(event) = reader.read_event() else {
            return false;
        };
        let end = reader.buffer_position() as usize;
        match event {
            Event::Start(ref e) | Event::Empty(ref e) => {
                if strict {
                    if depth == 0 {
                        roots += 1;
                    }
                    if roots > 1 || !tag_ok(&body[start..end], e.name().as_ref().len()) {
                        return false;
                    }
                }
                if matches!(event, Event::Start(_)) {
                    depth += 1;
                }
            }
            Event::End(_) => {
                if depth == 0 {
                    return false;
                }
                depth -= 1;
            }
            Event::Text(_) if strict && depth == 0 => {
                if !body[start..end].trim_matches(XML_WS).is_empty() {
                    return false;
                }
            }
            Event::CData(_) | Event::GeneralRef(_) if strict && depth == 0 => return false,
            Event::Eof => break,
            _ => {}
        }
    }
    depth == 0 && (!strict || roots == 1)
}

/// How a unit of character data reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dec {
    /// A character, literal or from a predefined or character reference.
    Ch(char),
    /// Another entity reference, shown as written.
    Raw,
}

/// One character of decoded text and the source it came from.
#[derive(Debug, Clone)]
struct Unit {
    raw: Range<usize>,
    dec: Dec,
    /// The CDATA section it is in.
    cdata: Option<usize>,
}

/// Split character data (`src[r]`) into units, with the byte ranges of its
/// CDATA sections (only looked for when `text`; attribute values have
/// none).
fn units(src: &str, r: Range<usize>, text: bool) -> (Vec<Unit>, Vec<Range<usize>>) {
    let mut out = Vec::new();
    let mut secs = Vec::new();
    let mut i = r.start;
    while i < r.end {
        let rest = &src[i..r.end];
        if text && rest.starts_with("<![CDATA[") {
            let inner = i + 9;
            let close = src[inner..r.end].find("]]>").map_or(r.end, |k| inner + k);
            let k = secs.len();
            let end = (close + 3).min(r.end);
            secs.push(i..end);
            for (o, c) in src[inner..close].char_indices() {
                out.push(Unit {
                    raw: inner + o..inner + o + c.len_utf8(),
                    dec: Dec::Ch(c),
                    cdata: Some(k),
                });
            }
            i = end;
            continue;
        }
        let Some(c) = rest.chars().next() else {
            break;
        };
        if c == '&'
            && let Some(semi) = rest.find(';')
            && is_ref(&rest[1..semi])
        {
            let dec = decode_ref(&rest[1..semi]).map_or(Dec::Raw, Dec::Ch);
            out.push(Unit {
                raw: i..i + semi + 1,
                dec,
                cdata: None,
            });
            i += semi + 1;
            continue;
        }
        out.push(Unit {
            raw: i..i + c.len_utf8(),
            dec: Dec::Ch(c),
            cdata: None,
        });
        i += c.len_utf8();
    }
    (out, secs)
}

impl Doc {
    /// The source this tree was parsed from.
    pub fn source(&self) -> &str {
        &self.src
    }

    /// Child positions from the document down to `i`.
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

    /// The node at a path of child positions (`[]` is the document).
    pub fn find(&self, path: &[usize]) -> Option<usize> {
        let mut i = 0;
        self.items.first()?;
        for &k in path {
            i = *self.items[i].children.get(k)?;
        }
        Some(i)
    }

    /// An element's or attribute's name as written.
    pub fn name(&self, i: usize) -> Option<&str> {
        self.src.get(self.items.get(i)?.b.name.clone()?)
    }

    fn dec_str<'a>(&'a self, u: &Unit, buf: &'a mut [u8; 4]) -> &'a str {
        match u.dec {
            Dec::Ch(c) => c.encode_utf8(buf),
            Dec::Raw => &self.src[u.raw.clone()],
        }
    }

    fn decoded(&self, us: &[Unit]) -> String {
        let mut s = String::new();
        let mut buf = [0; 4];
        for u in us {
            s.push_str(self.dec_str(u, &mut buf));
        }
        s
    }

    /// The character data a value edit works on: its byte range, whether
    /// it is element text (else an attribute value) and its quote.
    fn data(&self, i: usize) -> Option<(Range<usize>, Option<char>)> {
        let it = self.items.get(i)?;
        match it.kind {
            Kind::Attribute => {
                let v = it.b.value.clone()?;
                let q = self.src[v.start..].chars().next()?;
                Some((v.start + 1..v.end - 1, Some(q)))
            }
            Kind::Element if it.text_only => Some((
                it.b.value.clone().unwrap_or(it.b.whole.end..it.b.whole.end),
                None,
            )),
            Kind::Text => Some((it.b.whole.clone(), None)),
            _ => None,
        }
    }

    /// The units of a node's data and the editable part of them: element
    /// text without leading and trailing literal whitespace.
    fn data_units(
        &self,
        r: Range<usize>,
        text: bool,
    ) -> (Vec<Unit>, Vec<Range<usize>>, Range<usize>) {
        let (us, secs) = units(&self.src, r, text);
        let space = |u: &Unit| {
            u.cdata.is_none() && self.src[u.raw.clone()].chars().all(|c| XML_WS.contains(&c))
        };
        let (mut lo, mut hi) = (0, us.len());
        if text {
            while lo < hi && space(&us[lo]) {
                lo += 1;
            }
            while hi > lo && space(&us[hi - 1]) {
                hi -= 1;
            }
        }
        (us, secs, lo..hi)
    }

    /// The value shown and edited for a node: an attribute's value, the
    /// text of a text-only element or of a text node (without surrounding
    /// whitespace). Predefined and character references are decoded;
    /// other entity references read as written (`&name;`).
    pub fn value(&self, i: usize) -> Option<String> {
        let (r, quote) = self.data(i)?;
        let (us, _, keep) = self.data_units(r, quote.is_none());
        Some(self.decoded(&us[keep]))
    }

    /// Escape typed text for element content: `&`, `<`, `>` and CR become
    /// references; `&name;` stays a reference when the document uses that
    /// entity.
    fn escape(&self, s: &str, quote: Option<char>) -> String {
        let mut out = String::with_capacity(s.len());
        for (k, c) in s.char_indices() {
            match c {
                '&' => {
                    let kept = s[k + 1..]
                        .find(';')
                        .is_some_and(|e| self.refs.contains(&s[k + 1..k + 1 + e]));
                    out.push_str(if kept { "&" } else { "&amp;" });
                }
                '<' => out.push_str("&lt;"),
                '>' if quote.is_none() => out.push_str("&gt;"),
                '"' if quote == Some('"') => out.push_str("&quot;"),
                '\'' if quote == Some('\'') => out.push_str("&apos;"),
                '\t' if quote.is_some() => out.push_str("&#9;"),
                '\n' if quote.is_some() => out.push_str("&#10;"),
                '\r' => out.push_str("&#13;"),
                c => out.push(c),
            }
        }
        out
    }

    /// The smallest replacement turning the data in `r` into `new`: only
    /// the units that differ change, so references and CDATA around them
    /// stay as written. `None` if nothing changes.
    fn data_edit(
        &self,
        r: Range<usize>,
        quote: Option<char>,
        new: &str,
    ) -> Option<(Range<usize>, String)> {
        let text = quote.is_none();
        let (us, secs, keep) = self.data_units(r.clone(), text);
        let mut buf = [0; 4];
        let (mut p, mut pb) = (keep.start, 0);
        while p < keep.end {
            let d = self.dec_str(&us[p], &mut buf);
            if !new[pb..].starts_with(d) {
                break;
            }
            pb += d.len();
            p += 1;
        }
        let (mut q, mut qb) = (keep.end, new.len());
        while q > p {
            let d = self.dec_str(&us[q - 1], &mut buf);
            if qb < pb + d.len() || !new[..qb].ends_with(d) {
                break;
            }
            qb -= d.len();
            q -= 1;
        }
        if p == q && pb == qb {
            return None;
        }
        let mid = &new[pb..qb];
        let n = us.len();
        let raw = |a: usize, b: usize| us[a].raw.start..us[b - 1].raw.end;
        if !text {
            let at = if p < q {
                raw(p, q)
            } else {
                let x = us.get(p).map_or(r.end, |u| u.raw.start);
                x..x
            };
            return Some((at, self.escape(mid, quote)));
        }
        let sec = |k: usize| us.get(k).and_then(|u| u.cdata);
        // Entirely inside one CDATA section: insert as is, if the section
        // stays closed.
        let inside = if p < q {
            sec(p).filter(|&k| us[p..q].iter().all(|u| u.cdata == Some(k)))
        } else {
            (p > 0)
                .then(|| sec(p - 1))
                .flatten()
                .filter(|&k| sec(p) == Some(k))
        };
        if let Some(k) = inside {
            let first = us.iter().position(|u| u.cdata == Some(k)).unwrap_or(0);
            let last = us.iter().rposition(|u| u.cdata == Some(k)).unwrap_or(0);
            let inner = format!(
                "{}{mid}{}",
                self.decoded(&us[first..p]),
                self.decoded(&us[q..last + 1])
            );
            if !inner.contains("]]>") {
                let at = if p < q {
                    raw(p, q)
                } else {
                    us[p].raw.start..us[p].raw.start
                };
                return Some((at, mid.to_owned()));
            }
        } else if p < q && us[p..q].iter().all(|u| u.cdata.is_none()) {
            return Some((raw(p, q), self.escape(mid, None)));
        } else if p == q {
            let at = if p > 0 && sec(p - 1).is_none() {
                Some(us[p - 1].raw.end)
            } else if p < n && sec(p).is_none() {
                Some(us[p].raw.start)
            } else if n == 0 {
                Some(r.start)
            } else {
                None
            };
            if let Some(x) = at {
                return Some((x..x, self.escape(mid, None)));
            }
        }
        // The change touches CDATA partly: replace the sections it touches
        // whole.
        let mut touched: Vec<usize> = if p < q {
            us[p..q].iter().filter_map(|u| u.cdata).collect()
        } else {
            [p.checked_sub(1).and_then(sec), sec(p)]
                .into_iter()
                .flatten()
                .collect()
        };
        touched.dedup();
        let (mut a, mut b) = (p, q);
        for &k in &touched {
            let first = us.iter().position(|u| u.cdata == Some(k)).unwrap_or(a);
            let last = us.iter().rposition(|u| u.cdata == Some(k)).unwrap_or(b);
            a = a.min(first);
            b = b.max(last + 1);
        }
        let pre = self.decoded(&us[a..p]).len();
        let post = self.decoded(&us[q..b]).len();
        let mid = &new[pb - pre..qb + post];
        let start = sec(a).map_or(us[a].raw.start, |k| secs[k].start);
        let end = sec(b - 1).map_or(us[b - 1].raw.end, |k| secs[k].end);
        let one = touched.len() == 1 && us[a..b].iter().all(|u| u.cdata == Some(touched[0]));
        let text = if one && !mid.contains("]]>") {
            format!("<![CDATA[{mid}]]>")
        } else {
            self.escape(mid, None)
        };
        Some((start..end, text))
    }

    /// Set an attribute's value or the text of a text-only element or text
    /// node to `new` (plain text: it is escaped). An attribute keeps its
    /// quote style; CDATA stays CDATA while the new text allows it.
    pub fn set_value(&self, i: usize, new: &str) -> Result<Option<Vec<Edit>>, EditError> {
        let (r, quote) = self.data(i).ok_or(EditError::NotApplicable)?;
        if !new.chars().all(is_xml_char) {
            return Err(EditError::NotWellFormed);
        }
        let it = &self.items[i];
        if it.kind == Kind::Element && it.b.value.is_none() {
            // `<a/>` becomes `<a>text</a>`.
            if new.is_empty() {
                return Ok(None);
            }
            let end = it.b.whole.end;
            if !self.src[..end].ends_with("/>") {
                return Err(EditError::NotApplicable);
            }
            let name = self.name(i).unwrap_or_default();
            let text = format!(">{}</{name}>", self.escape(new, None));
            return self.checked(vec![(end - 2..end, text)]).map(Some);
        }
        match self.data_edit(r, quote, new) {
            Some(e) => self.checked(vec![e]).map(Some),
            None => Ok(None),
        }
    }

    /// Rename an element (start and end tag together) or an attribute.
    pub fn rename(&self, i: usize, name: &str) -> Result<Option<Vec<Edit>>, EditError> {
        let it = self.items.get(i).ok_or(EditError::NotApplicable)?;
        if !matches!(it.kind, Kind::Element | Kind::Attribute) {
            return Err(EditError::NotApplicable);
        }
        if !is_name(name) {
            return Err(EditError::InvalidName);
        }
        if self.name(i) == Some(name) {
            return Ok(None);
        }
        if it.kind == Kind::Attribute && self.attribute_taken(it.parent, Some(i), name) {
            return Err(EditError::DuplicateName);
        }
        let edits = [&it.b.name, &it.b.end_name]
            .into_iter()
            .flatten()
            .map(|r| (r.clone(), name.to_owned()))
            .collect();
        self.checked(edits).map(Some)
    }

    fn attribute_taken(&self, element: Option<usize>, except: Option<usize>, name: &str) -> bool {
        element.and_then(|e| self.items.get(e)).is_some_and(|e| {
            e.children.iter().any(|&c| {
                Some(c) != except
                    && self.items[c].kind == Kind::Attribute
                    && self.name(c) == Some(name)
            })
        })
    }

    /// An attribute name element `i` does not have: `base`, `base2`, ….
    pub fn unique_attribute(&self, i: usize, base: &str) -> String {
        let mut name = base.to_owned();
        let mut n = 1;
        while self.attribute_taken(Some(i), None, &name) {
            n += 1;
            name = format!("{base}{n}");
        }
        name
    }

    /// Remove an attribute (with its leading whitespace) or an element that
    /// is not the root (with its line, when it has one of its own).
    pub fn delete(&self, i: usize) -> Result<Vec<Edit>, EditError> {
        let it = self.items.get(i).ok_or(EditError::NotApplicable)?;
        let removable = match it.kind {
            Kind::Attribute => true,
            Kind::Element => it.parent.is_some_and(|p| p != 0),
            _ => false,
        };
        if !removable {
            return Err(EditError::NotApplicable);
        }
        self.checked(vec![(it.b.removal.clone(), String::new())])
    }

    /// Add `name="value"` after the element's last attribute (or its name),
    /// with one space and double quotes.
    pub fn add_attribute(&self, i: usize, name: &str, value: &str) -> Result<Vec<Edit>, EditError> {
        let it = self.items.get(i).ok_or(EditError::NotApplicable)?;
        if it.kind != Kind::Element || !it.attrs_ok {
            return Err(EditError::NotApplicable);
        }
        if !is_name(name) {
            return Err(EditError::InvalidName);
        }
        if self.attribute_taken(Some(i), None, name) {
            return Err(EditError::DuplicateName);
        }
        if !value.chars().all(is_xml_char) {
            return Err(EditError::NotWellFormed);
        }
        let at = it
            .children
            .iter()
            .rev()
            .find(|&&c| self.items[c].kind == Kind::Attribute)
            .map_or(it.b.name.as_ref().map_or(0, |n| n.end), |&c| {
                self.items[c].b.whole.end
            });
        let text = format!(" {name}=\"{}\"", self.escape(value, Some('"')));
        self.checked(vec![(at..at, text)])
    }

    /// Add an empty element `<name/>` after element `i` (not the root), on
    /// a line of its own with the same line break and indentation when `i`
    /// has one.
    pub fn add_after(&self, i: usize, name: &str) -> Result<Vec<Edit>, EditError> {
        let it = self.items.get(i).ok_or(EditError::NotApplicable)?;
        if it.kind != Kind::Element || it.parent.is_none_or(|p| p == 0) {
            return Err(EditError::NotApplicable);
        }
        if !is_name(name) {
            return Err(EditError::InvalidName);
        }
        let at = it.b.whole.end;
        let text = match line_start(&self.src, it.b.whole.start) {
            Some((brk, ind)) if !brk.is_empty() => {
                format!("{}{ind}<{name}/>", &self.src[brk])
            }
            _ => format!("<{name}/>"),
        };
        self.checked(vec![(at..at, text)])
    }

    /// Add an empty element `<name/>` as the last child element of `i`:
    /// after the last child element if there is one, else at the end of
    /// the content, one indentation level deeper on a line of its own when
    /// the element is laid out on lines. `<a/>` becomes `<a><name/></a>`.
    pub fn add_child(&self, i: usize, name: &str) -> Result<Vec<Edit>, EditError> {
        let it = self.items.get(i).ok_or(EditError::NotApplicable)?;
        if it.kind != Kind::Element {
            return Err(EditError::NotApplicable);
        }
        if !is_name(name) {
            return Err(EditError::InvalidName);
        }
        if let Some(&last) = it
            .children
            .iter()
            .rev()
            .find(|&&c| self.items[c].kind == Kind::Element)
        {
            return self.add_after(last, name);
        }
        let own = line_start(&self.src, it.b.whole.start);
        let nl = match &own {
            Some((brk, _)) if !brk.is_empty() => &self.src[brk.clone()],
            _ => crate::dominant_break(&self.src),
        };
        let unit = &self.unit;
        let child = format!("<{name}/>");
        let edit = match &it.b.value {
            None => {
                let end = it.b.whole.end;
                if !self.src[..end].ends_with("/>") {
                    return Err(EditError::NotApplicable);
                }
                let tag = self.name(i).unwrap_or_default();
                let text = match own {
                    Some((_, ind)) => format!(">{nl}{ind}{unit}{child}{nl}{ind}</{tag}>"),
                    None => format!(">{child}</{tag}>"),
                };
                (end - 2..end, text)
            }
            Some(c) if c.is_empty() => {
                let text = match own {
                    Some((_, ind)) => format!("{nl}{ind}{unit}{child}{nl}{ind}"),
                    None => child,
                };
                (c.start..c.start, text)
            }
            Some(c) => {
                // An end tag on its own line: the child goes on a new line
                // before it, indented one level more than the end tag.
                match line_start(&self.src, c.end) {
                    Some((brk, ind)) if !brk.is_empty() && brk.start >= c.start => {
                        let text = format!("{}{ind}{unit}{child}", &self.src[brk.clone()]);
                        (brk.start..brk.start, text)
                    }
                    _ => (c.end..c.end, child),
                }
            }
        };
        self.checked(vec![edit])
    }

    /// The edits as char ranges, if the text they produce is still well
    /// formed (as strictly as the source was).
    fn checked(&self, mut edits: Vec<(Range<usize>, String)>) -> Result<Vec<Edit>, EditError> {
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
        if !well_formed(&out, self.strict) {
            return Err(EditError::NotWellFormed);
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
        d.find(path).unwrap()
    }

    /// Check the result parses strictly and return it.
    fn ok(src: &str, edits: &[Edit]) -> String {
        let out = apply(src, edits);
        assert!(well_formed(&out, true), "{out}");
        assert!(doc(&out).is_ok());
        out
    }

    const SAMPLE: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\r\n<!-- asetukset -->\r\n<cfg:root xmlns:cfg=\"urn:x\" nimi='Äiti &amp; \"isä\"'>\r\n  <cfg:item id=\"1\">Hyvää &lt;päivää&gt; &#65;&custom;</cfg:item>\r\n  <code><![CDATA[a < b && c]]></code>\r\n  <empty/>\r\n  <p>Some <b>bold</b> text</p>\r\n</cfg:root>\r\n";

    #[test]
    fn spans_are_chars_and_cover_the_source() {
        let d = doc(SAMPLE).unwrap();
        let chars: Vec<char> = SAMPLE.chars().collect();
        let s = |r: &Range<usize>| chars[r.clone()].iter().collect::<String>();
        let root = at(&d, &[0]);
        let sp = &d.items[root].spans;
        assert_eq!(s(sp.name.as_ref().unwrap()), "cfg:root");
        assert_eq!(s(sp.end_name.as_ref().unwrap()), "cfg:root");
        assert!(s(&sp.whole).starts_with("<cfg:root") && s(&sp.whole).ends_with("</cfg:root>"));
        let nimi = at(&d, &[0, 1]);
        assert_eq!(d.items[nimi].kind, Kind::Attribute);
        assert_eq!(s(&d.items[nimi].spans.whole), " nimi='Äiti &amp; \"isä\"'");
        assert_eq!(
            s(d.items[nimi].spans.value.as_ref().unwrap()),
            "'Äiti &amp; \"isä\"'"
        );
        assert_eq!(d.value(nimi).as_deref(), Some("Äiti & \"isä\""));
        let item = at(&d, &[0, 2]);
        assert!(d.items[item].text_only);
        assert_eq!(d.value(item).as_deref(), Some("Hyvää <päivää> A&custom;"));
        assert_eq!(d.value(at(&d, &[0, 3])).as_deref(), Some("a < b && c"));
        let empty = at(&d, &[0, 4]);
        assert_eq!(d.items[empty].spans.end_name, None);
        assert_eq!(d.items[empty].spans.value, None);
        assert_eq!(s(&d.items[empty].spans.removal), "\r\n  <empty/>");
        // Mixed content: text nodes and the child element.
        let p = at(&d, &[0, 5]);
        assert!(!d.items[p].text_only);
        let kinds: Vec<Kind> = d.items[p]
            .children
            .iter()
            .map(|&c| d.items[c].kind)
            .collect();
        assert_eq!(kinds, [Kind::Text, Kind::Element, Kind::Text]);
        assert_eq!(d.value(at(&d, &[0, 5, 0])).as_deref(), Some("Some"));
        assert_eq!(d.path_of(at(&d, &[0, 5, 1])), vec![0, 5, 1]);
        assert_eq!(d.find(&[0, 9]), None);
        assert!(d.strict);
        assert_eq!(d.unit, "  ");
    }

    #[test]
    fn attribute_values_keep_quotes_and_references() {
        let d = doc(SAMPLE).unwrap();
        let nimi = at(&d, &[0, 1]);
        // Change only the tail: the leading reference stays as written.
        let e = d.set_value(nimi, "Äiti & 'isä'").unwrap().unwrap();
        assert_eq!(e.len(), 1);
        let out = ok(SAMPLE, &e);
        assert_eq!(
            out,
            SAMPLE.replace("'Äiti &amp; \"isä\"'", "'Äiti &amp; &apos;isä&apos;'")
        );
        let id = at(&d, &[0, 2, 0]);
        let out = ok(SAMPLE, &d.set_value(id, "<\"&\">").unwrap().unwrap());
        assert!(out.contains("id=\"&lt;&quot;&amp;&quot;>\""), "{out}");
        assert_eq!(d.set_value(id, "1"), Ok(None));
        assert_eq!(
            d.set_value(id, "a\u{1}"),
            Err(EditError::NotWellFormed),
            "not representable in XML"
        );
        let out = ok(SAMPLE, &d.set_value(id, "a\tb").unwrap().unwrap());
        assert!(out.contains("id=\"a&#9;b\""));
        // A known custom entity is kept as a reference when typed.
        let out = ok(
            SAMPLE,
            &d.set_value(id, "&custom; &other;").unwrap().unwrap(),
        );
        assert!(out.contains("id=\"&custom; &amp;other;\""), "{out}");
    }

    #[test]
    fn text_edits_escape_and_keep_untouched_references() {
        let d = doc(SAMPLE).unwrap();
        let item = at(&d, &[0, 2]);
        let e = d
            .set_value(item, "Hyvää <iltaa> A&custom;")
            .unwrap()
            .unwrap();
        let out = ok(SAMPLE, &e);
        assert_eq!(
            out,
            SAMPLE.replace("&lt;päivää&gt;", "&lt;iltaa&gt;"),
            "only the changed letters, references kept"
        );
        assert_eq!(e[0].text, "iltaa");
        let out = ok(SAMPLE, &d.set_value(item, "x & y > z").unwrap().unwrap());
        assert!(out.contains(">x &amp; y &gt; z</cfg:item>"), "{out}");
        // Text nodes in mixed content.
        let some = at(&d, &[0, 5, 0]);
        let out = ok(SAMPLE, &d.set_value(some, "Very").unwrap().unwrap());
        assert!(out.contains("<p>Very <b>bold</b> text</p>"));
        // Surrounding whitespace is not part of the value and stays.
        let src = "<a>\n  <b>\n    hello\n  </b>\n</a>";
        let d = doc(src).unwrap();
        let b = at(&d, &[0, 0]);
        assert_eq!(d.value(b).as_deref(), Some("hello"));
        let out = ok(src, &d.set_value(b, "hi").unwrap().unwrap());
        assert_eq!(out, "<a>\n  <b>\n    hi\n  </b>\n</a>");
        let out = ok(src, &d.set_value(b, "").unwrap().unwrap());
        assert_eq!(out, "<a>\n  <b>\n    \n  </b>\n</a>");
        // Empty elements: a pair gets the text, `<x/>` becomes a pair.
        for (src, want) in [
            ("<a><x></x></a>", "<a><x>t &lt;</x></a>"),
            ("<a><x/></a>", "<a><x>t &lt;</x></a>"),
            ("<a><x y='1' /></a>", "<a><x y='1' >t &lt;</x></a>"),
        ] {
            let d = doc(src).unwrap();
            let e = d.set_value(at(&d, &[0, 0]), "t <").unwrap().unwrap();
            assert_eq!(ok(src, &e), want);
            assert_eq!(d.set_value(at(&d, &[0, 0]), ""), Ok(None));
        }
        // Not editable: an element with child elements.
        let d = doc(SAMPLE).unwrap();
        assert_eq!(
            d.set_value(at(&d, &[0, 5]), "x"),
            Err(EditError::NotApplicable)
        );
        assert_eq!(d.set_value(0, "x"), Err(EditError::NotApplicable));
    }

    #[test]
    fn cdata_stays_cdata_when_it_can() {
        let d = doc(SAMPLE).unwrap();
        let code = at(&d, &[0, 3]);
        let e = d.set_value(code, "a < b || c").unwrap().unwrap();
        assert_eq!(e[0].text, "||", "inside the section, unescaped");
        let out = ok(SAMPLE, &e);
        assert!(out.contains("<code><![CDATA[a < b || c]]></code>"));
        let out = ok(SAMPLE, &d.set_value(code, "x]]>y").unwrap().unwrap());
        assert!(out.contains("<code>x]]&gt;y</code>"), "{out}");
        let out = ok(SAMPLE, &d.set_value(code, "").unwrap().unwrap());
        assert!(out.contains("<code><![CDATA[]]></code>"), "{out}");
        let out = ok(SAMPLE, &d.set_value(code, "<a < b && c>").unwrap().unwrap());
        assert!(
            out.contains("<code><![CDATA[<a < b && c>]]></code>"),
            "{out}"
        );
        // Text next to a section: a change in the text stays text.
        let src = "<r><t>x &amp; <![CDATA[<y>]]> z</t></r>";
        let d = doc(src).unwrap();
        let t = at(&d, &[0, 0]);
        assert_eq!(d.value(t).as_deref(), Some("x & <y> z"));
        let out = ok(src, &d.set_value(t, "x & <y> w").unwrap().unwrap());
        assert_eq!(out, "<r><t>x &amp; <![CDATA[<y>]]> w</t></r>");
        let out = ok(src, &d.set_value(t, "x & <yy> z").unwrap().unwrap());
        assert_eq!(out, "<r><t>x &amp; <![CDATA[<yy>]]> z</t></r>");
        // A change across text and CDATA rewrites the section as text.
        let out = ok(src, &d.set_value(t, "x & <q").unwrap().unwrap());
        assert_eq!(out, "<r><t>x &amp; &lt;q</t></r>");
    }

    #[test]
    fn rename_elements_and_attributes() {
        let d = doc(SAMPLE).unwrap();
        let item = at(&d, &[0, 2]);
        let e = d.rename(item, "cfg:kohta").unwrap().unwrap();
        assert_eq!(e.len(), 2, "start and end tag");
        let out = ok(SAMPLE, &e);
        assert_eq!(
            out,
            SAMPLE
                .replace("<cfg:item ", "<cfg:kohta ")
                .replace("</cfg:item>", "</cfg:kohta>")
        );
        let empty = at(&d, &[0, 4]);
        let e = d.rename(empty, "tyhjä").unwrap().unwrap();
        assert_eq!(e.len(), 1);
        assert!(ok(SAMPLE, &e).contains("\r\n  <tyhjä/>\r\n"));
        assert_eq!(d.rename(item, "cfg:item"), Ok(None));
        for bad in ["", "1a", "a b", ":a", "a:", "a:b:c", "a>", "-x"] {
            assert_eq!(d.rename(item, bad), Err(EditError::InvalidName), "{bad}");
        }
        let id = at(&d, &[0, 2, 0]);
        let out = ok(SAMPLE, &d.rename(id, "xml:lang").unwrap().unwrap());
        assert!(out.contains("<cfg:item xml:lang=\"1\">"));
        let root = at(&d, &[0]);
        let xmlns = d.items[root].children[0];
        assert_eq!(d.rename(xmlns, "nimi"), Err(EditError::DuplicateName));
        assert_eq!(d.rename(0, "x"), Err(EditError::NotApplicable));
        assert_eq!(
            d.rename(at(&d, &[0, 5, 0]), "x"),
            Err(EditError::NotApplicable)
        );
        assert!(is_name("ä-1.b") && is_name("_x") && is_name("x\u{b7}y"));
    }

    #[test]
    fn delete_removes_the_line_or_attribute() {
        let d = doc(SAMPLE).unwrap();
        let del = |path: &[usize]| ok(SAMPLE, &d.delete(at(&d, path)).unwrap());
        assert_eq!(del(&[0, 4]), SAMPLE.replace("\r\n  <empty/>", ""));
        assert_eq!(
            del(&[0, 2]),
            SAMPLE.replace(
                "\r\n  <cfg:item id=\"1\">Hyvää &lt;päivää&gt; &#65;&custom;</cfg:item>",
                ""
            )
        );
        assert_eq!(del(&[0, 5, 1]), SAMPLE.replace("<b>bold</b>", ""));
        assert_eq!(del(&[0, 2, 0]), SAMPLE.replace(" id=\"1\"", ""));
        assert_eq!(
            del(&[0, 1]),
            SAMPLE.replace(" nimi='Äiti &amp; \"isä\"'", "")
        );
        assert_eq!(
            d.delete(at(&d, &[0])),
            Err(EditError::NotApplicable),
            "root"
        );
        assert_eq!(d.delete(0), Err(EditError::NotApplicable));
        assert_eq!(
            d.delete(at(&d, &[0, 5, 0])),
            Err(EditError::NotApplicable),
            "text"
        );
        // Tabs, trailing spaces and attributes on their own lines.
        let src = "<a>\n\t<b\n\t\tx=\"1\"\n\t\ty=\"2\"/>  \n\t<c/>\n</a>";
        let d = doc(src).unwrap();
        assert_eq!(
            ok(src, &d.delete(at(&d, &[0, 0, 0])).unwrap()),
            "<a>\n\t<b\n\t\ty=\"2\"/>  \n\t<c/>\n</a>"
        );
        assert_eq!(
            ok(src, &d.delete(at(&d, &[0, 0])).unwrap()),
            "<a>\n\t<c/>\n</a>"
        );
        assert_eq!(
            ok(src, &d.delete(at(&d, &[0, 1])).unwrap()),
            "<a>\n\t<b\n\t\tx=\"1\"\n\t\ty=\"2\"/>  \n</a>"
        );
        // Inline: only the element.
        let src = "<a><b/> <c/></a>";
        let d = doc(src).unwrap();
        assert_eq!(ok(src, &d.delete(at(&d, &[0, 1])).unwrap()), "<a><b/> </a>");
    }

    #[test]
    fn add_attribute_after_the_last_one() {
        let d = doc(SAMPLE).unwrap();
        let item = at(&d, &[0, 2]);
        let out = ok(SAMPLE, &d.add_attribute(item, "uusi", "a\"b").unwrap());
        assert!(out.contains("<cfg:item id=\"1\" uusi=\"a&quot;b\">"));
        let empty = at(&d, &[0, 4]);
        let out = ok(SAMPLE, &d.add_attribute(empty, "x", "").unwrap());
        assert!(out.contains("<empty x=\"\"/>"));
        assert_eq!(
            d.add_attribute(item, "id", ""),
            Err(EditError::DuplicateName)
        );
        assert_eq!(d.add_attribute(item, "1d", ""), Err(EditError::InvalidName));
        assert_eq!(d.unique_attribute(item, "id"), "id2");
        assert_eq!(d.unique_attribute(item, "attribute"), "attribute");
    }

    #[test]
    fn add_elements_copy_layout() {
        let d = doc(SAMPLE).unwrap();
        let out = ok(SAMPLE, &d.add_after(at(&d, &[0, 4]), "uusi").unwrap());
        assert_eq!(
            out,
            SAMPLE.replace("<empty/>\r\n", "<empty/>\r\n  <uusi/>\r\n")
        );
        // Into an empty element on its own line: one level deeper, CRLF.
        let out = ok(SAMPLE, &d.add_child(at(&d, &[0, 4]), "c").unwrap());
        assert!(
            out.contains("\r\n  <empty>\r\n    <c/>\r\n  </empty>\r\n"),
            "{out}"
        );
        // After the last child element.
        let out = ok(SAMPLE, &d.add_child(at(&d, &[0]), "z").unwrap());
        assert!(out.ends_with("</p>\r\n  <z/>\r\n</cfg:root>\r\n"), "{out}");
        // Text-only content: at its end.
        let out = ok(SAMPLE, &d.add_child(at(&d, &[0, 3]), "z").unwrap());
        assert!(out.contains("<code><![CDATA[a < b && c]]><z/></code>"));
        // Inline siblings stay inline.
        let out = ok(SAMPLE, &d.add_after(at(&d, &[0, 5, 1]), "i").unwrap());
        assert!(out.contains("<b>bold</b><i/> text"));
        assert_eq!(
            d.add_after(at(&d, &[0]), "x"),
            Err(EditError::NotApplicable),
            "a second root"
        );
        assert_eq!(
            d.add_after(at(&d, &[0, 4]), "a b"),
            Err(EditError::InvalidName)
        );
        for (src, path, want) in [
            ("<a>\n\t<b/>\n</a>", &[0][..], "<a>\n\t<b/>\n\t<n/>\n</a>"),
            (
                "<a>\n\t<b></b>\n</a>",
                &[0, 0][..],
                "<a>\n\t<b>\n\t\t<n/>\n\t</b>\n</a>",
            ),
            ("<a></a>", &[0][..], "<a>\n  <n/>\n</a>"),
            ("<a>\r\n</a>", &[0][..], "<a>\r\n  <n/>\r\n</a>"),
            (
                "<a>\n  <!-- c -->\n</a>",
                &[0][..],
                "<a>\n  <!-- c -->\n  <n/>\n</a>",
            ),
            ("<r><a/></r>", &[0, 0][..], "<r><a><n/></a></r>"),
            ("<a/>", &[0][..], "<a>\n  <n/>\n</a>"),
        ] {
            let d = doc(src).unwrap();
            assert_eq!(
                ok(src, &d.add_child(at(&d, path), "n").unwrap()),
                want,
                "{src}"
            );
        }
    }

    #[test]
    fn bom_and_non_ascii_offsets() {
        let src = "\u{feff}<ä a=\"ö\">\n  <b>🙂</b>\n</ä>\n";
        let d = doc(src).unwrap();
        let chars: Vec<char> = src.chars().collect();
        let s = |r: &Range<usize>| chars[r.clone()].iter().collect::<String>();
        let root = at(&d, &[0]);
        assert_eq!(s(d.items[root].spans.name.as_ref().unwrap()), "ä");
        let b = at(&d, &[0, 1]);
        assert_eq!(s(&d.items[b].spans.whole), "<b>🙂</b>");
        let out = ok(src, &d.set_value(b, "🙂🙂").unwrap().unwrap());
        assert_eq!(out, src.replace("🙂", "🙂🙂"));
        let out = ok(src, &d.rename(root, "öö").unwrap().unwrap());
        assert!(out.starts_with("\u{feff}<öö a") && out.ends_with("</öö>\n"));
        let out = ok(src, &d.delete(b).unwrap());
        assert_eq!(out, "\u{feff}<ä a=\"ö\">\n</ä>\n");
        let out = ok(src, &d.set_value(at(&d, &[0, 0]), "x").unwrap().unwrap());
        assert_eq!(out, "\u{feff}<ä a=\"x\">\n  <b>🙂</b>\n</ä>\n");
    }

    #[test]
    fn refuses_what_breaks_the_document() {
        // A lenient source (two roots) still edits, without new breakage.
        let src = "<a/><b/>";
        let d = doc(src).unwrap();
        assert!(!d.strict);
        assert!(d.rename(at(&d, &[1]), "c").is_ok());
        // Entities are never expanded.
        let bomb = "<!DOCTYPE l [<!ENTITY a \"aaaa\"><!ENTITY b \"&a;&a;&a;\">]><l>&b;</l>";
        let d = doc(bomb).unwrap();
        assert_eq!(d.value(at(&d, &[0])).as_deref(), Some("&b;"));
        let out = ok(bomb, &d.set_value(at(&d, &[0]), "&b; x").unwrap().unwrap());
        assert!(out.ends_with("<l>&b; x</l>"));
        assert!(doc("<a><b></a>").is_err());
        assert!(doc("<a>").is_err());
        let deep = format!("{}{}", "<a>".repeat(100_000), "</a>".repeat(100_000));
        assert!(doc(&deep).is_ok());
    }

    /// A random XML element as text, with the given indentation style.
    fn arb_doc() -> impl proptest::strategy::Strategy<Value = String> {
        use proptest::prelude::*;
        let text = prop_oneof![
            Just("plain".to_owned()),
            Just("a &amp; b &lt;c&gt;".to_owned()),
            Just("&#228;&#x1F642;".to_owned()),
            Just("<![CDATA[ <x> & ]]>".to_owned()),
            Just("ÄÖ 🙂".to_owned()),
            Just("".to_owned()),
            Just("<!-- c -->t".to_owned()),
            Just("x &amp; <i>y</i> <![CDATA[z]]>".to_owned()),
        ];
        let attrs = proptest::collection::vec(0..5usize, 0..3).prop_map(|v| {
            let all = [
                " id=\"1\"",
                " x:y='a &amp; &quot;'",
                "\n  k=\"ä\"",
                " q = \"\"",
                " xmlns:x=\"urn:x\"",
            ];
            let mut seen = Vec::new();
            for k in v {
                if !seen.contains(&k) {
                    seen.push(k);
                }
            }
            seen.iter().map(|&k| all[k]).collect::<String>()
        });
        let names = prop_oneof![Just("a"), Just("x:b"), Just("ä"), Just("c-1")];
        let leaf =
            (names.clone(), attrs.clone(), text, any::<bool>()).prop_map(|(n, a, t, empty)| {
                if empty && t.is_empty() {
                    format!("<{n}{a}/>")
                } else {
                    format!("<{n}{a}>{t}</{n}>")
                }
            });
        leaf.prop_recursive(4, 40, 4, move |inner| {
            (
                names.clone(),
                attrs.clone(),
                proptest::collection::vec(inner, 0..4),
                0..4usize,
            )
                .prop_map(|(n, a, kids, style)| {
                    let (sep, end) = match style {
                        0 => ("", ""),
                        1 => ("\r\n  ", "\r\n"),
                        2 => ("\n\t", "\n"),
                        _ => (" ", " "),
                    };
                    let body: String = kids.iter().map(|k| format!("{sep}{k}")).collect();
                    format!("<{n}{a}>{body}{end}</{n}>")
                })
        })
    }

    proptest::proptest! {
        #[test]
        fn random_edits_keep_well_formed_xml(
            root in arb_doc(),
            decl in proptest::bool::ANY,
            ops in proptest::collection::vec((0..7usize, 0..64usize, 0..6usize), 1..6),
        ) {
            let mut src = if decl {
                format!("\u{feff}<?xml version=\"1.0\"?>\r\n<!-- top -->\r\n{root}\r\n")
            } else {
                root
            };
            let texts = ["new", "a & b < c > d", "]]>", "", "ä🙂 \"'", "&custom;"];
            let names = ["n", "x:n", "ä", "id", "1bad", "q"];
            for (op, n, s) in ops {
                let d = doc(&src).unwrap();
                proptest::prop_assert!(d.strict, "{}", src);
                let i = n % d.items.len();
                let before = d.items.len();
                let result = match op {
                    0 => d.set_value(i, texts[s]).map(|e| e.unwrap_or_default()),
                    1 => d.rename(i, names[s]).map(|e| e.unwrap_or_default()),
                    2 => d.delete(i),
                    3 => d.add_attribute(i, names[s], texts[s]),
                    4 => d.add_after(i, names[s]),
                    5 => d.add_child(i, names[s]),
                    _ => d.delete(i),
                };
                let Ok(edits) = result else { continue };
                if edits.is_empty() {
                    continue;
                }
                let n_chars = src.chars().count();
                for e in &edits {
                    proptest::prop_assert!(e.range.start <= e.range.end && e.range.end <= n_chars);
                }
                let out = apply(&src, &edits);
                proptest::prop_assert!(well_formed(&out, true), "{} -> {}", src, out);
                // Bytes outside the edits are the source's.
                let first = edits[0].range.start;
                let last = edits[edits.len() - 1].range.end;
                let pre: String = src.chars().take(first).collect();
                let post: String = src.chars().skip(last).collect();
                proptest::prop_assert!(out.starts_with(&pre) && out.ends_with(&post));
                let nd = doc(&out).unwrap();
                match op {
                    0 if d.items[i].kind == Kind::Text && texts[s].is_empty() => {}
                    0 => {
                        let j = nd.find(&d.path_of(i)).unwrap();
                        let v = nd.value(j);
                        proptest::prop_assert_eq!(v.as_deref(), Some(texts[s]), "{} -> {}", src, out);
                    }
                    1 => {
                        let j = nd.find(&d.path_of(i)).unwrap();
                        proptest::prop_assert_eq!(nd.name(j), Some(names[s]));
                    }
                    3..=5 => proptest::prop_assert!(nd.items.len() > before),
                    _ => proptest::prop_assert!(nd.items.len() < before),
                }
                src = out;
            }
        }
    }
}
