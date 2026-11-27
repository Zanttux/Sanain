//! The status bar's character and word count, computed off the UI thread.
//!
//! Each frame the shell asks [`Counter::view`] for the active tab's counts.
//! Results are cached by document revision (and, for the selection, by the
//! selected ranges); anything missing is queued for a worker thread, and
//! the last result is shown until the new one arrives, so typing never
//! waits for a count. The worker has one slot for the document and one for
//! the selection: a newer request replaces a queued one and cancels a
//! running one of the same kind (checked between lines).
//!
//! Text over [`LAZY_BYTES`] is counted only on demand: the status bar shows
//! "—" until the counts summary is opened for that tab, after which the tab
//! keeps its counts up to date like a small one.
//!
//! See [`text3_core::stats`] for what counts as a character, word or line.

use std::collections::HashMap;
use std::ops::Range;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

use text3_core::Revision;
use text3_core::stats::{self, Counts};
use text3_io::{EncodedLen, TextEncoding};

use crate::workspace::TabId;

/// Text larger than this is counted only on demand (UTF-8 bytes for the
/// document, chars for the selection).
pub const LAZY_BYTES: usize = 8 * 1024 * 1024;

/// Counts of one text plus its size in the file's encoding (`None` when a
/// character cannot be encoded in it).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Stats {
    pub counts: Counts,
    pub bytes: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Doc,
    Sel,
}

/// What a result is valid for.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Key {
    tab: TabId,
    revision: Revision,
    encoding: TextEncoding,
    /// Selected char ranges (empty for the document).
    ranges: Vec<Range<usize>>,
}

#[derive(Debug)]
struct Job {
    key: Key,
    generation: u64,
    rope: ropey::Rope,
}

#[derive(Debug, Default)]
struct Slots {
    doc: Option<Job>,
    sel: Option<Job>,
    closed: bool,
}

#[derive(Debug, Default)]
struct Shared {
    slots: Mutex<Slots>,
    wake: Condvar,
    doc_gen: AtomicU64,
    sel_gen: AtomicU64,
}

impl Shared {
    fn generation(&self, kind: Kind) -> &AtomicU64 {
        match kind {
            Kind::Doc => &self.doc_gen,
            Kind::Sel => &self.sel_gen,
        }
    }
}

#[derive(Debug)]
struct Done {
    kind: Kind,
    key: Key,
    stats: Stats,
}

/// What the status bar shows for the active tab.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct View {
    /// The document's counts (possibly of an earlier revision while the
    /// current one is counted); `None` when not counted yet.
    pub doc: Option<Stats>,
    /// The selection's counts when there is a non-empty selection.
    pub sel: Option<Stats>,
    /// Whether any text is selected.
    pub selected: bool,
    /// Counting is in progress (results are stale or missing).
    pub pending: bool,
    /// The text is over [`LAZY_BYTES`] and waits for the summary to open.
    pub deferred: bool,
}

#[derive(Debug, Default)]
struct TabState {
    doc: Option<(Key, Stats)>,
    sel: Option<(Key, Stats)>,
    /// A large document whose counts the user asked for.
    on_demand: bool,
}

/// The count cache and its worker thread.
#[derive(Debug)]
pub struct Counter {
    shared: Arc<Shared>,
    rx: Receiver<Done>,
    tabs: HashMap<TabId, TabState>,
    /// Latest key submitted per kind (only the latest can finish).
    in_flight: [Option<Key>; 2],
}

impl Default for Counter {
    fn default() -> Self {
        Self::start()
    }
}

impl Drop for Counter {
    fn drop(&mut self) {
        if let Ok(mut s) = self.shared.slots.lock() {
            s.closed = true;
        }
        // Stop a running count too.
        self.shared.doc_gen.fetch_add(1, Ordering::Relaxed);
        self.shared.sel_gen.fetch_add(1, Ordering::Relaxed);
        self.shared.wake.notify_all();
    }
}

fn slot(kind: Kind) -> usize {
    match kind {
        Kind::Doc => 0,
        Kind::Sel => 1,
    }
}

impl Counter {
    pub fn start() -> Self {
        let shared = Arc::new(Shared::default());
        let (tx, rx) = channel();
        let worker = shared.clone();
        let spawned = thread::Builder::new()
            .name("text3-counts".into())
            .spawn(move || run(&worker, &tx));
        if let Err(e) = spawned {
            eprintln!("text3: character count disabled: {e}");
        }
        Self {
            shared,
            rx,
            tabs: HashMap::new(),
            in_flight: [None, None],
        }
    }

    /// Take finished counts into the cache.
    pub fn poll(&mut self) {
        while let Ok(d) = self.rx.try_recv() {
            self.take(d);
        }
    }

    fn take(&mut self, d: Done) {
        let i = slot(d.kind);
        if self.in_flight[i].as_ref() == Some(&d.key) {
            self.in_flight[i] = None;
        }
        let t = self.tabs.entry(d.key.tab).or_default();
        let entry = Some((d.key, d.stats));
        match d.kind {
            Kind::Doc => t.doc = entry,
            Kind::Sel => t.sel = entry,
        }
    }

