//! A safe, native subset of Mermaid flowcharts, sequence, state and class
//! diagrams.
//!
//! [`parse_diagram`] turns the text of a ```` ```mermaid ```` block into a
//! small model: a [`Flowchart`] (laid out by [`layout`] with a simple
//! layered, Sugiyama-style algorithm), a [`sequence::Sequence`] (laid
//! out by [`sequence::layout`]), a [`state::StateDiagram`] or a
//! [`class::ClassDiagram`] (both laid out by the flowchart layout with
//! node sizes of their own, [`layout_sized`]). Nothing here touches the network, opens
//! links or runs document content: `click`, `style`, `classDef`, `class`
//! and `linkStyle` lines are ignored, and anything outside the subset is
//! reported as an [`Error`] so the UI can show the source instead.
//!
//! Flowchart subgraphs (`subgraph id [title]` … `end`, nested up to
//! [`MAX_SUBGRAPH_DEPTH`]) are laid out as blocks: in every layer the
//! members of a subgraph are contiguous, sibling subgraphs keep the same
//! order in every layer, and the frame around a subgraph never overlaps a
//! node that is not a member or another frame that it does not contain.
//! A node belongs to the innermost subgraph in which it is first
//! mentioned (a node first used outside any subgraph moves into the first
//! subgraph that mentions it). A link to or from a subgraph id is laid out
//! through the subgraph's first member node and drawn to the frame's edge.
//! `direction` inside a subgraph is accepted and ignored: the whole chart
//! uses the header direction.
//!
//! All work is bounded by the `MAX_*` caps and is deterministic: the same
//! text and the same label sizes always give the same picture.

pub mod class;
pub mod sequence;
pub mod state;

/// Diagram text larger than this is not drawn.
pub const MAX_SOURCE_BYTES: usize = 16 * 1024;
pub const MAX_NODES: usize = 150;
pub const MAX_EDGES: usize = 300;
pub const MAX_SUBGRAPHS: usize = 50;
/// Subgraphs nested deeper than this are not drawn.
pub const MAX_SUBGRAPH_DEPTH: usize = 3;
/// Labels longer than this (in characters) are cut with an ellipsis.
pub const MAX_LABEL_CHARS: usize = 120;
/// Virtual nodes inserted for edges that span several layers.
pub const MAX_DUMMIES: usize = 3000;
/// Barycenter ordering sweeps (down and up count as one each).
const ORDER_PASSES: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// Top to bottom (`TB` or `TD`).
    Down,
    /// Bottom to top (`BT`).
    Up,
    /// Left to right (`LR`).
    Right,
    /// Right to left (`RL`).
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    Rect,
    Round,
    Stadium,
    Diamond,
    Circle,
    /// `(((text)))`
    DoubleCircle,
    /// `{{text}}`
    Hexagon,
    /// `[(text)]`
    Cylinder,
    /// `[/text/]`, leaning right.
    Parallelogram,
    /// `[\text\]`, leaning left.
    ParallelogramAlt,
    /// `[/text\]`, narrow at the top.
    Trapezoid,
    /// `[\text/]`, narrow at the bottom.
    TrapezoidAlt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stroke {
    Solid,
    Dotted,
    Thick,
    /// `~~~`: affects the layout but is not drawn.
    Invisible,
}

