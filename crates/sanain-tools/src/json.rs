//! JSON validate / format / minify that preserves the source's lexemes.
//!
//! Unlike a parse-then-serialise round trip, tokens are re-emitted exactly
//! as written: duplicate keys stay, numbers keep their spelling (`1.0`,
//! `1e3`, huge integers), and strings keep their escapes. Only whitespace
//! between tokens changes. The parser uses an explicit stack, so nesting
//! depth cannot overflow the call stack.

use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{message} at line {line}, column {column}")]
pub struct JsonError {
    pub message: &'static str,
    /// 1-based line and column (in chars) of the problem.
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tok {
    OpenObj,
    CloseObj,
    OpenArr,
    CloseArr,
    Colon,
    Comma,
    /// String, number, true, false or null.
    Scalar,
}

/// Tokens with their byte ranges, validated against the JSON grammar.
fn tokenize(src: &str) -> Result<Vec<(Tok, Range<usize>)>, JsonError> {
    let b = src.as_bytes();
    let err = |message, at: usize| {
        let before = &src[..at.min(src.len())];
        let line = before.matches('\n').count() + 1;
        let column = before.rsplit('\n').next().map_or(0, |l| l.chars().count()) + 1;
        JsonError {
            message,
            line,
            column,
            offset: at,
        }
    };
    let mut toks = Vec::new();
    let mut i = 0;
    // Skip a UTF-8 BOM if the text still has one.
    if src.starts_with('\u{FEFF}') {
        i = 3;
    }
    while i < b.len() {
        let c = b[i];
        let start = i;
        let tok = match c {
            b' ' | b'\t' | b'\n' | b'\r' => {
                i += 1;
                continue;
            }
            b'{' => Tok::OpenObj,
            b'}' => Tok::CloseObj,
            b'[' => Tok::OpenArr,
            b']' => Tok::CloseArr,
            b':' => Tok::Colon,
            b',' => Tok::Comma,
            b'"' => {
                i += 1;
                loop {
                    match b.get(i) {
                        None => return Err(err("unterminated string", start)),
                        Some(b'"') => break,
                        Some(b'\\') => match b.get(i + 1) {
                            Some(b'"' | b'\\' | b'/' | b'b' | b'f' | b'n' | b'r' | b't') => i += 2,
                            Some(b'u') => {
                                let hex = b
                                    .get(i + 2..i + 6)
                                    .ok_or_else(|| err("invalid \\u escape", i))?;
                                if !hex.iter().all(u8::is_ascii_hexdigit) {
                                    return Err(err("invalid \\u escape", i));
                                }
                                i += 6;
                            }
                            _ => return Err(err("invalid escape", i)),
                        },
                        Some(&ch) if ch < 0x20 => {
                            return Err(err("control character in string", i));
                        }
                        Some(_) => i += 1,
                    }
                }
                i += 1;
                toks.push((Tok::Scalar, start..i));
                continue;
            }
            b'-' | b'0'..=b'9' => {
                if b[i] == b'-' {
                    i += 1;
                }
                match b.get(i) {
                    Some(b'0') => i += 1,
                    Some(b'1'..=b'9') => {
                        while matches!(b.get(i), Some(b'0'..=b'9')) {
                            i += 1;
                        }
                    }
                    _ => return Err(err("invalid number", start)),
                }
                if b.get(i) == Some(&b'.') {
                    i += 1;
                    if !matches!(b.get(i), Some(b'0'..=b'9')) {
                        return Err(err("invalid number", start));
                    }
                    while matches!(b.get(i), Some(b'0'..=b'9')) {
                        i += 1;
                    }
                }
                if matches!(b.get(i), Some(b'e' | b'E')) {
                    i += 1;
                    if matches!(b.get(i), Some(b'+' | b'-')) {
                        i += 1;
                    }
                    if !matches!(b.get(i), Some(b'0'..=b'9')) {
                        return Err(err("invalid number", start));
                    }
                    while matches!(b.get(i), Some(b'0'..=b'9')) {
                        i += 1;
                    }
                }
                toks.push((Tok::Scalar, start..i));
                continue;
            }
            _ => {
                let rest = &src[i..];
                let word = ["true", "false", "null"]
                    .into_iter()
                    .find(|w| rest.starts_with(w));
                match word {
                    Some(w) => {
                        i += w.len();
                        toks.push((Tok::Scalar, start..i));
                        continue;
                    }
                    None => return Err(err("unexpected character", i)),
                }
            }
        };
        i += 1;
        toks.push((tok, start..i));
    }
    validate(&toks, src, &err)?;
    Ok(toks)
}

#[derive(Clone, Copy, PartialEq)]
enum Ctx {
    Object,
    Array,
}

/// Grammar check with an explicit stack.
fn validate(
    toks: &[(Tok, Range<usize>)],
    src: &str,
    err: &dyn Fn(&'static str, usize) -> JsonError,
) -> Result<(), JsonError> {
    #[derive(Clone, Copy, PartialEq)]
    enum Want {
        Value,
        /// After a value: comma, close, or end.
        After,
        /// In an object: key or `}` (just opened) / key (after comma).
        Key {
            allow_close: bool,
        },
        Colon,
        /// In an array right after `[`: value or `]`.
        ValueOrClose,
    }
    let mut stack: Vec<Ctx> = Vec::new();
    let mut want = Want::Value;
    let mut done = false;
    for (t, r) in toks {
        let at = r.start;
        if done {
            return Err(err("unexpected content after the value", at));
        }
        let is_string = *t == Tok::Scalar && src.as_bytes()[r.start] == b'"';
        want = match (want, t) {
            (Want::Value | Want::ValueOrClose, Tok::Scalar) => Want::After,
            (Want::Value | Want::ValueOrClose, Tok::OpenObj) => {
                stack.push(Ctx::Object);
                Want::Key { allow_close: true }
            }
            (Want::Value | Want::ValueOrClose, Tok::OpenArr) => {
                stack.push(Ctx::Array);
                Want::ValueOrClose
            }
            (Want::ValueOrClose, Tok::CloseArr) => {
                stack.pop();
                Want::After
            }
            (Want::Key { .. }, Tok::Scalar) if is_string => Want::Colon,
            (Want::Key { allow_close: true }, Tok::CloseObj) => {
                stack.pop();
                Want::After
            }
            (Want::Colon, Tok::Colon) => Want::Value,
            (Want::After, Tok::Comma) => match stack.last() {
                Some(Ctx::Object) => Want::Key { allow_close: false },
                Some(Ctx::Array) => Want::Value,
                None => return Err(err("unexpected comma", at)),
            },
            (Want::After, Tok::CloseObj) if stack.last() == Some(&Ctx::Object) => {
                stack.pop();
                Want::After
            }
            (Want::After, Tok::CloseArr) if stack.last() == Some(&Ctx::Array) => {
                stack.pop();
                Want::After
            }
            (Want::Key { .. }, _) => return Err(err("expected a string key", at)),
            (Want::Colon, _) => return Err(err("expected ':'", at)),
            (Want::After, _) => return Err(err("expected ',' or a closing bracket", at)),
            _ => return Err(err("expected a value", at)),
        };
        if want == Want::After && stack.is_empty() {
            done = true;
        }
    }
    if !done {
        let at = src.len();
        return Err(if toks.is_empty() {
            err("empty document", at)
        } else {
            err("unexpected end of input", at)
        });
    }
    Ok(())
}

/// Check that `src` is valid JSON.
pub fn validate_json(src: &str) -> Result<(), JsonError> {
    tokenize(src).map(|_| ())
}

/// Pretty-print with `indent` per level and `newline` between lines.
/// Empty objects and arrays stay on one line (`{}`, `[]`).
pub fn format(src: &str, indent: &str, newline: &str) -> Result<String, JsonError> {
    let toks = tokenize(src)?;
    // Indentation grows with depth, so pathological nesting could expand a
    // small file into gigabytes. Refuse beyond a generous bound.
    let limit = src.len().saturating_mul(16).saturating_add(16 << 20);
    let mut out = String::with_capacity(src.len() + src.len() / 4);
    let mut level = 0usize;
    let line = |out: &mut String, level: usize| {
        out.push_str(newline);
        for _ in 0..level {
            out.push_str(indent);
        }
    };
    for (k, (t, r)) in toks.iter().enumerate() {
        let text = &src[r.clone()];
        let next = toks.get(k + 1).map(|(t, _)| *t);
        match t {
            Tok::OpenObj | Tok::OpenArr => {
                out.push_str(text);
                let empty = matches!(
                    (t, next),
                    (Tok::OpenObj, Some(Tok::CloseObj)) | (Tok::OpenArr, Some(Tok::CloseArr))
                );
                if !empty {
                    level += 1;
                    line(&mut out, level);
                }
            }
            Tok::CloseObj | Tok::CloseArr => {
                let prev = k.checked_sub(1).map(|p| toks[p].0);
                let empty = matches!(prev, Some(Tok::OpenObj | Tok::OpenArr));
                if !empty {
                    level = level.saturating_sub(1);
                    line(&mut out, level);
                }
                out.push_str(text);
            }
            Tok::Comma => {
                out.push(',');
                line(&mut out, level);
            }
            Tok::Colon => out.push_str(": "),
            Tok::Scalar => out.push_str(text),
        }
        if out.len() > limit {
            return Err(JsonError {
                message: "too deeply nested to format",
                line: 1,
                column: 1,
                offset: r.start,
            });
        }
    }
    if src.ends_with('\n') {
        out.push_str(newline);
    }
    Ok(out)
}

/// Remove all whitespace between tokens.
pub fn minify(src: &str) -> Result<String, JsonError> {
    let toks = tokenize(src)?;
    let mut out = String::with_capacity(src.len());
    for (_, r) in toks {
        out.push_str(&src[r]);
    }
    Ok(out)
}

/// A node of a read-only JSON tree. Scalars keep their source lexeme.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    /// Members as (raw key lexeme including quotes, value).
    Object(Vec<(String, Node)>),
    Array(Vec<Node>),
    Scalar(String),
}

// The default drop of a deep tree recurses once per level and can
// overflow the stack on hostile input; drop iteratively instead.
impl Drop for Node {
    fn drop(&mut self) {
        let mut stack: Vec<Node> = Vec::new();
        let take = |n: &mut Node, stack: &mut Vec<Node>| match n {
            Node::Object(m) => stack.extend(m.drain(..).map(|(_, v)| v)),
            Node::Array(a) => stack.append(a),
            Node::Scalar(_) => {}
        };
        take(self, &mut stack);
        while let Some(mut n) = stack.pop() {
            take(&mut n, &mut stack);
        }
    }
}

/// Parse into a tree without recursion. Duplicate keys and number lexemes
/// are kept as written.
pub fn tree(src: &str) -> Result<Node, JsonError> {
    let toks = tokenize(src)?;
    // Stack of containers being built, with the pending key for objects.
    enum Open {
        Obj(Vec<(String, Node)>, Option<String>),
        Arr(Vec<Node>),
    }
    let mut stack: Vec<Open> = Vec::new();
    let mut root = None;
    let push_value =
        |stack: &mut Vec<Open>, v: Node, root: &mut Option<Node>| match stack.last_mut() {
            Some(Open::Obj(members, key)) => {
                if let Some(k) = key.take() {
                    members.push((k, v));
                }
            }
            Some(Open::Arr(items)) => items.push(v),
            None => *root = Some(v),
        };
    for (t, r) in &toks {
        let text = &src[r.clone()];
        match t {
            Tok::OpenObj => stack.push(Open::Obj(Vec::new(), None)),
            Tok::OpenArr => stack.push(Open::Arr(Vec::new())),
            Tok::CloseObj | Tok::CloseArr => {
                let node = match stack.pop() {
                    Some(Open::Obj(m, _)) => Node::Object(m),
                    Some(Open::Arr(a)) => Node::Array(a),
                    None => continue,
                };
                push_value(&mut stack, node, &mut root);
            }
            Tok::Scalar => {
                // In an object with no pending key, a string is the key.
                if let Some(Open::Obj(_, key @ None)) = stack.last_mut() {
                    *key = Some(text.to_owned());
                } else {
                    push_value(&mut stack, Node::Scalar(text.to_owned()), &mut root);
                }
            }
            Tok::Colon | Tok::Comma => {}
        }
    }
    Ok(root.unwrap_or(Node::Scalar(String::new())))
}

pub use crate::csv::Edit;

/// The kind of a node in an editable JSON tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Object,
    Array,
    String,
    /// A number, `true`, `false` or `null`.
    Literal,
}

