//! Mermaid `stateDiagram` / `stateDiagram-v2`, drawn with the flowchart
//! layered layout.
//!
//! Supported: states (`s1`, `s1 : description`, `state "Long name" as
//! s1`), transitions `a --> b` with an optional `: label`, start and end
//! (`[*]`, one of each per composite state or region), composite states
//! `state X { ... }` nested up to [`MAX_SUBGRAPH_DEPTH`] deep (titled
//! frames, laid out like flowchart subgraphs), `<<choice>>` (small
//! diamond), `<<fork>>` and `<<join>>` (bars), notes `note left of X :
//! text` / `note right of X : text` and multi-line `note ... end note`,
//! `direction` at the top level (accepted and ignored inside composites),
//! and `--` concurrency separators inside a composite: each region becomes
//! an untitled, dashed frame inside the composite's frame. Notes are boxes
//! joined to their state by a dotted line; `left of` puts the note one
//! layer before the state and `right of` one layer after it (left and
//! right in `LR` charts, above and below in `TB` charts).
//!
//! `classDef`, `class`, `style` and `click` lines and `:::class` suffixes
//! are ignored (theme colours are used). Anything else makes the diagram
//! [`Error::Unsupported`], so the source is shown.

use super::{
    Cursor, Direction, Edge, Error, Flowchart, Head, Layout, MAX_EDGES, MAX_NODES,
    MAX_SOURCE_BYTES, MAX_SUBGRAPH_DEPTH, MAX_SUBGRAPHS, Node, PResult, Shape, Spacing, Stroke,
    Subgraph, clean_label, direction, header_line, layout_sized, strip_comment, truncate,
};

/// What a node of [`StateDiagram::graph`] is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    State,
    /// `[*]` as a source: a filled dot.
    Start,
    /// `[*]` as a target: a ringed dot.
    End,
    /// `<<choice>>`: a small diamond.
    Choice,
    /// `<<fork>>`: a bar.
    Fork,
    /// `<<join>>`: a bar.
    Join,
    Note,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StateDiagram {
    /// The layout graph: nodes are states and notes, edges are
    /// transitions (solid, arrow) and note links (dotted, no head),
    /// subgraphs are composite states and their concurrent regions.
    pub graph: Flowchart,
    /// Per node of `graph`.
    pub kinds: Vec<Kind>,
    /// Per subgraph of `graph`: `true` for a concurrent region (untitled,
    /// dashed), `false` for a composite state.
    pub regions: Vec<bool>,
}

struct St {
    id: String,
    /// `state "label" as id`
    label: Option<String>,
    /// `id : description` lines.
    descs: Vec<String>,
    kind: Kind,
    /// Region the state is drawn in (0 is the top level).
    region: usize,
    /// Composite body, if `state id {` opened one.
    comp: Option<usize>,
    /// Named states can be referred to; start and end are not.
    named: bool,
}

struct Comp {
    state: usize,
    /// Body regions (more than one after `--`).
    regions: Vec<usize>,
    line: usize,
}

#[derive(Default)]
struct Reg {
    start: Option<usize>,
    end: Option<usize>,
}

struct Trans {
    from: usize,
    to: usize,
    label: Option<String>,
}

struct NoteDef {
    target: usize,
    left: bool,
    text: String,
}

#[derive(Default)]
struct Parser {
    direction: Option<Direction>,
    states: Vec<St>,
    comps: Vec<Comp>,
    regs: Vec<Reg>,
    trans: Vec<Trans>,
    notes: Vec<NoteDef>,
    /// Open composites, innermost last.
    open: Vec<usize>,
    /// A multi-line note being read: target, left, lines, start line.
    note: Option<(usize, bool, Vec<String>, usize)>,
}

const IGNORED: &[&str] = &["classDef", "class", "style", "click"];

/// Parse a `stateDiagram` / `stateDiagram-v2` block.
pub fn parse(src: &str) -> Result<StateDiagram, Error> {
    if src.len() > MAX_SOURCE_BYTES {
        return Err(Error::TooLarge);
    }
    let Some((start, head)) = header_line(src) else {
        return Err(Error::UnsupportedType);
    };
    let mut words = head.split_whitespace();
    if !matches!(words.next(), Some("stateDiagram" | "stateDiagram-v2")) {
        return Err(Error::UnsupportedType);
    }
    if words.next().is_some() {
        return Err(Error::Unsupported { line: start + 1 });
    }
    let mut p = Parser {
        regs: vec![Reg::default()],
        ..Parser::default()
    };
    for (i, raw) in src.lines().enumerate().skip(start + 1) {
        let line = i + 1;
        let text = strip_comment(raw).trim();
        p.statement(text, line)
            .map_err(|e| e.unwrap_or(Error::Unsupported { line }))?;
    }
    p.finish()
}

/// Byte index of `pat` in `s` outside double quotes.
pub(crate) fn find_unquoted(s: &str, pat: &str) -> Option<usize> {
    let mut quoted = false;
    for (i, c) in s.char_indices() {
        if c == '"' {
            quoted = !quoted;
        } else if !quoted && s[i..].starts_with(pat) {
            return Some(i);
        }
    }
    None
}

