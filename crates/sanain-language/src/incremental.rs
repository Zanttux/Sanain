//! Incremental syntax analysis: the syntax trees, highlight spans and
//! folds of one document, updated after an edit instead of recomputed.
//!
//! The previous [`Syntax`] keeps its text and its language layers (see
//! [`crate::layers`]). A new snapshot is compared with the old text
//! (common prefix and suffix), which gives one [`InputEdit`] covering
//! every change; the top tree is edited and parsed again, reusing the
//! unchanged parts. No edit history is needed, so the result cannot drift
//! from the text whatever happened in between (undo, reload, several
//! edits).
//!
//! Injections (JavaScript in HTML, Markdown's inline grammar and fenced
//! code, the HTML around PHP) are looked for again only where the
//! enclosing tree changed; an injected layer is parsed again
//! (incrementally) only when the edit or a change of its ranges reaches
//! it, and other layers are shifted. Highlights are computed only for the
//! region that may have changed: the edit and the ranges whose structure
//! changed in any layer, widened to the enclosing syntax nodes and whole
//! lines, merged over the layers like `tree_sitter_highlight` does. Spans
//! outside it are kept and shifted. Folds are collected from the whole top
//! tree.
//!
//! An incremental result is exact for the trees it has (tested for trees
//! without syntax errors; inside error nodes Tree-sitter's changed ranges
//! can miss a change). The trees themselves can differ from a fresh parse
//! in broken or ambiguous code, so [`Syntax::is_fresh`] tells callers when
//! to analyse again from scratch to get the canonical result.

use std::ops::{ControlFlow, Range};
use std::sync::atomic::{AtomicUsize, Ordering};

use tree_sitter::{InputEdit, ParseOptions, Parser, Point, Range as TsRange, Tree};

use crate::layers::{self, Ctx, Layer, whole_lines};
use crate::{HighlightError, Highlighter, Lang, Span, folds};

/// When the region to highlight again exceeds this share of the document
/// (1/n), the whole document is highlighted instead.
const FULL_FRACTION: usize = 2;

/// Syntax state of one document: its text, language layers, highlight
/// spans and folds. Pass it back to [`Highlighter::analyze`] with the next
/// snapshot to update it incrementally.
pub struct Syntax {
    lang: Lang,
    text: String,
    top: Layer,
    spans: Vec<Span>,
    folds: Vec<Range<usize>>,
    rehighlighted: Range<usize>,
    /// Parsed by reusing an earlier tree.
    incremental: bool,
}

impl std::fmt::Debug for Syntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Syntax")
            .field("lang", &self.lang)
            .field("len", &self.text.len())
            .field("spans", &self.spans.len())
            .field("rehighlighted", &self.rehighlighted)
            .finish_non_exhaustive()
    }
}

impl Syntax {
    pub fn lang(&self) -> Lang {
        self.lang
    }

    /// The analysed text.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Highlight spans, as from [`Highlighter::highlight`].
    pub fn spans(&self) -> &[Span] {
        &self.spans
    }

    /// Fold ranges, as from [`Highlighter::folds`].
    pub fn folds(&self) -> &[Range<usize>] {
        &self.folds
    }

    /// The byte range whose highlights were computed by the last update
    /// (the whole text after a full analysis; empty when the text did not
    /// change).
    pub fn rehighlighted(&self) -> Range<usize> {
        self.rehighlighted.clone()
    }

    /// Whether the trees were parsed from scratch. An incrementally
    /// updated tree is a valid parse of the text but not always the same
    /// one: in ambiguous or broken code, Tree-sitter can reuse old
    /// subtrees where a fresh parse would recover or decide differently.
    /// Callers that need the canonical result analyse the text again
    /// without `prev` (the syntax worker does when typing pauses).
    pub fn is_fresh(&self) -> bool {
        !self.incremental
    }
}

