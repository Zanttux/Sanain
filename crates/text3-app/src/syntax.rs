//! Background syntax highlighting. The UI thread submits a document
//! snapshot tagged with its revision; a worker thread highlights it and
//! sends back char-range spans and syntax fold ranges. Newer submissions
//! cancel running work, and results for old revisions are recognisable by
//! their revision tag.
//!
//! The worker keeps the last analysis ([`Syntax`]) of a few recently
//! highlighted documents and updates it incrementally from the next
//! snapshot (see [`Highlighter::analyze`]). An incremental result can
//! differ from a fresh one in broken or ambiguous code, so once no request
//! has come for [`SETTLE_AFTER`], the worker analyses such a document
//! again from scratch and sends that result under the same revision if it
//! differs. When it does not, the incremental state is kept: Tree-sitter
//! updates an incrementally parsed tree faster than a fresh one (about
//! 50 ms against 10 ms for the first keystroke in 500 KB of Markdown).

use std::collections::HashMap;
use std::ops::Range;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender, TryRecvError, channel};
use std::thread;
use std::time::Duration;

use text3_core::Revision;
use text3_language::{HighlightError, Highlighter, Lang, Span, Style, Syntax};

use crate::workspace::TabId;

/// Documents larger than this are not highlighted (plain text).
pub const MAX_BYTES: usize = 8 * 1024 * 1024;

/// Analyses kept for incremental updates, least recently used dropped.
const KEEP: usize = 4;

/// Idle time after which incremental results are replaced by fresh ones.
pub const SETTLE_AFTER: Duration = Duration::from_millis(300);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharSpan {
    pub range: Range<usize>,
    pub style: Style,
}

#[derive(Debug)]
struct Request {
    tab: TabId,
    revision: Revision,
    lang: Lang,
    /// Converted to a string on the worker thread.
    rope: ropey::Rope,
}

#[derive(Debug)]
enum Msg {
    Highlight(Request),
    /// The tab closed: drop what the worker keeps for it.
    Forget(TabId),
}

#[derive(Debug)]
pub struct Highlighted {
    pub tab: TabId,
    pub revision: Revision,
    pub spans: Vec<CharSpan>,
    /// Fold ranges in document lines (see
    /// [`text3_language::Highlighter::folds`]), or `None` when the syntax
    /// tree cannot provide them and indentation folding applies.
    pub folds: Option<Vec<Range<usize>>>,
}

#[derive(Debug)]
pub struct SyntaxWorker {
    tx: Sender<Msg>,
    rx: Receiver<Highlighted>,
    cancel: Arc<AtomicUsize>,
}

impl Default for SyntaxWorker {
    fn default() -> Self {
        Self::start()
    }
}

impl SyntaxWorker {
    pub fn start() -> Self {
        let (tx, jobs) = channel::<Msg>();
        let (out, rx) = channel();
        let cancel = Arc::new(AtomicUsize::new(0));
        let flag = cancel.clone();
        let spawned = thread::Builder::new()
            .name("text3-syntax".into())
            .spawn(move || run(jobs, out, flag));
        if let Err(e) = spawned {
            eprintln!("text3: syntax highlighting disabled: {e}");
        }
        Self { tx, rx, cancel }
    }

    /// Queue a snapshot for highlighting; cancels work in progress.
    pub fn submit(&self, tab: TabId, revision: Revision, lang: Lang, rope: ropey::Rope) {
        if rope.len_bytes() > MAX_BYTES {
            return;
        }
        self.cancel.store(1, Ordering::Relaxed);
        let _ = self.tx.send(Msg::Highlight(Request {
            tab,
            revision,
            lang,
            rope,
        }));
    }

    /// Forget a closed tab's syntax state.
    pub fn forget(&self, tab: TabId) {
        let _ = self.tx.send(Msg::Forget(tab));
    }

    pub fn poll(&self) -> Vec<Highlighted> {
        self.rx.try_iter().collect()
    }
}

/// What the worker keeps of a highlighted document.
struct Kept {
    revision: Revision,
    syntax: Syntax,
    /// Whether syntax folds apply (Tree-sitter's lines are the editor's).
    folds_apply: bool,
    /// Whether the result equals a fresh analysis.
    settled: bool,
    /// For least-recently-used eviction.
    used: u64,
}