/// Why a tree edit was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditError {
    /// The typed text is not one JSON value.
    InvalidValue,
    /// Another member of the same object already has this key.
    DuplicateKey,
    /// The operation does not apply to this node.
    NotApplicable,
}

/// One node of an editable JSON tree. Spans are char ranges in the source
/// (what the editor's rope uses); the private byte ranges mirror them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
    pub parent: Option<usize>,
    pub kind: Kind,
    /// Indices of the members or elements, in source order.
    pub children: Vec<usize>,
    /// The key token, quotes included, for object members.
    pub key_span: Option<Range<usize>>,
    /// The value: the token for scalars, bracket to bracket for containers.
    pub value_span: Range<usize>,
    /// What deleting this member removes: from the key (or value) through
    /// the value plus exactly one adjacent comma and the whitespace
    /// between; an only child removes the container's whole interior.
    /// Empty for the root.
    pub removal_span: Range<usize>,
    key_b: Option<Range<usize>>,
    value_b: Range<usize>,
    removal_b: Range<usize>,
}

/// An editable JSON tree over its source text. `items[0]` is the root.
/// Every operation returns one minimal [`Edit`] that touches only the
/// node concerned; whitespace, escapes and number spellings elsewhere are
/// never rewritten, and the edited text is checked to still be JSON.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Doc {
    src: String,
    pub items: Vec<Item>,
}

