//! A safe, native subset of Mermaid flowcharts.
//!
//! `parse` turns the text of a ```` ```mermaid ```` block into a small graph
//! model and `layout` places it with a simple layered (Sugiyama-style)
//! algorithm. Nothing here touches the network, opens links or runs
//! document content: `click`, `style`, `classDef`, `class` and `linkStyle`
//! lines are ignored, and anything outside the subset is reported as an
//! [`Error`] so the UI can show the source instead.
//!
//! All work is bounded by the `MAX_*` caps and is deterministic: the same
//! text and the same label sizes always give the same picture.

/// Diagram text larger than this is not drawn.
pub const MAX_SOURCE_BYTES: usize = 16 * 1024;
pub const MAX_NODES: usize = 150;
pub const MAX_EDGES: usize = 300;
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stroke {
    Solid,
    Dotted,
    Thick,
    /// `~~~`: affects the layout but is not drawn.
    Invisible,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub id: String,
    pub label: String,
    pub shape: Shape,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub label: Option<String>,
    pub stroke: Stroke,
    pub arrow: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Flowchart {
    pub direction: Direction,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

/// Why a block is shown as source instead of a diagram.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// Another diagram type (sequence, class, gantt, pie, ...).
    UnsupportedType,
    /// A flowchart feature outside the subset, or text that does not
    /// parse. `line` is 1-based within the block.
    Unsupported { line: usize },
    /// Over one of the size caps.
    TooLarge,
}

/// Parse a Mermaid block. Only `flowchart` / `graph` are supported.
pub fn parse(src: &str) -> Result<Flowchart, Error> {
    if src.len() > MAX_SOURCE_BYTES {
        return Err(Error::TooLarge);
    }
    let mut p = Parser {
        nodes: Vec::new(),
        edges: Vec::new(),
    };
    let mut direction = None;
    let mut front_matter = false;
    for (i, raw) in src.lines().enumerate() {
        let line = i + 1;
        let text = strip_comment(raw).trim();
        if direction.is_none() {
            // Optional YAML front matter (title, config) before the header.
            if text == "---" {
                front_matter = !front_matter;
                continue;
            }
            if front_matter || text.is_empty() {
                continue;
            }
            direction = Some(header(text, line)?);
            continue;
        }
        for stmt in split_statements(text) {
            p.statement(stmt.trim())
                .map_err(|e| e.unwrap_or(Error::Unsupported { line }))?;
        }
    }
    let Some(direction) = direction else {
        return Err(Error::UnsupportedType);
    };
    Ok(Flowchart {
        direction,
        nodes: p.nodes,
        edges: p.edges,
    })
}

/// `%%` starts a comment (also `%%{init}%%` directives) unless inside quotes.
fn strip_comment(line: &str) -> &str {
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
        None | Some("TB" | "TD") => Direction::Down,
        Some("BT") => Direction::Up,
        Some("LR") => Direction::Right,
        Some("RL") => Direction::Left,
        Some(_) => return Err(Error::Unsupported { line }),
    };
    if words.next().is_some() {
        // Statements on the header line are rare; keep the parser simple.
        return Err(Error::Unsupported { line });
    }
    Ok(dir)
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

struct Parser {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

/// `Err(None)` is a line-level "unsupported"; `Err(Some(e))` is specific.
type PResult<T> = Result<T, Option<Error>>;

/// Styling and interaction lines that are ignored (theme colours are used
/// and links are never opened).
const IGNORED: &[&str] = &["classDef", "class", "style", "linkStyle", "click"];

impl Parser {
    fn statement(&mut self, s: &str) -> PResult<()> {
        if s.is_empty() {
            return Ok(());
        }
        let first = s.split_whitespace().next().unwrap_or("");
        if IGNORED.contains(&first) {
            return Ok(());
        }
        if matches!(first, "subgraph" | "end" | "direction") {
            return Err(None);
        }
        let mut c = Cursor { s, at: 0 };
        let mut left = self.group(&mut c)?;
        loop {
            c.skip_ws();
            if c.done() {
                return Ok(());
            }
            let (stroke, arrow, mut label) = edge(&mut c)?;
            c.skip_ws();
            if c.peek() == Some('|') {
                c.bump();
                let text = c.take_until('|').ok_or(None)?;
                label = Some(clean_label(text)?);
            }
            c.skip_ws();
            let right = self.group(&mut c)?;
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
                        arrow,
                    });
                }
            }
            left = right;
        }
    }

    /// `node (& node)*`
    fn group(&mut self, c: &mut Cursor<'_>) -> PResult<Vec<usize>> {
        let mut ids = vec![self.node(c)?];
        loop {
            let save = c.at;
            c.skip_ws();
            if c.peek() == Some('&') {
                c.bump();
                c.skip_ws();
                ids.push(self.node(c)?);
            } else {
                c.at = save;
                return Ok(ids);
            }
        }
    }

    fn node(&mut self, c: &mut Cursor<'_>) -> PResult<usize> {
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
                });
                self.nodes.len() - 1
            }
        };
        if let Some((shape, label)) = shaped {
            self.nodes[idx].shape = shape;
            self.nodes[idx].label = label;
        }
        Ok(idx)
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
        let word = |c: char| c.is_alphanumeric() || c == '_';
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