/// Byte index of the first `:` outside quotes that is not part of a
/// `:::className` suffix.
pub(crate) fn find_colon(s: &str) -> Option<usize> {
    let mut quoted = false;
    let b = s.as_bytes();
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            b'"' => quoted = !quoted,
            b':' if !quoted => {
                if s[i..].starts_with(":::") {
                    i += 3;
                    continue;
                }
                return Some(i);
            }
            _ => {}
        }
        i += 1;
    }
    None
}

/// A label that is drawn as plain text: `<br>` becomes a line break and
/// other markup is kept as written (nothing is interpreted).
pub(crate) fn text(s: &str) -> String {
    clean_label(s).unwrap_or_else(|_| truncate(s.trim()))
}

/// Drop a `:::className` suffix (styling only).
pub(crate) fn strip_class(tok: &str) -> PResult<&str> {
    match tok.find(":::") {
        Some(i) => {
            let mut c = Cursor {
                s: &tok[i + 3..],
                at: 0,
            };
            if c.ident().is_none() || !c.done() {
                return Err(None);
            }
            Ok(tok[..i].trim_end())
        }
        None => Ok(tok),
    }
}

/// The whole of `tok` as an id.
pub(crate) fn whole_ident(tok: &str) -> PResult<&str> {
    let mut c = Cursor { s: tok, at: 0 };
    let id = c.ident().ok_or(None)?;
    if !c.done() {
        return Err(None);
    }
    Ok(id)
}

enum Ref<'a> {
    Star,
    Id(&'a str),
}

fn state_ref(tok: &str) -> PResult<Ref<'_>> {
    let tok = strip_class(tok.trim())?;
    if tok == "[*]" {
        return Ok(Ref::Star);
    }
    whole_ident(tok).map(Ref::Id)
}

impl Parser {
    fn current(&self) -> usize {
        self.open
            .last()
            .and_then(|&c| self.comps[c].regions.last().copied())
            .unwrap_or(0)
    }

    fn node_count(&self) -> usize {
        self.states.len() + self.notes.len()
    }

    fn push_state(&mut self, st: St) -> PResult<usize> {
        if self.node_count() >= MAX_NODES {
            return Err(Some(Error::TooLarge));
        }
        self.states.push(st);
        Ok(self.states.len() - 1)
    }

    /// The named state `id`, created on first use in the current region.
    /// A state first used at the top level moves into the first
    /// composite that mentions it (as flowchart subgraphs do).
    fn mention(&mut self, id: &str) -> PResult<usize> {
        let cur = self.current();
        if let Some(i) = self.states.iter().position(|s| s.named && s.id == id) {
            let s = &mut self.states[i];
            if s.region == 0 && cur != 0 && s.comp.is_none() {
                s.region = cur;
            }
            return Ok(i);
        }
        self.push_state(St {
            id: id.to_owned(),
            label: None,
            descs: Vec::new(),
            kind: Kind::State,
            region: cur,
            comp: None,
            named: true,
        })
    }

    /// Start (`end == false`) or end state of the current region.
    fn star(&mut self, end: bool) -> PResult<usize> {
        let r = self.current();
        let have = if end {
            self.regs[r].end
        } else {
            self.regs[r].start
        };
        if let Some(i) = have {
            return Ok(i);
        }
        let i = self.push_state(St {
            id: "[*]".to_owned(),
            label: None,
            descs: Vec::new(),
            kind: if end { Kind::End } else { Kind::Start },
            region: r,
            comp: None,
            named: false,
        })?;
        if end {
            self.regs[r].end = Some(i);
        } else {
            self.regs[r].start = Some(i);
        }
        Ok(i)
    }