/// Turns byte offsets into char offsets, moving forward only.
struct CharCursor {
    byte: usize,
    char: usize,
}

impl CharCursor {
    fn at(&mut self, src: &str, byte: usize) -> usize {
        self.char += src[self.byte..byte].chars().count();
        self.byte = byte;
        self.char
    }
}

const JSON_WS: [char; 4] = [' ', '\t', '\n', '\r'];

/// Parse `src` into an editable tree without recursion.
pub fn doc(src: &str) -> Result<Doc, JsonError> {
    let toks = tokenize(src)?;
    let mut items: Vec<Item> = Vec::new();
    let mut stack: Vec<usize> = Vec::new();
    let mut key: Option<(Range<usize>, Range<usize>)> = None;
    let mut cur = CharCursor { byte: 0, char: 0 };
    let add = |items: &mut Vec<Item>, stack: &[usize], kind, b: Range<usize>, c, key| {
        let parent = stack.last().copied();
        let (key_b, key_span) = match key {
            Some((kb, kc)) => (Some(kb), Some(kc)),
            None => (None, None),
        };
        items.push(Item {
            parent,
            kind,
            children: Vec::new(),
            key_span,
            value_span: c,
            removal_span: 0..0,
            key_b,
            value_b: b,
            removal_b: 0..0,
        });
        let i = items.len() - 1;
        if let Some(p) = parent {
            items[p].children.push(i);
        }
        i
    };
    for (t, r) in &toks {
        let cs = cur.at(src, r.start);
        let ce = cur.at(src, r.end);
        match t {
            Tok::OpenObj | Tok::OpenArr => {
                let kind = if *t == Tok::OpenObj {
                    Kind::Object
                } else {
                    Kind::Array
                };
                let i = add(&mut items, &stack, kind, r.clone(), cs..ce, key.take());
                stack.push(i);
            }
            Tok::CloseObj | Tok::CloseArr => {
                if let Some(i) = stack.pop() {
                    items[i].value_b.end = r.end;
                    items[i].value_span.end = ce;
                }
            }
            Tok::Scalar => {
                let in_obj = stack.last().is_some_and(|&p| items[p].kind == Kind::Object);
                if in_obj && key.is_none() {
                    key = Some((r.clone(), cs..ce));
                } else {
                    let kind = if src.as_bytes()[r.start] == b'"' {
                        Kind::String
                    } else {
                        Kind::Literal
                    };
                    add(&mut items, &stack, kind, r.clone(), cs..ce, key.take());
                }
            }
            Tok::Colon | Tok::Comma => {}
        }
    }
    // Removal spans, from the spans of each container's children.
    for p in 0..items.len() {
        let n = items[p].children.len();
        for k in 0..n {
            let c = items[p].children[k];
            let (rb, rc) = if n == 1 {
                let (vb, vc) = (&items[p].value_b, &items[p].value_span);
                (vb.start + 1..vb.end - 1, vc.start + 1..vc.end - 1)
            } else if k + 1 < n {
                let next = &items[items[p].children[k + 1]];
                (
                    items[c].start_b()..next.start_b(),
                    items[c].start()..next.start(),
                )
            } else {
                let prev = &items[items[p].children[k - 1]];
                (
                    prev.value_b.end..items[c].value_b.end,
                    prev.value_span.end..items[c].value_span.end,
                )
            };
            items[c].removal_b = rb;
            items[c].removal_span = rc;
        }
    }
    Ok(Doc {
        src: src.to_owned(),
        items,
    })
}