/// One edit turning `old` into `new`: bytes `start..old_end` of `old`
/// became `start..new_end` of `new`. `None` when the texts are equal.
fn diff(old: &str, new: &str) -> Option<(usize, usize, usize)> {
    let (a, b) = (old.as_bytes(), new.as_bytes());
    let max = a.len().min(b.len());
    // Compare in blocks first (memcmp), then byte by byte.
    const BLOCK: usize = 256;
    let mut p = 0;
    while p + BLOCK <= max && a[p..p + BLOCK] == b[p..p + BLOCK] {
        p += BLOCK;
    }
    while p < max && a[p] == b[p] {
        p += 1;
    }
    if p == a.len() && p == b.len() {
        return None;
    }
    let rest = max - p;
    let mut s = 0;
    while s + BLOCK <= rest
        && a[a.len() - s - BLOCK..a.len() - s] == b[b.len() - s - BLOCK..b.len() - s]
    {
        s += BLOCK;
    }
    while s < rest && a[a.len() - s - 1] == b[b.len() - s - 1] {
        s += 1;
    }
    // Keep the edit on character boundaries; the shared prefix and
    // suffix have the same boundaries in both texts.
    while !new.is_char_boundary(p) {
        p -= 1;
    }
    let (mut old_end, mut new_end) = (a.len() - s, b.len() - s);
    while !new.is_char_boundary(new_end) || !old.is_char_boundary(old_end) {
        old_end += 1;
        new_end += 1;
    }
    Some((p, old_end, new_end))
}

fn count_lines(bytes: &[u8]) -> usize {
    bytes.iter().filter(|&&b| b == b'\n').count()
}

/// The point at byte `at` of `text`, given the point at byte `from`.
fn advance(text: &[u8], from: usize, at: Point, to: usize) -> Point {
    let part = &text[from..to];
    match part.iter().rposition(|&b| b == b'\n') {
        Some(i) => Point::new(at.row + count_lines(part), part.len() - i - 1),
        None => Point::new(at.row, at.column + part.len()),
    }
}

fn input_edit(old: &str, new: &str, (start, old_end, new_end): (usize, usize, usize)) -> InputEdit {
    let start_position = advance(new.as_bytes(), 0, Point::new(0, 0), start);
    InputEdit {
        start_byte: start,
        old_end_byte: old_end,
        new_end_byte: new_end,
        start_position,
        old_end_position: advance(old.as_bytes(), start, start_position, old_end),
        new_end_position: advance(new.as_bytes(), start, start_position, new_end),
    }
}