    /// The counts to show for `tab`, queueing whatever is out of date.
    /// `ranges` are the selections' char ranges; `demand` is true while
    /// the counts summary is open (counts large text too).
    pub fn view(
        &mut self,
        tab: TabId,
        rope: &ropey::Rope,
        revision: Revision,
        encoding: TextEncoding,
        ranges: impl IntoIterator<Item = Range<usize>>,
        demand: bool,
    ) -> View {
        self.poll();
        let ranges: Vec<Range<usize>> = ranges.into_iter().filter(|r| !r.is_empty()).collect();
        let selected = !ranges.is_empty();
        let large = rope.len_bytes() > LAZY_BYTES;
        let sel_large = ranges.iter().map(|r| r.len()).sum::<usize>() > LAZY_BYTES;
        let state = self.tabs.entry(tab).or_default();
        if demand && (large || sel_large) {
            state.on_demand = true;
        }
        let allowed = |big: bool| !big || state.on_demand;
        let doc_key = Key {
            tab,
            revision,
            encoding,
            ranges: Vec::new(),
        };
        let mut want: Vec<(Kind, Key)> = Vec::new();
        let doc_fresh = state.doc.as_ref().is_some_and(|(k, _)| *k == doc_key);
        let doc_ok = allowed(large);
        if !doc_fresh && doc_ok {
            want.push((Kind::Doc, doc_key));
        }
        let sel_ok = allowed(sel_large);
        if selected {
            let sel_key = Key {
                tab,
                revision,
                encoding,
                ranges: ranges.clone(),
            };
            let sel_fresh = state.sel.as_ref().is_some_and(|(k, _)| *k == sel_key);
            if !sel_fresh && sel_ok {
                want.push((Kind::Sel, sel_key));
            }
        }
        // Results for this tab, or none: another tab's numbers never show.
        let doc = state.doc.as_ref().filter(|_| doc_ok).map(|(_, s)| *s);
        let sel = state
            .sel
            .as_ref()
            .filter(|_| selected && sel_ok)
            .map(|(_, s)| *s);
        let view = View {
            doc,
            sel,
            selected,
            pending: !want.is_empty(),
            deferred: !doc_ok || (selected && !sel_ok),
        };
        for (kind, key) in want {
            self.submit(kind, key, rope);
        }
        view
    }

    fn submit(&mut self, kind: Kind, key: Key, rope: &ropey::Rope) {
        let i = slot(kind);
        if self.in_flight[i].as_ref() == Some(&key) {
            return;
        }
        self.in_flight[i] = Some(key.clone());
        // Cancel a running count of the same kind.
        let generation = self.shared.generation(kind).fetch_add(1, Ordering::Relaxed) + 1;
        let job = Job {
            key,
            generation,
            rope: rope.clone(),
        };
        if let Ok(mut s) = self.shared.slots.lock() {
            match kind {
                Kind::Doc => s.doc = Some(job),
                Kind::Sel => s.sel = Some(job),
            }
        }
        self.shared.wake.notify_one();
    }

    /// Drop a closed tab's counts.
    pub fn forget(&mut self, tab: TabId) {
        self.tabs.remove(&tab);
    }

    /// Wait for queued counts to finish (tests).
    #[cfg(test)]
    fn settle(&mut self) {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(20);
        while self.in_flight.iter().any(Option::is_some) && std::time::Instant::now() < deadline {
            if let Ok(d) = self.rx.recv_timeout(std::time::Duration::from_millis(50)) {
                self.take(d);
            }
        }
    }
}

fn run(shared: &Shared, out: &Sender<Done>) {
    loop {
        let (kind, job) = {
            let Ok(mut s) = shared.slots.lock() else {
                return;
            };
            loop {
                if s.closed {
                    return;
                }
                // The selection first: it is usually small and changes most.
                if let Some(j) = s.sel.take() {
                    break (Kind::Sel, j);
                }
                if let Some(j) = s.doc.take() {
                    break (Kind::Doc, j);
                }
                s = match shared.wake.wait(s) {
                    Ok(s) => s,
                    Err(_) => return,
                };
            }
        };
        let generation = shared.generation(kind);
        let cancel = || generation.load(Ordering::Relaxed) != job.generation;
        if let Some(stats) = count(&job.rope, &job.key, &cancel)
            && out
                .send(Done {
                    kind,
                    key: job.key,
                    stats,
                })
                .is_err()
        {
            return;
        }
    }
}