    fn statement(&mut self, s: &str, line: usize) -> PResult<()> {
        if let Some((target, left, lines, _)) = &mut self.note {
            if s == "end note" {
                let text = text(&lines.join("\n"));
                let (target, left) = (*target, *left);
                self.note = None;
                return self.add_note(target, left, text);
            }
            if lines.len() < 64 {
                lines.push(s.to_owned());
            }
            return Ok(());
        }
        if s.is_empty() {
            return Ok(());
        }
        if s == "}" {
            return self.open.pop().map(|_| ()).ok_or(None);
        }
        if s == "--" {
            let &c = self.open.last().ok_or(None)?;
            if self.regs.len() > 2 * MAX_SUBGRAPHS {
                return Err(Some(Error::TooLarge));
            }
            self.regs.push(Reg::default());
            let r = self.regs.len() - 1;
            self.comps[c].regions.push(r);
            return Ok(());
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
                if self.open.is_empty() {
                    self.direction = Some(d);
                }
                return Ok(());
            }
            "state" => return self.state_stmt(s["state".len()..].trim(), line),
            "note" => return self.note_stmt(s["note".len()..].trim(), line),
            _ => {}
        }
        if let Some(i) = find_unquoted(s, "-->") {
            let left = state_ref(&s[..i])?;
            let rest = &s[i + 3..];
            let (right, label) = match find_colon(rest) {
                Some(j) => (&rest[..j], Some(text(&rest[j + 1..]))),
                None => (rest, None),
            };
            let right = state_ref(right)?;
            let from = match left {
                Ref::Star => self.star(false)?,
                Ref::Id(id) => self.mention(id)?,
            };
            let to = match right {
                Ref::Star => self.star(true)?,
                Ref::Id(id) => self.mention(id)?,
            };
            if self.trans.len() + self.notes.len() >= MAX_EDGES {
                return Err(Some(Error::TooLarge));
            }
            self.trans.push(Trans {
                from,
                to,
                label: label.filter(|l| !l.is_empty()),
            });
            return Ok(());
        }
        // `id : description` or a bare `id`.
        let (id, desc) = match find_colon(s) {
            Some(j) => (&s[..j], Some(&s[j + 1..])),
            None => (s, None),
        };
        let Ref::Id(id) = state_ref(id)? else {
            return Err(None);
        };
        let i = self.mention(id)?;
        if let Some(d) = desc.map(text).filter(|d| !d.is_empty())
            && self.states[i].descs.len() < 16
        {
            self.states[i].descs.push(d);
        }
        Ok(())
    }

    /// `state id`, `state id <<choice>>`, `state "label" as id`,
    /// `state id : description`, each optionally opening `{`.
    fn state_stmt(&mut self, rest: &str, line: usize) -> PResult<()> {
        let (rest, opens) = match rest.strip_suffix('{') {
            Some(r) => (r.trim_end(), true),
            None => (rest, false),
        };
        let (label, rest) = if let Some(q) = rest.strip_prefix('"') {
            let end = q.find('"').ok_or(None)?;
            let after = q[end + 1..].trim_start();
            let after = after.strip_prefix("as").ok_or(None)?;
            if !after.starts_with(char::is_whitespace) {
                return Err(None);
            }
            (Some(text(&q[..end])), after.trim())
        } else {
            (None, rest)
        };
        let (head, desc) = match find_colon(rest) {
            Some(j) if label.is_none() => (rest[..j].trim(), Some(text(&rest[j + 1..]))),
            _ => (rest, None),
        };
        let (id_tok, kind) = match head.find("<<") {
            Some(k) => {
                let tag = head[k..].trim();
                let kind = match tag {
                    "<<choice>>" => Kind::Choice,
                    "<<fork>>" => Kind::Fork,
                    "<<join>>" => Kind::Join,
                    _ => return Err(None),
                };
                (head[..k].trim(), Some(kind))
            }
            None => (head, None),
        };
        let Ref::Id(id) = state_ref(id_tok)? else {
            return Err(None);
        };
        let i = self.mention(id)?;
        if let Some(l) = label {
            self.states[i].label = Some(l);
        }
        if let Some(d) = desc.filter(|d| !d.is_empty())
            && self.states[i].descs.len() < 16
        {
            self.states[i].descs.push(d);
        }
        if let Some(k) = kind {
            if self.states[i].comp.is_some() {
                return Err(None);
            }
            self.states[i].kind = k;
        }
        if opens {
            if self.open.len() >= MAX_SUBGRAPH_DEPTH
                || self.states[i].comp.is_some()
                || self.states[i].kind != Kind::State
            {
                return Err(None);
            }
            if self.comps.len() >= MAX_SUBGRAPHS || self.regs.len() > 2 * MAX_SUBGRAPHS {
                return Err(Some(Error::TooLarge));
            }
            self.regs.push(Reg::default());
            self.comps.push(Comp {
                state: i,
                regions: vec![self.regs.len() - 1],
                line,
            });
            let c = self.comps.len() - 1;
            self.states[i].comp = Some(c);
            self.open.push(c);
        }
        Ok(())
    }

    /// `left of X : text`, `right of X : text`, or the first line of a
    /// multi-line note.
    fn note_stmt(&mut self, rest: &str, line: usize) -> PResult<()> {
        let (left, rest) = if let Some(r) = rest.strip_prefix("left of") {
            (true, r)
        } else if let Some(r) = rest.strip_prefix("right of") {
            (false, r)
        } else {
            return Err(None);
        };
        if !rest.starts_with(char::is_whitespace) {
            return Err(None);
        }
        let (target, body) = match find_colon(rest) {
            Some(j) => (&rest[..j], Some(&rest[j + 1..])),
            None => (rest, None),
        };
        let Ref::Id(id) = state_ref(target)? else {
            return Err(None);
        };
        let target = self.mention(id)?;
        match body {
            Some(b) => self.add_note(target, left, text(b)),
            None => {
                self.note = Some((target, left, Vec::new(), line));
                Ok(())
            }
        }
    }

    fn add_note(&mut self, target: usize, left: bool, text: String) -> PResult<()> {
        if self.node_count() >= MAX_NODES || self.trans.len() + self.notes.len() >= MAX_EDGES {
            return Err(Some(Error::TooLarge));
        }
        self.notes.push(NoteDef { target, left, text });
        Ok(())
    }

    fn finish(self) -> Result<StateDiagram, Error> {
        if let Some((.., line)) = self.note {
            return Err(Error::Unsupported { line });
        }
        if let Some(&c) = self.open.last() {
            return Err(Error::Unsupported {
                line: self.comps[c].line,
            });
        }
        let nr = self.regs.len();
        // Regions that hold something (a state, a composite or a note).
        let mut used = vec![false; nr];
        for s in &self.states {
            used[s.region] = true;
        }
        for n in &self.notes {
            used[self.states[n.target].region] = true;
        }
        let frame: Vec<bool> = self
            .comps
            .iter()
            .map(|c| c.regions.iter().any(|&r| used[r]))
            .collect();
        // Region -> composite owning it.
        let mut owner = vec![None; nr];
        for (c, comp) in self.comps.iter().enumerate() {
            for &r in &comp.regions {
                owner[r] = Some(c);
            }
        }
        // Nesting depth of frames (and no cycles).
        for (c, comp) in self.comps.iter().enumerate() {
            if !frame[c] {
                continue;
            }
            let mut depth = 1;
            let mut r = self.states[comp.state].region;
            let mut steps = 0;
            while let Some(o) = owner[r] {
                depth += usize::from(frame[o]);
                r = self.states[self.comps[o].state].region;
                steps += 1;
                if steps > self.comps.len() || depth > MAX_SUBGRAPH_DEPTH {
                    return Err(Error::Unsupported { line: comp.line });
                }
            }
        }
        // Flatten composites and regions into subgraphs, parents first.
        let mut subgraphs: Vec<Subgraph> = Vec::new();
        let mut regions: Vec<bool> = Vec::new();
        let mut rsub: Vec<Option<usize>> = vec![None; nr];
        let mut csub: Vec<Option<usize>> = vec![None; self.comps.len()];
        let mut stack: Vec<(usize, Option<usize>)> = vec![(0, None)];
        let mut seen = vec![false; nr];
        while let Some((r, sub)) = stack.pop() {
            if seen[r] {
                continue;
            }
            seen[r] = true;
            rsub[r] = sub;
            let mut next: Vec<(usize, Option<usize>)> = Vec::new();
            for (c, comp) in self.comps.iter().enumerate() {
                if !frame[c] || self.states[comp.state].region != r {
                    continue;
                }
                let st = &self.states[comp.state];
                subgraphs.push(Subgraph {
                    id: st.id.clone(),
                    title: display(st),
                    parent: sub,
                });
                regions.push(false);
                let k = subgraphs.len() - 1;
                csub[c] = Some(k);
                let live: Vec<usize> = comp.regions.iter().copied().filter(|&r| used[r]).collect();
                if live.len() > 1 {
                    for (j, &r2) in live.iter().enumerate() {
                        subgraphs.push(Subgraph {
                            id: format!("{}#{j}", st.id),
                            title: String::new(),
                            parent: Some(k),
                        });
                        regions.push(true);
                        next.push((r2, Some(subgraphs.len() - 1)));
                    }
                } else {
                    next.extend(comp.regions.iter().map(|&r2| (r2, Some(k))));
                }
                if subgraphs.len() > MAX_SUBGRAPHS {
                    return Err(Error::TooLarge);
                }
            }
            // Depth first, in order: push in reverse.
            stack.extend(next.into_iter().rev());
        }
        if let Some(c) = (0..self.comps.len()).find(|&c| frame[c] && csub[c].is_none()) {
            return Err(Error::Unsupported {
                line: self.comps[c].line,
            });
        }
        // Nodes: states that are not frames, then notes.
        let mut map = vec![usize::MAX; self.states.len()];
        let mut nodes = Vec::new();
        let mut kinds = Vec::new();
        for (i, s) in self.states.iter().enumerate() {
            if s.comp.is_some_and(|c| frame[c]) {
                continue;
            }
            map[i] = nodes.len();
            nodes.push(Node {
                id: s.id.clone(),
                label: match s.kind {
                    Kind::State => display(s),
                    _ => String::new(),
                },
                shape: shape(s.kind),
                subgraph: rsub[s.region],
            });
            kinds.push(s.kind);
        }
        let note_start = nodes.len();
        for n in &self.notes {
            nodes.push(Node {
                id: String::new(),
                label: n.text.clone(),
                shape: Shape::Rect,
                subgraph: rsub[self.states[n.target].region],
            });
            kinds.push(Kind::Note);
        }
        // First node inside each subgraph (through nested ones).
        let mut rep = vec![usize::MAX; subgraphs.len()];
        for (i, n) in nodes.iter().enumerate() {
            let mut s = n.subgraph;
            let mut steps = 0;
            while let Some(k) = s {
                if rep[k] == usize::MAX {
                    rep[k] = i;
                }
                s = subgraphs[k].parent;
                steps += 1;
                if steps > subgraphs.len() {
                    break;
                }
            }
        }
        if rep.contains(&usize::MAX) {
            return Err(Error::Unsupported { line: 1 });
        }
        let end = |i: usize| match self.states[i].comp.and_then(|c| csub[c]) {
            Some(k) => (rep[k], Some(k)),
            None => (map[i], None),
        };
        let mut edges: Vec<Edge> = self
            .trans
            .iter()
            .map(|t| {
                let ((from, from_group), (to, to_group)) = (end(t.from), end(t.to));
                Edge {
                    from,
                    to,
                    label: t.label.clone(),
                    stroke: Stroke::Solid,
                    head: Head::Arrow,
                    tail: Head::None,
                    from_group,
                    to_group,
                }
            })
            .collect();
        for (j, n) in self.notes.iter().enumerate() {
            let note = note_start + j;
            let (t, group) = end(n.target);
            let ((from, from_group), (to, to_group)) = if n.left {
                ((note, None), (t, group))
            } else {
                ((t, group), (note, None))
            };
            edges.push(Edge {
                from,
                to,
                label: None,
                stroke: Stroke::Dotted,
                head: Head::None,
                tail: Head::None,
                from_group,
                to_group,
            });
        }
        Ok(StateDiagram {
            graph: Flowchart {
                direction: self.direction.unwrap_or(Direction::Down),
                nodes,
                edges,
                subgraphs,
            },
            kinds,
            regions,
        })
    }
}

