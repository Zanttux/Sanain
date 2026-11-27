//! Mermaid `classDiagram`, drawn with the flowchart layered layout.
//!
//! Supported: classes declared with `class A`, `class A { ... }` (one
//! member per line or `;`-separated), `class A["Label"]` and
//! `A : +member`; members ending in `)` (or with `(` ... `)` and a return
//! type after it, shown as `foo() : Type`) are methods, the rest
//! attributes; visibility markers `+ - # ~` are kept as written, a `$`
//! suffix marks a static member (underlined) and `*` an abstract one
//! (italic). Generics `List~T~` are shown as `List<T>`, also nested.
//! Annotations `<<interface>> A` or `<<interface>>` inside the body.
//! Relationships `<|--`, `*--`, `o--`, `-->`, `--`, `..>`, `..|>`, `..`
//! with the end on either side (or both, such as `<|--|>`), optional
//! cardinalities `A "1" --> "*" B` and `: label`. `namespace X { ... }`
//! groups classes in a titled frame (not nested). Notes: `note for A
//! "text"` (joined to the class by a dotted line) and `note "text"`.
//! `direction` sets the layout direction.
//!
//! `classDef`, `style`, `cssClass`, `click`, `link` and `callback` lines
//! and `:::class` suffixes are ignored; links and callbacks never run.
//! Anything else (lollipop `()--` interfaces, ...) makes the diagram
//! [`Error::Unsupported`], so the source is shown.

use super::state::{find_colon, strip_class, text, whole_ident};
use super::{
    Direction, Edge, Error, Flowchart, Head, Layout, MAX_EDGES, MAX_NODES, MAX_SOURCE_BYTES,
    MAX_SUBGRAPHS, Node, PResult, Shape, Spacing, Stroke, Subgraph, direction, header_line,
    layout_sized, strip_comment, truncate,
};

/// Members (attributes and methods) of all classes together.
pub const MAX_MEMBERS: usize = 500;