/// Count the document (no ranges) or the ranges of `rope`.
fn count(rope: &ropey::Rope, key: &Key, cancel: &dyn Fn() -> bool) -> Option<Stats> {
    let whole = key.ranges.is_empty();
    let counts = if whole {
        stats::document(rope, cancel)?
    } else {
        stats::ranges(rope, &key.ranges, cancel)?
    };
    let mut len = EncodedLen::new(key.encoding, whole);
    let len_chars = rope.len_chars();
    let full = std::iter::once(0..len_chars);
    let parts: Vec<Range<usize>> = if whole {
        full.collect()
    } else {
        key.ranges.clone()
    };
    for r in parts {
        let r = r.start.min(len_chars)..r.end.min(len_chars);
        for (i, chunk) in rope.slice(r).chunks().enumerate() {
            if i.is_multiple_of(256) && cancel() {
                return None;
            }
            len.feed(chunk);
        }
    }
    Some(Stats {
        counts,
        bytes: len.finish(),
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::single_range_in_vec_init)]
mod tests {
    use super::*;

    fn rev(n: u64) -> Revision {
        Revision(n)
    }

    #[test]
    fn counts_document_and_selection_in_the_background() {
        let mut c = Counter::start();
        let rope = ropey::Rope::from_str("Hello wörld\r\nsecond line\r\n");
        let v = c.view(1, &rope, rev(1), TextEncoding::Utf8Bom, [], false);
        // Nothing yet: queued.
        assert!(v.pending && v.doc.is_none() && !v.selected);
        c.settle();
        let v = c.view(1, &rope, rev(1), TextEncoding::Utf8Bom, [], false);
        assert!(!v.pending && !v.deferred);
        let doc = v.doc.unwrap();
        assert_eq!(doc.counts.chars, 22);
        assert_eq!(doc.counts.words, 4);
        assert_eq!(doc.counts.lines, 3);
        // UTF-8 with BOM: the text's bytes plus three.
        assert_eq!(doc.bytes, Some(rope.len_bytes() as u64 + 3));
        // Two selections (a caret is ignored): "Hello" and "second".
        let sels = [0..5, 13..19, 3..3];
        let v = c.view(1, &rope, rev(1), TextEncoding::Utf8Bom, sels.clone(), false);
        assert!(v.selected && v.pending);
        c.settle();
        let v = c.view(1, &rope, rev(1), TextEncoding::Utf8Bom, sels, false);
        let sel = v.sel.unwrap();
        assert_eq!((sel.counts.chars, sel.counts.words), (11, 2));
        // A part of a file has no BOM.
        assert_eq!(sel.bytes, Some(11));
        assert_eq!(v.doc, Some(doc));
    }

    #[test]
    fn edits_keep_the_old_count_until_the_new_one_arrives() {
        let mut c = Counter::start();
        let mut rope = ropey::Rope::from_str("one two");
        c.view(1, &rope, rev(1), TextEncoding::Utf8, [], false);
        c.settle();
        rope.insert(7, " three");
        let v = c.view(1, &rope, rev(2), TextEncoding::Utf8, [], false);
        assert!(v.pending);
        assert_eq!(v.doc.unwrap().counts.words, 2);
        c.settle();
        let v = c.view(1, &rope, rev(2), TextEncoding::Utf8, [], false);
        assert_eq!(v.doc.unwrap().counts.words, 3);
        // Another tab never shows this tab's numbers.
        let v = c.view(2, &rope, rev(2), TextEncoding::Utf8, [], false);
        assert_eq!(v.doc, None);
        c.forget(1);
        let v = c.view(1, &rope, rev(2), TextEncoding::Utf8, [], false);
        assert_eq!(v.doc, None);
    }

    #[test]
    fn large_text_waits_for_the_summary() {
        let mut c = Counter::start();
        let rope = ropey::Rope::from_str(&"word ".repeat(LAZY_BYTES / 5 + 1));
        assert!(rope.len_bytes() > LAZY_BYTES);
        let v = c.view(1, &rope, rev(1), TextEncoding::Utf8, [], false);
        assert!(v.deferred && !v.pending && v.doc.is_none());
        // A small selection in it is still counted right away.
        let v = c.view(1, &rope, rev(1), TextEncoding::Utf8, [0..9], false);
        assert!(v.deferred && v.pending);
        c.settle();
        let v = c.view(1, &rope, rev(1), TextEncoding::Utf8, [0..9], false);
        assert_eq!(v.sel.unwrap().counts.words, 2);
        assert_eq!(v.doc, None);
        // Opening the summary counts it, and the tab stays counted.
        let v = c.view(1, &rope, rev(1), TextEncoding::Utf8, [], true);
        assert!(v.pending && !v.deferred);
        c.settle();
        let v = c.view(1, &rope, rev(2), TextEncoding::Utf8, [], false);
        assert!(!v.deferred);
        assert_eq!(v.doc.unwrap().counts.words, (LAZY_BYTES / 5 + 1) as u64);
    }

    #[test]
    fn unencodable_text_has_no_byte_count() {
        let rope = ropey::Rope::from_str("a世");
        let key = Key {
            tab: 1,
            revision: rev(1),
            encoding: TextEncoding::Legacy(encoding_rs::WINDOWS_1252),
            ranges: Vec::new(),
        };
        let s = count(&rope, &key, &|| false).unwrap();
        assert_eq!((s.counts.chars, s.bytes), (2, None));
        let key = Key {
            encoding: TextEncoding::Utf16LeBom,
            ..key
        };
        assert_eq!(count(&rope, &key, &|| false).unwrap().bytes, Some(6));
    }
}