/// Shown text of a state: its label or id, or its description when it has
/// exactly one and no label; more descriptions go on lines below.
fn display(s: &St) -> String {
    match (&s.label, s.descs.as_slice()) {
        (None, [d]) => d.clone(),
        (l, ds) => {
            let mut t = l.clone().unwrap_or_else(|| truncate(&s.id));
            for d in ds {
                t.push('\n');
                t.push_str(d);
            }
            truncate(&t)
        }
    }
}

/// Outline used to clip transitions.
fn shape(kind: Kind) -> Shape {
    match kind {
        Kind::State => Shape::Round,
        Kind::Start => Shape::Circle,
        Kind::End => Shape::DoubleCircle,
        Kind::Choice => Shape::Diamond,
        Kind::Fork | Kind::Join | Kind::Note => Shape::Rect,
    }
}

/// Lay out a state diagram. `measure` returns the size of a label.
pub fn layout(
    sd: &StateDiagram,
    sp: Spacing,
    measure: &mut dyn FnMut(&str) -> (f32, f32),
) -> Result<Layout, Error> {
    let horizontal = matches!(sd.graph.direction, Direction::Right | Direction::Left);
    let pad = sp.pad;
    let sizes: Vec<(f32, f32)> = sd
        .graph
        .nodes
        .iter()
        .zip(&sd.kinds)
        .map(|(n, k)| match k {
            Kind::State | Kind::Note => {
                let (w, h) = measure(&n.label);
                let h = h + pad;
                ((w + 2.0 * pad).max(h * 1.6), h)
            }
            Kind::Start => (pad * 1.4, pad * 1.4),
            Kind::End => (pad * 1.8, pad * 1.8),
            Kind::Choice => (pad * 2.6, pad * 2.6),
            Kind::Fork | Kind::Join => {
                let (long, thin) = (pad * 5.0, pad * 0.6);
                if horizontal {
                    (thin, long)
                } else {
                    (long, thin)
                }
            }
        })
        .collect();
    // Regions have no title: no room is kept for one.
    let mut m = |s: &str| {
        if s.is_empty() { (0.0, 0.0) } else { measure(s) }
    };
    layout_sized(&sd.graph, sp, &sizes, &mut m)
}