/// The end of a relationship line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum End {
    None,
    /// `<` / `>`: an open arrow (association, dependency).
    Arrow,
    /// `<|` / `|>`: a hollow triangle (inheritance, realization).
    Triangle,
    /// `*`: a filled diamond (composition).
    Diamond,
    /// `o`: a hollow diamond (aggregation).
    DiamondOpen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Classifier {
    None,
    /// `$`: drawn underlined.
    Static,
    /// `*`: drawn in italics.
    Abstract,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Member {
    pub text: String,
    pub classifier: Classifier,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Class {
    pub id: String,
    /// Shown name, with generics as `<T>`.
    pub name: String,
    /// `<<interface>>` and the like, without the brackets.
    pub annotation: Option<String>,
    pub attributes: Vec<Member>,
    pub methods: Vec<Member>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Relation {
    /// Class written on the left.
    pub from: usize,
    pub to: usize,
    pub from_end: End,
    pub to_end: End,
    /// `..` instead of `--`.
    pub dashed: bool,
    pub label: Option<String>,
    pub from_card: Option<String>,
    pub to_card: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassDiagram {
    pub classes: Vec<Class>,
    pub relations: Vec<Relation>,
    /// Note texts.
    pub notes: Vec<String>,
    /// The layout graph: nodes are the classes then the notes, edges the
    /// relations then the note links, subgraphs the namespaces.
    pub graph: Flowchart,
}

#[derive(Default)]
struct Parser {
    direction: Option<Direction>,
    classes: Vec<Class>,
    /// Namespace of each class.
    ns_of: Vec<Option<usize>>,
    namespaces: Vec<String>,
    relations: Vec<Relation>,
    /// Text and the class it is for.
    notes: Vec<(String, Option<usize>)>,
    members: usize,
    /// Class whose `{ ... }` body is open, and the line it opened on.
    body: Option<(usize, usize)>,
    /// Open namespace and its line.
    ns: Option<(usize, usize)>,
}

const IGNORED: &[&str] = &["classDef", "style", "cssClass", "click", "link", "callback"];

/// Parse a `classDiagram` block.
pub fn parse(src: &str) -> Result<ClassDiagram, Error> {
    if src.len() > MAX_SOURCE_BYTES {
        return Err(Error::TooLarge);
    }
    let Some((start, head)) = header_line(src) else {
        return Err(Error::UnsupportedType);
    };
    let mut words = head.split_whitespace();
    if !matches!(words.next(), Some("classDiagram" | "classDiagram-v2")) {
        return Err(Error::UnsupportedType);
    }
    if words.next().is_some() {
        return Err(Error::Unsupported { line: start + 1 });
    }
    let mut p = Parser::default();
    for (i, raw) in src.lines().enumerate().skip(start + 1) {
        let line = i + 1;
        let text = strip_comment(raw).trim();
        p.statement(text, line)
            .map_err(|e| e.unwrap_or(Error::Unsupported { line }))?;
    }
    p.finish()
}

/// `~` pairs as angle brackets: `List~List~int~~` is `List<List<int>>`.
pub fn generics(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut out = String::with_capacity(s.len());
    let mut depth = 0usize;
    for (i, &c) in chars.iter().enumerate() {
        if c != '~' {
            out.push(c);
            continue;
        }
        let next_word = chars
            .get(i + 1)
            .is_some_and(|&n| n.is_alphanumeric() || n == '_');
        if depth > 0 && !next_word {
            out.push('>');
            depth -= 1;
        } else if next_word {
            out.push('<');
            depth += 1;
        } else {
            out.push('~');
        }
    }
    out
}

/// A class reference: id and optional generic (`Name~T~`).
fn class_ref(tok: &str) -> PResult<(&str, Option<&str>)> {
    let tok = strip_class(tok.trim())?;
    match tok.find('~') {
        Some(i) => {
            let g = tok[i..]
                .strip_prefix('~')
                .and_then(|g| g.strip_suffix('~'))
                .filter(|g| !g.is_empty())
                .ok_or(None)?;
            Ok((whole_ident(&tok[..i])?, Some(g)))
        }
        None => Ok((whole_ident(tok)?, None)),
    }
}

/// The relationship operator in `s`: its start, length 2, and whether it
/// is dashed (`..`).
fn find_op(s: &str) -> Option<(usize, bool)> {
    let mut quoted = false;
    for (i, c) in s.char_indices() {
        match c {
            '"' => quoted = !quoted,
            '-' if !quoted && s[i..].starts_with("--") => return Some((i, false)),
            '.' if !quoted && s[i..].starts_with("..") => return Some((i, true)),
            _ => {}
        }
    }
    None
}

/// A quoted cardinality at the end (`left`) or start of `s`, and the rest.
fn card(s: &str, left: bool) -> PResult<(Option<String>, &str)> {
    let s = s.trim();
    if left {
        let Some(body) = s.strip_suffix('"') else {
            return Ok((None, s));
        };
        let open = body.rfind('"').ok_or(None)?;
        Ok((Some(text(&body[open + 1..])), &body[..open]))
    } else {
        let Some(body) = s.strip_prefix('"') else {
            return Ok((None, s));
        };
        let close = body.find('"').ok_or(None)?;
        Ok((Some(text(&body[..close])), &body[close + 1..]))
    }
}

fn member(raw: &str) -> (bool, Member) {
    let mut t = raw.trim();
    let mut classifier = Classifier::None;
    if let Some(r) = t.strip_suffix('$') {
        (t, classifier) = (r.trim_end(), Classifier::Static);
    } else if let Some(r) = t.strip_suffix('*') {
        (t, classifier) = (r.trim_end(), Classifier::Abstract);
    }
    let method = match (t.find('('), t.rfind(')')) {
        (Some(o), Some(c)) if o < c => Some(c),
        _ => None,
    };
    let shown = match method {
        Some(c) => {
            let mut after = &t[c + 1..];
            if let Some(r) = after.strip_prefix('$') {
                (after, classifier) = (r, Classifier::Static);
            } else if let Some(r) = after.strip_prefix('*') {
                (after, classifier) = (r, Classifier::Abstract);
            }
            let ret = after.trim();
            if ret.is_empty() {
                t[..=c].to_owned()
            } else {
                format!("{} : {ret}", &t[..=c])
            }
        }
        None => t.to_owned(),
    };
    (
        method.is_some(),
        Member {
            text: truncate(&generics(&shown)),
            classifier,
        },
    )
}

fn end_left(lhs: &str) -> PResult<(End, &str)> {
    if lhs.ends_with("()") {
        // Lollipop interfaces are outside the subset.
        return Err(None);
    }
    if let Some(l) = lhs.strip_suffix("<|") {
        return Ok((End::Triangle, l));
    }
    if let Some(l) = lhs.strip_suffix('<') {
        return Ok((End::Arrow, l));
    }
    if let Some(l) = lhs.strip_suffix('*') {
        return Ok((End::Diamond, l));
    }
    if let Some(l) = lhs.strip_suffix('o')
        && l.ends_with(|c: char| c.is_whitespace() || c == '"')
    {
        return Ok((End::DiamondOpen, l));
    }
    Ok((End::None, lhs))
}

fn end_right(rhs: &str) -> PResult<(End, &str)> {
    if rhs.starts_with("()") {
        return Err(None);
    }
    if let Some(r) = rhs.strip_prefix("|>") {
        return Ok((End::Triangle, r));
    }
    if let Some(r) = rhs.strip_prefix('>') {
        return Ok((End::Arrow, r));
    }
    if let Some(r) = rhs.strip_prefix('*') {
        return Ok((End::Diamond, r));
    }
    if let Some(r) = rhs.strip_prefix('o')
        && r.starts_with(|c: char| c.is_whitespace() || c == '"')
    {
        return Ok((End::DiamondOpen, r));
    }
    Ok((End::None, rhs))
}

impl Parser {
    /// The class `id`, created on first use (inside the open namespace).
    fn class(&mut self, id: &str, generic: Option<&str>) -> PResult<usize> {
        let i = match self.classes.iter().position(|c| c.id == id) {
            Some(i) => i,
            None => {
                if self.classes.len() + self.notes.len() >= MAX_NODES {
                    return Err(Some(Error::TooLarge));
                }
                self.classes.push(Class {
                    id: id.to_owned(),
                    name: truncate(id),
                    annotation: None,
                    attributes: Vec::new(),
                    methods: Vec::new(),
                });
                self.ns_of.push(self.ns.map(|n| n.0));
                self.classes.len() - 1
            }
        };
        if let Some(g) = generic {
            self.classes[i].name = truncate(&format!("{id}<{}>", generics(g)));
        }
        Ok(i)
    }

    fn add_member(&mut self, class: usize, raw: &str) -> PResult<()> {
        let raw = raw.trim();
        if raw.is_empty() {
            return Ok(());
        }
        if let Some(a) = raw.strip_prefix("<<").and_then(|r| r.strip_suffix(">>")) {
            self.classes[class].annotation = Some(truncate(a.trim()));
            return Ok(());
        }
        if self.members >= MAX_MEMBERS {
            return Err(Some(Error::TooLarge));
        }
        self.members += 1;
        let (method, m) = member(raw);
        let c = &mut self.classes[class];
        if method {
            c.methods.push(m);
        } else {
            c.attributes.push(m);
        }
        Ok(())
    }

    /// Members inside `{ ... }`: one per line, or separated by `;`.
    fn body_text(&mut self, class: usize, s: &str) -> PResult<()> {
        for part in s.split(';') {
            self.add_member(class, part)?;
        }
        Ok(())
    }

    fn statement(&mut self, s: &str, line: usize) -> PResult<()> {
        if let Some((class, _)) = self.body {
            let (content, closes) = match s.strip_suffix('}') {
                Some(c) => (c, true),
                None => (s, false),
            };
            self.body_text(class, content)?;
            if closes {
                self.body = None;
            }
            return Ok(());
        }
        if s.is_empty() {
            return Ok(());
        }
        if s == "}" {
            return self.ns.take().map(|_| ()).ok_or(None);
        }
        let first = s.split_whitespace().next().unwrap_or("");
        if IGNORED.contains(&first) {
            return Ok(());
        }
        match first {
            "direction" => {
                let mut w = s.split_whitespace().skip(1);
                let (Some(d), None) = (w.next().and_then(direction), w.next()) else {
                    return Err(None);
                };
                self.direction = Some(d);
                return Ok(());
            }
            "namespace" => {
                let rest = s["namespace".len()..].trim();
                let name = rest.strip_suffix('{').ok_or(None)?.trim();
                let name = whole_ident(name)?;
                if self.ns.is_some() {
                    return Err(None);
                }
                let k = match self.namespaces.iter().position(|n| n == name) {
                    Some(k) => k,
                    None => {
                        if self.namespaces.len() >= MAX_SUBGRAPHS {
                            return Err(Some(Error::TooLarge));
                        }
                        self.namespaces.push(name.to_owned());
                        self.namespaces.len() - 1
                    }
                };
                self.ns = Some((k, line));
                return Ok(());
            }
            "note" => return self.note(s["note".len()..].trim()),
            "class" => return self.class_decl(s["class".len()..].trim(), line),
            _ => {}
        }
        if let Some(rest) = s.strip_prefix("<<") {
            let close = rest.find(">>").ok_or(None)?;
            let (id, g) = class_ref(&rest[close + 2..])?;
            let c = self.class(id, g)?;
            self.classes[c].annotation = Some(truncate(rest[..close].trim()));
            return Ok(());
        }
        let colon = find_colon(s);
        let rel = &s[..colon.unwrap_or(s.len())];
        if let Some((i, dashed)) = find_op(rel) {
            let rhs = &rel[i + 2..];
            if rhs.starts_with(['-', '.']) {
                return Err(None);
            }
            let (from_end, lhs) = end_left(&rel[..i])?;
            let (to_end, rhs) = end_right(rhs)?;
            let (from_card, lhs) = card(lhs, true)?;
            let (to_card, rhs) = card(rhs, false)?;
            let (a, ga) = class_ref(lhs)?;
            let (b, gb) = class_ref(rhs)?;
            let from = self.class(a, ga)?;
            let to = self.class(b, gb)?;
            if self.relations.len() + self.notes.len() >= MAX_EDGES {
                return Err(Some(Error::TooLarge));
            }
            let label = colon.map(|j| text(&s[j + 1..])).filter(|l| !l.is_empty());
            self.relations.push(Relation {
                from,
                to,
                from_end,
                to_end,
                dashed,
                label,
                from_card: from_card.filter(|c| !c.is_empty()),
                to_card: to_card.filter(|c| !c.is_empty()),
            });
            return Ok(());
        }
        let (id, g) = class_ref(rel)?;
        let c = self.class(id, g)?;
        if let Some(j) = colon {
            self.add_member(c, &s[j + 1..])?;
        }
        Ok(())
    }

    /// `A`, `A~T~`, `A["Label"]`, each optionally with a `{ ... }` body.
    fn class_decl(&mut self, rest: &str, line: usize) -> PResult<()> {
        let (head, body) = match rest.find('{') {
            Some(k) => (rest[..k].trim(), Some(&rest[k + 1..])),
            None => (rest, None),
        };
        let (head, label) = match head.find('[') {
            Some(k) => {
                let l = head[k + 1..]
                    .strip_suffix(']')
                    .map(str::trim)
                    .and_then(|l| l.strip_prefix('"'))
                    .and_then(|l| l.strip_suffix('"'))
                    .ok_or(None)?;
                (head[..k].trim(), Some(text(l)))
            }
            None => (head, None),
        };
        let (id, g) = class_ref(head)?;
        let c = self.class(id, g)?;
        if let Some(l) = label.filter(|l| !l.is_empty()) {
            self.classes[c].name = l;
        }
        if let Some((k, _)) = self.ns
            && self.ns_of[c].is_none()
        {
            self.ns_of[c] = Some(k);
        }
        if let Some(body) = body {
            match body.trim_end().strip_suffix('}') {
                Some(inline) => self.body_text(c, inline)?,
                None => {
                    self.body_text(c, body)?;
                    self.body = Some((c, line));
                }
            }
        }
        Ok(())
    }

    /// `for A "text"` or `"text"`.
    fn note(&mut self, rest: &str) -> PResult<()> {
        let (target, quoted) = match rest.strip_prefix("for") {
            Some(r) if r.starts_with(char::is_whitespace) => {
                let r = r.trim_start();
                let q = r.find('"').ok_or(None)?;
                let (id, g) = class_ref(&r[..q])?;
                (Some(self.class(id, g)?), &r[q..])
            }
            _ => (None, rest),
        };
        let body = quoted
            .strip_prefix('"')
            .and_then(|q| q.strip_suffix('"'))
            .ok_or(None)?;
        if body.contains('"') {
            return Err(None);
        }
        if self.classes.len() + self.notes.len() >= MAX_NODES
            || self.relations.len() + self.notes.len() >= MAX_EDGES
        {
            return Err(Some(Error::TooLarge));
        }
        self.notes
            .push((text(&body.replace("\\n", "<br>")), target));
        Ok(())
    }

    fn finish(self) -> Result<ClassDiagram, Error> {
        if let Some((_, line)) = self.body.or(self.ns) {
            return Err(Error::Unsupported { line });
        }
        // Frames only for namespaces holding a class.
        let mut sub = vec![None; self.namespaces.len()];
        let mut subgraphs = Vec::new();
        for (k, name) in self.namespaces.iter().enumerate() {
            if self.ns_of.contains(&Some(k)) {
                sub[k] = Some(subgraphs.len());
                subgraphs.push(Subgraph {
                    id: name.clone(),
                    title: truncate(name),
                    parent: None,
                });
            }
        }
        let ns = |c: usize| self.ns_of[c].and_then(|k| sub[k]);
        let mut nodes: Vec<Node> = self
            .classes
            .iter()
            .enumerate()
            .map(|(i, c)| Node {
                id: c.id.clone(),
                label: c.name.clone(),
                shape: Shape::Rect,
                subgraph: ns(i),
            })
            .collect();
        let mut edges: Vec<Edge> = self
            .relations
            .iter()
            .map(|r| Edge {
                from: r.from,
                to: r.to,
                label: r.label.clone(),
                stroke: if r.dashed {
                    Stroke::Dotted
                } else {
                    Stroke::Solid
                },
                head: Head::None,
                tail: Head::None,
                from_group: None,
                to_group: None,
            })
            .collect();
        for (text, target) in &self.notes {
            let at = nodes.len();
            nodes.push(Node {
                id: String::new(),
                label: text.clone(),
                shape: Shape::Rect,
                subgraph: target.and_then(ns),
            });
            if let Some(t) = *target {
                edges.push(Edge {
                    from: t,
                    to: at,
                    label: None,
                    stroke: Stroke::Dotted,
                    head: Head::None,
                    tail: Head::None,
                    from_group: None,
                    to_group: None,
                });
            }
        }
        Ok(ClassDiagram {
            graph: Flowchart {
                direction: self.direction.unwrap_or(Direction::Down),
                nodes,
                edges,
                subgraphs,
            },
            classes: self.classes,
            relations: self.relations,
            notes: self.notes.into_iter().map(|n| n.0).collect(),
        })
    }
}

/// Compartments of a class box, relative to its top edge.
#[derive(Debug, Clone, PartialEq)]
pub struct ClassBox {
    /// Bottom of the name compartment.
    pub header: f32,
    /// Bottom of the attribute compartment.
    pub attributes: f32,
    /// Centre of each text line: the annotation (if any), the name, the
    /// attributes, then the methods.
    pub rows: Vec<f32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassLayout {
    /// Boxes for classes then notes, relation paths then note links, and
    /// namespace frames.
    pub base: Layout,
    /// Per class.
    pub boxes: Vec<ClassBox>,
    /// Per relation: centres of the cardinality texts at the `from` and
    /// `to` ends.
    pub cards: Vec<[Option<(f32, f32)>; 2]>,
}

/// The annotation as shown: `«interface»`.
pub fn annotation_text(a: &str) -> String {
    format!("«{a}»")
}

/// The point `d` along `ps` from its start (at most 45 % of the way),
/// moved `off` to the side.
fn along(ps: &[(f32, f32)], d: f32, off: f32) -> (f32, f32) {
    type Seg = ((f32, f32), (f32, f32), f32);
    let segs: Vec<Seg> = ps
        .windows(2)
        .map(|w| {
            let l = ((w[1].0 - w[0].0).powi(2) + (w[1].1 - w[0].1).powi(2)).sqrt();
            (w[0], w[1], l)
        })
        .filter(|s| s.2 > 0.0)
        .collect();
    let total: f32 = segs.iter().map(|s| s.2).sum();
    let mut left = d.min(total * 0.45);
    for (k, &(a, b, l)) in segs.iter().enumerate() {
        if left <= l || k + 1 == segs.len() {
            let t = (left / l).min(1.0);
            let (ux, uy) = ((b.0 - a.0) / l, (b.1 - a.1) / l);
            return (
                a.0 + (b.0 - a.0) * t - uy * off,
                a.1 + (b.1 - a.1) * t + ux * off,
            );
        }
        left -= l;
    }
    ps.first().copied().unwrap_or((0.0, 0.0))
}

/// Lay out a class diagram. `measure` returns the size of a text line.
pub fn layout(
    cd: &ClassDiagram,
    sp: Spacing,
    measure: &mut dyn FnMut(&str) -> (f32, f32),
) -> Result<ClassLayout, Error> {
    let pad = sp.pad;
    let mut sizes = Vec::with_capacity(cd.graph.nodes.len());
    let mut boxes = Vec::with_capacity(cd.classes.len());
    for c in &cd.classes {
        let mut rows = Vec::new();
        let mut width = 0f32;
        let mut y = pad * 0.4;
        let mut line = |t: &str, y: &mut f32, rows: &mut Vec<f32>| {
            let (w, h) = measure(t);
            width = width.max(w);
            rows.push(*y + h / 2.0);
            *y += h;
        };
        if let Some(a) = &c.annotation {
            line(&annotation_text(a), &mut y, &mut rows);
        }
        line(&c.name, &mut y, &mut rows);
        let header = y + pad * 0.4;
        let mut compartment = |ms: &[Member], top: f32, rows: &mut Vec<f32>| {
            let mut y = top + pad * 0.3;
            for m in ms {
                line(&m.text, &mut y, rows);
            }
            (y + pad * 0.3).max(top + pad * 0.6)
        };
        let attributes = compartment(&c.attributes, header, &mut rows);
        let bottom = compartment(&c.methods, attributes, &mut rows);
        sizes.push(((width + 2.0 * pad).max(pad * 5.0), bottom));
        boxes.push(ClassBox {
            header,
            attributes,
            rows,
        });
    }
    for n in &cd.notes {
        let (w, h) = measure(n);
        sizes.push((w + 2.0 * pad, h + pad));
    }
    // Larger ends than flowchart arrows, and room for cardinalities at
    // both ends of a line.
    let cards_used = cd
        .relations
        .iter()
        .any(|r| r.from_card.is_some() || r.to_card.is_some());
    let sp = Spacing {
        layer_gap: sp.layer_gap * if cards_used { 1.7 } else { 1.3 },
        ..sp
    };
    let base = layout_sized(&cd.graph, sp, &sizes, measure)?;
    let cards = cd
        .relations
        .iter()
        .zip(&base.edges)
        .map(|(r, e)| {
            let mut rev = e.points.clone();
            rev.reverse();
            let (d, off) = (pad * 1.8, pad * 1.2);
            [
                r.from_card.as_ref().map(|_| along(&e.points, d, off)),
                r.to_card.as_ref().map(|_| along(&rev, d, -off)),
            ]
        })
        .collect();
    Ok(ClassLayout { base, boxes, cards })
}

#[cfg(test)]
mod tests {
    use super::super::tests::{assert_frames_ok, assert_no_overlap, measure};
    use super::*;

    fn lay(src: &str) -> (ClassDiagram, ClassLayout) {
        let cd = parse(src).unwrap_or_else(|e| panic!("{e:?} in {src}"));
        let l = layout(&cd, Spacing::for_font(14.0), &mut measure).expect("lays out");
        (cd, l)
    }

    fn class<'a>(cd: &'a ClassDiagram, id: &str) -> &'a Class {
        cd.classes.iter().find(|c| c.id == id).expect("class")
    }

    fn texts(ms: &[Member]) -> Vec<&str> {
        ms.iter().map(|m| m.text.as_str()).collect()
    }

    #[test]
    fn headers() {
        assert!(parse("classDiagram\nA").is_ok());
        assert!(parse("classDiagram-v2\nA").is_ok());
        assert_eq!(
            parse("classDiagram LR\nA"),
            Err(Error::Unsupported { line: 1 })
        );
        assert_eq!(parse("stateDiagram\nA"), Err(Error::UnsupportedType));
    }

    #[test]
    fn members_generics_and_classifiers() {
        let cd = parse(
            "classDiagram\nclass Animal {\n  +int age\n  +String gender\n  -List~int~ ids\n  +isMammal() bool\n  +mate()\n  +count()$ int\n  +someAbstract()*\n  +String KEY$\n}\nclass Point { +int x; +int y; +move(dx, dy) Point }\nAnimal : +bar()\nAnimal : #Map~String, List~int~~ index\nclass Square~Shape~\nclass Empty {}\nclass Named[\"Pretty name\"]",
        )
        .expect("parses");
        let a = class(&cd, "Animal");
        assert_eq!(
            texts(&a.attributes),
            [
                "+int age",
                "+String gender",
                "-List<int> ids",
                "+String KEY",
                "#Map<String, List<int>> index"
            ]
        );
        assert_eq!(
            texts(&a.methods),
            [
                "+isMammal() : bool",
                "+mate()",
                "+count() : int",
                "+someAbstract()",
                "+bar()"
            ]
        );
        assert_eq!(a.methods[2].classifier, Classifier::Static);
        assert_eq!(a.methods[3].classifier, Classifier::Abstract);
        assert_eq!(a.attributes[3].classifier, Classifier::Static);
        assert_eq!(a.attributes[0].classifier, Classifier::None);
        let p = class(&cd, "Point");
        assert_eq!(texts(&p.attributes), ["+int x", "+int y"]);
        assert_eq!(texts(&p.methods), ["+move(dx, dy) : Point"]);
        assert_eq!(class(&cd, "Square").name, "Square<Shape>");
        assert!(class(&cd, "Empty").methods.is_empty());
        assert_eq!(class(&cd, "Named").name, "Pretty name");
        assert_eq!(generics("List~List~int~~"), "List<List<int>>");
        assert_eq!(generics("a ~ b"), "a ~ b");
    }

    #[test]
    fn annotations() {
        let cd = parse(
            "classDiagram\n<<interface>> Shape\nclass Color {\n  <<enumeration>>\n  RED\n  GREEN\n}",
        )
        .expect("parses");
        assert_eq!(class(&cd, "Shape").annotation.as_deref(), Some("interface"));
        let c = class(&cd, "Color");
        assert_eq!(c.annotation.as_deref(), Some("enumeration"));
        assert_eq!(texts(&c.attributes), ["RED", "GREEN"]);
        assert_eq!(annotation_text("interface"), "«interface»");
    }

    #[test]
    fn relationships_both_ways() {
        use End::{Arrow, Diamond, DiamondOpen, None as N, Triangle};
        let cases = [
            ("A <|-- B", Triangle, N, false),
            ("A --|> B", N, Triangle, false),
            ("A *-- B", Diamond, N, false),
            ("A --* B", N, Diamond, false),
            ("A o-- B", DiamondOpen, N, false),
            ("A --o B", N, DiamondOpen, false),
            ("A --> B", N, Arrow, false),
            ("A <-- B", Arrow, N, false),
            ("A -- B", N, N, false),
            ("A ..> B", N, Arrow, true),
            ("A <.. B", Arrow, N, true),
            ("A ..|> B", N, Triangle, true),
            ("A <|.. B", Triangle, N, true),
            ("A .. B", N, N, true),
            ("A <|--|> B", Triangle, Triangle, false),
            ("A *--o B", Diamond, DiamondOpen, false),
            ("A<|--B", Triangle, N, false),
            ("Foo--Bar", N, N, false),
        ];
        for (src, f, t, dashed) in cases {
            let cd =
                parse(&format!("classDiagram\n{src}")).unwrap_or_else(|e| panic!("{e:?} {src}"));
            let r = &cd.relations[0];
            assert_eq!((r.from_end, r.to_end, r.dashed), (f, t, dashed), "{src}");
            assert_eq!(cd.classes.len(), 2, "{src}");
            assert_eq!((r.from, r.to), (0, 1), "{src}");
        }
        let cd = parse(
            "classDiagram\nCustomer \"1\" --> \"*\" Ticket : buys\nGalaxy \"many\" o-- \"1..*\" Star\nA --> B:uses",
        )
        .expect("parses");
        let r = &cd.relations[0];
        assert_eq!(
            (
                r.from_card.as_deref(),
                r.to_card.as_deref(),
                r.label.as_deref()
            ),
            (Some("1"), Some("*"), Some("buys"))
        );
        let r = &cd.relations[1];
        assert_eq!(r.from_end, End::DiamondOpen);
        assert_eq!(
            (r.from_card.as_deref(), r.to_card.as_deref()),
            (Some("many"), Some("1..*"))
        );
        assert_eq!(cd.relations[2].label.as_deref(), Some("uses"));
        // Ids ending in `o` keep it.
        let cd = parse("classDiagram\nFoo -- Bar").expect("parses");
        assert_eq!(cd.classes[0].id, "Foo");
    }

    #[test]
    fn namespaces_notes_and_direction() {
        let src = "classDiagram\ndirection LR\nnamespace Shapes {\n  class Triangle\n  class Square {\n    +side\n  }\n}\nnamespace Unused {\n}\nShape <|-- Triangle\nShape <|-- Square\nnote for Shape \"Base of\\nall shapes\"\nnote \"Free\"";
        let (cd, l) = lay(src);
        assert_eq!(cd.graph.direction, Direction::Right);
        assert_eq!(cd.graph.subgraphs.len(), 1);
        assert_eq!(cd.graph.subgraphs[0].title, "Shapes");
        let ns: Vec<Option<usize>> = cd.graph.nodes.iter().map(|n| n.subgraph).collect();
        assert_eq!(ns, [Some(0), Some(0), None, None, None]);
        assert_eq!(cd.notes, ["Base of\nall shapes", "Free"]);
        assert_eq!(cd.graph.edges.len(), 3);
        assert_eq!(cd.graph.edges[2].stroke, Stroke::Dotted);
        assert_no_overlap(&l.base);
        assert_frames_ok(&cd.graph, &l.base);
        assert_eq!(
            parse("classDiagram\nnamespace A {\nnamespace B {\n}\n}"),
            Err(Error::Unsupported { line: 3 })
        );
    }

    #[test]
    fn ignored_and_unsupported() {
        let cd = parse(
            "classDiagram\n%% comment\nclassDef x fill:#f00\nstyle A fill:#f00\ncssClass \"A\" x\nclick A href \"https://example.com\"\nlink A \"https://example.com\"\ncallback A \"cb\"\nA:::x --> B %% trailing",
        )
        .expect("parses");
        assert_eq!(cd.classes.len(), 2);
        for (src, line) in [
            ("classDiagram\nA ()-- B", 2),
            ("classDiagram\nA --() B", 2),
            ("classDiagram\nA --- B", 2),
            ("classDiagram\nfoo bar", 2),
            ("classDiagram\nclass A {\n+x", 2),
            ("classDiagram\nnamespace N {\nclass A", 2),
            ("classDiagram\n}", 2),
            ("classDiagram\nnote for A text", 2),
            ("classDiagram\nA \"1 --> B", 2),
            ("classDiagram\nclass A[Label]", 2),
            ("classDiagram\nA~ --> B", 2),
            ("classDiagram\ndirection XY", 2),
        ] {
            assert_eq!(parse(src), Err(Error::Unsupported { line }), "{src}");
        }
    }

    #[test]
    fn size_caps() {
        let many: String = (0..=MAX_NODES).map(|i| format!("class C{i}\n")).collect();
        assert_eq!(
            parse(&format!("classDiagram\n{many}")),
            Err(Error::TooLarge)
        );
        let edges: String = (0..=MAX_EDGES).map(|_| "A --> B\n").collect();
        assert_eq!(
            parse(&format!("classDiagram\n{edges}")),
            Err(Error::TooLarge)
        );
        let members: String = (0..=MAX_MEMBERS).map(|i| format!("A : m{i}\n")).collect();
        assert_eq!(
            parse(&format!("classDiagram\n{members}")),
            Err(Error::TooLarge)
        );
        let cd = parse(&format!("classDiagram\nA : {}", "z".repeat(400))).expect("parses");
        assert_eq!(
            cd.classes[0].attributes[0].text.chars().count(),
            super::super::MAX_LABEL_CHARS
        );
    }

    #[test]
    fn boxes_have_compartments_and_cards_sit_near_their_ends() {
        let (cd, l) = lay(
            "classDiagram\nclass Duck {\n  <<interface>>\n  +String beak\n  +swim()\n  +quack()\n}\nAnimal \"1\" <|-- \"many\" Duck : is",
        );
        let duck = &l.boxes[0];
        assert_eq!(duck.rows.len(), 5);
        assert!(duck.rows.windows(2).all(|w| w[0] < w[1]), "{duck:?}");
        assert!(duck.header < duck.attributes);
        let b = l.base.nodes[0];
        assert!(duck.attributes < b.h && duck.rows[4] < b.h);
        // Annotation and name in the header, then the attribute.
        assert!(duck.rows[1] < duck.header && duck.rows[2] > duck.header);
        assert!(duck.rows[2] < duck.attributes && duck.rows[3] > duck.attributes);
        // An empty class still has three compartments.
        let animal = &l.boxes[1];
        assert_eq!(animal.rows.len(), 1);
        assert!(animal.header < animal.attributes && animal.attributes < l.base.nodes[1].h);
        let [from, to] = l.cards[0];
        let path = &l.base.edges[0].points;
        let dist =
            |a: (f32, f32), b: (f32, f32)| ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt();
        let (from, to) = (from.expect("from card"), to.expect("to card"));
        let (start, end) = (path[0], *path.last().expect("end"));
        assert!(dist(from, start) < dist(from, end));
        assert!(dist(to, end) < dist(to, start));
        assert_eq!(cd.relations[0].label.as_deref(), Some("is"));
        assert!(l.base.edges[0].label_at.is_some());
        assert_no_overlap(&l.base);
    }

    #[test]
    fn random_diagrams_lay_out() {
        let mut s: u64 = 0xA076_1D64_78BD_642F;
        let mut next = |m: u64| {
            s ^= s << 13;
            s ^= s >> 7;
            s ^= s << 17;
            s % m
        };
        let ops = [
            "<|--", "--|>", "*--", "--*", "o--", "--o", "-->", "<--", "--", "..>", "<..", "..|>",
            "<|..", "..", "<|--|>",
        ];
        for round in 0..300 {
            let dir = ["TB", "BT", "LR", "RL"][round % 4];
            let mut src = format!("classDiagram\ndirection {dir}\n");
            let classes = 2 + next(20);
            let mut in_ns = false;
            for _ in 0..(3 + next(40)) {
                match next(9) {
                    0 if !in_ns => {
                        src.push_str(&format!("namespace N{} {{\n", next(3)));
                        in_ns = true;
                    }
                    1 if in_ns => {
                        src.push_str("}\n");
                        in_ns = false;
                    }
                    2 => src.push_str(&format!(
                        "class C{} {{\n  +int f{}\n  +m() int\n}}\n",
                        next(classes),
                        next(5)
                    )),
                    3 => src.push_str(&format!("note for C{} \"n\"\n", next(classes))),
                    4 => src.push_str(&format!("C{} : +x()$\n", next(classes))),
                    _ => {
                        let (a, b) = (next(classes), next(classes));
                        let op = ops[next(ops.len() as u64) as usize];
                        src.push_str(&format!("C{a} \"1\" {op} \"*\" C{b} : l\n"));
                    }
                }
            }
            if in_ns {
                src.push_str("class Fill\n}\n");
            }
            let cd = parse(&src).unwrap_or_else(|e| panic!("{e:?} in {src}"));
            let a = layout(&cd, Spacing::for_font(13.0), &mut measure).expect("lays out");
            assert_no_overlap(&a.base);
            assert_frames_ok(&cd.graph, &a.base);
            assert_eq!(a.boxes.len(), cd.classes.len());
            assert_eq!(a.cards.len(), cd.relations.len());
            let b = layout(&cd, Spacing::for_font(13.0), &mut measure).expect("lays out");
            assert_eq!(a, b, "deterministic: {src}");
        }
    }

    #[test]
    fn random_input_never_panics() {
        let tokens = [
            "A",
            "B",
            "Foo",
            "o",
            "*",
            "<|",
            "|>",
            "<",
            ">",
            "--",
            "..",
            "-",
            ".",
            "\"",
            "\"1\"",
            ":",
            " : ",
            "+",
            "-",
            "#",
            "~",
            "$",
            "(",
            ")",
            "()",
            "{",
            "}",
            "[",
            "]",
            ";",
            "class ",
            "namespace ",
            "note ",
            "for ",
            "<<",
            ">>",
            "interface",
            "\n",
            " ",
            "%%",
            ":::",
            "direction LR",
            "ä",
            "🙂",
            "<br>",
            "\\n",
        ];
        let mut s: u64 = 0x9E37_79B9_7F4A_7C15;
        let mut parsed = 0;
        for _ in 0..5000 {
            let mut src = String::from("classDiagram\n");
            let len = (s % 60) as usize;
            for _ in 0..len {
                s ^= s << 13;
                s ^= s >> 7;
                s ^= s << 17;
                src.push_str(tokens[(s % tokens.len() as u64) as usize]);
            }
            if let Ok(cd) = parse(&src)
                && let Ok(l) = layout(&cd, Spacing::for_font(12.0), &mut measure)
            {
                parsed += 1;
                assert!(l.base.width.is_finite() && l.base.height.is_finite());
                assert_eq!(l.base.nodes.len(), cd.classes.len() + cd.notes.len());
                assert_frames_ok(&cd.graph, &l.base);
            }
            s = s.wrapping_add(0x2545_F491_4F6C_DD1D);
        }
        assert!(parsed > 100, "only {parsed} parsed");
    }
}