fn run(jobs: Receiver<Msg>, out: Sender<Highlighted>, cancel: Arc<AtomicUsize>) {
    let mut highlighter = Highlighter::new();
    // Latest pending request per tab, so bursts of edits collapse.
    let mut pending: HashMap<TabId, Request> = HashMap::new();
    let mut kept: HashMap<TabId, Kept> = HashMap::new();
    let mut clock = 0u64;
    let take = |msg: Msg,
                pending: &mut HashMap<TabId, Request>,
                kept: &mut HashMap<TabId, Kept>| {
        match msg {
            Msg::Highlight(r) => {
                pending.insert(r.tab, r);
            }
            Msg::Forget(tab) => {
                pending.remove(&tab);
                kept.remove(&tab);
            }
        }
    };
    loop {
        loop {
            match jobs.try_recv() {
                Ok(m) => take(m, &mut pending, &mut kept),
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => return,
            }
        }
        if pending.is_empty() {
            // Idle: wait for work; after a pause, settle an incremental
            // result.
            let unsettled = kept
                .iter()
                .filter(|(_, k)| !k.settled)
                .max_by_key(|(_, k)| k.used)
                .map(|(&tab, _)| tab);
            let Some(tab) = unsettled else {
                match jobs.recv() {
                    Ok(m) => take(m, &mut pending, &mut kept),
                    Err(_) => return,
                }
                continue;
            };
            match jobs.recv_timeout(SETTLE_AFTER) {
                Ok(m) => take(m, &mut pending, &mut kept),
                Err(RecvTimeoutError::Timeout) => {
                    cancel.store(0, Ordering::Relaxed);
                    if !settle(&mut highlighter, &mut kept, tab, &out, &cancel) {
                        return;
                    }
                }
                Err(RecvTimeoutError::Disconnected) => return,
            }
            continue;
        }
        let Some(&tab) = pending.keys().next() else {
            continue;
        };
        let Some(req) = pending.remove(&tab) else {
            continue;
        };
        cancel.store(0, Ordering::Relaxed);
        let prev = kept.get(&req.tab).map(|k| &k.syntax);
        match highlighter.analyze(req.lang, req.rope.to_string(), prev, Some(&cancel)) {
            Ok(syntax) => {
                // Tree-sitter counts lines by `\n` only; skip syntax folds
                // when the document also breaks lines elsewhere (lone
                // `\r`, Unicode separators).
                let folds_apply = req.rope.len_lines() == line_count(syntax.text());
                let result = Highlighted {
                    tab: req.tab,
                    revision: req.revision,
                    spans: to_char_spans(syntax.text(), syntax.spans()),
                    folds: folds_apply.then(|| syntax.folds().to_vec()),
                };
                if out.send(result).is_err() {
                    return;
                }
                clock += 1;
                kept.insert(
                    req.tab,
                    Kept {
                        revision: req.revision,
                        settled: syntax.is_fresh(),
                        syntax,
                        folds_apply,
                        used: clock,
                    },
                );
                while kept.len() > KEEP {
                    let Some(oldest) = kept.iter().min_by_key(|(_, k)| k.used).map(|(&t, _)| t)
                    else {
                        break;
                    };
                    kept.remove(&oldest);
                }
            }
            // Interrupted by a newer submission: retry unless superseded.
            Err(HighlightError::Cancelled) => {
                pending.entry(req.tab).or_insert(req);
            }
            Err(HighlightError::Failed(e)) => {
                eprintln!("text3: highlighting failed: {e}");
                kept.remove(&req.tab);
            }
        }
    }
}

