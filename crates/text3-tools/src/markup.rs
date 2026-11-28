//! Read-only trees of XML and YAML documents for inspection, in the same
//! shape as the JSON tree so one view shows all three.
//!
//! Safety: nothing is fetched or expanded. XML entity references other
//! than the five predefined ones and character references are shown
//! literally (`&name;`), and DTDs are ignored, so entity expansion attacks
//! do nothing. YAML aliases are shown as references (`*N`), never
//! expanded. Both builders are iterative and stop at [`MAX_NODES`].

use crate::json::Node;

/// Upper bound on tree nodes built from one document.
pub const MAX_NODES: usize = 2_000_000;

/// Smallest output cap of [`format_xml`]; the cap grows with the input.
/// Indentation grows with depth squared, so deep nesting must stop here.
const MIN_FORMAT_OUTPUT: usize = 16 << 20;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeError {
    /// 1-based.
    pub line: usize,
    /// 1-based, in characters.
    pub column: usize,
    /// Parser message (English, from the parsing library) or `too large`.
    pub message: String,
}

pub(crate) fn position(text: &str, byte: usize) -> (usize, usize) {
    let mut end = byte.min(text.len());
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    let before = &text[..end];
    let line = before.matches('\n').count() + 1;
    let column = before.rsplit('\n').next().map_or(0, |l| l.chars().count()) + 1;
    (line, column)
}

pub(crate) fn too_large(text: &str, byte: usize) -> TreeError {
    let (line, column) = position(text, byte);
    TreeError {
        line,
        column,
        message: "too large".into(),
    }
}

/// An element being built: name, members (attributes, children, text).
struct Elem {
    name: String,
    members: Vec<(String, Node)>,
    text: String,
}

impl Elem {
    fn flush_text(&mut self) {
        let t = self.text.trim();
        if !t.is_empty() {
            self.members
                .push(("#text".into(), Node::Scalar(t.to_owned())));
        }
        self.text.clear();
    }

    fn finish(mut self) -> (String, Node) {
        if self.members.is_empty() {
            let t = self.text.trim().to_owned();
            return (self.name, Node::Scalar(t));
        }
        self.flush_text();
        (self.name, Node::Object(self.members))
    }
}

fn start_elem(e: &quick_xml::events::BytesStart<'_>) -> Elem {
    let name = e.name().as_ref().to_owned();
    let mut members = Vec::new();
    for a in e.attributes().with_checks(false).flatten() {
        let key = format!("@{}", a.key.as_ref());
        // Predefined entities only; anything else stays as written.
        let value = a
            .normalized_value(quick_xml::XmlVersion::Implicit1_0)
            .map(|v| v.into_owned())
            .unwrap_or_else(|_| a.value.to_string());
        members.push((key, Node::Scalar(value)));
    }
    Elem {
        name,
        members,
        text: String::new(),
    }
}