/// Optional shape and label after a node id.
fn shape(c: &mut Cursor<'_>) -> PResult<Option<(Shape, String)>> {
    let rest = c.rest();
    // Shapes outside the subset: double circle, cylinder, slants,
    // hexagon, flag and the `@{ shape: ... }` syntax.
    if ["(((", "[(", "[/", "[\\", "{{", ">", "@{"]
        .iter()
        .any(|o| rest.starts_with(o))
    {
        return Err(None);
    }
    let (shape, open, close) = if rest.starts_with("((") {
        (Shape::Circle, 2, "))")
    } else if rest.starts_with("([") {
        (Shape::Stadium, 2, "])")
    } else if rest.starts_with("[[") {
        (Shape::Rect, 2, "]]")
    } else if rest.starts_with('[') {
        (Shape::Rect, 1, "]")
    } else if rest.starts_with('(') {
        (Shape::Round, 1, ")")
    } else if rest.starts_with('{') {
        (Shape::Diamond, 1, "}")
    } else {
        return Ok(None);
    };
    c.at += open;
    let body = c.rest();
    let text = if body.trim_start().starts_with('"') {
        let lead = body.len() - body.trim_start().len();
        let inner = &body[lead + 1..];
        let end = inner.find('"').ok_or(None)?;
        let after = &inner[end + 1..];
        let tail = after.len() - after.trim_start().len();
        if !after.trim_start().starts_with(close) {
            return Err(None);
        }
        c.at += lead + 1 + end + 1 + tail + close.len();
        &inner[..end]
    } else {
        let end = body.find(close).ok_or(None)?;
        let text = &body[..end];
        // A stray opener means a shape outside the subset or nesting.
        if text.contains(['[', ']', '(', ')', '{', '}', '"']) {
            return Err(None);
        }
        c.at += end + close.len();
        text
    };
    Ok(Some((shape, clean_label(text)?)))
}

/// Edge operator: `-->`, `---`, `-.->`, `-.-`, `==>`, `===`, `~~~`, with
/// optional inline text (`-- text -->`, `-. text .->`, `== text ==>`).
fn edge(c: &mut Cursor<'_>) -> PResult<(Stroke, bool, Option<String>)> {
    let rest = c.rest();
    if rest.starts_with('<') || rest.starts_with("--o") || rest.starts_with("--x") {
        return Err(None);
    }
    let run = |s: &str, ch: char| s.chars().take_while(|&x| x == ch).count();
    if let Some(n) = Some(run(rest, '~')).filter(|&n| n >= 3) {
        c.at += n;
        return Ok((Stroke::Invisible, false, None));
    }
    for (ch, stroke) in [('-', Stroke::Solid), ('=', Stroke::Thick)] {
        let n = run(rest, ch);
        if n == 0 {
            continue;
        }
        let after = &rest[n..];
        if ch == '-' && n == 1 && after.starts_with('.') {
            return dotted(c);
        }
        if after.starts_with('>') && n >= 2 {
            c.at += n + 1;
            return Ok((stroke, true, None));
        }
        if n >= 3 {
            c.at += n;
            return Ok((stroke, false, None));
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
            let arrow = c.peek() == Some('>');
            if arrow {
                c.bump();
            } else if k < 3 {
                return Err(None);
            }
            return Ok((stroke, arrow, Some(clean_label(text)?)));
        }
        return Err(None);
    }
    Err(None)
}