/// Analyse `tab`'s kept text from scratch and send the result if it
/// differs from the incremental one. Returns false when the receiving
/// side is gone.
fn settle(
    highlighter: &mut Highlighter,
    kept: &mut HashMap<TabId, Kept>,
    tab: TabId,
    out: &Sender<Highlighted>,
    cancel: &AtomicUsize,
) -> bool {
    let Some(k) = kept.get_mut(&tab) else {
        return true;
    };
    let lang = k.syntax.lang();
    match highlighter.analyze(lang, k.syntax.text().to_owned(), None, Some(cancel)) {
        Ok(fresh) => {
            if fresh.spans() != k.syntax.spans() || fresh.folds() != k.syntax.folds() {
                let result = Highlighted {
                    tab,
                    revision: k.revision,
                    spans: to_char_spans(fresh.text(), fresh.spans()),
                    folds: k.folds_apply.then(|| fresh.folds().to_vec()),
                };
                if out.send(result).is_err() {
                    return false;
                }
                k.syntax = fresh;
            }
            // Same result: the incremental state is kept for its faster
            // updates.
            k.settled = true;
        }
        // A request came in; it is served first and settling starts over.
        Err(HighlightError::Cancelled) => {}
        Err(HighlightError::Failed(e)) => {
            eprintln!("text3: highlighting failed: {e}");
            kept.remove(&tab);
        }
    }
    true
}

/// Lines as Tree-sitter counts them.
fn line_count(text: &str) -> usize {
    text.bytes().filter(|&b| b == b'\n').count() + 1
}