/// Parse XML into a tree: an element is an object of `@attributes`,
/// child elements (by tag name, in order) and `#text`; an element with only
/// text is a scalar. The root is an object holding the document element.
pub fn xml_tree(text: &str) -> Result<Node, TreeError> {
    use quick_xml::events::Event;
    let mut reader = quick_xml::Reader::from_str(text);
    reader.config_mut().check_end_names = true;
    let mut stack: Vec<Elem> = vec![Elem {
        name: String::new(),
        members: Vec::new(),
        text: String::new(),
    }];
    let mut nodes = 0usize;
    let err = |reader: &quick_xml::Reader<&[u8]>, message: String| {
        let (line, column) = position(text, reader.error_position() as usize);
        TreeError {
            line,
            column,
            message,
        }
    };
    loop {
        let event = reader
            .read_event()
            .map_err(|e| err(&reader, e.to_string()))?;
        nodes += 1;
        if nodes > MAX_NODES {
            return Err(too_large(text, reader.buffer_position() as usize));
        }
        let top = stack.last_mut().expect("root frame");
        match event {
            Event::Start(e) => {
                top.flush_text();
                stack.push(start_elem(&e));
            }
            Event::Empty(e) => {
                top.flush_text();
                let (k, v) = start_elem(&e).finish();
                top.members.push((k, v));
            }
            Event::End(_) => {
                if stack.len() < 2 {
                    return Err(err(&reader, "unexpected closing tag".into()));
                }
                let done = stack.pop().expect("checked").finish();
                let parent = stack.last_mut().expect("root frame");
                parent.members.push(done);
            }
            Event::Text(t) => top.text.push_str(&t.xml10_content()),
            Event::CData(t) => top.text.push_str(&t.xml10_content()),
            Event::GeneralRef(r) => {
                let name = r.xml10_content();
                let resolved = match name.as_ref() {
                    "amp" => Some('&'),
                    "lt" => Some('<'),
                    "gt" => Some('>'),
                    "quot" => Some('"'),
                    "apos" => Some('\''),
                    _ => r.resolve_char_ref().ok().flatten(),
                };
                match resolved {
                    Some(c) => top.text.push(c),
                    None => {
                        top.text.push('&');
                        top.text.push_str(&name);
                        top.text.push(';');
                    }
                }
            }
            Event::Eof => break,
            // Declarations, comments, processing instructions and DTDs
            // are not part of the data tree.
            _ => {}
        }
    }
    if stack.len() > 1 {
        let name = stack.last().map(|e| e.name.clone()).unwrap_or_default();
        let (line, column) = position(text, text.len());
        return Err(TreeError {
            line,
            column,
            message: format!("unclosed element <{name}>"),
        });
    }
    let root = stack.pop().expect("root frame");
    Ok(Node::Object(root.members))
}

/// One parsed piece of an XML document, by byte span in the input.
#[derive(Clone, Copy)]
enum Piece {
    /// Start tag; index of its end tag and whether the whole element is
    /// copied as written.
    Open {
        close: usize,
        verbatim: bool,
    },
    Close,
    /// Copied as written on a line of its own: empty element, comment,
    /// processing instruction, declaration, DOCTYPE.
    Item,
    /// Whitespace-only text between tags.
    Space,
    /// Text with content, CDATA or an entity reference: mixed content.
    Content,
}

/// An open element while scanning: its piece index and what it holds.
struct Scan {
    open: usize,
    content: bool,
    markup: bool,
    preserve: bool,
}

fn preserves_space(e: &quick_xml::events::BytesStart<'_>) -> bool {
    e.attributes()
        .with_checks(false)
        .flatten()
        .any(|a| a.key.as_ref() == "xml:space" && a.value.as_ref() == "preserve")
}

fn line_breaks(s: &str) -> usize {
    s.matches('\n').count().max(s.matches('\r').count())
}