fn dotted(c: &mut Cursor<'_>) -> PResult<(Stroke, bool, Option<String>)> {
    // At `-.`
    let rest = c.rest();
    let dots = rest[1..].chars().take_while(|&x| x == '.').count();
    let after = &rest[1 + dots..];
    if after.starts_with("->") {
        c.at += 1 + dots + 2;
        return Ok((Stroke::Dotted, true, None));
    }
    if after.starts_with('-') {
        c.at += 1 + dots + 1;
        return Ok((Stroke::Dotted, false, None));
    }
    if dots != 1 {
        return Err(None);
    }
    // `-. text .->` or `-. text .-`
    c.at += 2;
    let body = c.rest();
    let end = body.find(".-").ok_or(None)?;
    let text = &body[..end];
    let arrow = body[end + 2..].starts_with('>');
    c.at += end + 2 + usize::from(arrow);
    Ok((Stroke::Dotted, arrow, Some(clean_label(text)?)))
}

/// Trim, turn `<br>` into a line break, refuse other HTML, cap the length.
fn clean_label(text: &str) -> PResult<String> {
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

fn truncate(s: &str) -> String {
    if s.chars().count() <= MAX_LABEL_CHARS {
        return s.to_owned();
    }
    let mut t: String = s.chars().take(MAX_LABEL_CHARS - 1).collect();
    t.push('…');
    t
}

// ---------------------------------------------------------------- layout

/// A placed node: centre and full size, in layout units.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NodeBox {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EdgePath {
    /// Polyline from the source boundary to the target boundary (the arrow
    /// head, if any, is at the last point).
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

/// Lay out `fc`. `measure` returns the (width, height) of a label (which
/// may hold `\n`).
pub fn layout(
    fc: &Flowchart,
    sp: Spacing,
    measure: &mut dyn FnMut(&str) -> (f32, f32),
) -> Result<Layout, Error> {
    let n = fc.nodes.len();
    let horizontal = matches!(fc.direction, Direction::Right | Direction::Left);
    // Node sizes.
    let sizes: Vec<(f32, f32)> = fc
        .nodes
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
                Shape::Stadium => (w + h * 0.5, h),
                _ => (w, h),
            }
        })
        .collect();
    let label_sizes: Vec<Option<(f32, f32)>> = fc
        .edges
        .iter()
        .map(|e| e.label.as_deref().map(&mut *measure))
        .collect();

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
    let layers = layer.iter().max().map_or(0, |m| m + 1);

    // 3. Split long edges into unit segments with virtual nodes.
    // Vertex ids: 0..n real, n.. virtual.
    let mut vlayer = layer.clone();
    let mut chains: Vec<Vec<usize>> = Vec::with_capacity(fc.edges.len());
    for d in &dag {
        let Some((u, v)) = *d else {
            chains.push(Vec::new());
            continue;
        };
        let mut chain = vec![u];
        for l in layer[u] + 1..layer[v] {
            if vlayer.len() - n >= MAX_DUMMIES {
                return Err(Error::TooLarge);
            }
            vlayer.push(l);
            chain.push(vlayer.len() - 1);
        }
        chain.push(v);
        chains.push(chain);
    }
    let total = vlayer.len();
    let mut up: Vec<Vec<usize>> = vec![Vec::new(); total];
    let mut down: Vec<Vec<usize>> = vec![Vec::new(); total];
    for ch in &chains {
        for w in ch.windows(2) {
            down[w[0]].push(w[1]);
            up[w[1]].push(w[0]);
        }
    }

    // 4. Ordering: first appearance, then barycenter sweeps.
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
    for pass in 0..ORDER_PASSES {
        let downward = pass.is_multiple_of(2);
        let order: Vec<usize> = if downward {
            (1..layers).collect()
        } else {
            (0..layers.saturating_sub(1)).rev().collect()
        };
        for l in order {
            let nbrs = if downward { &up } else { &down };
            let mut keyed: Vec<(f32, usize)> = rows[l]
                .iter()
                .map(|&v| {
                    let ns = &nbrs[v];
                    let key = if ns.is_empty() {
                        pos[v]
                    } else {
                        ns.iter().map(|&u| pos[u]).sum::<f32>() / ns.len() as f32
                    };
                    (key, v)
                })
                .collect();
            keyed.sort_by(|a, b| a.0.total_cmp(&b.0));
            rows[l] = keyed.into_iter().map(|(_, v)| v).collect();
            for (i, &v) in rows[l].iter().enumerate() {
                pos[v] = i as f32;
            }
        }
    }

    // 5. Coordinates. "Cross" is along a layer, "main" across layers.
    let cross = |v: usize| -> f32 {
        if v < n {
            if horizontal { sizes[v].1 } else { sizes[v].0 }
        } else {
            sp.node_gap * 0.25
        }
    };
    let main = |v: usize| -> f32 {
        if v < n {
            if horizontal { sizes[v].0 } else { sizes[v].1 }
        } else {
            0.0
        }
    };
    // Initial packing, then pull nodes towards their neighbours' mean
    // without letting them overlap (left-to-right sweep).
    let mut cpos = vec![0f32; total];
    for r in &rows {
        let mut at = 0.0;
        for &v in r {
            cpos[v] = at + cross(v) / 2.0;
            at += cross(v) + sp.node_gap;
        }
    }
    for pass in 0..4usize {
        let downward = pass.is_multiple_of(2);
        let order: Vec<usize> = if downward {
            (1..layers).collect()
        } else {
            (0..layers.saturating_sub(1)).rev().collect()
        };
        let nbrs = if downward { &up } else { &down };
        for l in order {
            let r = &rows[l];
            let want: Vec<f32> = r
                .iter()
                .map(|&v| {
                    let ns = &nbrs[v];
                    if ns.is_empty() {
                        cpos[v]
                    } else {
                        ns.iter().map(|&u| cpos[u]).sum::<f32>() / ns.len() as f32
                    }
                })
                .collect();
            let mut prev_right = f32::NEG_INFINITY;
            for (i, &v) in r.iter().enumerate() {
                let half = cross(v) / 2.0;
                let x = want[i].max(prev_right + sp.node_gap + half);
                cpos[v] = x;
                prev_right = x + half;
            }
        }
    }
    let min_c = (0..total)
        .map(|v| cpos[v] - cross(v) / 2.0)
        .fold(f32::INFINITY, f32::min);
    let min_c = if min_c.is_finite() { min_c } else { 0.0 };
    for c in &mut cpos {
        *c -= min_c;
    }
    let extent_c = (0..total)
        .map(|v| cpos[v] + cross(v) / 2.0)
        .fold(0f32, f32::max);
    // Layer thickness and gaps (room for edge labels between layers).
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
    let mut mstart = vec![0f32; layers];
    let mut at = 0.0;
    for l in 0..layers {
        mstart[l] = at;
        at += thick[l] + gap[l];
    }
    let extent_m = if layers == 0 {
        0.0
    } else {
        at - gap[layers - 1]
    };
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
    let pt = |v: usize| place(mpos(v), cpos[v]);
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

    // 6. Edge paths, clipped to node outlines, in original direction.
    let mut edges = Vec::with_capacity(fc.edges.len());
    for (i, e) in fc.edges.iter().enumerate() {
        let mut points: Vec<(f32, f32)> = if e.from == e.to {
            self_loop(&nodes[e.from], fc.direction)
        } else {
            let mut ps: Vec<(f32, f32)> = chains[i].iter().map(|&v| pt(v)).collect();
            if reversed[i] {
                ps.reverse();
            }
            let last = ps.len() - 1;
            ps[0] = clip(&nodes[e.from], fc.nodes[e.from].shape, ps[1]);
            ps[last] = clip(&nodes[e.to], fc.nodes[e.to].shape, ps[last - 1]);
            ps
        };
        points.dedup();
        let label_at = e.label.as_ref().map(|_| midpoint(&points));
        edges.push(EdgePath { points, label_at });
    }
    Ok(Layout {
        width,
        height,
        nodes,
        edges,
    })
}