#[cfg(test)]
mod tests {
    use super::super::tests::{assert_frames_ok, assert_no_overlap, measure};
    use super::*;

    fn lay(src: &str) -> (StateDiagram, Layout) {
        let sd = parse(src).unwrap_or_else(|e| panic!("{e:?} in {src}"));
        let l = layout(&sd, Spacing::for_font(14.0), &mut measure).expect("lays out");
        (sd, l)
    }

    fn node(sd: &StateDiagram, id: &str) -> usize {
        sd.graph
            .nodes
            .iter()
            .position(|n| n.id == id)
            .unwrap_or_else(|| panic!("no node {id}"))
    }

    fn rows(sd: &StateDiagram) -> Vec<(String, String, Option<&str>)> {
        let name = |i: usize| match sd.kinds[i] {
            Kind::Start => "start".to_owned(),
            Kind::End => "end".to_owned(),
            Kind::Note => format!("note:{}", sd.graph.nodes[i].label),
            _ => sd.graph.nodes[i].id.clone(),
        };
        sd.graph
            .edges
            .iter()
            .map(|e| (name(e.from), name(e.to), e.label.as_deref()))
            .collect()
    }

    #[test]
    fn headers() {
        for h in [
            "stateDiagram",
            "stateDiagram-v2",
            "---\ntitle: t\n---\nstateDiagram-v2",
        ] {
            assert!(parse(&format!("{h}\n[*] --> A")).is_ok(), "{h}");
        }
        assert_eq!(
            parse("stateDiagram x\nA"),
            Err(Error::Unsupported { line: 1 })
        );
        assert_eq!(parse("graph TD\nA"), Err(Error::UnsupportedType));
        assert!(matches!(
            super::super::parse_diagram("stateDiagram-v2\nA --> B"),
            Ok(super::super::Diagram::State(_))
        ));
    }