/// Re-indent XML without changing its meaning.
///
/// Only whitespace between tags in element-only content changes: each
/// child goes on its own line, nested by `indent`. An element holding text,
/// CDATA, entity references or `xml:space="preserve"`, and any element
/// without child markup, is copied byte for byte from its start tag to its
/// end tag. Tags, comments, processing instructions, the declaration and
/// DOCTYPE are copied as written; one blank line between siblings is kept.
/// The line ending style, leading indentation and final newline of `text`
/// are kept. Malformed XML is an error, never partial output, and so is
/// output far larger than the input (very deep nesting).
pub fn format_xml(text: &str, indent: &str) -> Result<String, TreeError> {
    // quick-xml skips a leading U+FEFF, which would shift every offset below
    // (corrupt output or a panic on a char boundary): format the rest and
    // put it back.
    if let Some(rest) = text.strip_prefix('\u{feff}') {
        return match format_xml(rest, indent) {
            Ok(out) => Ok(format!("\u{feff}{out}")),
            Err(mut e) => {
                if e.line == 1 {
                    e.column += 1;
                }
                Err(e)
            }
        };
    }
    use quick_xml::events::Event;
    let mut reader = quick_xml::Reader::from_str(text);
    reader.config_mut().check_end_names = true;
    let err = |reader: &quick_xml::Reader<&[u8]>, message: String| {
        let (line, column) = position(text, reader.error_position() as usize);
        TreeError {
            line,
            column,
            message,
        }
    };
    let mut pieces: Vec<(Piece, usize, usize)> = Vec::new();
    let mut stack: Vec<Scan> = Vec::new();
    let mut top_content = false;
    loop {
        let start = reader.buffer_position() as usize;
        let event = reader
            .read_event()
            .map_err(|e| err(&reader, e.to_string()))?;
        let end = reader.buffer_position() as usize;
        if pieces.len() >= MAX_NODES {
            return Err(too_large(text, start));
        }
        let piece = match event {
            Event::Start(e) => {
                if let Some(parent) = stack.last_mut() {
                    parent.markup = true;
                }
                stack.push(Scan {
                    open: pieces.len(),
                    content: false,
                    markup: false,
                    preserve: preserves_space(&e),
                });
                Piece::Open {
                    close: 0,
                    verbatim: false,
                }
            }
            Event::End(_) => {
                let Some(scan) = stack.pop() else {
                    return Err(err(&reader, "unexpected closing tag".into()));
                };
                let close = pieces.len();
                pieces[scan.open].0 = Piece::Open {
                    close,
                    verbatim: scan.content || scan.preserve || !scan.markup,
                };
                Piece::Close
            }
            Event::Text(t) => {
                let raw = &text[start..end];
                debug_assert_eq!(raw.len(), t.len());
                // Only XML whitespace is layout; U+00A0 and the like are text.
                if raw
                    .trim_matches(|c| matches!(c, ' ' | '\t' | '\r' | '\n'))
                    .is_empty()
                {
                    Piece::Space
                } else {
                    Piece::Content
                }
            }
            Event::CData(_) | Event::GeneralRef(_) => Piece::Content,
            Event::Eof => break,
            _ => {
                if let Some(parent) = stack.last_mut() {
                    parent.markup = true;
                }
                Piece::Item
            }
        };
        if let Piece::Content = piece {
            match stack.last_mut() {
                Some(parent) => parent.content = true,
                None => top_content = true,
            }
        }
        pieces.push((piece, start, end));
    }
    if let Some(scan) = stack.last() {
        let tag = &text[pieces[scan.open].1 + 1..pieces[scan.open].2];
        let name = tag
            .split(|c: char| c.is_whitespace() || c == '>' || c == '/')
            .next()
            .unwrap_or("");
        let (line, column) = position(text, text.len());
        return Err(TreeError {
            line,
            column,
            message: format!("unclosed element <{name}>"),
        });
    }
    // Text outside any element (a selected fragment): nothing is safe to move.
    if top_content || text.trim().is_empty() {
        return Ok(text.to_owned());
    }

    let nl = crate::dominant_break(text);
    let lead_len = text.len() - text.trim_start().len();
    let lead = &text[..lead_len];
    // Indentation of the first tag's line, used as the base for every line.
    let base = lead.rsplit(['\n', '\r']).next().unwrap_or("");
    let mut out = String::with_capacity(text.len() + text.len() / 4);
    out.push_str(lead);
    let mut depth = 0usize;
    let mut first = true;
    let mut has_sibling = false;
    let mut blank = false;
    let new_line = |out: &mut String, depth: usize, blank: bool| {
        if blank {
            out.push_str(nl);
        }
        out.push_str(nl);
        out.push_str(base);
        for _ in 0..depth {
            out.push_str(indent);
        }
    };
    let limit = text.len().saturating_mul(8).max(MIN_FORMAT_OUTPUT);
    let mut i = 0;
    while i < pieces.len() {
        let (piece, start, end) = pieces[i];
        if out.len() > limit {
            return Err(too_large(text, start));
        }
        match piece {
            Piece::Space => {
                if has_sibling && line_breaks(&text[start..end]) >= 2 {
                    blank = true;
                }
            }
            Piece::Close => {
                depth -= 1;
                new_line(&mut out, depth, false);
                out.push_str(&text[start..end]);
                has_sibling = true;
                blank = false;
            }
            Piece::Content => unreachable!("content is inside verbatim elements"),
            Piece::Item | Piece::Open { .. } => {
                if !first {
                    new_line(&mut out, depth, blank);
                }
                first = false;
                blank = false;
                match piece {
                    Piece::Open {
                        close,
                        verbatim: true,
                    } => {
                        out.push_str(&text[start..pieces[close].2]);
                        i = close;
                        has_sibling = true;
                    }
                    Piece::Open { .. } => {
                        out.push_str(&text[start..end]);
                        depth += 1;
                        has_sibling = false;
                    }
                    _ => {
                        out.push_str(&text[start..end]);
                        has_sibling = true;
                    }
                }
            }
        }
        i += 1;
    }
    if text.ends_with(['\n', '\r']) {
        out.push_str(nl);
    }
    Ok(out)
}