impl Item {
    /// Char where the member starts: its key, or its value in an array.
    pub fn start(&self) -> usize {
        self.key_span
            .as_ref()
            .map_or(self.value_span.start, |k| k.start)
    }

    fn start_b(&self) -> usize {
        self.key_b.as_ref().map_or(self.value_b.start, |k| k.start)
    }
}

impl Doc {
    /// The source this tree was parsed from.
    pub fn source(&self) -> &str {
        &self.src
    }

    /// The key token as written, quotes included.
    pub fn key_text(&self, i: usize) -> Option<&str> {
        let r = self.items.get(i)?.key_b.clone()?;
        self.src.get(r)
    }

    /// The key with its escapes decoded.
    pub fn key(&self, i: usize) -> Option<String> {
        self.key_text(i).map(decode_string)
    }

    /// The value as written (whole container text for containers).
    pub fn value_text(&self, i: usize) -> Option<&str> {
        self.src.get(self.items.get(i)?.value_b.clone())
    }

    /// Child positions from the root down to `i`.
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

    /// The node at a path of child positions.
    pub fn find(&self, path: &[usize]) -> Option<usize> {
        let mut i = 0;
        self.items.first()?;
        for &k in path {
            i = *self.items[i].children.get(k)?;
        }
        Some(i)
    }

    /// The value text `input` would become, or `None` if it would not
    /// change. Input is JSON text for a value; when editing a string,
    /// text that is not JSON is taken as the string's content and encoded.
    pub fn check_value(&self, i: usize, input: &str) -> Result<Option<String>, EditError> {
        let it = self.items.get(i).ok_or(EditError::NotApplicable)?;
        if !matches!(it.kind, Kind::String | Kind::Literal) {
            return Err(EditError::NotApplicable);
        }
        let current = self.value_text(i).unwrap_or_default();
        let trimmed = input.trim_matches(JSON_WS);
        let new = if is_one_value(trimmed) {
            trimmed.to_owned()
        } else if it.kind == Kind::String {
            encode_string(input)
        } else {
            return Err(EditError::InvalidValue);
        };
        Ok((new != current).then_some(new))
    }

    /// The key token `name` would become, or `None` if it is unchanged.
    pub fn check_key(&self, i: usize, name: &str) -> Result<Option<String>, EditError> {
        let it = self.items.get(i).ok_or(EditError::NotApplicable)?;
        if it.key_b.is_none() {
            return Err(EditError::NotApplicable);
        }
        if self.key(i).as_deref() == Some(name) {
            return Ok(None);
        }
        self.check_new_key(it.parent, Some(i), name)?;
        Ok(Some(encode_string(name)))
    }

    fn check_new_key(
        &self,
        parent: Option<usize>,
        except: Option<usize>,
        name: &str,
    ) -> Result<(), EditError> {
        let Some(p) = parent.and_then(|p| self.items.get(p)) else {
            return Ok(());
        };
        let taken = p
            .children
            .iter()
            .any(|&c| Some(c) != except && self.key(c).as_deref() == Some(name));
        if taken {
            Err(EditError::DuplicateKey)
        } else {
            Ok(())
        }
    }

    /// Replace a scalar value (see [`Doc::check_value`]).
    pub fn set_value(&self, i: usize, input: &str) -> Result<Option<Edit>, EditError> {
        let Some(new) = self.check_value(i, input)? else {
            return Ok(None);
        };
        let it = &self.items[i];
        self.checked(it.value_b.clone(), it.value_span.clone(), new)
            .map(Some)
    }

    /// Rename an object member's key; the name is encoded as a JSON string.
    pub fn rename_key(&self, i: usize, name: &str) -> Result<Option<Edit>, EditError> {
        let Some(new) = self.check_key(i, name)? else {
            return Ok(None);
        };
        let it = &self.items[i];
        let (Some(kb), Some(kc)) = (it.key_b.clone(), it.key_span.clone()) else {
            return Err(EditError::NotApplicable);
        };
        self.checked(kb, kc, new).map(Some)
    }

    /// Remove a member or element with one adjacent comma. The root
    /// cannot be deleted.
    pub fn delete(&self, i: usize) -> Option<Edit> {
        let it = self.items.get(i)?;
        it.parent?;
        self.checked(it.removal_b.clone(), it.removal_span.clone(), String::new())
            .ok()
    }

    /// A key not used by any member of object `p`: `base`, `base2`, ….
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

