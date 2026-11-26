//! Find and replace bar.

use eframe::egui::{self, Key};
use sanain_app::Editor;
use sanain_app::workspace::TabId;
use sanain_core::Revision;
use sanain_core::search::{Matcher, Matches, Query};

use crate::theme;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, channel};

/// Documents with more chars than this are searched on a background thread.
const BACKGROUND_SEARCH: usize = 1_000_000;

/// Localized lookup with up to two integer arguments.
pub type Tr<'a> = &'a dyn Fn(&str, Option<(&str, i64)>, Option<(&str, i64)>) -> String;

/// What the bar asks the shell to do this frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindEvent {
    Next,
    Prev,
    Replace,
    ReplaceAll,
    Close,
}

#[derive(Default)]
pub struct FindBar {
    pub open: bool,
    pub replace_open: bool,
    pub query: Query,
    pub replacement: String,
    matcher: Option<Matcher>,
    pub matches: Matches,
    pub error: Option<String>,
    /// What `matches` was computed for (or is being computed for).
    key: Option<(TabId, Revision, Query)>,
    /// A background search for a large document: its cancel flag and
    /// result channel.
    pending: Option<(Arc<AtomicBool>, Receiver<Option<Matches>>)>,
    focus_query: bool,
}

impl FindBar {
    pub fn open(&mut self, replace: bool, seed: Option<String>) {
        self.open = true;
        self.replace_open |= replace;
        if let Some(s) = seed.filter(|s| !s.contains('\n') && !s.is_empty()) {
            self.query.pattern = s;
        }
        self.focus_query = true;
    }

    fn cancel(&mut self) {
        if let Some((flag, _)) = self.pending.take() {
            flag.store(true, Ordering::Relaxed);
        }
    }

    /// Whether a background search is still running.
    pub fn searching(&self) -> bool {
        self.pending.is_some()
    }

    pub fn matcher(&self) -> Option<&Matcher> {
        self.matcher.as_ref()
    }

    /// Recompute matches if the document, tab or query changed.
    pub fn refresh(&mut self, tab: TabId, editor: &Editor, ctx: &egui::Context) {
        if !self.open {
            self.cancel();
            self.matches = Matches::default();
            self.key = None;
            return;
        }
        if let Some((_, rx)) = &self.pending
            && let Ok(result) = rx.try_recv()
        {
            if let Some(m) = result {
                self.matches = m;
            }
            self.pending = None;
        }
        let key = (tab, editor.document().revision(), self.query.clone());
        if self.key.as_ref() == Some(&key) {
            return;
        }
        self.key = Some(key);
        self.cancel();
        match self.query.compile() {
            Ok(m) => {
                self.error = None;
                let rope = editor.document().rope();
                match &m {
                    Some(matcher) if rope.len_chars() > BACKGROUND_SEARCH => {
                        // Large document: search a snapshot off the UI
                        // thread; the previous matches stay until done.
                        let (tx, rx) = channel();
                        let cancel = Arc::new(AtomicBool::new(false));
                        let (rope, matcher, flag, ctx) =
                            (rope.clone(), matcher.clone(), cancel.clone(), ctx.clone());
                        let spawned = std::thread::Builder::new()
                            .name("sanain-find".into())
                            .spawn(move || {
                                let _ = tx.send(matcher.find_all_cancellable(&rope, &flag));
                                ctx.request_repaint();
                            });
                        if spawned.is_ok() {
                            self.pending = Some((cancel, rx));
                        }
                    }
                    Some(matcher) => self.matches = matcher.find_all(rope),
                    None => self.matches = Matches::default(),
                }
                self.matcher = m;
            }
            Err(e) => {
                self.error = Some(e.to_string());
                self.matcher = None;
                self.matches = Matches::default();
            }
        }
    }

    /// Draw the bar. `t` resolves localized strings; `current` is the index
    /// of the match the primary selection is on, if any.
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        t: Tr<'_>,
        current: Option<usize>,
    ) -> Option<FindEvent> {
        let p = theme::palette(ui.ctx());
        let mut event = None;
        let enter = |r: &egui::Response, ui: &egui::Ui| {
            r.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter))
        };
        ui.horizontal(|ui| {
            let field = egui::TextEdit::singleline(&mut self.query.pattern)
                .id_salt("sanain-find-query")
                .hint_text(t("find-placeholder", None, None))
                .desired_width(260.0);
            let r = ui.add(field);
            if std::mem::take(&mut self.focus_query) {
                r.request_focus();
            }
            if enter(&r, ui) {
                let back = ui.input(|i| i.modifiers.shift);
                event = Some(if back {
                    FindEvent::Prev
                } else {
                    FindEvent::Next
                });
                r.request_focus();
            }
            // egui's text field gives up focus on Escape before we see it,
            // so accept either state.
            if (r.has_focus() || r.lost_focus()) && ui.input(|i| i.key_pressed(Key::Escape)) {
                event = Some(FindEvent::Close);
            }
            for (on, label, tip) in [
                (&mut self.query.case_sensitive, "Aa", "find-match-case"),
                (&mut self.query.whole_word, "W", "find-whole-word"),
                (&mut self.query.regex, ".*", "find-regex"),
            ] {
                let b = ui
                    .add(egui::Button::selectable(
                        *on,
                        egui::RichText::new(label).monospace(),
                    ))
                    .on_hover_text(t(tip, None, None));
                if b.clicked() {
                    *on = !*on;
                }
            }
            let status = if let Some(err) = &self.error {
                egui::RichText::new(err.lines().last().unwrap_or(err)).color(p.error)
            } else if self.query.pattern.is_empty() {
                egui::RichText::new("")
            } else if self.searching() {
                egui::RichText::new(t("find-searching", None, None)).color(p.muted)
            } else if self.matches.ranges.is_empty() {
                egui::RichText::new(t("find-no-results", None, None)).color(p.muted)
            } else {
                let total = self.matches.ranges.len() as i64;
                let text = match current {
                    Some(i) => t(
                        "find-count-current",
                        Some(("current", i as i64 + 1)),
                        Some(("total", total)),
                    ),
                    None => t("find-count", Some(("total", total)), None),
                };
                let text = if self.matches.truncated {
                    format!("{text}+")
                } else {
                    text
                };
                egui::RichText::new(text).color(p.muted)
            };
            if ui
                .button("↑")
                .on_hover_text(t("find-prev", None, None))
                .clicked()
            {
                event = Some(FindEvent::Prev);
            }
            if ui
                .button("↓")
                .on_hover_text(t("find-next", None, None))
                .clicked()
            {
                event = Some(FindEvent::Next);
            }
            ui.label(status);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .button("×")
                    .on_hover_text(t("find-close", None, None))
                    .clicked()
                {
                    event = Some(FindEvent::Close);
                }
            });
        });
        if self.replace_open {
            ui.horizontal(|ui| {
                let field = egui::TextEdit::singleline(&mut self.replacement)
                    .id_salt("sanain-find-replace")
                    .hint_text(t("replace-placeholder", None, None))
                    .desired_width(260.0);
                let r = ui.add(field);
                if enter(&r, ui) {
                    event = Some(FindEvent::Replace);
                    r.request_focus();
                }
                let has = !self.matches.ranges.is_empty();
                if ui
                    .add_enabled(has, egui::Button::new(t("replace-one", None, None)))
                    .clicked()
                {
                    event = Some(FindEvent::Replace);
                }
                if ui
                    .add_enabled(has, egui::Button::new(t("replace-all", None, None)))
                    .clicked()
                {
                    event = Some(FindEvent::ReplaceAll);
                }
            });
        }
        event
    }
}