enum Frame {
    Seq(Vec<Node>),
    /// Members so far and the key waiting for its value.
    Map(Vec<(String, Node)>, Option<String>),
}

fn key_text(n: &Node) -> String {
    match n {
        Node::Scalar(s) => s.clone(),
        Node::Array(_) | Node::Object(_) => "?".into(),
    }
}

/// Parse YAML into a tree: mappings are objects, sequences arrays,
/// scalars as written. Several documents become an array of documents.
pub fn yaml_tree(text: &str) -> Result<Node, TreeError> {
    use saphyr_parser::{Event, Parser};
    let mut docs: Vec<Node> = Vec::new();
    let mut stack: Vec<Frame> = Vec::new();
    let mut nodes = 0usize;
    for item in Parser::new_from_str(text) {
        let (event, span) = item.map_err(|e| TreeError {
            line: e.marker().line(),
            column: e.marker().col() + 1,
            message: e.info().to_owned(),
        })?;
        nodes += 1;
        if nodes > MAX_NODES {
            let last_index = span.start.index();
            let byte = text
                .char_indices()
                .nth(last_index)
                .map_or(text.len(), |(b, _)| b);
            return Err(too_large(text, byte));
        }
        let value = match event {
            Event::SequenceStart(..) => {
                stack.push(Frame::Seq(Vec::new()));
                continue;
            }
            Event::MappingStart(..) => {
                stack.push(Frame::Map(Vec::new(), None));
                continue;
            }
            Event::SequenceEnd => match stack.pop() {
                Some(Frame::Seq(items)) => Node::Array(items),
                _ => continue,
            },
            Event::MappingEnd => match stack.pop() {
                Some(Frame::Map(members, _)) => Node::Object(members),
                _ => continue,
            },
            Event::Scalar(v, ..) => Node::Scalar(v.into_owned()),
            Event::Alias(id) => Node::Scalar(format!("*{id}")),
            _ => continue,
        };
        match stack.last_mut() {
            None => docs.push(value),
            Some(Frame::Seq(items)) => items.push(value),
            Some(Frame::Map(members, pending)) => match pending.take() {
                None => *pending = Some(key_text(&value)),
                Some(k) => members.push((k, value)),
            },
        }
    }
    Ok(match docs.len() {
        0 => Node::Scalar(String::new()),
        1 => docs.pop().expect("one document"),
        _ => Node::Array(docs),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn obj(n: &Node) -> &[(String, Node)] {
        match n {
            Node::Object(m) => m,
            _ => panic!("not an object: {n:?}"),
        }
    }

    fn scalar(n: &Node) -> &str {
        match n {
            Node::Scalar(s) => s,
            _ => panic!("not a scalar: {n:?}"),
        }
    }

    fn fmt(src: &str) -> String {
        let out = format_xml(src, "  ").unwrap();
        assert_eq!(format_xml(&out, "  ").unwrap(), out, "idempotent");
        out
    }

    #[test]
    fn format_xml_nests_elements() {
        let src = "<?xml version=\"1.0\"?>\n<a><b><c>1</c><d/></b>\n\n\n<e x='1'  y=\"&quot;q&amp;\"></e></a>\n";
        assert_eq!(
            fmt(src),
            "<?xml version=\"1.0\"?>\n<a>\n  <b>\n    <c>1</c>\n    <d/>\n  </b>\n\n  <e x='1'  y=\"&quot;q&amp;\"></e>\n</a>\n"
        );
        assert_eq!(
            format_xml("<a><b/></a>", "\t").unwrap(),
            "<a>\n\t<b/>\n</a>"
        );
    }

    #[test]
    fn format_xml_keeps_comments_pis_and_doctype() {
        let src =
            "<!DOCTYPE r [<!ENTITY e \"x\">]><!-- top --><r><?pi  data ?><!--\n  c\n--><s/></r>";
        assert_eq!(
            fmt(src),
            "<!DOCTYPE r [<!ENTITY e \"x\">]>\n<!-- top -->\n<r>\n  <?pi  data ?>\n  <!--\n  c\n-->\n  <s/>\n</r>"
        );
    }

    #[test]
    fn format_xml_leaves_mixed_content_and_cdata() {
        let src = "<r><p>Some <b>bold</b>   <i> x </i>\n tail</p><q><![CDATA[ <a>  </a> ]]><z/></q><t>&e;<u/></t><s xml:space=\"preserve\"> <v/> </s><w>  </w></r>";
        assert_eq!(
            fmt(src),
            "<r>\n  <p>Some <b>bold</b>   <i> x </i>\n tail</p>\n  <q><![CDATA[ <a>  </a> ]]><z/></q>\n  <t>&e;<u/></t>\n  <s xml:space=\"preserve\"> <v/> </s>\n  <w>  </w>\n</r>"
        );
        // Text outside elements: unchanged.
        assert_eq!(fmt("x <a><b/></a>"), "x <a><b/></a>");
    }

    #[test]
    fn format_xml_keeps_empty_element_style_and_crlf() {
        let src = "<a>\r\n<b/><c></c>\r\n    <d >\r\n<e/></d></a>\r\n";
        assert_eq!(
            fmt(src),
            "<a>\r\n  <b/>\r\n  <c></c>\r\n  <d >\r\n    <e/>\r\n  </d>\r\n</a>\r\n"
        );
        assert_eq!(fmt("<a><b/></a>"), "<a>\n  <b/>\n</a>");
    }

    #[test]
    fn format_xml_keeps_base_indentation_of_a_selection() {
        assert_eq!(fmt("    <a><b/></a>"), "    <a>\n      <b/>\n    </a>");
    }

    #[test]
    fn format_xml_rejects_malformed_input() {
        let e = format_xml("<a>\n  <b></c>\n</a>", "  ").unwrap_err();
        assert_eq!(e.line, 2, "{e:?}");
        let e = format_xml("<a><b></b>", "  ").unwrap_err();
        assert!(e.message.contains("<a>"), "{e:?}");
        assert!(format_xml("<a></a></b>", "  ").is_err());
        assert!(format_xml("<a><!-- x </a>", "  ").is_err());
    }

    #[test]
    fn format_xml_keeps_bom_and_non_xml_whitespace() {
        let out = format_xml("\u{feff}<a><b\u{e9}/></a>", "  ").unwrap();
        assert_eq!(out, "\u{feff}<a>\n  <b\u{e9}/>\n</a>");
        assert_eq!(format_xml(&out, "  ").unwrap(), out);
        for ws in ["\u{a0}", "\u{3000}", "\u{85}", "\u{2028}"] {
            let src = format!("<a><b/>{ws}<c/></a>");
            let out = format_xml(&src, "  ").unwrap();
            assert!(out.contains(ws), "{ws:?} kept: {out:?}");
        }
        let e = format_xml("\u{feff}<a></b>", "  ").unwrap_err();
        assert_eq!(e.line, 1);
    }

    #[test]
    fn format_xml_handles_deep_nesting_and_empty_input() {
        let nest = |n| format!("{}<x/>{}", "<a>".repeat(n), "</a>".repeat(n));
        assert!(format_xml(&nest(1_000), " ").is_ok());
        let e = format_xml(&nest(50_000), " ").unwrap_err();
        assert_eq!(e.message, "too large");
        assert_eq!(fmt(""), "");
        assert_eq!(fmt("  \n"), "  \n");
    }

    #[test]
    fn xml_elements_attributes_and_text() {
        let src = r#"<?xml version="1.0"?>
<!-- comment -->
<config version="2" name="a &amp; b">
  <item id="1">first &lt;one&gt; &#65;</item>
  <item id="2"/>
  <note>mixed <b>bold</b> tail</note>
  <empty></empty>
</config>"#;
        let root = xml_tree(src).unwrap();
        let (name, config) = &obj(&root)[0];
        assert_eq!(name, "config");
        let m = obj(config);
        assert_eq!(m[0], ("@version".into(), Node::Scalar("2".into())));
        assert_eq!(scalar(&m[1].1), "a & b");
        let item1 = obj(&m[2].1);
        assert_eq!(item1[0].0, "@id");
        assert_eq!(
            item1[1],
            ("#text".into(), Node::Scalar("first <one> A".into()))
        );
        assert_eq!(obj(&m[3].1)[0].0, "@id");
        let note: Vec<&str> = obj(&m[4].1).iter().map(|(k, _)| k.as_str()).collect();
        assert_eq!(note, vec!["#text", "b", "#text"]);
        assert_eq!(m[5], ("empty".into(), Node::Scalar(String::new())));
    }

    #[test]
    fn xml_entities_are_never_expanded() {
        let bomb = r#"<?xml version="1.0"?>
<!DOCTYPE lolz [<!ENTITY lol "lol"><!ENTITY lol2 "&lol;&lol;&lol;&lol;">]>
<lolz>&lol2;</lolz>"#;
        let root = xml_tree(bomb).unwrap();
        assert_eq!(scalar(&obj(&root)[0].1), "&lol2;");
        let ext = r#"<!DOCTYPE x [<!ENTITY e SYSTEM "file:///etc/passwd">]><x>&e;</x>"#;
        assert_eq!(scalar(&obj(&xml_tree(ext).unwrap())[0].1), "&e;");
    }

    #[test]
    fn xml_errors_have_positions() {
        let e = xml_tree("<a>\n  <b></c>\n</a>").unwrap_err();
        assert_eq!(e.line, 2, "{e:?}");
        let e = xml_tree("<a><b></b>").unwrap_err();
        assert!(e.message.contains("<a>"), "{e:?}");
    }

    #[test]
    fn yaml_maps_sequences_and_aliases() {
        let src = "name: Text3\nlist:\n  - one\n  - two: 2\nbase: &b {x: 1}\ncopy: *b\n";
        let root = yaml_tree(src).unwrap();
        let m = obj(&root);
        assert_eq!(m[0], ("name".into(), Node::Scalar("Text3".into())));
        match &m[1].1 {
            Node::Array(items) => {
                assert_eq!(scalar(&items[0]), "one");
                assert_eq!(obj(&items[1])[0].0, "two");
            }
            n => panic!("{n:?}"),
        }
        assert!(
            scalar(&m[3].1).starts_with('*'),
            "alias shown, not expanded"
        );
    }

    #[test]
    fn yaml_billion_laughs_stays_small() {
        let mut src = String::from("a: &a [x, x, x, x, x, x, x, x, x]\n");
        let mut prev = 'a';
        for c in 'b'..='j' {
            src.push_str(&format!("{c}: &{c} [*{prev}, *{prev}, *{prev}, *{prev}, *{prev}, *{prev}, *{prev}, *{prev}, *{prev}]\n"));
            prev = c;
        }
        let root = yaml_tree(&src).unwrap();
        assert_eq!(obj(&root).len(), 10);
    }

    #[test]
    fn yaml_multiple_documents_and_errors() {
        match &yaml_tree("a: 1\n---\nb: 2\n").unwrap() {
            Node::Array(d) => assert_eq!(d.len(), 2),
            n => panic!("{n:?}"),
        }
        let e = yaml_tree("a: [1, 2\nb: 3\n").unwrap_err();
        assert!(e.line >= 1 && !e.message.is_empty(), "{e:?}");
    }

    #[test]
    fn deep_nesting_does_not_overflow() {
        let deep = format!("{}{}", "<a>".repeat(100_000), "</a>".repeat(100_000));
        assert!(xml_tree(&deep).is_ok());
        let deep_yaml = format!("{}{}", "[".repeat(50_000), "]".repeat(50_000));
        let _ = yaml_tree(&deep_yaml);
    }
}