    /// Insert a new member (`key` is required in objects, ignored in
    /// arrays) or element after node `i`, copying the whitespace style of
    /// its siblings: the line ending and indentation after their commas
    /// and the spacing around the colon. `value` is JSON text.
    pub fn add_after(&self, i: usize, key: Option<&str>, value: &str) -> Result<Edit, EditError> {
        let it = self.items.get(i).ok_or(EditError::NotApplicable)?;
        let p = it.parent.ok_or(EditError::NotApplicable)?;
        let parent = &self.items[p];
        let siblings = &parent.children;
        let pos = siblings
            .iter()
            .position(|&c| c == i)
            .ok_or(EditError::NotApplicable)?;
        // Whitespace that follows a comma in this container, else after the
        // opening bracket.
        let next = if pos >= 1 { pos } else { 1 };
        let ws = match siblings.get(next) {
            Some(&s) => {
                let prev = &self.items[siblings[next - 1]];
                let gap = &self.src[prev.value_b.end..self.items[s].start_b()];
                gap.find(',').map_or("", |c| &gap[c + 1..])
            }
            None => &self.src[parent.value_b.start + 1..it.start_b()],
        };
        let colon = match &it.key_b {
            Some(kb) => &self.src[kb.end..it.value_b.start],
            None => ": ",
        };
        // An inline only child: one space after the comma, unless its colon
        // has none (`{"a":1}` stays compact).
        let ws = if ws.is_empty() && siblings.len() == 1 && colon.ends_with(' ') {
            " "
        } else {
            ws
        };
        let member = self.member_text(parent.kind, p, key, colon, value)?;
        let at_b = it.value_b.end;
        let at_c = it.value_span.end;
        self.checked(at_b..at_b, at_c..at_c, format!(",{ws}{member}"))
    }

    /// Add a member or element at the end of container `i`; into an empty
    /// one inline, replacing only the whitespace inside its brackets.
    pub fn add_child(&self, i: usize, key: Option<&str>, value: &str) -> Result<Edit, EditError> {
        let it = self.items.get(i).ok_or(EditError::NotApplicable)?;
        if !matches!(it.kind, Kind::Object | Kind::Array) {
            return Err(EditError::NotApplicable);
        }
        if let Some(&last) = it.children.last() {
            return self.add_after(last, key, value);
        }
        let member = self.member_text(it.kind, i, key, ": ", value)?;
        let (b, c) = (&it.value_b, &it.value_span);
        self.checked(b.start + 1..b.end - 1, c.start + 1..c.end - 1, member)
    }

    fn member_text(
        &self,
        container: Kind,
        p: usize,
        key: Option<&str>,
        colon: &str,
        value: &str,
    ) -> Result<String, EditError> {
        let value = value.trim_matches(JSON_WS);
        if !is_one_value(value) {
            return Err(EditError::InvalidValue);
        }
        Ok(match container {
            Kind::Object => {
                let key = key.ok_or(EditError::NotApplicable)?;
                self.check_new_key(Some(p), None, key)?;
                format!("{}{colon}{value}", encode_string(key))
            }
            _ => value.to_owned(),
        })
    }

    /// The edit, if the text it produces is still valid JSON.
    fn checked(&self, b: Range<usize>, c: Range<usize>, text: String) -> Result<Edit, EditError> {
        let before = self.src.get(..b.start).ok_or(EditError::NotApplicable)?;
        let after = self.src.get(b.end..).ok_or(EditError::NotApplicable)?;
        let mut out = String::with_capacity(self.src.len() + text.len());
        out.push_str(before);
        out.push_str(&text);
        out.push_str(after);
        if validate_json(&out).is_err() {
            return Err(EditError::InvalidValue);
        }
        Ok(Edit { range: c, text })
    }
}

/// Whether `s` is exactly one JSON value (no byte-order mark).
fn is_one_value(s: &str) -> bool {
    !s.starts_with('\u{FEFF}') && validate_json(s).is_ok()
}