/// Convert ascending byte-range spans into char ranges in one pass.
fn to_char_spans(text: &str, spans: &[Span]) -> Vec<CharSpan> {
    let (mut byte, mut chr) = (0usize, 0usize);
    let mut out = Vec::with_capacity(spans.len());
    for s in spans {
        if s.range.start < byte {
            continue; // not ascending; skip defensively
        }
        chr += text[byte..s.range.start].chars().count();
        let start = chr;
        chr += text[s.range.clone()].chars().count();
        byte = s.range.end;
        out.push(CharSpan {
            range: start..chr,
            style: s.style,
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    fn wait(worker: &SyntaxWorker) -> Highlighted {
        let t = Instant::now();
        loop {
            if let Some(h) = worker.poll().into_iter().last() {
                return h;
            }
            assert!(t.elapsed() < Duration::from_secs(10), "no result");
            thread::sleep(Duration::from_millis(5));
        }
    }

    #[test]
    fn highlights_in_background_with_char_offsets() {
        let w = SyntaxWorker::start();
        // "ä" is two bytes; the keyword after it must start at char 2.
        w.submit(7, Revision(3), Lang::Rust, "//ä\nfn x() {}".into());
        let h = wait(&w);
        assert_eq!((h.tab, h.revision), (7, Revision(3)));
        let kw = h.spans.iter().find(|s| s.style == Style::Keyword).unwrap();
        assert_eq!(kw.range, 4..6);
    }

    #[test]
    fn folds_come_with_spans() {
        let w = SyntaxWorker::start();
        w.submit(1, Revision(1), Lang::Json, "{\n}\n".into());
        assert_eq!(wait(&w).folds, Some(vec![]), "nothing to hide");
        w.submit(
            1,
            Revision(2),
            Lang::Json,
            "{\n  \"a\": 1,\n  \"b\": 2\n}\n".into(),
        );
        assert_eq!(
            wait(&w).folds.as_deref(),
            Some(std::slice::from_ref(&(0..3)))
        );
        // A lone CR is a line break for the editor but not for Tree-sitter.
        w.submit(
            1,
            Revision(3),
            Lang::Json,
            "{\r  \"a\": 1,\n  \"b\": 2\n}\n".into(),
        );
        assert_eq!(wait(&w).folds, None);
    }

    fn fresh(lang: Lang, text: &str) -> Vec<CharSpan> {
        let spans = Highlighter::new()
            .highlight(lang, text.as_bytes(), None)
            .unwrap();
        to_char_spans(text, &spans)
    }

    /// The last result for `rev` once the worker has settled it.
    fn settled(worker: &SyntaxWorker, rev: Revision) -> Highlighted {
        let mut last = None;
        let t = Instant::now();
        while t.elapsed() < SETTLE_AFTER * 3 || last.is_none() {
            for h in worker.poll() {
                if h.revision == rev {
                    last = Some(h);
                }
            }
            assert!(t.elapsed() < Duration::from_secs(10), "no result");
            thread::sleep(Duration::from_millis(5));
        }
        last.unwrap()
    }

    #[test]
    fn edits_update_incrementally_and_match_a_fresh_highlight() {
        let w = SyntaxWorker::start();
        let base = "fn a() -> u32 { 1 }\n// c\nstruct S { x: String }\n".repeat(50);
        let mut rope = ropey::Rope::from_str(&base);
        w.submit(1, Revision(1), Lang::Rust, rope.clone());
        wait(&w);
        // Typing that passes through invalid code (`"` opens a string),
        // then deleting it again.
        let at = rope.line_to_char(20);
        let mut rev = 1;
        for (insert, text) in [(true, "let"), (true, " \""), (false, ""), (true, "ä")] {
            rev += 1;
            if insert {
                rope.insert(at, text);
            } else {
                rope.remove(at + 3..at + 5);
            }
            w.submit(1, Revision(rev), Lang::Rust, rope.clone());
            let h = settled(&w, Revision(rev));
            assert_eq!(h.spans, fresh(Lang::Rust, &rope.to_string()), "rev {rev}");
        }
        // A closed tab's state is dropped; highlighting it again works.
        w.forget(1);
        w.submit(1, Revision(rev + 1), Lang::Rust, rope.clone());
        assert_eq!(wait(&w).spans, fresh(Lang::Rust, &rope.to_string()));
    }

    #[test]
    fn latest_submission_wins() {
        let w = SyntaxWorker::start();
        let big = "fn f() {}\n".repeat(20_000);
        w.submit(1, Revision(1), Lang::Rust, ropey::Rope::from_str(&big));
        w.submit(1, Revision(2), Lang::Rust, "fn g() {}".into());
        let t = Instant::now();
        let mut last = None;
        while t.elapsed() < Duration::from_secs(10) {
            for h in w.poll() {
                last = Some(h.revision);
            }
            if last == Some(Revision(2)) {
                break;
            }
            thread::sleep(Duration::from_millis(5));
        }
        assert_eq!(last, Some(Revision(2)));
    }
}

/// Typing latency in large files: time from submitting a one-character
/// edit to its highlight result. Release build:
/// `cargo test --release -p text3-app --lib -- --ignored --nocapture bench_`
#[cfg(test)]
mod bench {
    use super::*;
    use std::time::{Duration, Instant};

    /// About `bytes` of varied Rust source.
    pub(crate) fn rust_source(bytes: usize) -> String {
        let mut s =
            String::from("//! Synthetic benchmark source.\nuse std::collections::HashMap;\n\n");
        let mut i = 0usize;
        while s.len() < bytes {
            s.push_str(&format!(
                "/// Item {i}.\n#[derive(Debug, Clone)]\npub struct Item{i} {{\n    name: String,\n    count: u32,\n}}\n\nimpl Item{i} {{\n    pub fn new(name: &str) -> Self {{\n        let mut m: HashMap<&str, u32> = HashMap::new();\n        m.insert(\"k{i}\", {i});\n        // Count the entries.\n        Self {{ name: format!(\"{{name}}-{i}\"), count: m.len() as u32 + 0x{i:x} }}\n    }}\n}}\n\n"
            ));
            i += 1;
        }
        s
    }

    pub(crate) fn json_source(bytes: usize) -> String {
        let mut s = String::from("{\n  \"items\": [\n");
        let mut i = 0usize;
        while s.len() < bytes {
            s.push_str(&format!(
                "    {{\"id\": {i}, \"name\": \"item {i}\", \"ok\": {}, \"tags\": [\"a\", \"b\"], \"ratio\": {i}.5, \"next\": null}},\n",
                i.is_multiple_of(2)
            ));
            i += 1;
        }
        s.push_str("    {}\n  ]\n}\n");
        s
    }

    pub(crate) fn markdown_source(bytes: usize) -> String {
        let mut s = String::new();
        let mut i = 0usize;
        while s.len() < bytes {
            s.push_str(&format!(
                "# Section {i}\n\nSome *emphasis*, **strong** and `code {i}` with a [link](https://example.com/{i}).\n\n- item one\n- item two\n\n```rust\nfn f{i}() -> u32 {{ {i} }}\n```\n\n> A quote.\n\n"
            ));
            i += 1;
        }
        s
    }

    pub(crate) fn html_source(bytes: usize) -> String {
        let mut s =
            String::from("<!DOCTYPE html>\n<html>\n<head><title>Bench</title></head>\n<body>\n");
        let mut i = 0usize;
        while s.len() < bytes {
            s.push_str(&format!(
                "<section id=\"s{i}\" class=\"item\">\n  <h2>Item {i}</h2>\n  <p>Some <b>bold</b> text &amp; a <a href=\"/x/{i}\">link</a>.</p>\n  <script>\n    const n{i} = {i};\n    function f{i}(x) {{ return `v${{x}}` + n{i}; }}\n  </script>\n  <style>\n    .item > h2 {{ color: #{:03x}; margin: 0 1px; }}\n  </style>\n  <!-- comment {i} -->\n</section>\n",
                i % 4096
            ));
            i += 1;
        }
        s.push_str("</body>\n</html>\n");
        s
    }

    pub(crate) fn php_source(bytes: usize) -> String {
        let mut s = String::from("<!DOCTYPE html>\n<html>\n<body>\n");
        let mut i = 0usize;
        while s.len() < bytes {
            s.push_str(&format!(
                "<div class=\"row\" id=\"r{i}\">\n  <p>Row {i}: <?php echo htmlspecialchars($rows[{i}]); ?></p>\n<?php\n/** Doc {i}. */\nfunction row{i}(array $a): string {{\n    $x = $a['k{i}'] ?? 0; // note\n    return sprintf('%d', $x + {i});\n}}\n?>\n  <script>let r{i} = {i};</script>\n</div>\n"
            ));
            i += 1;
        }
        s.push_str("</body>\n</html>\n");
        s
    }

    fn wait_for(w: &SyntaxWorker, rev: Revision) -> Highlighted {
        let t = Instant::now();
        loop {
            for h in w.poll() {
                if h.revision == rev {
                    return h;
                }
            }
            assert!(t.elapsed() < Duration::from_secs(60), "no result");
            thread::sleep(Duration::from_micros(200));
        }
    }

    fn run(name: &str, lang: Lang, text: &str) {
        let w = SyntaxWorker::start();
        let mut rope = ropey::Rope::from_str(text);
        let t = Instant::now();
        w.submit(1, Revision(0), lang, rope.clone());
        wait_for(&w, Revision(0));
        let first = t.elapsed();
        let mid = rope.len_chars() / 2;
        let mut times = Vec::new();
        for i in 1..=30u64 {
            // Alternate typing a character and deleting it again.
            if !i.is_multiple_of(2) {
                rope.insert(mid, "x");
            } else {
                rope.remove(mid..mid + 1);
            }
            let t = Instant::now();
            w.submit(1, Revision(i), lang, rope.clone());
            wait_for(&w, Revision(i));
            times.push(t.elapsed());
        }
        // A keystroke after a pause, once the worker has settled.
        thread::sleep(SETTLE_AFTER * 4);
        let _ = w.poll();
        rope.insert(mid, "y");
        let t = Instant::now();
        w.submit(1, Revision(31), lang, rope.clone());
        wait_for(&w, Revision(31));
        let paused = t.elapsed();
        times.sort();
        let ms = |d: Duration| d.as_secs_f64() * 1000.0;
        println!(
            "{name}: {} KB, first {:.1} ms, edit median {:.2} ms, p95 {:.2} ms, max {:.2} ms (n={}), after a pause {:.2} ms",
            text.len() / 1024,
            ms(first),
            ms(times[times.len() / 2]),
            ms(times[times.len() * 95 / 100]),
            ms(times[times.len() - 1]),
            times.len(),
            ms(paused)
        );
    }

    #[test]
    #[ignore = "benchmark; run in release"]
    fn bench_one_char_edit() {
        run("rust", Lang::Rust, &rust_source(1024 * 1024));
        run("json", Lang::Json, &json_source(512 * 1024));
        run("markdown", Lang::Markdown, &markdown_source(512 * 1024));
        run("html", Lang::Html, &html_source(512 * 1024));
        run("php", Lang::Php, &php_source(512 * 1024));
    }
}