    #[test]
    fn states_descriptions_and_transitions() {
        let sd = parse(
            "stateDiagram-v2\n[*] --> Still\nStill --> Moving : push\nMoving --> Still: stop\nMoving --> Crash\nCrash --> [*]\nstate \"A long name\" as Long\nCrash : It broke\nStill:::red\nLong --> Still",
        )
        .expect("parses");
        let g = &sd.graph;
        assert_eq!(g.direction, Direction::Down);
        assert_eq!(g.nodes[node(&sd, "Crash")].label, "It broke");
        assert_eq!(g.nodes[node(&sd, "Long")].label, "A long name");
        assert_eq!(g.nodes[node(&sd, "Still")].label, "Still");
        assert_eq!(
            rows(&sd),
            [
                ("start".into(), "Still".into(), None),
                ("Still".into(), "Moving".into(), Some("push")),
                ("Moving".into(), "Still".into(), Some("stop")),
                ("Moving".into(), "Crash".into(), None),
                ("Crash".into(), "end".into(), None),
                ("Long".into(), "Still".into(), None),
            ]
        );
        assert!(
            g.edges
                .iter()
                .all(|e| e.head == Head::Arrow && e.stroke == Stroke::Solid)
        );
        // Two descriptions: the name with both below it.
        let sd = parse("stateDiagram\nS : one\nS : two").expect("parses");
        assert_eq!(sd.graph.nodes[0].label, "S\none\ntwo");
        // Labelled state with a description.
        let sd = parse("stateDiagram\nstate \"Name\" as S\nS : more").expect("parses");
        assert_eq!(sd.graph.nodes[0].label, "Name\nmore");
    }

    #[test]
    fn start_and_end_are_per_scope() {
        let sd = parse(
            "stateDiagram-v2\n[*] --> A\nA --> [*]\nstate A {\n  [*] --> a1\n  a1 --> [*]\n}\n[*] --> B",
        )
        .expect("parses");
        let starts = sd.kinds.iter().filter(|k| **k == Kind::Start).count();
        let ends = sd.kinds.iter().filter(|k| **k == Kind::End).count();
        assert_eq!((starts, ends), (2, 2));
        // The outer start is reused by the second `[*] -->`.
        assert_eq!(sd.graph.edges[0].from, sd.graph.edges[4].from);
        // A is a frame: links to it end at the frame.
        assert_eq!(sd.graph.subgraphs.len(), 1);
        assert_eq!(sd.graph.edges[0].to_group, Some(0));
        assert_eq!(sd.graph.edges[1].from_group, Some(0));
        let (sd, l) = lay(
            "stateDiagram-v2\n[*] --> A\nA --> [*]\nstate A {\n  [*] --> a1\n  a1 --> [*]\n}\n[*] --> B",
        );
        assert_no_overlap(&l);
        assert_frames_ok(&sd.graph, &l);
    }

    #[test]
    fn composite_states_nest_three_deep() {
        let src = "stateDiagram-v2\n[*] --> First\nstate First {\n  [*] --> Second\n  state Second {\n    [*] --> s2\n    state \"Third level\" as Third {\n      t1 --> t2\n    }\n    s2 --> Third\n  }\n}\nFirst --> Out";
        for dir in ["", "direction LR\n", "direction BT\n", "direction RL\n"] {
            let src = src.replacen("\n", &format!("\n{dir}"), 1);
            let (sd, l) = lay(&src);
            assert_eq!(sd.graph.subgraphs.len(), 3);
            assert_eq!(sd.graph.subgraphs[2].title, "Third level");
            assert_eq!(sd.graph.subgraphs[2].parent, Some(1));
            assert_no_overlap(&l);
            assert_frames_ok(&sd.graph, &l);
            let (_, again) = lay(&src);
            assert_eq!(l, again, "deterministic");
        }
        let deep = "stateDiagram\nstate a {\nstate b {\nstate c {\nstate d {\nx\n}\n}\n}\n}";
        assert_eq!(parse(deep), Err(Error::Unsupported { line: 5 }));
        // An empty composite is a plain state.
        let sd = parse("stateDiagram\nstate E {\n}\nE --> F").expect("parses");
        assert!(sd.graph.subgraphs.is_empty());
        assert_eq!(sd.graph.nodes[node(&sd, "E")].label, "E");
        // Unclosed or stray braces.
        assert_eq!(
            parse("stateDiagram\nstate A {\nb"),
            Err(Error::Unsupported { line: 2 })
        );
        assert_eq!(
            parse("stateDiagram\n}"),
            Err(Error::Unsupported { line: 2 })
        );
        // A state used at the top level moves into the composite using it.
        let sd = parse("stateDiagram\nx --> y\nstate C {\n  x --> z\n}").expect("parses");
        assert_eq!(sd.graph.nodes[node(&sd, "x")].subgraph, Some(0));
        assert_eq!(sd.graph.nodes[node(&sd, "y")].subgraph, None);
    }

    #[test]
    fn choice_fork_join() {
        let (sd, l) = lay(
            "stateDiagram-v2\nstate pick <<choice>>\nstate f <<fork>>\nstate j <<join>>\n[*] --> f\nf --> a\nf --> b\na --> j\nb --> j\nj --> pick\npick --> big : n > 9\npick --> small : n <= 9",
        );
        assert_eq!(sd.kinds[node(&sd, "pick")], Kind::Choice);
        assert_eq!(sd.kinds[node(&sd, "f")], Kind::Fork);
        assert_eq!(sd.kinds[node(&sd, "j")], Kind::Join);
        assert_eq!(sd.graph.nodes[node(&sd, "pick")].label, "");
        // `<` in a label is plain text.
        assert_eq!(sd.graph.edges[7].label.as_deref(), Some("n <= 9"));
        let bar = l.nodes[node(&sd, "f")];
        assert!(bar.w > 4.0 * bar.h, "{bar:?}");
        assert_no_overlap(&l);
        let (sd, l) = lay("stateDiagram\ndirection LR\nstate f <<fork>>\nf --> a");
        let bar = l.nodes[node(&sd, "f")];
        assert!(bar.h > 4.0 * bar.w, "{bar:?}");
        assert_eq!(
            parse("stateDiagram\nstate x <<history>>"),
            Err(Error::Unsupported { line: 2 })
        );
        assert_eq!(
            parse("stateDiagram\nstate x <<choice>> {\n}"),
            Err(Error::Unsupported { line: 2 })
        );
    }

