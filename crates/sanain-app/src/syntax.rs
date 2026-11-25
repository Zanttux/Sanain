//! Background syntax highlighting. The UI thread submits a document
//! snapshot tagged with its revision; a worker thread highlights it and
//! sends back char-range spans. Newer submissions cancel running work, and
//! results for old revisions are recognisable by their revision tag.

use std::collections::HashMap;
use std::ops::Range;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{Receiver, Sender, TryRecvError, channel};
use std::thread;

use sanain_core::Revision;
use sanain_language::{HighlightError, Highlighter, Lang, Style};

use crate::workspace::TabId;

/// Documents larger than this are not highlighted (plain text).
pub const MAX_BYTES: usize = 8 * 1024 * 1024;

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
    text: String,
}

#[derive(Debug)]
pub struct Highlighted {
    pub tab: TabId,
    pub revision: Revision,
    pub spans: Vec<CharSpan>,
}

#[derive(Debug)]
pub struct SyntaxWorker {
    tx: Sender<Request>,
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
        let (tx, jobs) = channel::<Request>();
        let (out, rx) = channel();
        let cancel = Arc::new(AtomicUsize::new(0));
        let flag = cancel.clone();
        let spawned = thread::Builder::new()
            .name("sanain-syntax".into())
            .spawn(move || run(jobs, out, flag));
        if let Err(e) = spawned {
            eprintln!("sanain: syntax highlighting disabled: {e}");
        }
        Self { tx, rx, cancel }
    }

    /// Queue a snapshot for highlighting; cancels work in progress.
    pub fn submit(&self, tab: TabId, revision: Revision, lang: Lang, text: String) {
        if text.len() > MAX_BYTES {
            return;
        }
        self.cancel.store(1, Ordering::Relaxed);
        let _ = self.tx.send(Request {
            tab,
            revision,
            lang,
            text,
        });
    }

    pub fn poll(&self) -> Vec<Highlighted> {
        self.rx.try_iter().collect()
    }
}

fn run(jobs: Receiver<Request>, out: Sender<Highlighted>, cancel: Arc<AtomicUsize>) {
    let mut highlighter = Highlighter::new();
    // Latest pending request per tab, so bursts of edits collapse.
    let mut pending: HashMap<TabId, Request> = HashMap::new();
    loop {
        if pending.is_empty() {
            match jobs.recv() {
                Ok(r) => {
                    pending.insert(r.tab, r);
                }
                Err(_) => return,
            }
        }
        loop {
            match jobs.try_recv() {
                Ok(r) => {
                    pending.insert(r.tab, r);
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => return,
            }
        }
        let Some(&tab) = pending.keys().next() else {
            continue;
        };
        let Some(req) = pending.remove(&tab) else {
            continue;
        };
        cancel.store(0, Ordering::Relaxed);
        match highlighter.highlight(req.lang, req.text.as_bytes(), Some(&cancel)) {
            Ok(spans) => {
                let spans = to_char_spans(&req.text, spans);
                let result = Highlighted {
                    tab: req.tab,
                    revision: req.revision,
                    spans,
                };
                if out.send(result).is_err() {
                    return;
                }
            }
            // Interrupted by a newer submission: retry unless superseded.
            Err(HighlightError::Cancelled) => {
                pending.entry(req.tab).or_insert(req);
            }
            Err(HighlightError::Failed(e)) => eprintln!("sanain: highlighting failed: {e}"),
        }
    }
}

/// Convert ascending byte-range spans into char ranges in one pass.
fn to_char_spans(text: &str, spans: Vec<sanain_language::Span>) -> Vec<CharSpan> {
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
    fn latest_submission_wins() {
        let w = SyntaxWorker::start();
        let big = "fn f() {}\n".repeat(20_000);
        w.submit(1, Revision(1), Lang::Rust, big);
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
