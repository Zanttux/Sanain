//! "Find in Folder" panel: query, folder, streamed results.

use std::path::PathBuf;

use eframe::egui::{self, RichText, text::LayoutJob, text::TextFormat};
use sanain_app::filesearch::{Event, Hit, Search, Stats};
use sanain_core::search::Query;

use crate::find::Tr;
use crate::theme;

#[derive(Default)]
pub struct FolderPanel {
    pub open: bool,
    pub root: Option<PathBuf>,
    pub query: Query,
    search: Option<Search>,
    hits: Vec<Hit>,
    stats: Option<Stats>,
    error: Option<String>,
    focus: bool,
}

/// What the panel asks the shell to do.
pub enum PanelEvent {
    ChooseFolder,
    OpenHit(Hit),
    Close,
}

impl FolderPanel {
    pub fn show_panel(&mut self, root: Option<PathBuf>) {
        self.open = true;
        if self.root.is_none() {
            self.root = root;
        }
        self.focus = true;
    }

    pub fn running(&self) -> bool {
        self.search.is_some()
    }

    fn start(&mut self, ctx: &egui::Context) {
        self.hits.clear();
        self.stats = None;
        self.error = None;
        let Some(root) = self.root.clone() else {
            return;
        };
        match self.query.compile() {
            Ok(Some(m)) => {
                let ctx = ctx.clone();
                self.search = Some(Search::start(root, m, move || ctx.request_repaint()));
            }
            Ok(None) => {}
            Err(e) => self.error = Some(e.to_string()),
        }
    }

    fn poll(&mut self) {
        let Some(s) = &self.search else { return };
        let mut done = None;
        for e in s.events.try_iter() {
            match e {
                Event::Hits(h) => self.hits.extend(h),
                Event::Done(st) => done = Some(st),
            }
        }
        if done.is_some() {
            self.stats = done;
            self.search = None;
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, t: Tr<'_>) -> Option<PanelEvent> {
        self.poll();
        let p = theme::palette(ui.ctx());
        let mut event = None;
        ui.horizontal(|ui| {
            let r = ui.add(
                egui::TextEdit::singleline(&mut self.query.pattern)
                    .id_salt("sanain-folder-query")
                    .hint_text(t("folder-search-placeholder", None, None))
                    .desired_width(260.0),
            );
            if std::mem::take(&mut self.focus) {
                r.request_focus();
            }
            let enter = r.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
            for (on, label, tip) in [
                (&mut self.query.case_sensitive, "Aa", "find-match-case"),
                (&mut self.query.whole_word, "W", "find-whole-word"),
                (&mut self.query.regex, ".*", "find-regex"),
            ] {
                if ui
                    .add(egui::Button::selectable(
                        *on,
                        RichText::new(label).monospace(),
                    ))
                    .on_hover_text(t(tip, None, None))
                    .clicked()
                {
                    *on = !*on;
                }
            }
            if self.running() {
                if ui.button(t("folder-search-stop", None, None)).clicked()
                    && let Some(s) = &self.search
                {
                    s.cancel();
                }
                ui.spinner();
            } else if (ui.button(t("folder-search-run", None, None)).clicked() || enter)
                && self.root.is_some()
            {
                self.start(ui.ctx());
            }
            let folder = self.root.as_ref().and_then(|r| r.file_name()).map_or_else(
                || t("folder-search-no-folder", None, None),
                |n| n.to_string_lossy().into_owned(),
            );
            if ui
                .button(folder)
                .on_hover_text(
                    self.root
                        .as_ref()
                        .map(|r| r.display().to_string())
                        .unwrap_or_default(),
                )
                .clicked()
            {
                event = Some(PanelEvent::ChooseFolder);
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .button("×")
                    .on_hover_text(t("find-close", None, None))
                    .clicked()
                {
                    event = Some(PanelEvent::Close);
                }
                let files = {
                    let mut f: Vec<&PathBuf> = self.hits.iter().map(|h| &h.path).collect();
                    f.dedup();
                    f.len() as i64
                };
                let status = match (&self.error, self.stats) {
                    (Some(e), _) => RichText::new(e).color(p.error),
                    (None, Some(st)) => {
                        let mut s = t(
                            "folder-search-summary",
                            Some(("count", self.hits.len() as i64)),
                            Some(("files", files)),
                        );
                        if st.truncated {
                            s.push_str(" +");
                        }
                        RichText::new(s).color(p.muted)
                    }
                    (None, None) if self.running() => RichText::new(t(
                        "folder-search-summary",
                        Some(("count", self.hits.len() as i64)),
                        Some(("files", files)),
                    ))
                    .color(p.muted),
                    _ => RichText::new(""),
                };
                ui.label(status);
            });
        });
        ui.separator();
        let root = self.root.clone();
        egui::ScrollArea::vertical()
            .auto_shrink(false)
            .show(ui, |ui| {
                let mut last: Option<&PathBuf> = None;
                for hit in &self.hits {
                    if last != Some(&hit.path) {
                        let rel = root
                            .as_ref()
                            .and_then(|r| hit.path.strip_prefix(r).ok())
                            .unwrap_or(&hit.path);
                        ui.add_space(2.0);
                        ui.label(
                            RichText::new(rel.display().to_string())
                                .strong()
                                .color(p.text),
                        );
                        last = Some(&hit.path);
                    }
                    let mut job = LayoutJob::default();
                    let mono = egui::FontId::monospace(12.0);
                    let fmt = |color| TextFormat::simple(mono.clone(), color);
                    job.append(&format!("{:>6}  ", hit.line + 1), 0.0, fmt(p.muted));
                    let chars: Vec<char> = hit.preview.chars().collect();
                    let r = hit.preview_match.start.min(chars.len())
                        ..hit.preview_match.end.min(chars.len());
                    let s: String = chars[..r.start].iter().collect();
                    let m: String = chars[r.clone()].iter().collect();
                    let e: String = chars[r.end..].iter().collect();
                    job.append(&s, 0.0, fmt(p.text));
                    let mut hl = fmt(p.text);
                    hl.background = p.warning.gamma_multiply(0.35);
                    job.append(&m, 0.0, hl);
                    job.append(&e, 0.0, fmt(p.text));
                    if ui
                        .add(egui::Label::new(job).sense(egui::Sense::click()).truncate())
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        event = Some(PanelEvent::OpenHit(hit.clone()));
                    }
                }
            });
        event
    }
}