    #[test]
    fn notes() {
        let src = "stateDiagram-v2\nA --> B\nnote left of A : Left note\nnote right of B\n  line one\n  line two\nend note\nstate C {\n  c1\n}\nnote right of C : on a frame";
        let (sd, l) = lay(src);
        let notes: Vec<&str> = sd
            .graph
            .nodes
            .iter()
            .zip(&sd.kinds)
            .filter(|(_, k)| **k == Kind::Note)
            .map(|(n, _)| n.label.as_str())
            .collect();
        assert_eq!(notes, ["Left note", "line one\nline two", "on a frame"]);
        let r = rows(&sd);
        assert_eq!(r[1].0, "note:Left note");
        assert_eq!(r[1].1, "A");
        assert_eq!(r[2], ("B".into(), "note:line one\nline two".into(), None));
        let e = &sd.graph.edges[3];
        assert_eq!(
            (e.stroke, e.head, e.from_group),
            (Stroke::Dotted, Head::None, Some(0))
        );
        // A note on a frame is outside it.
        assert_eq!(sd.graph.nodes[e.to].subgraph, None);
        assert_no_overlap(&l);
        assert_frames_ok(&sd.graph, &l);
        // Left of A comes before A in the flow; right of B after B.
        let y = |i: usize| l.nodes[i].y;
        assert!(y(sd.graph.edges[1].from) < y(node(&sd, "A")));
        assert!(y(sd.graph.edges[2].to) > y(node(&sd, "B")));
        assert_eq!(
            parse("stateDiagram\nnote right of A\ntext"),
            Err(Error::Unsupported { line: 2 })
        );
        assert_eq!(
            parse("stateDiagram\nnote over A : x"),
            Err(Error::Unsupported { line: 2 })
        );
        assert_eq!(
            parse("stateDiagram\nnote \"free\" as N"),
            Err(Error::Unsupported { line: 2 })
        );
    }

    #[test]
    fn concurrent_regions() {
        let src = "stateDiagram-v2\n[*] --> Active\nstate Active {\n  [*] --> NumOff\n  NumOff --> NumOn\n  --\n  [*] --> CapsOff\n  CapsOff --> CapsOn\n  --\n}\nActive --> [*]";
        let (sd, l) = lay(src);
        // Active plus two regions (the trailing empty one is dropped).
        assert_eq!(sd.regions, [false, true, true]);
        assert_eq!(sd.graph.subgraphs[1].parent, Some(0));
        assert_eq!(sd.graph.subgraphs[1].title, "");
        assert_eq!(sd.graph.nodes[node(&sd, "NumOff")].subgraph, Some(1));
        assert_eq!(sd.graph.nodes[node(&sd, "CapsOn")].subgraph, Some(2));
        // Each region has its own start.
        let starts: Vec<Option<usize>> = sd
            .graph
            .nodes
            .iter()
            .zip(&sd.kinds)
            .filter(|(_, k)| **k == Kind::Start)
            .map(|(n, _)| n.subgraph)
            .collect();
        assert_eq!(starts, [None, Some(1), Some(2)]);
        assert_no_overlap(&l);
        assert_frames_ok(&sd.graph, &l);
        assert_eq!(
            parse("stateDiagram\nA\n--"),
            Err(Error::Unsupported { line: 3 })
        );
    }

    #[test]
    fn ignored_and_unsupported_lines() {
        let sd = parse(
            "stateDiagram-v2\n%% comment\nclassDef bad fill:#f00\nclass A bad\nstyle A fill:#f00\nA --> B %% trailing\ndirection LR",
        )
        .expect("parses");
        assert_eq!(sd.graph.nodes.len(), 2);
        assert_eq!(sd.graph.direction, Direction::Right);
        // `direction` inside a composite is accepted and ignored.
        let sd = parse("stateDiagram\nstate X {\ndirection LR\na\n}").expect("parses");
        assert_eq!(sd.graph.direction, Direction::Down);
        for (src, line) in [
            ("stateDiagram\nA -> B", 2),
            ("stateDiagram\nA --> B --> C", 2),
            ("stateDiagram\nhide empty description", 2),
            ("stateDiagram\nstate", 2),
            ("stateDiagram\nstate \"x\" A", 2),
            ("stateDiagram\nA B", 2),
            ("stateDiagram\n[*]", 2),
            ("stateDiagram\ndirection XY", 2),
            ("stateDiagram\nA\n  --", 3),
            ("stateDiagram\nnote left of [*] : x", 2),
        ] {
            assert_eq!(parse(src), Err(Error::Unsupported { line }), "{src}");
        }
    }