/// The end of a link.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Head {
    None,
    /// `>` (or `<` at the start).
    Arrow,
    /// `o`
    Circle,
    /// `x`
    Cross,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub id: String,
    pub label: String,
    pub shape: Shape,
    /// Innermost subgraph holding the node.
    pub subgraph: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edge {
    /// Source node. For a link from a subgraph this is the subgraph's
    /// first member node and `from_group` names the subgraph.
    pub from: usize,
    /// Target node (see `from`).
    pub to: usize,
    pub label: Option<String>,
    pub stroke: Stroke,
    /// Head drawn at `to`.
    pub head: Head,
    /// Head drawn at `from` (`<-->`, `o--o`, `x--x`).
    pub tail: Head,
    pub from_group: Option<usize>,
    pub to_group: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subgraph {
    pub id: String,
    pub title: String,
    pub parent: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Flowchart {
    pub direction: Direction,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    /// Parents come before their children.
    pub subgraphs: Vec<Subgraph>,
}

/// Any supported diagram.
#[derive(Debug, Clone, PartialEq)]
pub enum Diagram {
    Flowchart(Flowchart),
    Sequence(sequence::Sequence),
    State(state::StateDiagram),
    Class(class::ClassDiagram),
}

/// Why a block is shown as source instead of a diagram.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// Another diagram type (class, gantt, pie, ...).
    UnsupportedType,
    /// A feature outside the subset, or text that does not parse. `line`
    /// is 1-based within the block.
    Unsupported { line: usize },
    /// Over one of the size caps.
    TooLarge,
}

/// Parse a Mermaid block of any supported type.
pub fn parse_diagram(src: &str) -> Result<Diagram, Error> {
    if src.len() > MAX_SOURCE_BYTES {
        return Err(Error::TooLarge);
    }
    let Some((_, head)) = header_line(src) else {
        return Err(Error::UnsupportedType);
    };
    match head.split_whitespace().next() {
        Some("sequenceDiagram") => return sequence::parse(src).map(Diagram::Sequence),
        Some("stateDiagram" | "stateDiagram-v2") => return state::parse(src).map(Diagram::State),
        Some("classDiagram" | "classDiagram-v2") => return class::parse(src).map(Diagram::Class),
        _ => {}
    }
    parse(src).map(Diagram::Flowchart)
}

/// Parse a flowchart (`flowchart` / `graph`); other types are
/// [`Error::UnsupportedType`].
pub fn parse(src: &str) -> Result<Flowchart, Error> {
    if src.len() > MAX_SOURCE_BYTES {
        return Err(Error::TooLarge);
    }
    let Some((start, head)) = header_line(src) else {
        return Err(Error::UnsupportedType);
    };
    let direction = header(head, start + 1)?;
    let mut p = Parser::default();
    for (i, raw) in src.lines().enumerate().skip(start + 1) {
        let line = i + 1;
        let text = strip_comment(raw).trim();
        for stmt in split_statements(text) {
            p.statement(stmt.trim(), line)
                .map_err(|e| e.unwrap_or(Error::Unsupported { line }))?;
        }
    }
    p.finish(direction)
}

/// Index and text of the header line, after optional YAML front matter
/// (title, config), blank lines and comments.
pub(crate) fn header_line(src: &str) -> Option<(usize, &str)> {
    let mut front_matter = false;
    for (i, raw) in src.lines().enumerate() {
        let text = strip_comment(raw).trim();
        if text == "---" {
            front_matter = !front_matter;
            continue;
        }
        if front_matter || text.is_empty() {
            continue;
        }
        return Some((i, text));
    }
    None
}

/// `%%` starts a comment (also `%%{init}%%` directives) unless inside quotes.
pub(crate) fn strip_comment(line: &str) -> &str {
    let mut quoted = false;
    let b = line.as_bytes();
    for i in 0..b.len() {
        match b[i] {
            b'"' => quoted = !quoted,
            b'%' if !quoted && b.get(i + 1) == Some(&b'%') => return &line[..i],
            _ => {}
        }
    }
    line
}

fn header(text: &str, line: usize) -> Result<Direction, Error> {
    let text = text.trim_end_matches(';');
    let mut words = text.split_whitespace();
    let kind = words.next().unwrap_or("");
    if kind != "flowchart" && kind != "graph" {
        return Err(Error::UnsupportedType);
    }
    let dir = match words.next() {
        None => Direction::Down,
        Some(d) => direction(d).ok_or(Error::Unsupported { line })?,
    };
    if words.next().is_some() {
        // Statements on the header line are rare; keep the parser simple.
        return Err(Error::Unsupported { line });
    }
    Ok(dir)
}

fn direction(word: &str) -> Option<Direction> {
    Some(match word {
        "TB" | "TD" => Direction::Down,
        "BT" => Direction::Up,
        "LR" => Direction::Right,
        "RL" => Direction::Left,
        _ => return None,
    })
}

/// Split on `;` outside quotes and brackets.
fn split_statements(text: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let (mut quoted, mut depth, mut start) = (false, 0usize, 0);
    for (i, c) in text.char_indices() {
        match c {
            '"' => quoted = !quoted,
            '[' | '(' | '{' if !quoted => depth += 1,
            ']' | ')' | '}' if !quoted => depth = depth.saturating_sub(1),
            ';' if !quoted && depth == 0 => {
                out.push(&text[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    out.push(&text[start..]);
    out
}

#[derive(Default)]
struct Parser {
    nodes: Vec<Node>,
    /// Line of an explicit shape or label, per node.
    shaped: Vec<Option<usize>>,
    edges: Vec<Edge>,
    subgraphs: Vec<Subgraph>,
    /// Line of each `subgraph`.
    sub_lines: Vec<usize>,
    /// Open subgraphs, innermost last.
    open: Vec<usize>,
}

/// `Err(None)` is a line-level "unsupported"; `Err(Some(e))` is specific.
type PResult<T> = Result<T, Option<Error>>;

/// Styling and interaction lines that are ignored (theme colours are used
/// and links are never opened).
const IGNORED: &[&str] = &["classDef", "class", "style", "linkStyle", "click"];

impl Parser {
    fn statement(&mut self, s: &str, line: usize) -> PResult<()> {
        if s.is_empty() {
            return Ok(());
        }
        let first = s.split_whitespace().next().unwrap_or("");
        if IGNORED.contains(&first) {
            return Ok(());
        }
        match first {
            "subgraph" => return self.subgraph(s["subgraph".len()..].trim(), line),
            "end" if s == "end" => {
                return self.open.pop().map(|_| ()).ok_or(None);
            }
            "direction" if !self.open.is_empty() => {
                // Accepted and ignored: the chart uses one direction.
                let mut w = s.split_whitespace().skip(1);
                return match (w.next().and_then(direction), w.next()) {
                    (Some(_), None) => Ok(()),
                    _ => Err(None),
                };
            }
            "end" | "direction" => return Err(None),
            _ => {}
        }
        let mut c = Cursor { s, at: 0 };
        let mut left = self.group(&mut c, line)?;
        loop {
            c.skip_ws();
            if c.done() {
                return Ok(());
            }
            let (stroke, tail, head, mut label) = edge(&mut c)?;
            c.skip_ws();
            if c.peek() == Some('|') {
                c.bump();
                let text = c.take_until('|').ok_or(None)?;
                label = Some(clean_label(text)?);
            }
            c.skip_ws();
            let right = self.group(&mut c, line)?;
            for &from in &left {
                for &to in &right {
                    if self.edges.len() >= MAX_EDGES {
                        return Err(Some(Error::TooLarge));
                    }
                    self.edges.push(Edge {
                        from,
                        to,
                        label: label.clone().filter(|l| !l.is_empty()),
                        stroke,
                        head,
                        tail,
                        from_group: None,
                        to_group: None,
                    });
                }
            }
            left = right;
        }
    }

    /// `subgraph id`, `subgraph id [title]`, `subgraph "title"` or
    /// `subgraph some title`.
    fn subgraph(&mut self, rest: &str, line: usize) -> PResult<()> {
        if self.open.len() >= MAX_SUBGRAPH_DEPTH {
            return Err(None);
        }
        if self.subgraphs.len() >= MAX_SUBGRAPHS {
            return Err(Some(Error::TooLarge));
        }
        if rest.is_empty() {
            return Err(None);
        }
        let (id, title) = if let Some(q) = rest.strip_prefix('"') {
            let text = q.strip_suffix('"').ok_or(None)?;
            if text.contains('"') {
                return Err(None);
            }
            let t = clean_label(text)?;
            (t.clone(), t)
        } else {
            let mut c = Cursor { s: rest, at: 0 };
            let id = c.ident();
            let after = c.rest().trim_start();
            match id {
                Some(id) if after.is_empty() => (id.to_owned(), truncate(id)),
                Some(id) if after.starts_with('[') => {
                    let mut c = Cursor { s: after, at: 0 };
                    let Some((Shape::Rect, title)) = shape(&mut c)? else {
                        return Err(None);
                    };
                    if !c.done() {
                        return Err(None);
                    }
                    let title = if title.is_empty() {
                        truncate(id)
                    } else {
                        title
                    };
                    (id.to_owned(), title)
                }
                _ => {
                    if rest.contains(['[', ']', '"', '{', '}', '(', ')']) {
                        return Err(None);
                    }
                    (rest.to_owned(), clean_label(rest)?)
                }
            }
        };
        if self.subgraphs.iter().any(|s| s.id == id) {
            return Err(None);
        }
        self.subgraphs.push(Subgraph {
            id,
            title,
            parent: self.open.last().copied(),
        });
        self.sub_lines.push(line);
        self.open.push(self.subgraphs.len() - 1);
        Ok(())
    }

    /// `node (& node)*`
    fn group(&mut self, c: &mut Cursor<'_>, line: usize) -> PResult<Vec<usize>> {
        let mut ids = vec![self.node(c, line)?];
        loop {
            let save = c.at;
            c.skip_ws();
            if c.peek() == Some('&') {
                c.bump();
                c.skip_ws();
                ids.push(self.node(c, line)?);
            } else {
                c.at = save;
                return Ok(ids);
            }
        }
    }

    fn node(&mut self, c: &mut Cursor<'_>, line: usize) -> PResult<usize> {
        let id = c.ident().ok_or(None)?;
        let shaped = shape(c)?;
        // `:::className` is styling only.
        if c.rest().starts_with(":::") {
            c.at += 3;
            c.ident().ok_or(None)?;
        }
        let idx = match self.nodes.iter().position(|n| n.id == id) {
            Some(i) => i,
            None => {
                if self.nodes.len() >= MAX_NODES {
                    return Err(Some(Error::TooLarge));
                }
                self.nodes.push(Node {
                    id: id.to_owned(),
                    label: truncate(id),
                    shape: Shape::Rect,
                    subgraph: None,
                });
                self.shaped.push(None);
                self.nodes.len() - 1
            }
        };
        if self.nodes[idx].subgraph.is_none() {
            self.nodes[idx].subgraph = self.open.last().copied();
        }
        if let Some((shape, label)) = shaped {
            self.nodes[idx].shape = shape;
            self.nodes[idx].label = label;
            self.shaped[idx] = Some(line);
        }
        Ok(idx)
    }

    /// Check the subgraphs and turn references to subgraph ids into links
    /// to their frames.
    fn finish(self, direction: Direction) -> Result<Flowchart, Error> {
        if let Some(&s) = self.open.last() {
            return Err(Error::Unsupported {
                line: self.sub_lines[s],
            });
        }
        let group_of = |n: &Node| self.subgraphs.iter().position(|s| s.id == n.id);
        let mut remap = vec![usize::MAX; self.nodes.len()];
        let mut nodes = Vec::with_capacity(self.nodes.len());
        for (i, n) in self.nodes.iter().enumerate() {
            if group_of(n).is_some() {
                if let Some(line) = self.shaped[i] {
                    return Err(Error::Unsupported { line });
                }
            } else {
                remap[i] = nodes.len();
                nodes.push(n.clone());
            }
        }
        // First member node of each subgraph (through nested ones).
        let mut rep = vec![usize::MAX; self.subgraphs.len()];
        for (i, n) in nodes.iter().enumerate() {
            let mut s = n.subgraph;
            while let Some(k) = s {
                if rep[k] == usize::MAX {
                    rep[k] = i;
                }
                s = self.subgraphs[k].parent;
            }
        }
        if let Some(k) = rep.iter().position(|&r| r == usize::MAX) {
            // An empty subgraph has nowhere to be drawn.
            return Err(Error::Unsupported {
                line: self.sub_lines[k],
            });
        }
        let end = |i: usize| match group_of(&self.nodes[i]) {
            Some(g) => (rep[g], Some(g)),
            None => (remap[i], None),
        };
        let edges = self
            .edges
            .iter()
            .map(|e| {
                let ((from, from_group), (to, to_group)) = (end(e.from), end(e.to));
                Edge {
                    from,
                    to,
                    from_group,
                    to_group,
                    ..e.clone()
                }
            })
            .collect();
        Ok(Flowchart {
            direction,
            nodes,
            edges,
            subgraphs: self.subgraphs,
        })
    }
}

struct Cursor<'a> {
    s: &'a str,
    at: usize,
}

impl<'a> Cursor<'a> {
    fn rest(&self) -> &'a str {
        &self.s[self.at..]
    }
    fn peek(&self) -> Option<char> {
        self.rest().chars().next()
    }
    fn bump(&mut self) {
        if let Some(ch) = self.peek() {
            self.at += ch.len_utf8();
        }
    }
    fn done(&self) -> bool {
        self.at >= self.s.len()
    }
    fn skip_ws(&mut self) {
        while self.peek().is_some_and(char::is_whitespace) {
            self.bump();
        }
    }
    /// Node id: letters, digits and `_`; `-` and `.` only between them
    /// (so `A-->B` is `A`, `-->`, `B`).
    fn ident(&mut self) -> Option<&'a str> {
        let start = self.at;
        loop {
            match self.peek() {
                Some(ch) if word(ch) => self.bump(),
                Some('-' | '.') if self.at > start => {
                    let mut next = self.rest().chars().skip(1);
                    if next.next().is_some_and(word) {
                        self.bump();
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
        (self.at > start).then(|| &self.s[start..self.at])
    }
    /// Text up to (not including) `close`; the cursor moves past `close`.
    fn take_until(&mut self, close: char) -> Option<&'a str> {
        let rest = self.rest();
        let end = rest.find(close)?;
        self.at += end + close.len_utf8();
        Some(&rest[..end])
    }
}

fn word(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Node shapes: opener and its possible closers, tried in this order.
const SHAPES: &[(&str, &[(&str, Shape)])] = &[
    ("(((", &[(")))", Shape::DoubleCircle)]),
    ("((", &[("))", Shape::Circle)]),
    ("([", &[("])", Shape::Stadium)]),
    ("[(", &[(")]", Shape::Cylinder)]),
    ("[[", &[("]]", Shape::Rect)]),
    (
        "[/",
        &[("/]", Shape::Parallelogram), ("\\]", Shape::Trapezoid)],
    ),
    (
        "[\\",
        &[
            ("\\]", Shape::ParallelogramAlt),
            ("/]", Shape::TrapezoidAlt),
        ],
    ),
    ("[", &[("]", Shape::Rect)]),
    ("(", &[(")", Shape::Round)]),
    ("{{", &[("}}", Shape::Hexagon)]),
    ("{", &[("}", Shape::Diamond)]),
];

/// Optional shape and label after a node id.
fn shape(c: &mut Cursor<'_>) -> PResult<Option<(Shape, String)>> {
    let rest = c.rest();
    // Shapes outside the subset: flag and the `@{ shape: ... }` syntax.
    if rest.starts_with('>') || rest.starts_with("@{") {
        return Err(None);
    }
    let Some(&(open, closes)) = SHAPES.iter().find(|(o, _)| rest.starts_with(o)) else {
        return Ok(None);
    };
    c.at += open.len();
    let body = c.rest();
    let (text, shape, used) = if body.trim_start().starts_with('"') {
        let lead = body.len() - body.trim_start().len();
        let inner = &body[lead + 1..];
        let end = inner.find('"').ok_or(None)?;
        let after = &inner[end + 1..];
        let tail = after.len() - after.trim_start().len();
        let &(close, shape) = closes
            .iter()
            .find(|(cl, _)| after.trim_start().starts_with(cl))
            .ok_or(None)?;
        (
            &inner[..end],
            shape,
            lead + 1 + end + 1 + tail + close.len(),
        )
    } else {
        let (end, close, shape) = closes
            .iter()
            .filter_map(|&(cl, sh)| body.find(cl).map(|i| (i, cl, sh)))
            .min_by_key(|&(i, _, _)| i)
            .ok_or(None)?;
        let text = &body[..end];
        // A stray opener means a shape outside the subset or nesting.
        if text.contains(['[', ']', '(', ')', '{', '}', '"']) {
            return Err(None);
        }
        (text, shape, end + close.len())
    };
    c.at += used;
    Ok(Some((shape, clean_label(text)?)))
}

fn run(s: &str, ch: char) -> usize {
    s.chars().take_while(|&x| x == ch).count()
}

/// A head at the end of a link operator: `>`, or `o`/`x` not followed by
/// more of an id.
fn end_head(s: &str) -> Option<Head> {
    let mut it = s.chars();
    let h = match it.next()? {
        '>' => return Some(Head::Arrow),
        'o' => Head::Circle,
        'x' => Head::Cross,
        _ => return None,
    };
    (!it.next().is_some_and(word)).then_some(h)
}

/// Link operator: `-->`, `---`, `-.->`, `-.-`, `==>`, `===`, `~~~`, with
/// `o`/`x` heads, an optional head at the start (`<-->`, `o--o`, `x--x`)
/// and optional inline text (`-- text -->`, `-. text .->`, `== text ==>`).
fn edge(c: &mut Cursor<'_>) -> PResult<(Stroke, Head, Head, Option<String>)> {
    let rest = c.rest();
    let link = |s: &str| s.starts_with(['-', '=']);
    let tail = match rest.chars().next() {
        Some('<') => Head::Arrow,
        Some('o') if link(&rest[1..]) => Head::Circle,
        Some('x') if link(&rest[1..]) => Head::Cross,
        _ => Head::None,
    };
    if tail != Head::None {
        c.at += 1;
    }
    let rest = c.rest();
    if let Some(n) = Some(run(rest, '~')).filter(|&n| n >= 3) {
        if tail != Head::None {
            return Err(None);
        }
        c.at += n;
        return Ok((Stroke::Invisible, Head::None, Head::None, None));
    }
    for (ch, stroke) in [('-', Stroke::Solid), ('=', Stroke::Thick)] {
        let n = run(rest, ch);
        if n == 0 {
            continue;
        }
        let after = &rest[n..];
        if ch == '-' && n == 1 && after.starts_with('.') {
            return dotted(c, tail);
        }
        if n >= 2
            && let Some(h) = end_head(after)
        {
            c.at += n + 1;
            return Ok((stroke, tail, h, None));
        }
        if n >= 3 {
            c.at += n;
            return Ok((stroke, tail, Head::None, None));
        }
        if n == 2 {
            // Inline text, closed by `-->`/`---` (or `==>`/`===`).
            c.at += 2;
            let body = c.rest();
            let pair: String = [ch, ch].iter().collect();
            let end = body.find(&pair).ok_or(None)?;
            let text = &body[..end];
            c.at += end;
            let k = run(c.rest(), ch);
            c.at += k;
            let head = end_head(c.rest());
            if head.is_some() {
                c.bump();
            } else if k < 3 {
                return Err(None);
            }
            return Ok((
                stroke,
                tail,
                head.unwrap_or(Head::None),
                Some(clean_label(text)?),
            ));
        }
        return Err(None);
    }
    Err(None)
}

fn dotted(c: &mut Cursor<'_>, tail: Head) -> PResult<(Stroke, Head, Head, Option<String>)> {
    // At `-.`
    let rest = c.rest();
    let dots = rest[1..].chars().take_while(|&x| x == '.').count();
    let after = &rest[1 + dots..];
    if let Some(h) = after.strip_prefix('-').and_then(end_head) {
        c.at += 1 + dots + 2;
        return Ok((Stroke::Dotted, tail, h, None));
    }
    if after.starts_with('-') {
        c.at += 1 + dots + 1;
        return Ok((Stroke::Dotted, tail, Head::None, None));
    }
    if dots != 1 {
        return Err(None);
    }
    // `-. text .->` or `-. text .-`
    c.at += 2;
    let body = c.rest();
    let end = body.find(".-").ok_or(None)?;
    let text = &body[..end];
    let head = end_head(&body[end + 2..]);
    c.at += end + 2 + usize::from(head.is_some());
    Ok((
        Stroke::Dotted,
        tail,
        head.unwrap_or(Head::None),
        Some(clean_label(text)?),
    ))
}

/// Trim, turn `<br>` into a line break, refuse other HTML, cap the length.
pub(crate) fn clean_label(text: &str) -> PResult<String> {
    let mut out = String::new();
    let mut rest = text.trim();
    while let Some(i) = rest.find('<') {
        out.push_str(&rest[..i]);
        let tag = &rest[i..];
        let end = tag.find('>').ok_or(None)?;
        let name = tag[1..end].trim().trim_end_matches('/').trim();
        if !name.eq_ignore_ascii_case("br") {
            return Err(None);
        }
        out.push('\n');
        rest = &tag[end + 1..];
    }
    out.push_str(rest);
    let out = out.trim().to_owned();
    Ok(truncate(&out))
}

pub(crate) fn truncate(s: &str) -> String {
    if s.chars().count() <= MAX_LABEL_CHARS {
        return s.to_owned();
    }
    let mut t: String = s.chars().take(MAX_LABEL_CHARS - 1).collect();
    t.push('…');
    t
}

// ---------------------------------------------------------------- layout

/// A placed node or frame: centre and full size, in layout units.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NodeBox {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl NodeBox {
    fn contains(&self, p: (f32, f32)) -> bool {
        (p.0 - self.x).abs() < self.w / 2.0 && (p.1 - self.y).abs() < self.h / 2.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EdgePath {
    /// Polyline from the source boundary to the target boundary (heads,
    /// if any, are at the ends).
    pub points: Vec<(f32, f32)>,
    /// Centre of the edge label, if the edge has one.
    pub label_at: Option<(f32, f32)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Layout {
    pub width: f32,
    pub height: f32,
    /// Same order as `Flowchart::nodes`.
    pub nodes: Vec<NodeBox>,
    /// Same order as `Flowchart::edges`.
    pub edges: Vec<EdgePath>,
    /// Subgraph frames, same order as `Flowchart::subgraphs`. The title
    /// goes inside the top edge; [`Layout::title_band`] is reserved for it.
    pub frames: Vec<NodeBox>,
    /// Height kept free for a frame title inside the top of each frame.
    pub title_band: Vec<f32>,
}

/// Spacing used by [`layout`], in the same units as the measured text.
#[derive(Debug, Clone, Copy)]
pub struct Spacing {
    /// Padding around a label inside its node.
    pub pad: f32,
    /// Gap between neighbours in a layer.
    pub node_gap: f32,
    /// Gap between layers (edge labels add to it).
    pub layer_gap: f32,
}

impl Spacing {
    /// Spacing proportional to a font size.
    pub fn for_font(size: f32) -> Self {
        Self {
            pad: size * 0.7,
            node_gap: size * 1.6,
            layer_gap: size * 2.6,
        }
    }
}

/// Subgraph ancestry of `s`, outermost first.
fn chain(fc: &Flowchart, s: Option<usize>) -> Vec<usize> {
    let mut out = Vec::new();
    let mut s = s;
    while let Some(k) = s {
        out.push(k);
        s = fc.subgraphs[k].parent;
        if out.len() > MAX_SUBGRAPHS {
            break;
        }
    }
    out.reverse();
    out
}

/// Sort `row` by `key` (aligned with `row`), keeping the members of each
/// subgraph contiguous: at every nesting level a subgraph is one block
/// ordered by `ckey`. Ties keep the current order.
fn cluster_sort(row: &mut Vec<usize>, key: &[f32], ckey: &[f32], vchain: &[Vec<usize>]) {
    type Key = Vec<(f32, u8, usize)>;
    let mut seqs: Vec<(Key, usize)> = row
        .iter()
        .enumerate()
        .map(|(i, &v)| {
            let mut s: Vec<(f32, u8, usize)> = vchain[v].iter().map(|&c| (ckey[c], 0, c)).collect();
            s.push((key[i], 1, i));
            (s, v)
        })
        .collect();
    seqs.sort_by(|a, b| {
        for (x, y) in a.0.iter().zip(&b.0) {
            let o = x.0.total_cmp(&y.0).then(x.1.cmp(&y.1)).then(x.2.cmp(&y.2));
            if o.is_ne() {
                return o;
            }
        }
        a.0.len().cmp(&b.0.len())
    });
    *row = seqs.into_iter().map(|(_, v)| v).collect();
}

/// Mean of `val` per subgraph over the vertices of `row` inside it.
fn cluster_means(
    row: &[usize],
    val: impl Fn(usize, usize) -> f32,
    vchain: &[Vec<usize>],
    ns: usize,
) -> Vec<Option<f32>> {
    let mut sum = vec![(0f32, 0usize); ns];
    for (i, &v) in row.iter().enumerate() {
        for &c in &vchain[v] {
            sum[c].0 += val(i, v);
            sum[c].1 += 1;
        }
    }
    sum.into_iter()
        .map(|(s, k)| (k > 0).then(|| s / k as f32))
        .collect()
}

/// Lay out `fc`. `measure` returns the (width, height) of a label (which
/// may hold `\n`).
pub fn layout(
    fc: &Flowchart,
    sp: Spacing,
    measure: &mut dyn FnMut(&str) -> (f32, f32),
) -> Result<Layout, Error> {
    let sizes = node_sizes(fc, sp, measure);
    layout_sized(fc, sp, &sizes, measure)
}

/// Size of each node from its label and shape.
fn node_sizes(
    fc: &Flowchart,
    sp: Spacing,
    measure: &mut dyn FnMut(&str) -> (f32, f32),
) -> Vec<(f32, f32)> {
    fc.nodes
        .iter()
        .map(|node| {
            let (tw, th) = measure(&node.label);
            let (w, h) = (tw + 2.0 * sp.pad, th + sp.pad);
            match node.shape {
                // The smallest diamond holding a w*h box with corners on
                // its edges: w/W + h/H = 1, with W = w + 2h.
                Shape::Diamond => (w + 2.0 * h, h + w / 2.0),
                Shape::Circle => {
                    let d = (tw.max(th) + sp.pad).max(h);
                    (d, d)
                }
                Shape::DoubleCircle => {
                    let d = (tw.max(th) + sp.pad).max(h) + sp.pad * 0.8;
                    (d, d)
                }
                Shape::Stadium => (w + h * 0.5, h),
                // Slanted sides inset by h / 2 at one end.
                Shape::Hexagon
                | Shape::Parallelogram
                | Shape::ParallelogramAlt
                | Shape::Trapezoid
                | Shape::TrapezoidAlt => (w + h, h),
                // Lids are ellipses h / 4 high.
                Shape::Cylinder => (w, h * 1.5),
                Shape::Rect | Shape::Round => (w, h),
            }
        })
        .collect()
}

/// Lay out `fc` with the given node sizes (same order as `fc.nodes`), for
/// diagrams whose nodes are not plain labels (state and class diagrams).
/// `measure` sizes edge labels and frame titles.
pub fn layout_sized(
    fc: &Flowchart,
    sp: Spacing,
    sizes: &[(f32, f32)],
    measure: &mut dyn FnMut(&str) -> (f32, f32),
) -> Result<Layout, Error> {
    let n = fc.nodes.len();
    let ns = fc.subgraphs.len();
    let horizontal = matches!(fc.direction, Direction::Right | Direction::Left);
    if sizes.len() != n {
        return Err(Error::TooLarge);
    }
    let label_sizes: Vec<Option<(f32, f32)>> = fc
        .edges
        .iter()
        .map(|e| e.label.as_deref().map(&mut *measure))
        .collect();
    let titles: Vec<(f32, f32)> = fc.subgraphs.iter().map(|s| measure(&s.title)).collect();

    // 1. Break cycles: reverse DFS back edges (iterative, in node order).
    let mut out: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (i, e) in fc.edges.iter().enumerate() {
        if e.from != e.to {
            out[e.from].push(i);
        }
    }
    let mut reversed = vec![false; fc.edges.len()];
    let mut state = vec![0u8; n]; // 0 new, 1 on stack, 2 done
    for root in 0..n {
        if state[root] != 0 {
            continue;
        }
        let mut stack = vec![(root, 0usize)];
        state[root] = 1;
        while let Some(top) = stack.last_mut() {
            let (v, k) = *top;
            if let Some(&ei) = out[v].get(k) {
                top.1 += 1;
                let w = fc.edges[ei].to;
                match state[w] {
                    0 => {
                        state[w] = 1;
                        stack.push((w, 0));
                    }
                    1 => reversed[ei] = true,
                    _ => {}
                }
            } else {
                state[v] = 2;
                stack.pop();
            }
        }
    }
    // DAG edges (u -> v) per original edge, self loops excluded.
    let dag: Vec<Option<(usize, usize)>> = fc
        .edges
        .iter()
        .zip(&reversed)
        .map(|(e, &r)| (e.from != e.to).then_some(if r { (e.to, e.from) } else { (e.from, e.to) }))
        .collect();

    // 2. Longest-path layering (Kahn order).
    let mut indeg = vec![0usize; n];
    let mut succ: Vec<Vec<usize>> = vec![Vec::new(); n];
    for &(u, v) in dag.iter().flatten() {
        succ[u].push(v);
        indeg[v] += 1;
    }
    let mut layer = vec![0usize; n];
    let mut queue: std::collections::VecDeque<usize> = (0..n).filter(|&v| indeg[v] == 0).collect();
    while let Some(u) = queue.pop_front() {
        for &v in &succ[u] {
            layer[v] = layer[v].max(layer[u] + 1);
            indeg[v] -= 1;
            if indeg[v] == 0 {
                queue.push_back(v);
            }
        }
    }
    // A source that only feeds later nodes sits just before its nearest
    // successor instead of at the top (a side input, a note left of a
    // state).
    let mut has_pred = vec![false; n];
    for &(_, v) in dag.iter().flatten() {
        has_pred[v] = true;
    }
    for v in 0..n {
        if !has_pred[v]
            && let Some(m) = succ[v].iter().map(|&s| layer[s]).min()
        {
            layer[v] = m.saturating_sub(1);
        }
    }
    let layers = layer.iter().max().map_or(0, |m| m + 1);

    // 3. Split long edges into unit segments with virtual nodes. A virtual
    // node belongs to the innermost subgraph holding both ends.
    // Vertex ids: 0..n real, n..phantom_start virtual, then phantoms.
    let mut vlayer = layer.clone();
    let mut vclus: Vec<Option<usize>> = fc.nodes.iter().map(|x| x.subgraph).collect();
    let mut chains: Vec<Vec<usize>> = Vec::with_capacity(fc.edges.len());
    for d in &dag {
        let Some((u, v)) = *d else {
            chains.push(Vec::new());
            continue;
        };
        let (cu, cv) = (chain(fc, vclus[u]), chain(fc, vclus[v]));
        let common = cu.iter().zip(&cv).take_while(|(a, b)| a == b).count();
        let owner = common.checked_sub(1).map(|i| cu[i]);
        let mut ch = vec![u];
        for l in layer[u] + 1..layer[v] {
            if vlayer.len() - n >= MAX_DUMMIES {
                return Err(Error::TooLarge);
            }
            vlayer.push(l);
            vclus.push(owner);
            ch.push(vlayer.len() - 1);
        }
        ch.push(v);
        chains.push(ch);
    }
    // Layer range of each subgraph (over its real nodes) and a zero-size
    // phantom in every layer of the range where it has no vertex, so the
    // frame keeps its place in that layer too.
    let mut range: Vec<Option<(usize, usize)>> = vec![None; ns];
    for (v, node) in fc.nodes.iter().enumerate() {
        for k in chain(fc, node.subgraph) {
            let r = range[k].get_or_insert((layer[v], layer[v]));
            *r = (r.0.min(layer[v]), r.1.max(layer[v]));
        }
    }
    let mut present = vec![vec![false; layers]; ns];
    for v in 0..vlayer.len() {
        for k in chain(fc, vclus[v]) {
            present[k][vlayer[v]] = true;
        }
    }
    let phantom_start = vlayer.len();
    for k in (0..ns).rev() {
        let Some((a, b)) = range[k] else { continue };
        for l in a..=b {
            if !present[k][l] {
                vlayer.push(l);
                vclus.push(Some(k));
                for c in chain(fc, Some(k)) {
                    present[c][l] = true;
                }
            }
        }
    }
    let total = vlayer.len();
    let vchain: Vec<Vec<usize>> = vclus.iter().map(|&c| chain(fc, c)).collect();
    let mut up: Vec<Vec<usize>> = vec![Vec::new(); total];
    let mut down: Vec<Vec<usize>> = vec![Vec::new(); total];
    for ch in &chains {
        for w in ch.windows(2) {
            down[w[0]].push(w[1]);
            up[w[1]].push(w[0]);
        }
    }

    // 4. Ordering: first appearance, then barycenter sweeps that keep
    // subgraphs contiguous, then one pass that orders sibling subgraphs
    // the same way in every layer.
    let mut rows: Vec<Vec<usize>> = vec![Vec::new(); layers];
    for v in 0..total {
        rows[vlayer[v]].push(v);
    }
    let mut pos = vec![0f32; total];
    for r in &rows {
        for (i, &v) in r.iter().enumerate() {
            pos[v] = i as f32;
        }
    }
    let empty: Vec<Option<f32>> = vec![None; ns];
    for pass in 0..ORDER_PASSES {
        let downward = pass.is_multiple_of(2);
        let order: Vec<usize> = if downward {
            (1..layers).collect()
        } else {
            (0..layers.saturating_sub(1)).rev().collect()
        };
        for l in order {
            let nbrs = if downward { &up } else { &down };
            let means = |r: Option<&Vec<usize>>| match r {
                Some(r) if ns > 0 => cluster_means(r, |_, v| pos[v], &vchain, ns),
                _ => empty.clone(),
            };
            let (above, below) = (
                means(l.checked_sub(1).map(|i| &rows[i])),
                means(rows.get(l + 1)),
            );
            let key: Vec<f32> = rows[l]
                .iter()
                .map(|&v| {
                    if v >= phantom_start {
                        let k = vchain[v].last().copied().unwrap_or(0);
                        let near: Vec<f32> = [above[k], below[k]].into_iter().flatten().collect();
                        return if near.is_empty() {
                            pos[v]
                        } else {
                            near.iter().sum::<f32>() / near.len() as f32
                        };
                    }
                    let ns = &nbrs[v];
                    if ns.is_empty() {
                        pos[v]
                    } else {
                        ns.iter().map(|&u| pos[u]).sum::<f32>() / ns.len() as f32
                    }
                })
                .collect();
            let ckey: Vec<f32> = cluster_means(&rows[l], |i, _| key[i], &vchain, ns)
                .into_iter()
                .map(|m| m.unwrap_or(0.0))
                .collect();
            cluster_sort(&mut rows[l], &key, &ckey, &vchain);
            for (i, &v) in rows[l].iter().enumerate() {
                pos[v] = i as f32;
            }
        }
    }
    if ns > 0 {
        let mut sum = vec![(0f32, 0usize); ns];
        for v in 0..total {
            for &c in &vchain[v] {
                sum[c].0 += pos[v];
                sum[c].1 += 1;
            }
        }
        let ckey: Vec<f32> = sum.iter().map(|&(s, k)| s / k.max(1) as f32).collect();
        for r in &mut rows {
            let key: Vec<f32> = r.iter().map(|&v| pos[v]).collect();
            cluster_sort(r, &key, &ckey, &vchain);
            for (i, &v) in r.iter().enumerate() {
                pos[v] = i as f32;
            }
        }
    }

    // 5. Coordinates. "Cross" is along a layer, "main" across layers.
    // Room on both sides of a node with a self-loop for the loop and its
    // label (the loop is drawn on one side, the label beyond it).
    let mut loop_room = vec![0f32; n];
    for (i, e) in fc.edges.iter().enumerate() {
        if e.from == e.to {
            let label = label_sizes[i].map_or(0.0, |(lw, lh)| {
                (if horizontal { lh } else { lw }) + sp.pad * 0.25
            });
            let room = loop_radius(sizes[e.from]) * 1.5 + label;
            loop_room[e.from] = loop_room[e.from].max(room);
        }
    }
    let cross = |v: usize| -> f32 {
        if v < n {
            (if horizontal { sizes[v].1 } else { sizes[v].0 }) + 2.0 * loop_room[v]
        } else if v < phantom_start {
            sp.node_gap * 0.25
        } else {
            0.0
        }
    };
    let main = |v: usize| -> f32 {
        if v < n {
            if horizontal { sizes[v].0 } else { sizes[v].1 }
        } else {
            0.0
        }
    };
    // Frame padding; the title band is at the top of the picture.
    let band: Vec<f32> = titles.iter().map(|t| t.1 + sp.pad * 0.5).collect();
    let pad_cs = |k: usize| sp.pad + if horizontal { band[k] } else { 0.0 };
    let pad_ce = sp.pad;
    // Separation constraints along each layer: variable b >= a + w.
    // Variables: vertices, then (left, right) edges of each subgraph.
    let nv = total + 2 * ns;
    let lvar = |k: usize| total + 2 * k;
    let rvar = |k: usize| total + 2 * k + 1;
    let mut preds: Vec<Vec<(usize, f32)>> = vec![Vec::new(); nv];
    let mut succs: Vec<Vec<usize>> = vec![Vec::new(); nv];
    let mut add = |a: usize, b: usize, w: f32| {
        preds[b].push((a, w));
        succs[a].push(b);
    };
    #[derive(Clone, Copy)]
    enum Item {
        V(usize),
        C(usize),
    }
    let right = |it: Item| match it {
        Item::V(v) => (v, cross(v) / 2.0),
        Item::C(k) => (rvar(k), 0.0),
    };
    for r in &rows {
        // Open subgraphs (innermost last) with the last item inside each.
        let mut stack: Vec<(usize, Option<Item>)> = Vec::new();
        let mut top_last: Option<Item> = None;
        let close = |stack: &mut Vec<(usize, Option<Item>)>,
                     top_last: &mut Option<Item>,
                     add: &mut dyn FnMut(usize, usize, f32)| {
            if let Some((k, last)) = stack.pop() {
                if let Some(it) = last {
                    let (a, off) = right(it);
                    add(a, rvar(k), off + pad_ce);
                }
                match stack.last_mut() {
                    Some(parent) => parent.1 = Some(Item::C(k)),
                    None => *top_last = Some(Item::C(k)),
                }
            }
        };
        for &v in r {
            let ch = &vchain[v];
            let keep = stack
                .iter()
                .zip(ch)
                .take_while(|((k, _), c)| k == *c)
                .count();
            while stack.len() > keep {
                close(&mut stack, &mut top_last, &mut add);
            }
            for &k in &ch[keep..] {
                let prev = match stack.last() {
                    Some(&(_, last)) => last,
                    None => top_last,
                };
                match (prev, stack.last()) {
                    (Some(it), _) => {
                        let (a, off) = right(it);
                        add(a, lvar(k), off + sp.node_gap);
                    }
                    (None, Some(&(parent, _))) => add(lvar(parent), lvar(k), pad_cs(parent)),
                    (None, None) => {}
                }
                let min_w = if horizontal {
                    0.0
                } else {
                    titles[k].0 + 2.0 * sp.pad
                };
                add(lvar(k), rvar(k), min_w);
                stack.push((k, None));
            }
            let prev = match stack.last() {
                Some(&(_, last)) => last,
                None => top_last,
            };
            match (prev, stack.last()) {
                (Some(it), _) => {
                    let (a, off) = right(it);
                    add(a, v, off + sp.node_gap + cross(v) / 2.0);
                }
                (None, Some(&(k, _))) => add(lvar(k), v, pad_cs(k) + cross(v) / 2.0),
                (None, None) => {}
            }
            match stack.last_mut() {
                Some(top) => top.1 = Some(Item::V(v)),
                None => top_last = Some(Item::V(v)),
            }
        }
        while !stack.is_empty() {
            close(&mut stack, &mut top_last, &mut add);
        }
    }
    let mut members: Vec<Vec<usize>> = vec![Vec::new(); ns];
    for (v, ch) in vchain.iter().enumerate().take(phantom_start) {
        for &k in ch {
            members[k].push(v);
        }
    }
    let mut row_at = vec![0usize; total];
    for r in &rows {
        for (i, &v) in r.iter().enumerate() {
            row_at[v] = i;
        }
    }
    // Initial packing, then pull nodes towards their neighbours' mean
    // without breaking the constraints (each variable is placed after the
    // ones it must follow, as far left as they and its wish allow).
    let mut xv = vec![0f32; nv];
    for r in &rows {
        let mut at = 0.0;
        for &v in r {
            xv[v] = at + cross(v) / 2.0;
            at += cross(v) + sp.node_gap;
        }
    }
    for pass in 0..4usize {
        let downward = pass.is_multiple_of(2);
        let nbrs = if downward { &up } else { &down };
        let want = |xv: &[f32], v: usize| -> f32 {
            let ns = &nbrs[v];
            if ns.is_empty() {
                xv[v]
            } else {
                ns.iter().map(|&u| xv[u]).sum::<f32>() / ns.len() as f32
            }
        };
        let prio = |u: usize| -> (usize, usize, usize) {
            if u < total {
                let l = if downward {
                    vlayer[u]
                } else {
                    layers - 1 - vlayer[u]
                };
                (l + 1, row_at[u], u)
            } else {
                (0, 0, u)
            }
        };
        let mut indeg: Vec<usize> = preds.iter().map(Vec::len).collect();
        let mut heap: std::collections::BinaryHeap<std::cmp::Reverse<(usize, usize, usize)>> = (0
            ..nv)
            .filter(|&u| indeg[u] == 0)
            .map(|u| std::cmp::Reverse(prio(u)))
            .collect();
        let mut done = vec![false; nv];
        let place = |u: usize, xv: &mut Vec<f32>| {
            let lb = preds[u]
                .iter()
                .map(|&(a, w)| xv[a] + w)
                .fold(f32::NEG_INFINITY, f32::max);
            let x = if u < phantom_start {
                want(xv, u).max(lb)
            } else if u < total {
                lb
            } else if (u - total).is_multiple_of(2) {
                let k = (u - total) / 2;
                let wish = members[k]
                    .iter()
                    .map(|&v| want(xv, v) - cross(v) / 2.0 - pad_cs(k))
                    .fold(f32::INFINITY, f32::min);
                if wish.is_finite() { wish.max(lb) } else { lb }
            } else {
                lb
            };
            xv[u] = if x.is_finite() { x } else { 0.0 };
        };
        while let Some(std::cmp::Reverse((_, _, u))) = heap.pop() {
            place(u, &mut xv);
            done[u] = true;
            for &s in &succs[u] {
                indeg[s] -= 1;
                if indeg[s] == 0 {
                    heap.push(std::cmp::Reverse(prio(s)));
                }
            }
        }
        // Only reachable if the constraints had a cycle; place the rest.
        for (u, _) in done.iter().enumerate().filter(|(_, d)| !**d) {
            place(u, &mut xv);
        }
    }
    let lo = (0..total)
        .map(|v| xv[v] - cross(v) / 2.0)
        .chain((0..ns).map(|k| xv[lvar(k)]))
        .fold(f32::INFINITY, f32::min);
    let lo = if lo.is_finite() { lo } else { 0.0 };
    for x in &mut xv {
        *x -= lo;
    }
    let extent_c = (0..total)
        .map(|v| xv[v] + cross(v) / 2.0)
        .chain((0..ns).map(|k| xv[rvar(k)]))
        .fold(0f32, f32::max);
    // Layer thickness and gaps (room for edge labels and frames between
    // layers).
    let mut thick = vec![0f32; layers];
    for v in 0..total {
        thick[vlayer[v]] = thick[vlayer[v]].max(main(v));
    }
    let mut gap = vec![sp.layer_gap; layers];
    for (i, d) in dag.iter().enumerate() {
        if let (Some((u, _)), Some((lw, lh))) = (d, label_sizes[i]) {
            let extra = if horizontal { lw } else { lh };
            let g = &mut gap[layer[*u]];
            *g = g.max(extra + sp.layer_gap * 0.8);
        }
    }
    // Frame room before the first and after the last layer of each
    // subgraph, growing with frames nested at the same layer.
    let mut need = vec![(0f32, 0f32); ns];
    for k in (0..ns).rev() {
        let Some((a, b)) = range[k] else { continue };
        let (mut s, mut e) = (0f32, 0f32);
        for c in (k + 1)..ns {
            if fc.subgraphs[c].parent == Some(k)
                && let Some((ca, cb)) = range[c]
            {
                if ca == a {
                    s = s.max(need[c].0);
                }
                if cb == b {
                    e = e.max(need[c].1);
                }
            }
        }
        let top_s = fc.direction == Direction::Down;
        let top_e = fc.direction == Direction::Up;
        need[k] = (
            s + sp.pad + if top_s { band[k] } else { 0.0 },
            e + sp.pad + if top_e { band[k] } else { 0.0 },
        );
    }
    let mut lead = vec![0f32; layers];
    let mut trail = vec![0f32; layers];
    for k in 0..ns {
        if let Some((a, b)) = range[k] {
            lead[a] = lead[a].max(need[k].0);
            trail[b] = trail[b].max(need[k].1);
        }
    }
    let mut mstart = vec![0f32; layers];
    let mut at = lead.first().copied().unwrap_or(0.0);
    for l in 0..layers {
        mstart[l] = at;
        at += thick[l];
        if l + 1 < layers {
            let frames = trail[l] + lead[l + 1];
            at += if frames > 0.0 {
                gap[l].max(frames + sp.layer_gap * 0.6)
            } else {
                gap[l]
            };
        }
    }
    let extent_m = at + trail.last().copied().unwrap_or(0.0);
    let mpos = |v: usize| mstart[vlayer[v]] + thick[vlayer[v]] / 2.0;
    let (width, height) = if horizontal {
        (extent_m, extent_c)
    } else {
        (extent_c, extent_m)
    };
    let place = |m: f32, c: f32| -> (f32, f32) {
        match fc.direction {
            Direction::Down => (c, m),
            Direction::Up => (c, extent_m - m),
            Direction::Right => (m, c),
            Direction::Left => (extent_m - m, c),
        }
    };
    let pt = |v: usize| place(mpos(v), xv[v]);
    let nodes: Vec<NodeBox> = (0..n)
        .map(|v| {
            let (x, y) = pt(v);
            NodeBox {
                x,
                y,
                w: sizes[v].0,
                h: sizes[v].1,
            }
        })
        .collect();
    let frames: Vec<NodeBox> = (0..ns)
        .map(|k| {
            let Some((a, b)) = range[k] else {
                return NodeBox {
                    x: 0.0,
                    y: 0.0,
                    w: 0.0,
                    h: 0.0,
                };
            };
            let p0 = place(mstart[a] - need[k].0, xv[lvar(k)]);
            let p1 = place(mstart[b] + thick[b] + need[k].1, xv[rvar(k)]);
            NodeBox {
                x: (p0.0 + p1.0) / 2.0,
                y: (p0.1 + p1.1) / 2.0,
                w: (p1.0 - p0.0).abs(),
                h: (p1.1 - p0.1).abs(),
            }
        })
        .collect();

    // 6. Edge paths, clipped to node outlines (or to the frame of a
    // subgraph end), in original direction.
    let mut edges = Vec::with_capacity(fc.edges.len());
    for (i, e) in fc.edges.iter().enumerate() {
        let mut points: Vec<(f32, f32)> = if e.from == e.to {
            self_loop(&nodes[e.from], fc.direction)
        } else {
            let mut ps: Vec<(f32, f32)> = chains[i].iter().map(|&v| pt(v)).collect();
            if reversed[i] {
                ps.reverse();
            }
            // Target end, then source end (on the reversed path).
            match e.to_group.and_then(|g| into_frame(&ps, &frames[g])) {
                Some(cut) => ps = cut,
                None => {
                    let k = ps.len() - 1;
                    ps[k] = clip(&nodes[e.to], fc.nodes[e.to].shape, ps[k - 1]);
                }
            }
            ps.reverse();
            match e.from_group.and_then(|g| into_frame(&ps, &frames[g])) {
                Some(cut) => ps = cut,
                None => {
                    let k = ps.len() - 1;
                    ps[k] = clip(&nodes[e.from], fc.nodes[e.from].shape, ps[k - 1]);
                }
            }
            ps.reverse();
            ps
        };
        points.dedup();
        if points.len() < 2 {
            // Both ends clipped to the same spot (overlapping frames of a
            // link between a subgraph and its own member).
            let p = points.first().copied().unwrap_or((0.0, 0.0));
            points = vec![p, p];
        }
        let label_at = match label_sizes[i] {
            None => None,
            Some((lw, lh)) if e.from == e.to => {
                let b = &nodes[e.from];
                let out = loop_radius((b.w, b.h)) * 1.5 + sp.pad * 0.25;
                Some(if horizontal {
                    (b.x, b.y + b.h / 2.0 + out + lh / 2.0)
                } else {
                    (b.x + b.w / 2.0 + out + lw / 2.0, b.y)
                })
            }
            Some(_) => Some(midpoint(&points)),
        };
        edges.push(EdgePath { points, label_at });
    }
    Ok(Layout {
        width,
        height,
        nodes,
        edges,
        frames,
        title_band: band,
    })
}

/// The polyline `ps` cut where it last enters `frame` (the new last point
/// is on the frame's edge), or `None` if `ps` starts inside the frame.
fn into_frame(ps: &[(f32, f32)], frame: &NodeBox) -> Option<Vec<(f32, f32)>> {
    let k = (1..ps.len()).rev().find(|&k| !frame.contains(ps[k - 1]))?;
    if !frame.contains(ps[k]) {
        return None;
    }
    let (a, b) = (ps[k - 1], ps[k]);
    // Bisection: a is outside, b inside.
    let (mut lo, mut hi) = (0f32, 1f32);
    for _ in 0..32 {
        let mid = (lo + hi) / 2.0;
        let p = (a.0 + (b.0 - a.0) * mid, a.1 + (b.1 - a.1) * mid);
        if frame.contains(p) {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    let mut out = ps[..k].to_vec();
    out.push((a.0 + (b.0 - a.0) * lo, a.1 + (b.1 - a.1) * lo));
    Some(out)
}

/// Size of the self-loop drawn on a node of size `(w, h)`.
fn loop_radius((w, h): (f32, f32)) -> f32 {
    w.min(h) * 0.35
}

/// A small loop on the side of the node facing away from the flow.
fn self_loop(b: &NodeBox, dir: Direction) -> Vec<(f32, f32)> {
    let r = loop_radius((b.w, b.h));
    match dir {
        Direction::Down | Direction::Up => {
            let x = b.x + b.w / 2.0;
            vec![
                (x, b.y - r * 0.6),
                (x + r * 1.5, b.y - r * 0.6),
                (x + r * 1.5, b.y + r * 0.6),
                (x, b.y + r * 0.6),
            ]
        }
        Direction::Right | Direction::Left => {
            let y = b.y + b.h / 2.0;
            vec![
                (b.x - r * 0.6, y),
                (b.x - r * 0.6, y + r * 1.5),
                (b.x + r * 0.6, y + r * 1.5),
                (b.x + r * 0.6, y),
            ]
        }
    }
}

/// Where the segment from the centre of `b` towards `to` leaves its outline.
fn clip(b: &NodeBox, shape: Shape, to: (f32, f32)) -> (f32, f32) {
    let (dx, dy) = (to.0 - b.x, to.1 - b.y);
    if dx == 0.0 && dy == 0.0 {
        return (b.x, b.y);
    }
    let (hw, hh) = (b.w / 2.0, b.h / 2.0);
    let t = match shape {
        Shape::Diamond => 1.0 / (dx.abs() / hw + dy.abs() / hh),
        Shape::Circle | Shape::DoubleCircle => hw / (dx * dx + dy * dy).sqrt(),
        _ => {
            let tx = if dx == 0.0 {
                f32::INFINITY
            } else {
                hw / dx.abs()
            };
            let ty = if dy == 0.0 {
                f32::INFINITY
            } else {
                hh / dy.abs()
            };
            tx.min(ty)
        }
    };
    let t = t.min(1.0);
    (b.x + dx * t, b.y + dy * t)
}

/// Point halfway along a polyline.
pub(crate) fn midpoint(ps: &[(f32, f32)]) -> (f32, f32) {
    let seg = |a: (f32, f32), b: (f32, f32)| ((b.0 - a.0).powi(2) + (b.1 - a.1).powi(2)).sqrt();
    let total: f32 = ps.windows(2).map(|w| seg(w[0], w[1])).sum();
    let mut left = total / 2.0;
    for w in ps.windows(2) {
        let l = seg(w[0], w[1]);
        if l >= left && l > 0.0 {
            let t = left / l;
            return (
                w[0].0 + (w[1].0 - w[0].0) * t,
                w[0].1 + (w[1].1 - w[0].1) * t,
            );
        }
        left -= l;
    }
    ps.first().copied().unwrap_or((0.0, 0.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    pub(crate) fn measure(s: &str) -> (f32, f32) {
        let lines = s.split('\n');
        let w = lines.clone().map(|l| l.chars().count()).max().unwrap_or(0);
        (w as f32 * 7.0, lines.count() as f32 * 14.0)
    }

    fn lay(src: &str) -> (Flowchart, Layout) {
        let fc = parse(src).expect("parses");
        let l = layout(&fc, Spacing::for_font(14.0), &mut measure).expect("lays out");
        (fc, l)
    }

    fn node<'a>(fc: &'a Flowchart, id: &str) -> &'a Node {
        fc.nodes.iter().find(|n| n.id == id).expect("node")
    }

    fn idx(fc: &Flowchart, id: &str) -> usize {
        fc.nodes.iter().position(|n| n.id == id).expect("node")
    }

    #[test]
    fn header_and_directions() {
        for (h, d) in [
            ("graph", Direction::Down),
            ("graph TD", Direction::Down),
            ("flowchart TB", Direction::Down),
            ("flowchart BT", Direction::Up),
            ("flowchart LR", Direction::Right),
            ("graph RL;", Direction::Left),
        ] {
            assert_eq!(
                parse(&format!("{h}\nA-->B")).map(|f| f.direction),
                Ok(d),
                "{h}"
            );
        }
        assert_eq!(
            parse("flowchart XY\nA"),
            Err(Error::Unsupported { line: 1 })
        );
    }

    #[test]
    fn other_diagram_types_are_unsupported() {
        for src in ["pie\n\"a\": 1", "gantt", "", "%% only a comment"] {
            assert_eq!(parse(src), Err(Error::UnsupportedType), "{src:?}");
            assert_eq!(parse_diagram(src), Err(Error::UnsupportedType), "{src:?}");
        }
        // `parse` is flowchart-only; `parse_diagram` dispatches.
        assert_eq!(
            parse("sequenceDiagram\nA->>B: hi"),
            Err(Error::UnsupportedType)
        );
        assert!(matches!(
            parse_diagram("---\ntitle: x\n---\n%% c\nsequenceDiagram\nA->>B: hi"),
            Ok(Diagram::Sequence(_))
        ));
        assert_eq!(parse("classDiagram\nA <|-- B"), Err(Error::UnsupportedType));
        assert!(matches!(
            parse_diagram("classDiagram\nA <|-- B"),
            Ok(Diagram::Class(_))
        ));
        assert!(matches!(
            parse_diagram("graph TD\nA-->B"),
            Ok(Diagram::Flowchart(_))
        ));
        assert_eq!(
            parse_diagram(&"x".repeat(MAX_SOURCE_BYTES + 1)),
            Err(Error::TooLarge)
        );
    }

    #[test]
    fn shapes_and_labels() {
        let fc = parse(
            "flowchart TD\nA[Rect]\nB(Round)\nC{Choice?}\nD((Circle))\nE([Stadium])\nF\nG[\"Quoted (with) [brackets]\"]\nH[[Sub]]",
        )
        .expect("parses");
        let s = |id| (node(&fc, id).shape, node(&fc, id).label.as_str());
        assert_eq!(s("A"), (Shape::Rect, "Rect"));
        assert_eq!(s("B"), (Shape::Round, "Round"));
        assert_eq!(s("C"), (Shape::Diamond, "Choice?"));
        assert_eq!(s("D"), (Shape::Circle, "Circle"));
        assert_eq!(s("E"), (Shape::Stadium, "Stadium"));
        assert_eq!(s("F"), (Shape::Rect, "F"));
        assert_eq!(s("G"), (Shape::Rect, "Quoted (with) [brackets]"));
        assert_eq!(s("H"), (Shape::Rect, "Sub"));
    }

    #[test]
    fn extra_shapes() {
        let fc = parse(
            "flowchart LR\nA{{Hex}} --> B[(Database)]\nC[/In/] --> D[\\Out\\]\nE[/Top\\] --> F[\\Bottom/]\nG(((Stop)))\nH[/\"a/b (c)\"/]",
        )
        .expect("parses");
        let s = |id| (node(&fc, id).shape, node(&fc, id).label.as_str());
        assert_eq!(s("A"), (Shape::Hexagon, "Hex"));
        assert_eq!(s("B"), (Shape::Cylinder, "Database"));
        assert_eq!(s("C"), (Shape::Parallelogram, "In"));
        assert_eq!(s("D"), (Shape::ParallelogramAlt, "Out"));
        assert_eq!(s("E"), (Shape::Trapezoid, "Top"));
        assert_eq!(s("F"), (Shape::TrapezoidAlt, "Bottom"));
        assert_eq!(s("G"), (Shape::DoubleCircle, "Stop"));
        assert_eq!(s("H"), (Shape::Parallelogram, "a/b (c)"));
        let (_, l) = lay("graph TD\nA{{Hex}} --> B[(Db)] --> C(((x))) --> D[/p/]");
        assert_no_overlap(&l);
        // Double circle and cylinder are larger than the plain shapes.
        let (_, l) = lay("graph TD\nA((x))\nB(((x)))\nC[x]\nD[(x)]");
        assert!(l.nodes[1].w > l.nodes[0].w && l.nodes[3].h > l.nodes[2].h);
    }

    type EdgeRow<'a> = (&'a str, &'a str, Stroke, Head, Head, Option<&'a str>);

    fn edge_rows(fc: &Flowchart) -> Vec<EdgeRow<'_>> {
        fc.edges
            .iter()
            .map(|e| {
                (
                    fc.nodes[e.from].id.as_str(),
                    fc.nodes[e.to].id.as_str(),
                    e.stroke,
                    e.tail,
                    e.head,
                    e.label.as_deref(),
                )
            })
            .collect()
    }

    #[test]
    fn edges_and_edge_labels() {
        use Head::{Arrow as A, None as N};
        let fc = parse(
            "graph LR\nA-->B\nB --- C\nC-.->D\nD-.-E\nE==>F\nF===G\nA -->|yes| C\nA -- no --> D\nB -. maybe .-> E\nC == big ==> F\nG ~~~ A",
        )
        .expect("parses");
        assert_eq!(
            edge_rows(&fc),
            [
                ("A", "B", Stroke::Solid, N, A, None),
                ("B", "C", Stroke::Solid, N, N, None),
                ("C", "D", Stroke::Dotted, N, A, None),
                ("D", "E", Stroke::Dotted, N, N, None),
                ("E", "F", Stroke::Thick, N, A, None),
                ("F", "G", Stroke::Thick, N, N, None),
                ("A", "C", Stroke::Solid, N, A, Some("yes")),
                ("A", "D", Stroke::Solid, N, A, Some("no")),
                ("B", "E", Stroke::Dotted, N, A, Some("maybe")),
                ("C", "F", Stroke::Thick, N, A, Some("big")),
                ("G", "A", Stroke::Invisible, N, N, None),
            ]
        );
    }

    #[test]
    fn arrow_heads() {
        use Head::{Arrow as A, Circle as O, Cross as X, None as N};
        let fc = parse(
            "graph TD\nA <--> B\nA --o B\nA --x B\nA o--o B\nA x--x B\nA <-.-> B\nA <==> B\nA -.-o B\nA ==x B\nA -- t --o B\nA <-- both --> B\nA -. d .-x B\nA ---o B\nA --oB\nAo --> xB",
        );
        // `A --oB` is inline text without a closing operator.
        assert_eq!(fc, Err(Error::Unsupported { line: 15 }));
        let fc = parse(
            "graph TD\nA <--> B\nA --o B\nA --x B\nA o--o B\nA x--x B\nA <-.-> B\nA <==> B\nA -.-o B\nA ==x B\nA -- t --o B\nA <-- both --> B\nA -. d .-x B\nA ---o B\nAo --> xB",
        )
        .expect("parses");
        assert_eq!(
            edge_rows(&fc),
            [
                ("A", "B", Stroke::Solid, A, A, None),
                ("A", "B", Stroke::Solid, N, O, None),
                ("A", "B", Stroke::Solid, N, X, None),
                ("A", "B", Stroke::Solid, O, O, None),
                ("A", "B", Stroke::Solid, X, X, None),
                ("A", "B", Stroke::Dotted, A, A, None),
                ("A", "B", Stroke::Thick, A, A, None),
                ("A", "B", Stroke::Dotted, N, O, None),
                ("A", "B", Stroke::Thick, N, X, None),
                ("A", "B", Stroke::Solid, N, O, Some("t")),
                ("A", "B", Stroke::Solid, A, A, Some("both")),
                ("A", "B", Stroke::Dotted, N, X, Some("d")),
                ("A", "B", Stroke::Solid, N, O, None),
                ("Ao", "xB", Stroke::Solid, N, A, None),
            ]
        );
        assert_eq!(
            parse("graph TD\nA <~~~ B"),
            Err(Error::Unsupported { line: 2 })
        );
    }

    #[test]
    fn chains_groups_semicolons_and_ids() {
        let fc = parse("graph TD\nA --> B --> C; C --> D\nA & B --> E & F\nnode-1 --> node.2")
            .expect("parses");
        let pairs: Vec<_> = fc
            .edges
            .iter()
            .map(|e| format!("{}{}", fc.nodes[e.from].id, fc.nodes[e.to].id))
            .collect();
        assert_eq!(
            pairs,
            ["AB", "BC", "CD", "AE", "AF", "BE", "BF", "node-1node.2"]
        );
    }

    #[test]
    fn comments_styles_and_clicks_are_ignored() {
        let fc = parse(
            "%%{init: {'theme':'dark'}}%%\n---\ntitle: T\n---\nflowchart TD\n%% comment\nA[\"100%% sure\"] --> B %% trailing\nclassDef red fill:#f00\nclass A red\nstyle B fill:#0f0\nlinkStyle 0 stroke:#f00\nclick A \"https://example.org\" \"tip\"\nC:::red --> A",
        )
        .expect("parses");
        assert_eq!(fc.nodes.len(), 3);
        assert_eq!(node(&fc, "A").label, "100%% sure");
        assert_eq!(fc.edges.len(), 2);
    }

    #[test]
    fn redefinition_updates_label_and_shape() {
        let fc = parse("graph TD\nA --> B\nB{Decide}").expect("parses");
        assert_eq!(node(&fc, "B").shape, Shape::Diamond);
        assert_eq!(node(&fc, "B").label, "Decide");
    }

    #[test]
    fn labels_line_breaks_html_and_caps() {
        let fc = parse("graph TD\nA[one<br>two<br/>three]").expect("parses");
        assert_eq!(node(&fc, "A").label, "one\ntwo\nthree");
        assert_eq!(
            parse("graph TD\nA --> B\nC[<b>bold</b>]"),
            Err(Error::Unsupported { line: 3 })
        );
        let long = "x".repeat(500);
        let fc = parse(&format!("graph TD\nA[{long}]")).expect("parses");
        assert_eq!(node(&fc, "A").label.chars().count(), MAX_LABEL_CHARS);
        assert!(node(&fc, "A").label.ends_with('…'));
    }

    #[test]
    fn unsupported_constructs_and_errors() {
        for (src, line) in [
            ("graph TD\nA>flag]", 2),
            ("graph TD\nA@{ shape: rect }", 2),
            ("graph TD\nA[unclosed", 2),
            ("graph TD\nA -->", 2),
            ("graph TD\nA -- text without close B", 2),
            ("graph TD\n--> B", 2),
            ("graph TD\nA[\"open quote]", 2),
            ("graph TD\nA{{hex}", 2),
            ("graph TD\nA[/slant]", 2),
            ("graph TD\nend", 2),
            ("graph TD\ndirection LR", 2),
            ("graph TD\nsubgraph", 2),
            ("graph TD\nsubgraph a\nA\nend\nend", 5),
            ("graph TD\nsubgraph a\nA", 2),
            ("graph TD\nsubgraph a\nend", 2),
            ("graph TD\nsubgraph a\ndirection XY\nend", 3),
            ("graph TD\nsubgraph a\nA\nend\nsubgraph a\nB\nend", 5),
            ("graph TD\nsubgraph a [t\nA\nend", 2),
            ("graph TD\nsubgraph a\nA\nend\nX --> a[Box]", 5),
            (
                "graph TD\nsubgraph a\nsubgraph b\nsubgraph c\nsubgraph d\nA\nend\nend\nend\nend",
                5,
            ),
        ] {
            assert_eq!(parse(src), Err(Error::Unsupported { line }), "{src:?}");
        }
    }

    #[test]
    fn size_caps() {
        assert_eq!(
            parse(&format!("graph TD\n{}", "A".repeat(MAX_SOURCE_BYTES))),
            Err(Error::TooLarge)
        );
        let many: String = (0..=MAX_NODES).map(|i| format!("n{i}\n")).collect();
        assert_eq!(parse(&format!("graph TD\n{many}")), Err(Error::TooLarge));
        let fan: String = (0..20).map(|i| format!("a{i} & ")).collect::<String>() + "x";
        let src = format!("graph TD\n{fan} --> {fan2}", fan2 = fan.replace('a', "b"));
        assert_eq!(parse(&src), Err(Error::TooLarge));
        let subs: String = (0..=MAX_SUBGRAPHS)
            .map(|i| format!("subgraph s{i}\nn{i}\nend\n"))
            .collect();
        assert_eq!(parse(&format!("graph TD\n{subs}")), Err(Error::TooLarge));
    }

    #[test]
    fn subgraph_syntax_and_membership() {
        let fc = parse(
            "flowchart TB\nX --> A\nsubgraph one [First]\n  A --> B\n  subgraph two[\"Second (inner)\"]\n    direction LR\n    C\n  end\n  X2\nend\nsubgraph Three words\nD --> A\nend\nsubgraph \"Quoted\"\nE\nend\nsubgraph solo\nF\nend\nsubgraph s; G; end",
        )
        .expect("parses");
        let titles: Vec<_> = fc
            .subgraphs
            .iter()
            .map(|s| (s.id.as_str(), s.title.as_str(), s.parent))
            .collect();
        assert_eq!(
            titles,
            [
                ("one", "First", None),
                ("two", "Second (inner)", Some(0)),
                ("Three words", "Three words", None),
                ("Quoted", "Quoted", None),
                ("solo", "solo", None),
                ("s", "s", None),
            ]
        );
        let sub = |id| node(&fc, id).subgraph;
        // X is used before any subgraph and never inside one.
        assert_eq!(sub("X"), None);
        // A is first used outside, then moves into `one`; a later mention
        // in `Three words` does not move it again.
        assert_eq!(sub("A"), Some(0));
        assert_eq!(sub("B"), Some(0));
        assert_eq!(sub("C"), Some(1));
        assert_eq!(sub("X2"), Some(0));
        assert_eq!(sub("D"), Some(2));
        assert_eq!(sub("E"), Some(3));
        assert_eq!(sub("F"), Some(4));
        assert_eq!(sub("G"), Some(5));
    }

    #[test]
    fn links_to_subgraphs_use_the_first_member_and_the_frame() {
        let (fc, l) = lay(
            "graph TD\nstart --> grp\nsubgraph grp [Group]\n  a --> b\nend\ngrp --> stop\nsubgraph other\n  c\nend\ngrp --> other",
        );
        assert!(fc.nodes.iter().all(|n| n.id != "grp" && n.id != "other"));
        let e = &fc.edges[0];
        assert_eq!((e.to, e.to_group), (idx(&fc, "a"), Some(0)));
        let e = &fc.edges[2];
        assert_eq!((e.from, e.from_group), (idx(&fc, "a"), Some(0)));
        let e = &fc.edges[3];
        assert_eq!((e.from_group, e.to_group), (Some(0), Some(1)));
        // The arrow into `grp` stops on the frame's top edge.
        let f = l.frames[0];
        let end = *l.edges[0].points.last().expect("points");
        assert!((end.1 - (f.y - f.h / 2.0)).abs() < 0.5, "{end:?} {f:?}");
        assert!((end.0 - f.x).abs() <= f.w / 2.0);
        // The link out of `grp` starts on the frame's bottom edge.
        let start = l.edges[2].points[0];
        assert!(on_edge(start, &f), "{start:?} {f:?}");
        let g = l.frames[1];
        assert!(
            on_edge(l.edges[3].points[0], &f)
                && on_edge(*l.edges[3].points.last().expect("end"), &g)
        );
        assert_frames_ok(&fc, &l);
    }

    pub(crate) fn on_edge(p: (f32, f32), f: &NodeBox) -> bool {
        let (dx, dy) = ((p.0 - f.x).abs() - f.w / 2.0, (p.1 - f.y).abs() - f.h / 2.0);
        (dx.abs() < 0.5 && dy <= 0.5) || (dy.abs() < 0.5 && dx <= 0.5)
    }

    pub(crate) fn overlaps(a: &NodeBox, b: &NodeBox) -> bool {
        (a.x - b.x).abs() < (a.w + b.w) / 2.0 && (a.y - b.y).abs() < (a.h + b.h) / 2.0
    }

    fn inside(inner: &NodeBox, outer: &NodeBox) -> bool {
        let e = 0.01;
        inner.x - inner.w / 2.0 >= outer.x - outer.w / 2.0 - e
            && inner.x + inner.w / 2.0 <= outer.x + outer.w / 2.0 + e
            && inner.y - inner.h / 2.0 >= outer.y - outer.h / 2.0 - e
            && inner.y + inner.h / 2.0 <= outer.y + outer.h / 2.0 + e
    }

    pub(crate) fn assert_no_overlap(l: &Layout) {
        for (i, a) in l.nodes.iter().enumerate() {
            for b in &l.nodes[i + 1..] {
                assert!(!overlaps(a, b), "{a:?} overlaps {b:?}");
            }
            assert!(a.x - a.w / 2.0 >= -0.01 && a.x + a.w / 2.0 <= l.width + 0.01);
            assert!(a.y - a.h / 2.0 >= -0.01 && a.y + a.h / 2.0 <= l.height + 0.01);
        }
    }

    /// Members are inside their frames, other nodes do not touch a frame,
    /// and two frames are either nested or apart; the title band is clear.
    pub(crate) fn assert_frames_ok(fc: &Flowchart, l: &Layout) {
        assert_eq!(l.frames.len(), fc.subgraphs.len());
        let within = |n: &Node, k: usize| chain(fc, n.subgraph).contains(&k);
        for (k, f) in l.frames.iter().enumerate() {
            assert!(f.w > 0.0 && f.h > 0.0, "frame {k} {f:?}");
            assert!(inside(
                f,
                &NodeBox {
                    x: l.width / 2.0,
                    y: l.height / 2.0,
                    w: l.width,
                    h: l.height
                }
            ));
            for (node, b) in fc.nodes.iter().zip(&l.nodes) {
                if within(node, k) {
                    assert!(inside(b, f), "{} outside frame {k}: {b:?} {f:?}", node.id);
                    // Below the title band.
                    let top = f.y - f.h / 2.0 + l.title_band[k];
                    assert!(b.y - b.h / 2.0 >= top - 0.01, "{} under title {k}", node.id);
                } else {
                    assert!(
                        !overlaps(b, f),
                        "{} overlaps frame {k}: {b:?} {f:?}",
                        node.id
                    );
                }
            }
            for (j, g) in l.frames.iter().enumerate().skip(k + 1) {
                let nested = chain(fc, Some(j)).contains(&k);
                if nested {
                    assert!(inside(g, f), "frame {j} not inside {k}");
                    let below = g.y - g.h / 2.0 - (f.y - f.h / 2.0 + l.title_band[k]);
                    assert!(below >= -0.01, "frame {j} under the title of {k}");
                } else {
                    assert!(!overlaps(f, g), "frames {k} and {j} overlap: {f:?} {g:?}");
                }
            }
        }
    }

    #[test]
    fn nested_subgraph_layout() {
        for dir in ["TD", "BT", "LR", "RL"] {
            let (fc, l) = lay(&format!(
                "flowchart {dir}\nin --> a1\nsubgraph outer [Outer frame with a long title]\n  a1 --> a2\n  subgraph mid [Mid]\n    b1 --> b2\n    subgraph deep [Deep]\n      c1 --> c2\n    end\n  end\n  a2 --> b1\nend\nsubgraph side [Side]\n  s1 --> s2\nend\nin --> s1\nc2 --> out\ns2 --> out\nin --> out"
            ));
            assert_no_overlap(&l);
            assert_frames_ok(&fc, &l);
            let (_, again) = lay(&format!(
                "flowchart {dir}\nin --> a1\nsubgraph outer [Outer frame with a long title]\n  a1 --> a2\n  subgraph mid [Mid]\n    b1 --> b2\n    subgraph deep [Deep]\n      c1 --> c2\n    end\n  end\n  a2 --> b1\nend\nsubgraph side [Side]\n  s1 --> s2\nend\nin --> s1\nc2 --> out\ns2 --> out\nin --> out"
            ));
            assert_eq!(l, again, "deterministic");
        }
    }

    #[test]
    fn random_subgraphs_keep_frames_apart() {
        let mut s: u64 = 0x0123_4567_89AB_CDEF;
        let mut next = |m: u64| {
            s ^= s << 13;
            s ^= s >> 7;
            s ^= s << 17;
            s % m
        };
        for round in 0..400 {
            let dir = ["TD", "BT", "LR", "RL"][round % 4];
            let mut src = format!("flowchart {dir}\n");
            let nodes = 2 + next(18);
            let mut depth = 0;
            let mut subs = 0;
            let mut open_nonempty: Vec<bool> = Vec::new();
            for _ in 0..(5 + next(40)) {
                match next(10) {
                    0 | 1 if depth < MAX_SUBGRAPH_DEPTH => {
                        src.push_str(&format!("subgraph g{subs} [Group {subs}]\n"));
                        subs += 1;
                        depth += 1;
                        open_nonempty.push(false);
                    }
                    2 if depth > 0 && open_nonempty.last() == Some(&true) => {
                        src.push_str("end\n");
                        depth -= 1;
                        open_nonempty.pop();
                        if let Some(o) = open_nonempty.last_mut() {
                            *o = true;
                        }
                    }
                    3 if subs > 0 => {
                        let g = next(subs);
                        let b = next(nodes);
                        src.push_str(&format!("n{b} --> g{g}\n"));
                        open_nonempty.fill(true);
                    }
                    _ => {
                        let (a, b) = (next(nodes), next(nodes));
                        src.push_str(&format!("n{a} --> n{b}\n"));
                        open_nonempty.fill(true);
                    }
                }
            }
            for _ in 0..depth {
                src.push_str("x_fill\nend\n");
            }
            let fc = match parse(&src) {
                Ok(fc) => fc,
                // A link to a subgraph defined later can make it empty, or
                // a group id may collide with a node used inside it.
                Err(Error::Unsupported { .. }) => continue,
                Err(e) => panic!("{e:?} in {src}"),
            };
            let a = layout(&fc, Spacing::for_font(13.0), &mut measure).expect("lays out");
            assert_no_overlap(&a);
            assert_frames_ok(&fc, &a);
            let b = layout(&fc, Spacing::for_font(13.0), &mut measure).expect("lays out");
            assert_eq!(a, b, "deterministic: {src}");
        }
    }

    #[test]
    fn subgraph_members_are_contiguous_in_each_layer() {
        // Interleaved links would mix the two groups without blocks.
        let (fc, l) = lay(
            "graph TD\nr --> a1 & b1 & a2 & b2\nsubgraph A\na1\na2\nend\nsubgraph B\nb1\nb2\nend",
        );
        let xs = |ids: &[&str]| {
            let v: Vec<f32> = ids.iter().map(|id| l.nodes[idx(&fc, id)].x).collect();
            (
                v.iter().copied().fold(f32::INFINITY, f32::min),
                v.iter().copied().fold(f32::NEG_INFINITY, f32::max),
            )
        };
        let (a, b) = (xs(&["a1", "a2"]), xs(&["b1", "b2"]));
        assert!(a.1 < b.0 || b.1 < a.0, "{a:?} {b:?}");
        assert_frames_ok(&fc, &l);
    }

    #[test]
    fn layered_layout_follows_direction() {
        let (fc, l) = lay("graph TD\nA --> B --> C\nA --> C");
        let y = |id| l.nodes[idx(&fc, id)].y;
        assert!(y("A") < y("B") && y("B") < y("C"));
        assert_no_overlap(&l);
        let (fc, l) = lay("graph BT\nA --> B");
        assert!(l.nodes[0].y > l.nodes[1].y, "{fc:?}");
        let (_, l) = lay("graph LR\nA --> B");
        assert!(l.nodes[0].x < l.nodes[1].x);
        let (_, l) = lay("graph RL\nA --> B");
        assert!(l.nodes[0].x > l.nodes[1].x);
        // A source feeding only a late node sits just before it.
        let (fc, l) = lay("graph TD\na --> b --> c --> d\nside --> d");
        assert_eq!(l.nodes[idx(&fc, "side")].y, l.nodes[idx(&fc, "c")].y);
        // A self-loop label is beside the loop, outside the node and the
        // loop, and inside the picture.
        for dir in ["TD", "LR"] {
            let (fc, l) = lay(&format!("graph {dir}\na --> b\nb -->|again| b\nb --> c"));
            let b = l.nodes[idx(&fc, "b")];
            let (x, y) = l.edges[1].label_at.expect("label");
            let (lw, lh) = measure("again");
            let lbl = NodeBox { x, y, w: lw, h: lh };
            assert!(!overlaps(&lbl, &b), "{lbl:?} {b:?}");
            let loop_far = l.edges[1]
                .points
                .iter()
                .map(|p| if dir == "TD" { p.0 } else { p.1 })
                .fold(f32::NEG_INFINITY, f32::max);
            if dir == "TD" {
                assert!(x - lw / 2.0 >= loop_far);
            } else {
                assert!(y - lh / 2.0 >= loop_far);
            }
            assert!(x + lw / 2.0 <= l.width + 0.01 && y + lh / 2.0 <= l.height + 0.01);
            assert_no_overlap(&l);
        }
        // Long edge A->C bends through a virtual node: 3 points.
        let (_, l) = lay("graph TD\nA --> B --> C\nA --> C");
        assert_eq!(l.edges[2].points.len(), 3);
    }

    #[test]
    fn cycles_and_self_loops_terminate() {
        let (fc, l) = lay("graph TD\nA --> B --> C --> A\nC --> C\nB --> A");
        assert_eq!(l.edges.len(), fc.edges.len());
        assert_no_overlap(&l);
        for e in &l.edges {
            assert!(e.points.len() >= 2);
        }
        // Arrow of the back edge C --> A still ends at A.
        let a = l.nodes[0];
        let end = *l.edges[2].points.last().expect("points");
        assert!((end.0 - a.x).abs() <= a.w / 2.0 + 0.5 && (end.1 - a.y).abs() <= a.h / 2.0 + 0.5);
        // A subgraph linked to itself and to its own member.
        let (fc, l) = lay("graph TD\nsubgraph g\na --> b\nend\ng --> g\ng --> b\na --> g");
        assert_eq!(l.edges.len(), 4);
        assert_frames_ok(&fc, &l);
    }

    #[test]
    fn layout_is_deterministic_and_wide_graphs_do_not_overlap() {
        let src = "flowchart LR\nA{Start} -->|go| B([Next]) & C((C)) & D\nB --> E --> F\nC -.-> F\nD ==> F\nF --> A\nE -- long label text --> A";
        let (_, a) = lay(src);
        let (_, b) = lay(src);
        assert_eq!(a, b);
        assert_no_overlap(&a);
        let fan: String = (0..40)
            .map(|i| format!("root --> n{i}\nn{i} --> sink\n"))
            .collect();
        let (_, l) = lay(&format!("graph TD\n{fan}"));
        assert_no_overlap(&l);
    }

    #[test]
    fn dummy_cap_is_enforced() {
        // A long chain plus many edges from the start to the end.
        let chain: String = (0..140).map(|i| format!("c{i} --> c{}\n", i + 1)).collect();
        let jumps: String = (0..40).map(|_| "c0 --> c140\n").collect();
        let fc = parse(&format!("graph TD\n{chain}{jumps}")).expect("parses");
        assert_eq!(
            layout(&fc, Spacing::for_font(14.0), &mut measure),
            Err(Error::TooLarge)
        );
    }

    #[test]
    fn random_graphs_lay_out_without_overlap() {
        let mut s: u64 = 0xD1B5_4A32_D192_ED03;
        let mut next = |m: u64| {
            s ^= s << 13;
            s ^= s >> 7;
            s ^= s << 17;
            s % m
        };
        let ops = [
            "-->", "---", "-.->", "==>", "-->|l|", "-- t -->", "~~~", "<-->", "--o", "--x", "o--o",
        ];
        let shapes = [
            ("[", "]"),
            ("(", ")"),
            ("{", "}"),
            ("((", "))"),
            ("([", "])"),
            ("{{", "}}"),
            ("[(", ")]"),
            ("[/", "/]"),
            ("[\\", "/]"),
            ("(((", ")))"),
        ];
        for round in 0..300 {
            let dir = ["TD", "BT", "LR", "RL"][round % 4];
            let mut src = format!("flowchart {dir}\n");
            let nodes = 1 + next(25);
            for _ in 0..next(60) {
                let (a, b) = (next(nodes), next(nodes));
                let (o, c) = shapes[next(shapes.len() as u64) as usize];
                let op = ops[next(ops.len() as u64) as usize];
                src.push_str(&format!("n{a}{o}label {a}{c} {op} n{b}\n"));
            }
            let fc = parse(&src).unwrap_or_else(|e| panic!("{e:?} in {src}"));
            let a = layout(&fc, Spacing::for_font(13.0), &mut measure).expect("lays out");
            assert_no_overlap(&a);
            assert_eq!(a.edges.len(), fc.edges.len());
            let b = layout(&fc, Spacing::for_font(13.0), &mut measure).expect("lays out");
            assert_eq!(a, b, "deterministic");
        }
    }

    #[test]
    fn random_input_never_panics() {
        // Deterministic xorshift; mixes Mermaid tokens with junk.
        let tokens = [
            "A",
            "B",
            "c_1",
            "-->",
            "---",
            "-.->",
            "-.-",
            "==>",
            "===",
            "--",
            "-.",
            "==",
            "|",
            "[",
            "]",
            "(",
            ")",
            "{",
            "}",
            "((",
            "))",
            "([",
            "])",
            "\"",
            "&",
            ";",
            "\n",
            " ",
            "%%",
            ":::",
            "<br>",
            "<",
            ">",
            "~~~",
            "ä",
            "🙂",
            "subgraph",
            "click",
            "-",
            ".",
            "x",
            "o",
            "end",
            "direction LR",
            "{{",
            "}}",
            "[(",
            ")]",
            "[/",
            "/]",
            "[\\",
            "\\]",
            "(((",
            ")))",
            "<-->",
            "--o",
            "--x",
            "\nsubgraph g\n",
            "\nend\n",
            "g",
        ];
        let mut s: u64 = 0x9E37_79B9_7F4A_7C15;
        for _ in 0..4000 {
            let mut src = String::from(if s.is_multiple_of(2) {
                "graph TD\n"
            } else {
                "flowchart LR\n"
            });
            let len = (s % 50) as usize;
            for _ in 0..len {
                s ^= s << 13;
                s ^= s >> 7;
                s ^= s << 17;
                src.push_str(tokens[(s % tokens.len() as u64) as usize]);
            }
            if let Ok(fc) = parse(&src) {
                let l = layout(&fc, Spacing::for_font(12.0), &mut measure);
                if let Ok(l) = l {
                    assert_eq!(l.nodes.len(), fc.nodes.len());
                    assert!(l.width.is_finite() && l.height.is_finite());
                    assert_frames_ok(&fc, &l);
                }
            }
            s = s.wrapping_add(0x2545_F491_4F6C_DD1D);
        }
    }
}
