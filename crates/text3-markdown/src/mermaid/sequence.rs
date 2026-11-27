//! Mermaid `sequenceDiagram`: participants and actors (with `as`
//! aliases), messages with eight arrow kinds, self-messages, notes,
//! `autonumber`, activations (`activate`/`deactivate` and `+`/`-`) and
//! `loop`/`alt`/`else`/`opt`/`par`/`and`/`critical`/`option`/`break`/
//! `rect` blocks (`rect` colours are ignored; theme colours are used).
//! Anything else (`box`, `create`, `destroy`, `link`, `title`, ...) makes
//! the whole diagram [`Error::Unsupported`], so the source is shown.

use super::{Error, MAX_SOURCE_BYTES, PResult, Spacing, clean_label, header_line, strip_comment};

pub const MAX_PARTICIPANTS: usize = 50;
/// Messages and notes together.
pub const MAX_MESSAGES: usize = 500;
/// Blocks (`loop`, `alt`, ...) nested deeper than this are not drawn.
pub const MAX_NESTING: usize = 8;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Participant {
    pub id: String,
    pub label: String,
    /// `actor`: drawn as a stick figure.
    pub actor: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Line {
    Solid,
    Dotted,
}

/// The end of a message line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tip {
    /// `->`, `-->`
    None,
    /// `->>`, `-->>`
    Arrow,
    /// `-x`, `--x`
    Cross,
    /// `-)`, `--)`: an open (async) arrow.
    Open,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockKind {
    Loop,
    Alt,
    Opt,
    Par,
    Critical,
    Break,
    Rect,
}

impl BlockKind {
    /// The Mermaid keyword, shown in the frame's tab (it is syntax, not
    /// translated text).
    pub fn keyword(self) -> &'static str {
        match self {
            Self::Loop => "loop",
            Self::Alt => "alt",
            Self::Opt => "opt",
            Self::Par => "par",
            Self::Critical => "critical",
            Self::Break => "break",
            Self::Rect => "rect",
        }
    }
    fn from_keyword(k: &str) -> Option<Self> {
        Some(match k {
            "loop" => Self::Loop,
            "alt" => Self::Alt,
            "opt" => Self::Opt,
            "par" => Self::Par,
            "critical" => Self::Critical,
            "break" => Self::Break,
            "rect" => Self::Rect,
            _ => return None,
        })
    }
    /// Keyword that starts another section of this block.
    fn branch(self) -> Option<&'static str> {
        match self {
            Self::Alt => Some("else"),
            Self::Par => Some("and"),
            Self::Critical => Some("option"),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotePlace {
    LeftOf(usize),
    RightOf(usize),
    /// Over one participant (both the same) or spanning two.
    Over(usize, usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    Message {
        from: usize,
        to: usize,
        text: String,
        line: Line,
        tip: Tip,
        /// `+`: activates `to`.
        activate: bool,
        /// `-`: deactivates `from`.
        deactivate: bool,
        /// Set by `autonumber`.
        number: Option<u32>,
    },
    Note {
        place: NotePlace,
        text: String,
    },
    Activate(usize),
    Deactivate(usize),
    Start {
        kind: BlockKind,
        label: String,
    },
    /// `else`, `and` or `option` inside the open block.
    Branch {
        label: String,
    },
    End,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sequence {
    pub participants: Vec<Participant>,
    pub items: Vec<Item>,
}

/// Message operators, longest first where one is a prefix of another.
const ARROWS: &[(&str, Line, Tip)] = &[
    ("-->>", Line::Dotted, Tip::Arrow),
    ("->>", Line::Solid, Tip::Arrow),
    ("--x", Line::Dotted, Tip::Cross),
    ("--)", Line::Dotted, Tip::Open),
    ("-->", Line::Dotted, Tip::None),
    ("->", Line::Solid, Tip::None),
    ("-x", Line::Solid, Tip::Cross),
    ("-)", Line::Solid, Tip::Open),
];

/// Parse a `sequenceDiagram` block.
pub fn parse(src: &str) -> Result<Sequence, Error> {
    if src.len() > MAX_SOURCE_BYTES {
        return Err(Error::TooLarge);
    }
    let Some((start, head)) = header_line(src) else {
        return Err(Error::UnsupportedType);
    };
    let mut words = head.trim_end_matches(';').split_whitespace();
    if words.next() != Some("sequenceDiagram") {
        return Err(Error::UnsupportedType);
    }
    if words.next().is_some() {
        return Err(Error::Unsupported { line: start + 1 });
    }
    let mut p = Parser::default();
    for (i, raw) in src.lines().enumerate().skip(start + 1) {
        let line = i + 1;
        let text = strip_comment(raw).trim().trim_end_matches(';').trim();
        if text.is_empty() {
            continue;
        }
        p.line(text, line)
            .map_err(|e| e.unwrap_or(Error::Unsupported { line }))?;
    }
    if let Some(&(_, line)) = p.blocks.last() {
        return Err(Error::Unsupported { line });
    }
    Ok(Sequence {
        participants: p.participants,
        items: p.items,
    })
}

#[derive(Default)]
struct Parser {
    participants: Vec<Participant>,
    items: Vec<Item>,
    /// Open blocks with the line that opened them.
    blocks: Vec<(BlockKind, usize)>,
    /// Activation depth per participant.
    active: Vec<usize>,
    /// Next number and step while `autonumber` is on.
    auto: Option<(u32, u32)>,
    shown: usize,
}

/// Participant ids: no spaces, punctuation used by the syntax, or arrows.
fn valid_id(s: &str) -> bool {
    !s.is_empty()
        && !s.contains(|c: char| c.is_whitespace() || ":,;<>+\"()[]{}|#&".contains(c))
        && !s.contains("->")
        && !s.contains("--")
        && !s.ends_with('-')
}

impl Parser {
    fn participant(&mut self, id: &str) -> PResult<usize> {
        if !valid_id(id) {
            return Err(None);
        }
        if let Some(i) = self.participants.iter().position(|p| p.id == id) {
            return Ok(i);
        }
        if self.participants.len() >= MAX_PARTICIPANTS {
            return Err(Some(Error::TooLarge));
        }
        self.participants.push(Participant {
            id: id.to_owned(),
            label: super::truncate(id),
            actor: false,
        });
        self.active.push(0);
        Ok(self.participants.len() - 1)
    }

    fn count(&mut self) -> PResult<()> {
        self.shown += 1;
        if self.shown > MAX_MESSAGES {
            return Err(Some(Error::TooLarge));
        }
        Ok(())
    }

    fn line(&mut self, text: &str, line: usize) -> PResult<()> {
        let (kw, rest) = match text.split_once(char::is_whitespace) {
            Some((k, r)) => (k, r.trim()),
            None => (text, ""),
        };
        match kw {
            "participant" | "actor" => self.declare(rest, kw == "actor"),
            _ if kw.eq_ignore_ascii_case("note") => self.note(rest),
            "autonumber" => {
                let nums: Vec<&str> = rest.split_whitespace().collect();
                self.auto = match nums.as_slice() {
                    [] => Some((1, 1)),
                    ["off"] => None,
                    [n] => Some((n.parse().map_err(|_| None)?, 1)),
                    [n, s] => Some((n.parse().map_err(|_| None)?, s.parse().map_err(|_| None)?)),
                    _ => return Err(None),
                };
                Ok(())
            }
            "activate" => {
                let p = self.participant(rest)?;
                self.active[p] += 1;
                self.items.push(Item::Activate(p));
                Ok(())
            }
            "deactivate" => {
                let p = self.participant(rest)?;
                if self.active[p] == 0 {
                    return Err(None);
                }
                self.active[p] -= 1;
                self.items.push(Item::Deactivate(p));
                Ok(())
            }
            "end" if rest.is_empty() => {
                self.blocks.pop().ok_or(None)?;
                self.items.push(Item::End);
                Ok(())
            }
            _ => {
                if let Some(kind) = BlockKind::from_keyword(kw) {
                    if self.blocks.len() >= MAX_NESTING {
                        return Err(Some(Error::TooLarge));
                    }
                    // `rect rgb(...)`: the colour is ignored.
                    let label = if kind == BlockKind::Rect {
                        String::new()
                    } else {
                        clean_label(rest)?
                    };
                    self.blocks.push((kind, line));
                    self.items.push(Item::Start { kind, label });
                    return Ok(());
                }
                if matches!(kw, "else" | "and" | "option") {
                    let open = self.blocks.last().and_then(|b| b.0.branch());
                    if open != Some(kw) {
                        return Err(None);
                    }
                    let label = clean_label(rest)?;
                    self.items.push(Item::Branch { label });
                    return Ok(());
                }
                self.message(text)
            }
        }
    }

    /// `participant A`, `actor A as Some label`.
    fn declare(&mut self, rest: &str, actor: bool) -> PResult<()> {
        let (id, alias) = match rest.split_once(char::is_whitespace) {
            Some((id, more)) => {
                let more = more.trim_start();
                let label = more
                    .strip_prefix("as")
                    .filter(|l| l.starts_with(char::is_whitespace));
                (id, Some(label.ok_or(None)?))
            }
            None => (rest, None),
        };
        let p = self.participant(id)?;
        if let Some(a) = alias {
            let label = clean_label(a)?;
            if label.is_empty() {
                return Err(None);
            }
            self.participants[p].label = label;
        }
        self.participants[p].actor = actor;
        Ok(())
    }

    /// `left of A: text`, `right of A: text`, `over A: text`, `over A,B: text`.
    fn note(&mut self, rest: &str) -> PResult<()> {
        let (place, text) = rest.split_once(':').ok_or(None)?;
        let words: Vec<&str> = place.split_whitespace().collect();
        let place = match words.as_slice() {
            ["left", "of", id] => NotePlace::LeftOf(self.participant(id)?),
            ["right", "of", id] => NotePlace::RightOf(self.participant(id)?),
            ["over", ..] => {
                let ids = place.trim_start()["over".len()..].trim();
                match ids.split_once(',') {
                    Some((a, b)) => {
                        NotePlace::Over(self.participant(a.trim())?, self.participant(b.trim())?)
                    }
                    None => {
                        let a = self.participant(ids)?;
                        NotePlace::Over(a, a)
                    }
                }
            }
            _ => return Err(None),
        };
        let text = clean_label(text)?;
        self.count()?;
        self.items.push(Item::Note { place, text });
        Ok(())
    }

    /// `A->>B: text`, with `+`/`-` before the target. The first arrow
    /// position that leaves two valid ids wins (ids may hold `-`).
    fn message(&mut self, text: &str) -> PResult<()> {
        let (head, body) = match text.split_once(':') {
            Some((h, b)) => (h, Some(b)),
            None => (text, None),
        };
        for (i, _) in head.char_indices().filter(|&(_, c)| c == '-') {
            for &(op, line, tip) in ARROWS {
                if !head[i..].starts_with(op) {
                    continue;
                }
                let from = head[..i].trim();
                let mut to = head[i + op.len()..].trim();
                let (mut activate, mut deactivate) = (false, false);
                if let Some(t) = to.strip_prefix('+') {
                    (activate, to) = (true, t.trim_start());
                } else if let Some(t) = to.strip_prefix('-') {
                    (deactivate, to) = (true, t.trim_start());
                }
                if !valid_id(from) || !valid_id(to) {
                    continue;
                }
                let text = clean_label(body.unwrap_or(""))?;
                let from = self.participant(from)?;
                let to = self.participant(to)?;
                if deactivate {
                    if self.active[from] == 0 {
                        return Err(None);
                    }
                    self.active[from] -= 1;
                }
                if activate {
                    self.active[to] += 1;
                }
                self.count()?;
                let number = self.auto.map(|(n, _)| n);
                if let Some((n, s)) = self.auto {
                    self.auto = Some((n.saturating_add(s), s));
                }
                self.items.push(Item::Message {
                    from,
                    to,
                    text,
                    line,
                    tip,
                    activate,
                    deactivate,
                    number,
                });
                return Ok(());
            }
        }
        Err(None)
    }
}

// ---------------------------------------------------------------- layout

/// Axis-aligned box: top-left corner and size.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MessagePath {
    /// Index in `Sequence::items`.
    pub item: usize,
    /// Start and end of the line (equal for a self-message).
    pub x1: f32,
    pub x2: f32,
    pub y: f32,
    /// Self-message loop: width to the right and height.
    pub loop_size: Option<(f32, f32)>,
    /// Centre of the message text.
    pub text_at: (f32, f32),
    /// Centre of the `autonumber` badge.
    pub number_at: Option<(f32, f32)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NoteBox {
    pub item: usize,
    pub rect: Rect,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FrameBox {
    /// Index of the block's `Start` item.
    pub item: usize,
    pub kind: BlockKind,
    pub label: String,
    pub rect: Rect,
    /// The keyword tab in the top-left corner.
    pub tab: Rect,
    /// Section separators (`else`, `and`, `option`): y and label.
    pub branches: Vec<(f32, String)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SeqLayout {
    pub width: f32,
    pub height: f32,
    /// Participant boxes at the top, same order as `participants`.
    pub heads: Vec<Rect>,
    /// The same boxes repeated at the bottom.
    pub feet: Vec<Rect>,
    /// Vertical extent of the lifelines.
    pub lifeline: (f32, f32),
    /// One per message, in item order.
    pub messages: Vec<MessagePath>,
    pub notes: Vec<NoteBox>,
    /// Innermost blocks first.
    pub frames: Vec<FrameBox>,
    pub activations: Vec<Rect>,
}

impl SeqLayout {
    /// Lifeline x of participant `i`.
    pub fn center(&self, i: usize) -> f32 {
        self.heads[i].x + self.heads[i].w / 2.0
    }
}

/// Frame or section label as drawn: `[label]`, or nothing.
pub fn bracketed(label: &str) -> String {
    if label.is_empty() {
        String::new()
    } else {
        format!("[{label}]")
    }
}

struct Open {
    item: usize,
    kind: BlockKind,
    label: String,
    y0: f32,
    lo: f32,
    hi: f32,
    min_w: f32,
    branches: Vec<(f32, String)>,
}

/// Lay out `seq`. `measure` returns the (width, height) of a text (which
/// may hold `\n`); empty text is (0, 0).
pub fn layout(
    seq: &Sequence,
    sp: Spacing,
    measure: &mut dyn FnMut(&str) -> (f32, f32),
) -> SeqLayout {
    let mut size = |s: &str| if s.is_empty() { (0.0, 0.0) } else { measure(s) };
    let pad = sp.pad;
    let line_h = size("Ag").1.max(1.0);
    let bar_w = pad * 1.2;
    let loop_w = sp.node_gap * 1.4;
    let loop_h = line_h * 1.2;
    let row_gap = pad * 1.2;
    let n = seq.participants.len();
    let boxes: Vec<(f32, f32)> = seq
        .participants
        .iter()
        .map(|p| {
            let (tw, th) = size(&p.label);
            if p.actor {
                ((tw + pad).max(line_h * 1.6), th + line_h * 2.4 + pad * 0.5)
            } else {
                (tw + 2.0 * pad, th + pad)
            }
        })
        .collect();
    let texts: Vec<(f32, f32)> = seq
        .items
        .iter()
        .map(|it| match it {
            Item::Message { text, .. } | Item::Note { text, .. } => size(text),
            Item::Start { kind, label } => {
                let k = size(kind.keyword());
                let l = size(&bracketed(label));
                (k.0 + pad * 1.6 + l.0, k.1.max(l.1))
            }
            Item::Branch { label } => size(&bracketed(label)),
            _ => (0.0, 0.0),
        })
        .collect();

    // Columns: each lifeline as far left as the boxes and texts between
    // it and the lifelines to its left allow.
    let mut need: Vec<Vec<(usize, f32)>> = vec![Vec::new(); n];
    for (it, &(tw, _)) in seq.items.iter().zip(&texts) {
        match *it {
            Item::Message { from, to, .. } if from != to => {
                let (lo, hi) = (from.min(to), from.max(to));
                need[hi].push((lo, tw + 2.0 * pad + bar_w));
            }
            Item::Message { from, .. } if from + 1 < n => {
                let w = loop_w.max(pad * 0.5 + line_h * 0.5 + tw) + pad + bar_w;
                need[from + 1].push((from, w));
            }
            Item::Note {
                place: NotePlace::RightOf(i),
                ..
            } if i + 1 < n => need[i + 1].push((i, tw + 2.0 * pad + pad * 1.5)),
            Item::Note {
                place: NotePlace::LeftOf(i),
                ..
            } if i > 0 => need[i].push((i - 1, tw + 2.0 * pad + pad * 1.5)),
            _ => {}
        }
    }
    let mut xs = vec![0f32; n];
    for j in 0..n {
        let mut x = boxes[j].0 / 2.0;
        if j > 0 {
            x = x.max(xs[j - 1] + boxes[j - 1].0 / 2.0 + boxes[j].0 / 2.0 + sp.node_gap);
        }
        for &(i, d) in &need[j] {
            x = x.max(xs[i] + d);
        }
        xs[j] = x;
    }

    let head_h = boxes.iter().map(|b| b.1).fold(0f32, f32::max);
    let heads: Vec<Rect> = (0..n)
        .map(|i| Rect {
            x: xs[i] - boxes[i].0 / 2.0,
            y: head_h - boxes[i].1,
            w: boxes[i].0,
            h: boxes[i].1,
        })
        .collect();
    let mut lo_all = heads.iter().map(|r| r.x).fold(0f32, f32::min);
    let mut hi_all = heads.iter().map(|r| r.x + r.w).fold(0f32, f32::max);
    let mut y = head_h + pad * 1.5;
    let mut last_line = y;
    let mut active: Vec<Vec<f32>> = vec![Vec::new(); n];
    let mut open: Vec<Open> = Vec::new();
    let mut messages = Vec::new();
    let mut notes = Vec::new();
    let mut frames = Vec::new();
    let mut activations = Vec::new();
    let mut extend = |open: &mut Vec<Open>, lo: f32, hi: f32| {
        if let Some(top) = open.last_mut() {
            top.lo = top.lo.min(lo);
            top.hi = top.hi.max(hi);
        }
        lo_all = lo_all.min(lo);
        hi_all = hi_all.max(hi);
    };
    let bar = |x: f32, level: usize, y0: f32, y1: f32| Rect {
        x: x + level as f32 * bar_w / 2.0 - bar_w / 2.0,
        y: y0,
        w: bar_w,
        h: (y1 - y0).max(pad),
    };
    // Where a line meets the lifeline or the top activation bar.
    let attach = |x: f32, depth: usize, toward: f32| {
        if depth == 0 {
            x
        } else {
            x + (depth - 1) as f32 * bar_w / 2.0 + toward * bar_w / 2.0
        }
    };
    for (idx, (it, &(tw, th))) in seq.items.iter().zip(&texts).enumerate() {
        match it {
            &Item::Message {
                from,
                to,
                activate,
                deactivate,
                number,
                ..
            } => {
                let line_y = y + th + pad * 0.4;
                if activate {
                    active[to].push(line_y);
                }
                let (x1, x2, text_at, loop_size, end_y) = if from != to {
                    let s = if xs[to] >= xs[from] { 1.0 } else { -1.0 };
                    let x1 = attach(xs[from], active[from].len(), s);
                    let x2 = attach(xs[to], active[to].len(), -s);
                    let mid = (x1 + x2) / 2.0;
                    extend(
                        &mut open,
                        x1.min(x2).min(mid - tw / 2.0),
                        x1.max(x2).max(mid + tw / 2.0),
                    );
                    (x1, x2, (mid, y + th / 2.0), None, line_y)
                } else {
                    let x1 = attach(xs[from], active[from].len(), 1.0);
                    // Clear of the autonumber badge.
                    let lead = pad * 0.5 + if number.is_some() { line_h * 0.5 } else { 0.0 };
                    let right = x1 + loop_w.max(lead + tw);
                    extend(&mut open, x1, right);
                    let text_at = (x1 + lead + tw / 2.0, y + th / 2.0);
                    (x1, x1, text_at, Some((loop_w, loop_h)), line_y + loop_h)
                };
                if deactivate && let Some(y0) = active[from].pop() {
                    activations.push(bar(xs[from], active[from].len(), y0, end_y));
                }
                messages.push(MessagePath {
                    item: idx,
                    x1,
                    x2,
                    y: line_y,
                    loop_size,
                    text_at,
                    number_at: number.map(|_| (x1, line_y)),
                });
                last_line = end_y;
                y = end_y + row_gap;
            }
            &Item::Note { place, .. } => {
                let w = tw + 2.0 * pad;
                let h = th + pad;
                let gap = pad * 0.5;
                let x = match place {
                    NotePlace::LeftOf(i) => xs[i] - gap - w,
                    NotePlace::RightOf(i) => xs[i] + gap,
                    NotePlace::Over(a, b) => {
                        let (l, r) = (xs[a].min(xs[b]), xs[a].max(xs[b]));
                        let span = (r - l + 2.0 * pad).max(w);
                        (l + r) / 2.0 - span / 2.0
                    }
                };
                let w = match place {
                    NotePlace::Over(a, b) => {
                        (xs[a].max(xs[b]) - xs[a].min(xs[b]) + 2.0 * pad).max(w)
                    }
                    _ => w,
                };
                let rect = Rect { x, y, w, h };
                extend(&mut open, x, x + w);
                notes.push(NoteBox { item: idx, rect });
                y += h + row_gap;
            }
            &Item::Activate(p) => active[p].push(last_line),
            &Item::Deactivate(p) => {
                if let Some(y0) = active[p].pop() {
                    activations.push(bar(xs[p], active[p].len(), y0, last_line));
                }
            }
            Item::Start { kind, label } => {
                open.push(Open {
                    item: idx,
                    kind: *kind,
                    label: label.clone(),
                    y0: y,
                    lo: f32::INFINITY,
                    hi: f32::NEG_INFINITY,
                    min_w: tw + 2.0 * pad,
                    branches: Vec::new(),
                });
                y += th + pad * 1.2;
            }
            Item::Branch { label } => {
                if let Some(top) = open.last_mut() {
                    top.branches.push((y, label.clone()));
                    top.min_w = top.min_w.max(tw + 2.0 * pad);
                }
                y += th + pad * 1.2;
            }
            Item::End => {
                let Some(f) = open.pop() else { continue };
                y += pad * 0.4;
                let (lo, hi) = if f.lo <= f.hi {
                    (f.lo - pad, f.hi + pad)
                } else {
                    let x = xs.first().copied().unwrap_or(0.0);
                    (x - pad, x + pad)
                };
                let w = (hi - lo).max(f.min_w);
                let rect = Rect {
                    x: lo,
                    y: f.y0,
                    w,
                    h: y - f.y0,
                };
                let (kw, kh) = size(f.kind.keyword());
                let tab = Rect {
                    x: lo,
                    y: f.y0,
                    w: kw + pad * 1.2,
                    h: kh + pad * 0.5,
                };
                extend(&mut open, lo, lo + w);
                frames.push(FrameBox {
                    item: f.item,
                    kind: f.kind,
                    label: f.label,
                    rect,
                    tab,
                    branches: f.branches,
                });
                y += row_gap;
            }
        }
    }
    let y_end = y + pad * 0.5;
    for (p, stack) in active.iter().enumerate() {
        for (level, &y0) in stack.iter().enumerate() {
            activations.push(bar(xs[p], level, y0, y_end));
        }
    }
    for r in &activations {
        lo_all = lo_all.min(r.x);
        hi_all = hi_all.max(r.x + r.w);
    }
    let feet: Vec<Rect> = heads.iter().map(|r| Rect { y: y_end, ..*r }).collect();
    // Shift everything right of zero.
    let dx = -lo_all;
    let shift = |r: &mut Rect| r.x += dx;
    let mut out = SeqLayout {
        width: hi_all - lo_all,
        height: y_end + head_h,
        heads,
        feet,
        lifeline: (head_h, y_end),
        messages,
        notes,
        frames,
        activations,
    };
    out.heads.iter_mut().for_each(shift);
    out.feet.iter_mut().for_each(shift);
    out.activations.iter_mut().for_each(shift);
    for nb in &mut out.notes {
        shift(&mut nb.rect);
    }
    for f in &mut out.frames {
        shift(&mut f.rect);
        shift(&mut f.tab);
    }
    for m in &mut out.messages {
        m.x1 += dx;
        m.x2 += dx;
        m.text_at.0 += dx;
        if let Some(p) = &mut m.number_at {
            p.0 += dx;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::super::tests::measure;
    use super::*;

    fn seq(src: &str) -> Sequence {
        parse(src).unwrap_or_else(|e| panic!("{e:?} in {src}"))
    }

    fn lay(src: &str) -> (Sequence, SeqLayout) {
        let s = seq(src);
        let l = layout(&s, Spacing::for_font(14.0), &mut measure);
        (s, l)
    }

    fn msgs(s: &Sequence) -> Vec<(usize, usize, &str, Line, Tip)> {
        s.items
            .iter()
            .filter_map(|it| match it {
                Item::Message {
                    from,
                    to,
                    text,
                    line,
                    tip,
                    ..
                } => Some((*from, *to, text.as_str(), *line, *tip)),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn participants_actors_and_aliases() {
        let s = seq(
            "sequenceDiagram\nparticipant A as Alice<br>Smith\nactor B as Bob\nparticipant web-server\nA->>C: hi\nactor A",
        );
        let p: Vec<_> = s
            .participants
            .iter()
            .map(|p| (p.id.as_str(), p.label.as_str(), p.actor))
            .collect();
        assert_eq!(
            p,
            [
                ("A", "Alice\nSmith", true),
                ("B", "Bob", true),
                ("web-server", "web-server", false),
                ("C", "C", false),
            ]
        );
        for bad in [
            "participant",
            "participant A B",
            "participant A as",
            "participant A:B",
            "actor A as <b>x</b>",
        ] {
            assert_eq!(
                parse(&format!("sequenceDiagram\n{bad}")),
                Err(Error::Unsupported { line: 2 }),
                "{bad}"
            );
        }
    }

    #[test]
    fn message_arrows() {
        let s = seq(
            "sequenceDiagram\nA->B: solid\nA-->B: dotted\nA->>B: arrow\nA-->>B: dotted arrow\nA-xB: cross\nA--xB: dotted cross\nA-)B: async\nA--)B: dotted async\nB->>B: self\nweb-xray->>db: hyphen ids\nA ->> B :  spaced  \nA->>B",
        );
        assert_eq!(
            msgs(&s),
            [
                (0, 1, "solid", Line::Solid, Tip::None),
                (0, 1, "dotted", Line::Dotted, Tip::None),
                (0, 1, "arrow", Line::Solid, Tip::Arrow),
                (0, 1, "dotted arrow", Line::Dotted, Tip::Arrow),
                (0, 1, "cross", Line::Solid, Tip::Cross),
                (0, 1, "dotted cross", Line::Dotted, Tip::Cross),
                (0, 1, "async", Line::Solid, Tip::Open),
                (0, 1, "dotted async", Line::Dotted, Tip::Open),
                (1, 1, "self", Line::Solid, Tip::Arrow),
                (2, 3, "hyphen ids", Line::Solid, Tip::Arrow),
                (0, 1, "spaced", Line::Solid, Tip::Arrow),
                (0, 1, "", Line::Solid, Tip::Arrow),
            ]
        );
        assert_eq!(s.participants[2].id, "web-xray");
        for bad in [
            "A<<->>B: both ways",
            "A->>: no target",
            "->>B: no source",
            "A=>B: x",
            "A->>B: <i>x</i>",
            "box Aqua Group",
            "title Hello",
            "create participant C",
        ] {
            assert_eq!(
                parse(&format!("sequenceDiagram\nA->>B: ok\n{bad}")),
                Err(Error::Unsupported { line: 3 }),
                "{bad}"
            );
        }
    }

    #[test]
    fn notes_autonumber_and_activations() {
        let s = seq(
            "sequenceDiagram\nautonumber\nA->>+B: one\nnote left of A: L\nNote right of B: R\nNote over A: O\nNote over A, B: span\nB-->>-A: two\nautonumber 10 5\nA->>B: ten\nA->>B: fifteen\nautonumber off\nA->>B: none\nactivate A\ndeactivate A",
        );
        let places: Vec<_> = s
            .items
            .iter()
            .filter_map(|it| match it {
                Item::Note { place, text } => Some((*place, text.as_str())),
                _ => None,
            })
            .collect();
        assert_eq!(
            places,
            [
                (NotePlace::LeftOf(0), "L"),
                (NotePlace::RightOf(1), "R"),
                (NotePlace::Over(0, 0), "O"),
                (NotePlace::Over(0, 1), "span"),
            ]
        );
        let nums: Vec<_> = s
            .items
            .iter()
            .filter_map(|it| match it {
                Item::Message {
                    number,
                    activate,
                    deactivate,
                    ..
                } => Some((*number, *activate, *deactivate)),
                _ => None,
            })
            .collect();
        assert_eq!(
            nums,
            [
                (Some(1), true, false),
                (Some(2), false, true),
                (Some(10), false, false),
                (Some(15), false, false),
                (None, false, false),
            ]
        );
        assert!(s.items.contains(&Item::Activate(0)));
        assert!(s.items.contains(&Item::Deactivate(0)));
        for bad in [
            "deactivate A",
            "A->>-B: not active",
            "Note above A: x",
            "Note over A",
            "autonumber x",
        ] {
            assert_eq!(
                parse(&format!("sequenceDiagram\nA->>B: ok\n{bad}")),
                Err(Error::Unsupported { line: 3 }),
                "{bad}"
            );
        }
    }

    #[test]
    fn blocks_and_branches() {
        let s = seq(
            "sequenceDiagram\nloop Every minute\nA->>B: ping\nend\nalt ok\nA->>B: yes\nelse fail\nA->>B: no\nelse\nA->>B: other\nend\nopt maybe\npar one\nA->>B: x\nand two\nA->>B: y\nend\nend\ncritical connect\nA->>B: c\noption timeout\nA->>B: t\nend\nbreak stop\nA->>B: s\nend\nrect rgb(10, 20, 30)\nA->>B: r\nend",
        );
        let kinds: Vec<_> = s
            .items
            .iter()
            .filter_map(|it| match it {
                Item::Start { kind, label } => Some((*kind, label.as_str())),
                _ => None,
            })
            .collect();
        assert_eq!(
            kinds,
            [
                (BlockKind::Loop, "Every minute"),
                (BlockKind::Alt, "ok"),
                (BlockKind::Opt, "maybe"),
                (BlockKind::Par, "one"),
                (BlockKind::Critical, "connect"),
                (BlockKind::Break, "stop"),
                (BlockKind::Rect, ""),
            ]
        );
        let branches = s
            .items
            .iter()
            .filter(|it| matches!(it, Item::Branch { .. }))
            .count();
        assert_eq!(branches, 4);
        for (bad, line) in [
            ("loop x\nA->>B: y", 2),
            ("end", 2),
            ("else x", 2),
            ("loop x\nelse y\nend", 3),
            ("alt x\nand y\nend", 3),
            ("par x\noption y\nend", 3),
        ] {
            assert_eq!(
                parse(&format!("sequenceDiagram\n{bad}")),
                Err(Error::Unsupported { line }),
                "{bad}"
            );
        }
    }

    #[test]
    fn caps() {
        let deep: String = (0..=MAX_NESTING).map(|i| format!("loop l{i}\n")).collect();
        assert_eq!(
            parse(&format!("sequenceDiagram\n{deep}")),
            Err(Error::TooLarge)
        );
        let ok: String = (0..MAX_NESTING).map(|i| format!("loop l{i}\n")).collect();
        let ends = "end\n".repeat(MAX_NESTING);
        assert!(parse(&format!("sequenceDiagram\n{ok}A->>B: x\n{ends}")).is_ok());
        let many: String = (0..=MAX_PARTICIPANTS)
            .map(|i| format!("participant p{i}\n"))
            .collect();
        assert_eq!(
            parse(&format!("sequenceDiagram\n{many}")),
            Err(Error::TooLarge)
        );
        let lots = "A->>B\n".repeat(MAX_MESSAGES + 1);
        assert_eq!(
            parse(&format!("sequenceDiagram\n{lots}")),
            Err(Error::TooLarge)
        );
        assert_eq!(
            parse(&format!(
                "sequenceDiagram\n{}",
                "x".repeat(MAX_SOURCE_BYTES)
            )),
            Err(Error::TooLarge)
        );
        assert_eq!(
            parse("sequenceDiagram extra"),
            Err(Error::Unsupported { line: 1 })
        );
        assert_eq!(parse("graph TD"), Err(Error::UnsupportedType));
    }

    fn assert_sane(s: &Sequence, l: &SeqLayout) {
        assert_eq!(l.heads.len(), s.participants.len());
        let n_msgs = s
            .items
            .iter()
            .filter(|i| matches!(i, Item::Message { .. }))
            .count();
        assert_eq!(l.messages.len(), n_msgs);
        let blocks = s
            .items
            .iter()
            .filter(|i| matches!(i, Item::Start { .. }))
            .count();
        assert_eq!(l.frames.len(), blocks);
        let e = 0.01;
        let within = |r: &Rect| {
            r.x >= -e && r.y >= -e && r.x + r.w <= l.width + e && r.y + r.h <= l.height + e
        };
        for r in l.heads.iter().chain(&l.feet).chain(&l.activations) {
            assert!(within(r), "{r:?} {} x {}", l.width, l.height);
        }
        for nb in &l.notes {
            assert!(within(&nb.rect));
        }
        for f in &l.frames {
            assert!(within(&f.rect));
        }
        // Head boxes do not overlap and keep their order.
        for w in l.heads.windows(2) {
            assert!(w[0].x + w[0].w <= w[1].x + e, "{w:?}");
        }
        // Rows go down the page.
        let mut ys: Vec<f32> = l.messages.iter().map(|m| m.y).collect();
        let sorted = {
            let mut v = ys.clone();
            v.sort_by(f32::total_cmp);
            v
        };
        assert_eq!(ys, sorted);
        ys.clear();
        for m in &l.messages {
            assert!(m.y > l.lifeline.0 && m.y < l.lifeline.1);
        }
    }

    #[test]
    fn layout_rows_columns_frames_and_bars() {
        let (s, l) = lay(
            "sequenceDiagram\nautonumber\nactor U as User\nparticipant S as Server\nparticipant D as Database\nU->>+S: A fairly long request message\nloop Retry\nS->>D: query\nalt found\nD-->>S: rows\nelse missing\nD--xS: error\nend\nend\nS->>S: think\nNote over S,D: cached\nNote left of U: start\nS-->>-U: done",
        );
        assert_sane(&s, &l);
        // The long message fits between U and S.
        let tw = measure("A fairly long request message").0;
        assert!(l.center(1) - l.center(0) >= tw);
        // Inner frame (alt) is inside the outer (loop) and both hold the
        // S and D lifelines.
        let (alt, lp) = (&l.frames[0], &l.frames[1]);
        assert_eq!((alt.kind, lp.kind), (BlockKind::Alt, BlockKind::Loop));
        assert!(alt.rect.x > lp.rect.x && alt.rect.x + alt.rect.w < lp.rect.x + lp.rect.w);
        assert!(alt.rect.y > lp.rect.y && alt.rect.y + alt.rect.h < lp.rect.y + lp.rect.h);
        assert!(alt.rect.x < l.center(1) && alt.rect.x + alt.rect.w > l.center(2));
        assert_eq!(alt.branches.len(), 1);
        let (by, _) = &alt.branches[0];
        assert!(*by > alt.rect.y && *by < alt.rect.y + alt.rect.h);
        // One activation bar on S, from the first to the last message.
        assert_eq!(l.activations.len(), 1);
        let bar = l.activations[0];
        assert!((bar.x + bar.w / 2.0 - l.center(1)).abs() < 0.01);
        assert!((bar.y - l.messages[0].y).abs() < 0.01);
        let last = l.messages.last().expect("message");
        assert!((bar.y + bar.h - last.y).abs() < 0.01);
        // Lines attach to the bar's edge, not the lifeline.
        assert!((l.messages[0].x2 - bar.x).abs() < 0.01);
        // Self-message loops; numbers are placed.
        assert!(
            l.messages
                .iter()
                .any(|m| m.loop_size.is_some() && m.x1 == m.x2)
        );
        assert!(l.messages.iter().all(|m| m.number_at.is_some()));
        // The left note pushes the picture right instead of off the edge.
        assert!(l.notes.iter().all(|nb| nb.rect.x >= -0.01));
        // Deterministic.
        assert_eq!(l, layout(&s, Spacing::for_font(14.0), &mut measure));
    }

    #[test]
    fn unclosed_activations_run_to_the_end() {
        let (s, l) = lay("sequenceDiagram\nA->>+B: x\nactivate B\nB->>A: y");
        assert_sane(&s, &l);
        assert_eq!(l.activations.len(), 2);
        for b in &l.activations {
            assert!((b.y + b.h - l.lifeline.1).abs() < 0.01);
        }
        // Nested bar is offset to the right.
        assert!(l.activations[1].x > l.activations[0].x);
        let (s, l) = lay("sequenceDiagram");
        assert_sane(&s, &l);
        assert_eq!(l.width, 0.0);
    }

    #[test]
    fn random_sequences_lay_out() {
        let mut s: u64 = 0xA5A5_1234_DEAD_BEEF;
        let mut next = |m: u64| {
            s ^= s << 13;
            s ^= s >> 7;
            s ^= s << 17;
            s % m
        };
        let arrows = ["->", "-->", "->>", "-->>", "-x", "--x", "-)", "--)"];
        for _ in 0..300 {
            let mut src = String::from("sequenceDiagram\n");
            let n = 1 + next(6);
            let mut depth = 0;
            let mut active = vec![0u32; n as usize];
            if next(2) == 0 {
                src.push_str("autonumber\n");
            }
            for _ in 0..next(40) {
                let (a, b) = (next(n), next(n));
                match next(12) {
                    0 if depth < MAX_NESTING => {
                        src.push_str(
                            ["loop x\n", "alt y\n", "opt z\n", "par p\n"][next(4) as usize],
                        );
                        depth += 1;
                    }
                    1 if depth > 0 => {
                        src.push_str("end\n");
                        depth -= 1;
                    }
                    2 => src.push_str(&format!("Note over p{a},p{b}: note {a}\n")),
                    3 => src.push_str(&format!("Note left of p{a}: left\n")),
                    4 => src.push_str(&format!("Note right of p{a}: right side note\n")),
                    5 => {
                        src.push_str(&format!("activate p{a}\n"));
                        active[a as usize] += 1;
                    }
                    6 if active[a as usize] > 0 => {
                        src.push_str(&format!("deactivate p{a}\n"));
                        active[a as usize] -= 1;
                    }
                    _ => {
                        let op = arrows[next(8) as usize];
                        let plus = if next(5) == 0 {
                            active[b as usize] += 1;
                            "+"
                        } else {
                            ""
                        };
                        src.push_str(&format!("p{a}{op}{plus}p{b}: message {a} to {b}\n"));
                    }
                }
            }
            src.push_str(&"end\n".repeat(depth));
            let sq = parse(&src).unwrap_or_else(|e| panic!("{e:?} in {src}"));
            let a = layout(&sq, Spacing::for_font(13.0), &mut measure);
            assert_sane(&sq, &a);
            assert_eq!(a, layout(&sq, Spacing::for_font(13.0), &mut measure));
        }
    }

    #[test]
    fn random_input_never_panics() {
        let tokens = [
            "A",
            "B",
            "c-d",
            "->",
            "-->",
            "->>",
            "-->>",
            "-x",
            "--x",
            "-)",
            "--)",
            "+",
            "-",
            ":",
            " ",
            "\n",
            "Note ",
            "over ",
            "left of ",
            "right of ",
            ",",
            "loop ",
            "alt ",
            "else",
            "and",
            "par ",
            "end",
            "activate ",
            "deactivate ",
            "autonumber",
            "participant ",
            "actor ",
            " as ",
            "<br>",
            "<",
            ">",
            "%%",
            "ä",
            "🙂",
            "rect ",
            "critical ",
            "option",
            ";",
            "\"",
            "x",
        ];
        let mut s: u64 = 0x1357_9BDF_2468_ACE0;
        for _ in 0..4000 {
            let mut src = String::from("sequenceDiagram\n");
            for _ in 0..(s % 60) {
                s ^= s << 13;
                s ^= s >> 7;
                s ^= s << 17;
                src.push_str(tokens[(s % tokens.len() as u64) as usize]);
            }
            if let Ok(sq) = parse(&src) {
                let l = layout(&sq, Spacing::for_font(12.0), &mut measure);
                assert!(l.width.is_finite() && l.height.is_finite());
                assert_sane(&sq, &l);
            }
            s = s.wrapping_add(0x2545_F491_4F6C_DD1D);
        }
    }
}