/// Encode text as a JSON string token: quotes, backslashes and control
/// characters are escaped; everything else is kept literally.
pub fn encode_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Decode a JSON string token (quotes included). A lone surrogate escape
/// becomes U+FFFD.
pub fn decode_string(tok: &str) -> String {
    let inner = tok
        .strip_prefix('"')
        .and_then(|t| t.strip_suffix('"'))
        .unwrap_or(tok);
    let mut out = String::with_capacity(inner.len());
    let mut chars = inner.chars();
    let hex4 = |chars: &mut std::str::Chars| -> Option<u32> {
        let h: String = chars.by_ref().take(4).collect();
        u32::from_str_radix(&h, 16).ok().filter(|_| h.len() == 4)
    };
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('n') => out.push('\n'),
            Some('r') => out.push('\r'),
            Some('t') => out.push('\t'),
            Some('b') => out.push('\u{8}'),
            Some('f') => out.push('\u{c}'),
            Some('u') => {
                let Some(u) = hex4(&mut chars) else {
                    out.push('\u{FFFD}');
                    continue;
                };
                if (0xD800..0xDC00).contains(&u) {
                    // A high surrogate: a low one must follow.
                    let rest = chars.as_str();
                    if let Some(low) = rest.strip_prefix("\\u").and_then(|r| {
                        r.get(..4)
                            .and_then(|h| u32::from_str_radix(h, 16).ok())
                            .filter(|l| (0xDC00..0xE000).contains(l))
                    }) {
                        chars = rest[6..].chars();
                        let cp = 0x10000 + ((u - 0xD800) << 10) + (low - 0xDC00);
                        out.push(char::from_u32(cp).unwrap_or('\u{FFFD}'));
                        continue;
                    }
                }
                out.push(char::from_u32(u).unwrap_or('\u{FFFD}'));
            }
            Some(c) => out.push(c),
            None => {}
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_preserves_lexemes_and_duplicates() {
        let src = r#"{"a":1.0,"a":1e3,"big":123456789012345678901234567890,"s":"\u00e4\n","e":{},"l":[]}"#;
        let out = format(src, "  ", "\n").unwrap();
        assert_eq!(
            out,
            "{\n  \"a\": 1.0,\n  \"a\": 1e3,\n  \"big\": 123456789012345678901234567890,\n  \"s\": \"\\u00e4\\n\",\n  \"e\": {},\n  \"l\": []\n}"
        );
        assert_eq!(minify(&out).unwrap(), src);
    }

    #[test]
    fn tree_keeps_lexemes_duplicates_and_order() {
        let t = tree(r#"{"b":[1.0,{"x":null}],"a":2,"a":"\u0041"}"#).unwrap();
        let Node::Object(m) = &t else { panic!() };
        let keys: Vec<&str> = m.iter().map(|(k, _)| k.as_str()).collect();
        assert_eq!(keys, ["\"b\"", "\"a\"", "\"a\""]);
        assert_eq!(
            m[0].1,
            Node::Array(vec![
                Node::Scalar("1.0".into()),
                Node::Object(vec![("\"x\"".into(), Node::Scalar("null".into()))]),
            ])
        );
        assert_eq!(m[2].1, Node::Scalar("\"\\u0041\"".into()));
        let deep = "[".repeat(100_000) + &"]".repeat(100_000);
        assert!(tree(&deep).is_ok(), "no recursion");
    }

    #[test]
    fn crlf_tabs_and_trailing_newline() {
        let out = format("[1,[2]]\n", "\t", "\r\n").unwrap();
        assert_eq!(out, "[\r\n\t1,\r\n\t[\r\n\t\t2\r\n\t]\r\n]\r\n");
    }

    #[test]
    fn errors_have_positions() {
        let e = validate_json("{\n  \"a\": 1,\n  \"b\": }").unwrap_err();
        assert_eq!((e.message, e.line, e.column), ("expected a value", 3, 8));
        for bad in [
            "",
            "{",
            "[1,]",
            "{\"a\":1,}",
            "{a:1}",
            "01",
            "1.",
            "-",
            "1e",
            "\"\\x\"",
            "\"a\nb\"",
            "[1] 2",
            "tru",
            "{\"a\" 1}",
            "]",
            "\"unterminated",
            "\"\\u12G4\"",
            "NaN",
        ] {
            assert!(validate_json(bad).is_err(), "{bad:?} should fail");
        }
        for good in [
            "0",
            "-0.5e-7",
            "\"\"",
            "[]",
            "{}",
            "true",
            " null ",
            "\u{FEFF}{}",
            "[[[]]]",
        ] {
            assert!(validate_json(good).is_ok(), "{good:?} should pass");
        }
    }

    #[test]
    fn deep_nesting_has_no_recursion_limit() {
        let deep = "[".repeat(200_000) + &"]".repeat(200_000);
        assert!(validate_json(&deep).is_ok());
        assert!(minify(&deep).is_ok());
        // Output would be ~depth^2 bytes: refused, quickly.
        let t = std::time::Instant::now();
        assert_eq!(
            format(&deep, "  ", "\n").unwrap_err().message,
            "too deeply nested to format"
        );
        assert!(t.elapsed() < std::time::Duration::from_secs(5));
        let moderate = "[".repeat(500) + &"]".repeat(500);
        assert!(format(&moderate, "  ", "\n").is_ok());
        assert!(validate_json(&("[".repeat(100_000))).is_err());
    }

    /// Apply an edit (char range) to `src`.
    fn apply(src: &str, e: &Edit) -> String {
        let at = |c: usize| src.char_indices().nth(c).map_or(src.len(), |(b, _)| b);
        let (s, t) = (at(e.range.start), at(e.range.end));
        format!("{}{}{}", &src[..s], e.text, &src[t..])
    }

    /// The node at a path of child positions.
    fn at(d: &Doc, path: &[usize]) -> usize {
        d.find(path).unwrap()
    }

    const NESTED: &str = "{\r\n    \"nimi\": \"Äiti 🙂\",\r\n    \"\\u00e4\": 123456789012345678901234567890,\r\n    \"lista\": [\r\n        1,\r\n        {\"x\": null},\r\n        3\r\n    ],\r\n    \"tyhjä\": {}\r\n}\r\n";

    #[test]
    fn spans_are_chars_and_cover_tokens() {
        let d = doc(NESTED).unwrap();
        let name = at(&d, &[0]);
        let chars: Vec<char> = NESTED.chars().collect();
        let s = |r: &Range<usize>| chars[r.clone()].iter().collect::<String>();
        assert_eq!(s(d.items[name].key_span.as_ref().unwrap()), "\"nimi\"");
        assert_eq!(s(&d.items[name].value_span), "\"Äiti 🙂\"");
        assert_eq!(d.items[name].kind, Kind::String);
        let list = at(&d, &[2]);
        assert_eq!(d.items[list].kind, Kind::Array);
        assert!(
            s(&d.items[list].value_span).starts_with('[')
                && s(&d.items[list].value_span).ends_with(']')
        );
        assert_eq!(d.key(at(&d, &[1])).as_deref(), Some("ä"));
        assert_eq!(
            d.value_text(at(&d, &[1])),
            Some("123456789012345678901234567890")
        );
        assert_eq!(d.path_of(at(&d, &[2, 1, 0])), vec![2, 1, 0]);
        assert_eq!(d.find(&[9]), None);
    }

    #[test]
    fn set_value_replaces_only_the_token() {
        let d = doc(NESTED).unwrap();
        let big = at(&d, &[1]);
        let e = d
            .set_value(big, " 98765432109876543210987654321.5e+300 ")
            .unwrap()
            .unwrap();
        assert_eq!(e.range, d.items[big].value_span);
        let out = apply(NESTED, &e);
        assert_eq!(
            out,
            NESTED.replace(
                "123456789012345678901234567890",
                "98765432109876543210987654321.5e+300"
            )
        );
        // A number takes only JSON.
        assert_eq!(d.set_value(big, "12 apples"), Err(EditError::InvalidValue));
        assert_eq!(d.set_value(big, "\u{FEFF}1"), Err(EditError::InvalidValue));
        assert_eq!(d.set_value(big, ""), Err(EditError::InvalidValue));
        assert_eq!(d.set_value(big, "123456789012345678901234567890"), Ok(None));
        // A string takes JSON, or plain text that is encoded.
        let name = at(&d, &[0]);
        let e = d
            .set_value(name, "Isä \"Ö\" \\ a\tb\nc\u{1}")
            .unwrap()
            .unwrap();
        assert_eq!(e.text, r#""Isä \"Ö\" \\ a\tb\nc\u0001""#);
        let out = apply(NESTED, &e);
        assert!(validate_json(&out).is_ok());
        assert_eq!(d.set_value(name, "null").unwrap().unwrap().text, "null");
        assert_eq!(
            d.set_value(name, "\"\\u00e4\"").unwrap().unwrap().text,
            "\"\\u00e4\""
        );
        assert_eq!(d.set_value(name, "").unwrap().unwrap().text, "\"\"");
        assert_eq!(d.set_value(name, "\"Äiti 🙂\""), Ok(None));
        // Containers are not scalars.
        assert_eq!(
            d.set_value(at(&d, &[2]), "1"),
            Err(EditError::NotApplicable)
        );
        // A scalar may become a container.
        let one = at(&d, &[2, 0]);
        let out = apply(
            NESTED,
            &d.set_value(one, "{\"a\": [true]}").unwrap().unwrap(),
        );
        assert!(out.contains("        {\"a\": [true]},\r\n"));
    }

    #[test]
    fn rename_encodes_and_refuses_duplicates() {
        let d = doc(NESTED).unwrap();
        let name = at(&d, &[0]);
        let e = d.rename_key(name, "na\"me").unwrap().unwrap();
        assert_eq!(e.range, d.items[name].key_span.clone().unwrap());
        assert_eq!(apply(NESTED, &e), NESTED.replace("\"nimi\"", r#""na\"me""#));
        // The escaped key decodes to "ä": renaming to the same is no edit.
        assert_eq!(d.rename_key(at(&d, &[1]), "ä"), Ok(None));
        assert_eq!(d.rename_key(name, "lista"), Err(EditError::DuplicateKey));
        assert_eq!(d.rename_key(name, "ä"), Err(EditError::DuplicateKey));
        assert_eq!(
            d.rename_key(at(&d, &[2, 0]), "k"),
            Err(EditError::NotApplicable)
        );
        assert_eq!(d.rename_key(0, "k"), Err(EditError::NotApplicable));
    }

    #[test]
    fn delete_removes_one_comma() {
        let d = doc(NESTED).unwrap();
        let del = |path: &[usize]| apply(NESTED, &d.delete(at(&d, path)).unwrap());
        // First, middle and last element of a multi-line array.
        assert!(
            del(&[2, 0]).contains("\"lista\": [\r\n        {\"x\": null},\r\n        3\r\n    ],")
        );
        assert!(del(&[2, 1]).contains("[\r\n        1,\r\n        3\r\n    ],"));
        assert!(del(&[2, 2]).contains("[\r\n        1,\r\n        {\"x\": null}\r\n    ],"));
        // The only member: the container's interior goes.
        assert!(del(&[2, 1, 0]).contains("        {},\r\n"));
        // First and last member of the object.
        assert_eq!(
            del(&[0]),
            NESTED.replace("    \"nimi\": \"Äiti 🙂\",\r\n", "")
        );
        assert_eq!(del(&[3]), NESTED.replace(",\r\n    \"tyhjä\": {}", ""));
        assert_eq!(d.delete(0), None, "the root stays");
        for (src, path, want) in [
            ("[1, 2, 3]", &[0][..], "[2, 3]"),
            ("[1, 2, 3]", &[1][..], "[1, 3]"),
            ("[1, 2, 3]", &[2][..], "[1, 2]"),
            ("[ 1 ]", &[0][..], "[]"),
            ("{\"a\":1,\"b\":2}", &[1][..], "{\"a\":1}"),
            ("[[1],[2]]", &[1, 0][..], "[[1],[]]"),
        ] {
            let d = doc(src).unwrap();
            assert_eq!(apply(src, &d.delete(at(&d, path)).unwrap()), want, "{src}");
        }
    }

    #[test]
    fn add_copies_sibling_style() {
        let d = doc(NESTED).unwrap();
        let k = d.unique_key(0, "key");
        assert_eq!(k, "key");
        let e = d.add_after(at(&d, &[0]), Some(&k), "null").unwrap();
        assert_eq!(e.range.start, e.range.end, "a pure insertion");
        assert_eq!(
            apply(NESTED, &e),
            NESTED.replace("🙂\",\r\n", "🙂\",\r\n    \"key\": null,\r\n")
        );
        // After the last element of a CRLF array.
        let e = d.add_after(at(&d, &[2, 2]), None, "\"ö\"").unwrap();
        assert!(apply(NESTED, &e).contains("        3,\r\n        \"ö\"\r\n    ],"));
        // Into an empty object, inline.
        let e = d.add_child(at(&d, &[3]), Some("key"), "null").unwrap();
        assert!(apply(NESTED, &e).contains("\"tyhjä\": {\"key\": null}\r\n}"));
        // Appending to a non-empty container adds after its last child.
        let e = d.add_child(at(&d, &[2, 1]), Some("y"), "2").unwrap();
        assert!(apply(NESTED, &e).contains("{\"x\": null, \"y\": 2}"));
        assert_eq!(
            d.add_after(at(&d, &[0]), Some("lista"), "1"),
            Err(EditError::DuplicateKey)
        );
        assert_eq!(
            d.add_after(at(&d, &[0]), None, "1"),
            Err(EditError::NotApplicable)
        );
        assert_eq!(
            d.add_after(at(&d, &[0]), Some("z"), "nope"),
            Err(EditError::InvalidValue)
        );
        assert_eq!(
            d.add_after(0, Some("z"), "1"),
            Err(EditError::NotApplicable)
        );
        for (src, path, want) in [
            ("[\n\t1,\n\t2\n]", &[0][..], "[\n\t1,\n\tnull,\n\t2\n]"),
            ("[\n\t1\n]", &[0][..], "[\n\t1,\n\tnull\n]"),
            ("[1, 2]", &[0][..], "[1, null, 2]"),
            ("[1,2]", &[1][..], "[1,2,null]"),
            ("[ [ ] ]", &[0][..], "[ [ ], null ]"),
        ] {
            let d = doc(src).unwrap();
            let e = d.add_after(at(&d, path), None, "null").unwrap();
            assert_eq!(apply(src, &e), want, "{src}");
        }
        let src = "{\"a\":1}";
        let d = doc(src).unwrap();
        assert_eq!(d.unique_key(0, "a"), "a2");
        let e = d.add_after(at(&d, &[0]), Some("b"), "[]").unwrap();
        assert_eq!(apply(src, &e), "{\"a\":1,\"b\":[]}");
        let src = "[\r\n]";
        let d = doc(src).unwrap();
        assert_eq!(apply(src, &d.add_child(0, None, "null").unwrap()), "[null]");
    }

    #[test]
    fn strings_round_trip_through_escapes() {
        for s in [
            "",
            "plain",
            "ä🙂",
            "q\"\\/",
            "\n\r\t\u{8}\u{c}\u{0}\u{1f}",
            "\u{2028}",
        ] {
            let enc = encode_string(s);
            assert!(validate_json(&enc).is_ok(), "{enc}");
            assert_eq!(decode_string(&enc), s);
        }
        assert_eq!(decode_string(r#""\ud83d\ude42 \u00E4 \/""#), "🙂 ä /");
        assert_eq!(decode_string(r#""\ud83d x""#), "\u{FFFD} x");
    }

    /// A random JSON document as compact text.
    fn arb_doc() -> impl proptest::strategy::Strategy<Value = String> {
        use proptest::prelude::*;
        let leaf = prop_oneof![
            Just("null".to_owned()),
            Just("true".to_owned()),
            Just("-0.5e7".to_owned()),
            Just("123456789012345678901234567890".to_owned()),
            Just("\"\"".to_owned()),
            Just("\"a\\u00e4\\n\"".to_owned()),
            Just("\"Öö 🙂\"".to_owned()),
        ];
        leaf.prop_recursive(4, 40, 5, |inner| {
            prop_oneof![
                proptest::collection::vec(inner.clone(), 0..5)
                    .prop_map(|v| format!("[{}]", v.join(","))),
                proptest::collection::vec((0..4usize, inner), 0..5).prop_map(|v| {
                    let keys = ["\"a\"", "\"\\u00e4\"", "\"k y\"", "\"\""];
                    let m: Vec<String> =
                        v.iter().map(|(k, v)| format!("{}:{v}", keys[*k])).collect();
                    format!("{{{}}}", m.join(","))
                }),
            ]
        })
    }

    proptest::proptest! {
        #[test]
        fn random_edits_keep_valid_json(
            compact in arb_doc(),
            style in 0..4usize,
            ops in proptest::collection::vec((0..5usize, 0..64usize, 0..8usize), 1..6),
        ) {
            let mut src = match style {
                0 => compact.clone(),
                1 => format(&compact, "    ", "\r\n").unwrap(),
                2 => format(&compact, "\t", "\n").unwrap() + "\n",
                _ => compact.replace(',', " ,  ").replace(':', " : "),
            };
            let inputs = ["1", "\"x\"", "plain text", "{\"a\":[1]}", "", "nul", "\\", "ä\"\n"];
            let keys = ["a", "ä", "key", "\"q\""];
            for (op, n, s) in ops {
                let d = doc(&src).unwrap();
                let i = n % d.items.len();
                let before = d.items.len();
                let subtree = {
                    let mut st = vec![i];
                    let mut count = 0;
                    while let Some(x) = st.pop() {
                        count += 1;
                        st.extend(&d.items[x].children);
                    }
                    count
                };
                let edit = match op {
                    0 => d.set_value(i, inputs[s]).ok().flatten(),
                    1 => d.rename_key(i, keys[s % 4]).ok().flatten(),
                    2 => d.delete(i),
                    3 => {
                        let k = d.items[i].parent.map(|p| d.unique_key(p, keys[s % 4]));
                        d.add_after(i, k.as_deref(), "null").ok()
                    }
                    _ => {
                        let k = d.unique_key(i, "key");
                        d.add_child(i, Some(&k), "null").ok()
                    }
                };
                let Some(e) = edit else { continue };
                let n_chars = src.chars().count();
                proptest::prop_assert!(e.range.start <= e.range.end && e.range.end <= n_chars);
                let out = apply(&src, &e);
                proptest::prop_assert!(validate_json(&out).is_ok(), "{} -> {}", src, out);
                let after = doc(&out).unwrap().items.len();
                match op {
                    2 => proptest::prop_assert_eq!(after, before - subtree),
                    3 | 4 => proptest::prop_assert_eq!(after, before + 1),
                    _ => {}
                }
                // No stray or double commas.
                proptest::prop_assert!(minify(&out).unwrap().find(",,").is_none());
                src = out;
            }
        }
    }

    proptest::proptest! {
        #[test]
        fn minify_of_format_is_minify(s in "[\\[\\]{}:, \"a-c0-9.e\\-\\n]{0,40}") {
            if let Ok(f) = format(&s, "  ", "\n") {
                proptest::prop_assert_eq!(minify(&f).unwrap(), minify(&s).unwrap());
            }
        }
    }
}