/// A small loop on the side of the node facing away from the flow.
fn self_loop(b: &NodeBox, dir: Direction) -> Vec<(f32, f32)> {
    let r = b.w.min(b.h) * 0.35;
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
        Shape::Circle => hw / (dx * dx + dy * dy).sqrt(),
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
fn midpoint(ps: &[(f32, f32)]) -> (f32, f32) {
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

    fn measure(s: &str) -> (f32, f32) {
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
        for src in [
            "sequenceDiagram\nA->>B: hi",
            "classDiagram\nA <|-- B",
            "pie\n\"a\": 1",
            "gantt",
            "",
            "%% only a comment",
        ] {
            assert_eq!(parse(src), Err(Error::UnsupportedType), "{src:?}");
        }
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
    fn edges_and_edge_labels() {
        let fc = parse(
            "graph LR\nA-->B\nB --- C\nC-.->D\nD-.-E\nE==>F\nF===G\nA -->|yes| C\nA -- no --> D\nB -. maybe .-> E\nC == big ==> F\nG ~~~ A",
        )
        .expect("parses");
        let e: Vec<_> = fc
            .edges
            .iter()
            .map(|e| {
                (
                    fc.nodes[e.from].id.as_str(),
                    fc.nodes[e.to].id.as_str(),
                    e.stroke,
                    e.arrow,
                    e.label.as_deref(),
                )
            })
            .collect();
        assert_eq!(
            e,
            [
                ("A", "B", Stroke::Solid, true, None),
                ("B", "C", Stroke::Solid, false, None),
                ("C", "D", Stroke::Dotted, true, None),
                ("D", "E", Stroke::Dotted, false, None),
                ("E", "F", Stroke::Thick, true, None),
                ("F", "G", Stroke::Thick, false, None),
                ("A", "C", Stroke::Solid, true, Some("yes")),
                ("A", "D", Stroke::Solid, true, Some("no")),
                ("B", "E", Stroke::Dotted, true, Some("maybe")),
                ("C", "F", Stroke::Thick, true, Some("big")),
                ("G", "A", Stroke::Invisible, false, None),
            ]
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
            ("graph TD\nsubgraph one\nA\nend", 2),
            ("graph TD\nA --> B\nA <--> B", 3),
            ("graph TD\nA --o B", 2),
            ("graph TD\nA{{hex}}", 2),
            ("graph TD\nA[/slant/]", 2),
            ("graph TD\nA>flag]", 2),
            ("graph TD\nA[unclosed", 2),
            ("graph TD\nA -->", 2),
            ("graph TD\nA -- text without close B", 2),
            ("graph TD\n--> B", 2),
            ("graph TD\nA[\"open quote]", 2),
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
    }

    fn overlaps(a: &NodeBox, b: &NodeBox) -> bool {
        (a.x - b.x).abs() < (a.w + b.w) / 2.0 && (a.y - b.y).abs() < (a.h + b.h) / 2.0
    }

    fn assert_no_overlap(l: &Layout) {
        for (i, a) in l.nodes.iter().enumerate() {
            for b in &l.nodes[i + 1..] {
                assert!(!overlaps(a, b), "{a:?} overlaps {b:?}");
            }
            assert!(a.x - a.w / 2.0 >= -0.01 && a.x + a.w / 2.0 <= l.width + 0.01);
            assert!(a.y - a.h / 2.0 >= -0.01 && a.y + a.h / 2.0 <= l.height + 0.01);
        }
    }

    #[test]
    fn layered_layout_follows_direction() {
        let (fc, l) = lay("graph TD\nA --> B --> C\nA --> C");
        let y = |id| l.nodes[fc.nodes.iter().position(|n| n.id == id).expect("id")].y;
        assert!(y("A") < y("B") && y("B") < y("C"));
        assert_no_overlap(&l);
        let (fc, l) = lay("graph BT\nA --> B");
        assert!(l.nodes[0].y > l.nodes[1].y, "{fc:?}");
        let (_, l) = lay("graph LR\nA --> B");
        assert!(l.nodes[0].x < l.nodes[1].x);
        let (_, l) = lay("graph RL\nA --> B");
        assert!(l.nodes[0].x > l.nodes[1].x);
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
        let ops = ["-->", "---", "-.->", "==>", "-->|l|", "-- t -->", "~~~"];
        let shapes = [
            ("[", "]"),
            ("(", ")"),
            ("{", "}"),
            ("((", "))"),
            ("([", "])"),
        ];
        for round in 0..300 {
            let dir = ["TD", "BT", "LR", "RL"][round % 4];
            let mut src = format!("flowchart {dir}\n");
            let nodes = 1 + next(25);
            for _ in 0..next(60) {
                let (a, b) = (next(nodes), next(nodes));
                let (o, c) = shapes[next(5) as usize];
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
            "A", "B", "c_1", "-->", "---", "-.->", "-.-", "==>", "===", "--", "-.", "==", "|", "[",
            "]", "(", ")", "{", "}", "((", "))", "([", "])", "\"", "&", ";", "\n", " ", "%%",
            ":::", "<br>", "<", ">", "~~~", "ä", "🙂", "subgraph", "click", "-", ".", "x",
        ];
        let mut s: u64 = 0x9E37_79B9_7F4A_7C15;
        for _ in 0..3000 {
            let mut src = String::from(if s.is_multiple_of(2) {
                "graph TD\n"
            } else {
                "flowchart LR\n"
            });
            let len = (s % 40) as usize;
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
                }
            }
            s = s.wrapping_add(0x2545_F491_4F6C_DD1D);
        }
    }
}