    #[test]
    fn size_caps() {
        let many: String = (0..=MAX_NODES).map(|i| format!("s{i}\n")).collect();
        assert_eq!(
            parse(&format!("stateDiagram\n{many}")),
            Err(Error::TooLarge)
        );
        let edges: String = (0..=MAX_EDGES).map(|_| "a --> b\n").collect();
        assert_eq!(
            parse(&format!("stateDiagram\n{edges}")),
            Err(Error::TooLarge)
        );
        assert_eq!(
            parse(&format!("stateDiagram\n{}", "x".repeat(MAX_SOURCE_BYTES))),
            Err(Error::TooLarge)
        );
        let long = "y".repeat(500);
        let sd = parse(&format!("stateDiagram\nA : {long}")).expect("parses");
        assert_eq!(
            sd.graph.nodes[0].label.chars().count(),
            super::super::MAX_LABEL_CHARS
        );
    }

    #[test]
    fn random_diagrams_lay_out_with_frames_apart() {
        let mut s: u64 = 0x5DEE_CE66_D1CE_4E5B;
        let mut next = |m: u64| {
            s ^= s << 13;
            s ^= s >> 7;
            s ^= s << 17;
            s % m
        };
        let mut laid = 0;
        for round in 0..400 {
            let dir = ["TB", "BT", "LR", "RL"][round % 4];
            let mut src = format!("stateDiagram-v2\ndirection {dir}\n");
            let states = 2 + next(16);
            let mut depth = 0;
            let mut comps = 0;
            for _ in 0..(4 + next(36)) {
                match next(12) {
                    0 | 1 if depth < MAX_SUBGRAPH_DEPTH => {
                        src.push_str(&format!("state C{comps} {{\n"));
                        comps += 1;
                        depth += 1;
                    }
                    2 if depth > 0 => {
                        src.push_str("}\n");
                        depth -= 1;
                    }
                    3 if depth > 0 => src.push_str("--\n"),
                    4 => src.push_str(&format!("[*] --> s{}\n", next(states))),
                    5 => src.push_str(&format!("s{} --> [*]\n", next(states))),
                    6 => src.push_str(&format!("note right of s{} : n\n", next(states))),
                    7 => src.push_str(&format!("state k{} <<choice>>\n", next(4))),
                    8 if comps > 0 => {
                        src.push_str(&format!("s{} --> C{}\n", next(states), next(comps)));
                    }
                    _ => {
                        let (a, b) = (next(states), next(states));
                        src.push_str(&format!("s{a} --> s{b} : t\n"));
                    }
                }
            }
            for _ in 0..depth {
                src.push_str("fill\n}\n");
            }
            let sd = match parse(&src) {
                Ok(sd) => sd,
                // A composite reopened, a choice opened as a composite, or
                // frames nested too deep after states moved.
                Err(Error::Unsupported { .. }) => continue,
                Err(e) => panic!("{e:?} in {src}"),
            };
            let a = layout(&sd, Spacing::for_font(13.0), &mut measure).expect("lays out");
            assert_no_overlap(&a);
            assert_frames_ok(&sd.graph, &a);
            assert_eq!(a.edges.len(), sd.graph.edges.len());
            let b = layout(&sd, Spacing::for_font(13.0), &mut measure).expect("lays out");
            assert_eq!(a, b, "deterministic: {src}");
            laid += 1;
        }
        assert!(laid > 300, "only {laid} laid out");
    }

    #[test]
    fn random_input_never_panics() {
        let tokens = [
            "s1",
            "s2",
            "X",
            "[*]",
            "-->",
            "--",
            ":",
            " : ",
            "label",
            "state ",
            "\"",
            " as ",
            "{",
            "}",
            "<<choice>>",
            "<<fork>>",
            "<<join>>",
            "<<",
            ">>",
            "note left of ",
            "note right of ",
            "end note",
            "\n",
            " ",
            "%%",
            ":::",
            "direction LR",
            "<br>",
            "ä",
            "🙂",
            "classDef",
            "-",
            "\nstate C {\n",
            "\n}\n",
            "\n--\n",
        ];
        let mut s: u64 = 0x2545_F491_4F6C_DD1D;
        let mut parsed = 0;
        for _ in 0..5000 {
            let mut src = String::from("stateDiagram-v2\n");
            let len = (s % 60) as usize;
            for _ in 0..len {
                s ^= s << 13;
                s ^= s >> 7;
                s ^= s << 17;
                src.push_str(tokens[(s % tokens.len() as u64) as usize]);
            }
            if let Ok(sd) = parse(&src) {
                parsed += 1;
                assert_eq!(sd.kinds.len(), sd.graph.nodes.len());
                assert_eq!(sd.regions.len(), sd.graph.subgraphs.len());
                if let Ok(l) = layout(&sd, Spacing::for_font(12.0), &mut measure) {
                    assert!(l.width.is_finite() && l.height.is_finite());
                    assert_frames_ok(&sd.graph, &l);
                }
            }
            s = s.wrapping_add(0x9E37_79B9_7F4A_7C15);
        }
        assert!(parsed > 100, "only {parsed} parsed");
    }
}