/// Parse `ranges` of `source` (all of it when empty), reusing `old`
/// (already edited) when given. Cancellable. `None` when Tree-sitter
/// refuses the ranges.
pub(crate) fn parse(
    parser: &mut Parser,
    language: &tree_sitter::Language,
    source: &[u8],
    ranges: &[TsRange],
    old: Option<&Tree>,
    cancel: Option<&AtomicUsize>,
) -> Result<Option<Tree>, HighlightError> {
    parser
        .set_language(language)
        .map_err(|e| HighlightError::Failed(e.to_string()))?;
    if parser.set_included_ranges(ranges).is_err() {
        return Ok(None);
    }
    let mut cancelled = false;
    let mut progress = |_: &tree_sitter::ParseState| {
        if cancel.is_some_and(|c| c.load(Ordering::Relaxed) != 0) {
            cancelled = true;
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    };
    let len = source.len();
    let tree = parser.parse_with_options(
        &mut |i, _| if i < len { &source[i..] } else { &[] },
        old,
        Some(ParseOptions::new().progress_callback(&mut progress)),
    );
    // The parser keeps partial state after an interruption.
    parser.reset();
    match tree {
        Some(tree) => Ok(Some(tree)),
        None if cancelled => Err(HighlightError::Cancelled),
        None => Err(HighlightError::Failed("parsing failed".into())),
    }
}

/// `old` spans (of the text before the edit) outside `region` (of the
/// text after it), shifted past the edit, with `fresh` spans for
/// `region` in between. Adjacent spans of one style are joined, as a
/// full highlight would produce them.
fn splice(old: &[Span], fresh: Vec<Span>, region: &Range<usize>, edit: &InputEdit) -> Vec<Span> {
    let delta = edit.new_end_byte as isize - edit.old_end_byte as isize;
    // The region in old coordinates; it contains the whole edit.
    let old_end = (region.end as isize - delta) as usize;
    let mut out: Vec<Span> = Vec::with_capacity(old.len() + fresh.len());
    let push = |out: &mut Vec<Span>, s: Span| {
        if s.range.is_empty() {
            return;
        }
        match out.last_mut() {
            Some(last) if last.style == s.style && last.range.end == s.range.start => {
                last.range.end = s.range.end;
            }
            _ => out.push(s),
        }
    };
    let before = old.partition_point(|s| s.range.start < region.start);
    for s in &old[..before] {
        push(
            &mut out,
            Span {
                range: s.range.start..s.range.end.min(region.start),
                style: s.style,
            },
        );
    }
    for s in fresh {
        push(&mut out, s);
    }
    let after = old.partition_point(|s| s.range.end <= old_end);
    for s in &old[after..] {
        let start = s.range.start.max(old_end);
        push(
            &mut out,
            Span {
                range: (start as isize + delta) as usize..(s.range.end as isize + delta) as usize,
                style: s.style,
            },
        );
    }
    out
}

impl Highlighter {
    /// Parse `source` with everything injected into it and highlight it
    /// from scratch.
    pub(crate) fn analyze_fresh(
        &mut self,
        lang: Lang,
        source: &[u8],
        cancel: Option<&AtomicUsize>,
    ) -> Result<(Layer, Vec<Span>), HighlightError> {
        let mut ctx = Ctx {
            hl: self,
            source,
            cancel,
            host_name: lang.name(),
            spots: Vec::new(),
        };
        let mut top = ctx.build_top(lang)?;
        let (spans, _) =
            layers::highlight(&mut top, &mut self.cursor, source, 0..source.len(), cancel)?;
        Ok((top, spans))
    }

    /// Highlight and fold `text`. With `prev`, the analysis of an earlier
    /// version of the same document, the trees are updated incrementally
    /// and only the region that may have changed is highlighted again.
    /// Without `prev` the result equals [`Self::highlight`] and
    /// [`Self::folds`]; with it, see the module documentation and
    /// [`Syntax::is_fresh`]. Cancellable like [`Self::highlight`]; `prev`
    /// is left untouched either way.
    pub fn analyze(
        &mut self,
        lang: Lang,
        text: String,
        prev: Option<&Syntax>,
        cancel: Option<&AtomicUsize>,
    ) -> Result<Syntax, HighlightError> {
        let prev = prev.filter(|p| p.lang == lang);
        let change = match prev {
            Some(p) => match diff(&p.text, &text) {
                Some(d) => Some((p, input_edit(&p.text, &text, d))),
                None => {
                    return Ok(Syntax {
                        lang,
                        text,
                        top: p.top.clone(),
                        spans: p.spans.clone(),
                        folds: p.folds.clone(),
                        rehighlighted: 0..0,
                        incremental: p.incremental,
                    });
                }
            },
            None => None,
        };
        let source = text.as_bytes();
        let Some((p, edit)) = change else {
            let (top, spans) = self.analyze_fresh(lang, source, cancel)?;
            let folds = folds::collect(lang, top.tree.root_node(), source);
            return Ok(Syntax {
                lang,
                rehighlighted: 0..text.len(),
                text,
                top,
                spans,
                folds,
                incremental: false,
            });
        };
        let mut top = p.top.clone();
        let mut ctx = Ctx {
            hl: self,
            source,
            cancel,
            host_name: lang.name(),
            spots: Vec::new(),
        };
        ctx.update_top(&mut top, &edit)?;
        let spots = std::mem::take(&mut ctx.spots);
        let folds = folds::collect(lang, top.tree.root_node(), source);
        let mut region = edit.start_byte..edit.new_end_byte;
        for s in spots {
            region.start = region.start.min(s.start);
            region.end = region.end.max(s.end);
        }
        let mut region = whole_lines(source, region);
        let full = 0..source.len();
        let (spans, rehighlighted) = loop {
            if region.len() * FULL_FRACTION >= source.len() {
                let (spans, _) =
                    layers::highlight(&mut top, &mut self.cursor, source, full.clone(), cancel)?;
                break (spans, full);
            }
            let (fresh, extent) =
                layers::highlight(&mut top, &mut self.cursor, source, region.clone(), cancel)?;
            // A highlighted node reaching outside the region may have
            // changed outside it too: widen and repeat.
            if extent.start < region.start || extent.end > region.end {
                region = whole_lines(
                    source,
                    region.start.min(extent.start)..region.end.max(extent.end),
                );
                continue;
            }
            break (splice(&p.spans, fresh, &region, &edit), region);
        };
        Ok(Syntax {
            lang,
            text,
            top,
            spans,
            folds,
            rehighlighted,
            incremental: true,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tree_sitter::QueryCursor;

    /// Deterministic pseudo-random numbers (xorshift).
    struct Rng(u64);
    impl Rng {
        fn next(&mut self) -> u64 {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            self.0
        }
        fn below(&mut self, n: usize) -> usize {
            (self.next() % n.max(1) as u64) as usize
        }
    }

    fn sample(lang: Lang) -> String {
        let unit = match lang {
            Lang::Rust => {
                "/// Doc {i}.\n#[derive(Debug)]\npub struct S{i} { a: u32, b: String }\n\nimpl S{i} {\n    fn f(&self, x: &str) -> Option<u32> {\n        // note\n        let v = vec![1, 2, 3];\n        if x == \"q{i}\" { return Some(self.a + 0x1f); }\n        println!(\"{}\", v.len()); /* block */ None\n    }\n}\n\n"
            }
            Lang::Json => {
                "  \"k{i}\": {\"a\": [1, 2.5, true, null], \"s\": \"text \\n {i}\", \"o\": {\"x\": false}},\n"
            }
            Lang::Python => {
                "class C{i}(Base):\n    \"\"\"Doc.\"\"\"\n    def f(self, x=1):\n        # c\n        if x > 2:\n            return f\"v{x}\"\n        return [i for i in range(3)]\n\n"
            }
            Lang::C => {
                "#include <stdio.h>\n/* c */\nstatic int f{i}(int x) {\n    if (x > 1) { return x * 2; }\n    printf(\"%d\\n\", x); // n\n    return 0;\n}\n\n"
            }
            Lang::Yaml => {
                "key{i}:\n  name: \"v\" # c\n  list:\n    - 1\n    - true\n  other: text\n"
            }
            Lang::Toml => {
                "[table{i}]\nkey = \"v\" # c\nnum = 1_000\narr = [1, 2]\ndate = 1979-05-27\n\n"
            }
            Lang::Css => {
                "/* c */\n.a{i} > b:hover { color: #fff; margin: 0 1px !important; }\n@media (max-width: 10px) { .x { top: 0; } }\n"
            }
            Lang::Go => {
                "// c\nfunc F{i}(x int) (string, error) {\n\tif x > 1 {\n\t\treturn \"s\", nil\n\t}\n\treturn fmt.Sprintf(\"%d\", x), nil\n}\n\n"
            }
            Lang::Bash => {
                "# c\nf{i}() {\n  local x=\"$1\"\n  if [ -n \"$x\" ]; then echo \"hi $x\"; fi\n}\n\n"
            }
            Lang::Lua => {
                "-- c\nlocal function f{i}(x)\n  if x then\n    return { a = 1, 'b' }\n  end\nend\n\n"
            }
            Lang::JavaScript => {
                "// c {i}\nconst a{i} = html`<p class=\"x\">${v}</p>`;\nconst r{i} = /a+b/g;\nfunction f{i}(x) { return html`<b>${x}</b>` + x; }\n\n"
            }
            Lang::TypeScript => {
                "// c\nexport class A{i} implements B {\n  private x: number = 1;\n  f(y: string): string { return `t${y}`; }\n}\n\n"
            }
            Lang::Html => {
                "<div class=\"a{i}\">\n  <p>t &amp; u</p>\n  <script>\n    const n = {i};\n    let s = `a${n}`;\n  </script>\n  <style>a { color: red; }</style>\n</div>\n"
            }
            Lang::Php => {
                "<p class=\"c{i}\">{i}</p>\n<?php\nfunction f{i}($a) {\n  return 'x' . $a; // c\n}\n$h = <<<HTML\n<b>{i}</b>\nHTML;\n?>\n<script>let a = {i};</script>\n<a href=\"<?php echo $u; ?>\">l</a>\n"
            }
            Lang::Markdown => {
                "# Title {i}\n\nSome *text* and `code` with <br> html.\n\n```rust\nfn f() {}\n```\n\n- item\n\n<div>\n<i>x</i>\n</div>\n\n```html\n<script>let x = 1;</script>\n```\n\n"
            }
            _ => "x{i}\n",
        };
        let mut s = String::new();
        if lang == Lang::Markdown {
            s.push_str("---\ntitle: x\n---\n\n");
        }
        if lang == Lang::Json {
            s.push_str("{\n");
        }
        for i in 0..30 {
            s.push_str(&unit.replace("{i}", &i.to_string()));
        }
        if lang == Lang::Json {
            s.push_str("  \"end\": 0\n}\n");
        }
        s
    }

    const TOKENS: [&str; 30] = [
        "x", " ", "\n", "{", "}", "(", ")", "[", "]", "\"", "'", "`", "/*", "*/", "//", "#", "<",
        ">", "</p>", ":", ",", ";", "fn ", "if ", "= ", "-", "ä", "🙂", "\n    ", "end",
    ];

    fn char_boundary(text: &str, at: usize) -> usize {
        let mut at = at.min(text.len());
        while !text.is_char_boundary(at) {
            at -= 1;
        }
        at
    }

    /// Byte offsets where lines start.
    fn line_starts(text: &str) -> Vec<usize> {
        std::iter::once(0)
            .chain(text.match_indices('\n').map(|(i, _)| i + 1))
            .filter(|&i| i < text.len())
            .collect()
    }

    /// Text that opens or closes injections.
    const INJECTING: [&str; 16] = [
        "```",
        "```rust\n",
        "\n```\n",
        "```py\n",
        "<script>",
        "</script>",
        "<style>",
        "<?php ",
        " ?>",
        "html`<b>${x}</b>`",
        "css`",
        "<b>",
        "*",
        "\n---\n",
        "<<<HTML\n",
        "<!--",
    ];

    fn mutate(rng: &mut Rng, lang: Lang, text: &mut String) {
        let at = char_boundary(text, rng.below(text.len() + 1));
        let lines = line_starts(text);
        let line = |i: usize| {
            let start = lines.get(i).copied().unwrap_or(text.len());
            let end = lines.get(i + 1).copied().unwrap_or(text.len());
            start..end
        };
        match rng.below(40) {
            0 => {
                // Paste a copy of some text elsewhere.
                let from = char_boundary(text, rng.below(text.len() + 1));
                let to = char_boundary(text, from + rng.below(300));
                let chunk = text[from..to].to_owned();
                text.insert_str(at, &chunk);
            }
            1 => {
                // Delete a larger block.
                let to = char_boundary(text, at + rng.below(400));
                text.replace_range(at..to, "");
            }
            2..=7 => {
                let to = char_boundary(text, at + 1 + rng.below(4));
                text.replace_range(at..to, "");
            }
            8..=9 => {
                let to = char_boundary(text, at + 1 + rng.below(6));
                text.replace_range(at..to, TOKENS[rng.below(TOKENS.len())]);
            }
            10..=11 => {
                // Duplicate a line somewhere else.
                let copy = text[line(rng.below(lines.len()))].to_owned();
                let to = line(rng.below(lines.len())).start;
                text.insert_str(to, &copy);
            }
            12..=13 => {
                let r = line(rng.below(lines.len()));
                text.replace_range(r, "");
            }
            14..=15 => {
                // Comment a line out, or back in.
                let r = line(rng.below(lines.len()));
                let prefix = lang.line_comment().unwrap_or("#");
                let body = text[r.clone()].to_owned();
                let indent = body.len() - body.trim_start().len();
                if body.trim_start().starts_with(prefix) {
                    text.replace_range(r.start + indent..r.start + indent + prefix.len(), "");
                } else {
                    text.insert_str(r.start + indent, prefix);
                }
            }
            16..=25 => {
                // Type inside a word, as most typing does.
                let word = text[at..]
                    .find(|c: char| c.is_alphanumeric())
                    .map_or(at, |i| at + i);
                text.insert(word, if rng.below(2) == 0 { 'z' } else { '7' });
            }
            36..=37 => text.insert_str(at, INJECTING[rng.below(INJECTING.len())]),
            38 => {
                // Open or close an injection at the start of a line.
                let r = line(rng.below(lines.len()));
                text.insert_str(r.start, INJECTING[rng.below(INJECTING.len())]);
            }
            _ => text.insert_str(at, TOKENS[rng.below(TOKENS.len())]),
        }
    }

    const LANGS: [Lang; 15] = [
        Lang::Rust,
        Lang::Json,
        Lang::Python,
        Lang::C,
        Lang::Yaml,
        Lang::Toml,
        Lang::Css,
        Lang::Go,
        Lang::Bash,
        Lang::Lua,
        Lang::TypeScript,
        // With injections.
        Lang::Html,
        Lang::JavaScript,
        Lang::Php,
        Lang::Markdown,
    ];

    /// Full highlights of the layers `syn` has.
    fn full_highlight(syn: &Syntax) -> Vec<Span> {
        let mut top = syn.top.clone();
        let source = syn.text.as_bytes();
        layers::highlight(
            &mut top,
            &mut QueryCursor::new(),
            source,
            0..source.len(),
            None,
        )
        .unwrap()
        .0
    }

    /// Counts per language from [`check_random_edits`].
    #[derive(Debug, Default)]
    struct Counts {
        partial: usize,
        diverged: usize,
        checked: usize,
    }

    /// Apply `steps` random edits per seed and language, analysing each
    /// version incrementally, and check after each:
    ///
    /// - the spans equal a full highlight of the same layers, so partial
    ///   highlighting and splicing lose nothing (asserted for trees
    ///   without syntax errors, where Tree-sitter's changed ranges are
    ///   complete);
    /// - when every layer equals a fresh analysis, spans and folds equal
    ///   the fresh ones and the spans equal `tree_sitter_highlight`'s.
    ///
    /// Otherwise the chain sometimes goes on from the incremental result
    /// (more typing) and sometimes from a fresh analysis (the worker
    /// settling it). Every few steps the text goes back to its last
    /// version without syntax errors, as when a typo is fixed.
    fn check_random_edits(seeds: u64, steps: usize) -> Vec<(Lang, Counts)> {
        let mut h = Highlighter::new();
        let mut fresh = Highlighter::new();
        let mut out = Vec::new();
        for (n, lang) in LANGS.into_iter().enumerate() {
            let mut counts = Counts::default();
            for seed in 0..seeds {
                let mut rng = Rng(0x9e37_79b9_7f4a_7c15 ^ ((n as u64 + 1) * 0x1000_0001 + seed));
                let mut text = sample(lang);
                let mut good = text.clone();
                let mut syn = h.analyze(lang, text.clone(), None, None).unwrap();
                for step in 0..steps {
                    if rng.below(2) == 0 {
                        text.clone_from(&good);
                    }
                    mutate(&mut rng, lang, &mut text);
                    syn = h.analyze(lang, text.clone(), Some(&syn), None).unwrap();
                    let at = format!("{lang:?} seed {seed} step {step}");
                    let errors = syn.top.has_error();
                    if !errors {
                        let full = full_highlight(&syn);
                        assert!(
                            syn.spans() == full.as_slice(),
                            "{at}: spans differ from a full highlight of the layers\n{}",
                            first_difference(syn.spans(), &full, &text)
                        );
                    }
                    let new = fresh.analyze(lang, text.clone(), None, None).unwrap();
                    if errors {
                        // Best effort until settled: Tree-sitter's changed
                        // ranges can miss changes inside error nodes.
                        if rng.below(2) == 0 {
                            syn = new;
                        }
                    } else if new.top.fingerprint() == syn.top.fingerprint() {
                        let reference = fresh.reference(lang, text.as_bytes());
                        assert!(
                            new.spans() == reference.as_slice(),
                            "{at}: fresh spans differ from tree_sitter_highlight\n{}",
                            first_difference(new.spans(), &reference, &text)
                        );
                        assert!(
                            syn.spans() == new.spans(),
                            "{at}: spans differ from fresh\n{}",
                            first_difference(syn.spans(), new.spans(), &text)
                        );
                        assert_eq!(syn.folds(), new.folds(), "{at}: folds");
                        counts.checked += 1;
                        if syn.rehighlighted().len() < text.len() {
                            counts.partial += 1;
                        }
                    } else {
                        counts.diverged += 1;
                        if rng.below(2) == 0 {
                            syn = new;
                        }
                    }
                    if !errors {
                        good.clone_from(&text);
                    }
                }
            }
            out.push((lang, counts));
        }
        out
    }

    #[test]
    fn incremental_equals_fresh_after_random_edits() {
        for (lang, c) in check_random_edits(2, 60) {
            // Not vacuous: many updates were checked and partial.
            assert!(c.partial >= 5, "{lang:?}: {c:?}");
        }
    }

    /// The same with many more edits: `cargo test --release -p
    /// sanain-language -- --ignored stress --nocapture`.
    #[test]
    #[ignore = "slow; run in release"]
    fn stress_incremental_equals_fresh() {
        for (lang, c) in check_random_edits(20, 300) {
            eprintln!(
                "{lang:?}: {} checked, {} partial, {} trees unlike a fresh parse",
                c.checked, c.partial, c.diverged
            );
        }
    }

    fn first_difference(a: &[Span], b: &[Span], text: &str) -> String {
        let i = a
            .iter()
            .zip(b)
            .position(|(x, y)| x != y)
            .unwrap_or(a.len().min(b.len()));
        let show = |s: Option<&Span>| {
            s.map(|s| {
                format!(
                    "{:?} {:?} {:?}",
                    s.range,
                    s.style,
                    text.get(s.range.clone())
                )
            })
        };
        format!(
            "incremental {:?}\nfresh       {:?}",
            show(a.get(i)),
            show(b.get(i))
        )
    }

    #[test]
    fn fresh_analysis_matches_tree_sitter_highlight_for_every_language() {
        let mut h = Highlighter::new();
        for lang in Lang::ALL {
            let text = sample(lang);
            let syn = h.analyze(lang, text.clone(), None, None).unwrap();
            let spans = Highlighter::new().reference(lang, text.as_bytes());
            assert!(
                syn.spans() == spans.as_slice(),
                "{lang:?}\n{}",
                first_difference(syn.spans(), &spans, &text)
            );
        }
    }

    #[test]
    fn small_edit_rehighlights_only_nearby_lines() {
        let mut h = Highlighter::new();
        let mut text = sample(Lang::Rust);
        let syn = h.analyze(Lang::Rust, text.clone(), None, None).unwrap();
        let at = text.find("q5").unwrap();
        text.insert(at, 'z');
        let next = h
            .analyze(Lang::Rust, text.clone(), Some(&syn), None)
            .unwrap();
        let r = next.rehighlighted();
        assert!(r.contains(&at) && r.len() < 400, "{r:?}");
        // Unchanged text: nothing to do.
        let same = h
            .analyze(Lang::Rust, text.clone(), Some(&next), None)
            .unwrap();
        assert_eq!(same.rehighlighted(), 0..0);
        assert_eq!(same.spans(), next.spans());
        // Another language: analysed from scratch.
        let other = h.analyze(Lang::C, text.clone(), Some(&next), None).unwrap();
        assert_eq!(other.rehighlighted(), 0..text.len());
    }

    #[test]
    fn cancelled_update_leaves_previous_state_usable() {
        let mut h = Highlighter::new();
        let text = sample(Lang::Rust).repeat(20);
        let syn = h.analyze(Lang::Rust, text.clone(), None, None).unwrap();
        let flag = AtomicUsize::new(1);
        let edited = format!("{text}fn tail() {{}}\n");
        assert_eq!(
            h.analyze(Lang::Rust, edited.clone(), Some(&syn), Some(&flag))
                .unwrap_err(),
            HighlightError::Cancelled
        );
        let next = h
            .analyze(Lang::Rust, edited.clone(), Some(&syn), None)
            .unwrap();
        let spans = h.highlight(Lang::Rust, edited.as_bytes(), None).unwrap();
        assert_eq!(next.spans(), spans.as_slice());
    }

    #[test]
    fn diff_finds_one_edit_on_char_boundaries() {
        assert_eq!(diff("abc", "abc"), None);
        assert_eq!(diff("abc", "abxc"), Some((2, 2, 3)));
        assert_eq!(diff("abc", "ac"), Some((1, 2, 1)));
        assert_eq!(diff("", "x"), Some((0, 0, 1)));
        // "ä" and "ö" share their first byte; the edit covers whole chars.
        assert_eq!(diff("aä", "aö"), Some((1, 3, 3)));
        let long = "x".repeat(1000);
        assert_eq!(
            diff(&format!("{long}a{long}"), &format!("{long}{long}")),
            Some((1000, 1001, 1000))
        );
        let e = input_edit(
            "a\nb\nc",
            "a\nbX\n\nc",
            diff("a\nb\nc", "a\nbX\n\nc").unwrap(),
        );
        assert_eq!(e.start_position, Point::new(1, 1));
        assert_eq!(e.old_end_position, Point::new(1, 1));
        assert_eq!(e.new_end_position, Point::new(2, 0));
    }
}
