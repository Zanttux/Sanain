//! Command palette: fuzzy-search every menu command and run it.

use eframe::egui::{self, Key};
use sanain_app::fuzzy;

use crate::menu::MenuAction;
use crate::theme;

/// One runnable entry as shown in the palette.
pub struct Entry {
    pub action: MenuAction,
    pub label: String,
    pub shortcut: String,
    pub enabled: bool,
}

#[derive(Default)]
pub struct Palette {
    pub open: bool,
    query: String,
    selected: usize,
    focus: bool,
}

const MAX_ROWS: usize = 12;

impl Palette {
    pub fn open(&mut self) {
        self.open = true;
        self.query.clear();
        self.selected = 0;
        self.focus = true;
    }

    /// Draw the palette; returns the command to run, if one was chosen.
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        entries: &[Entry],
        hint: &str,
    ) -> Option<MenuAction> {
        if !self.open {
            return None;
        }
        let p = theme::palette(ctx);
        let labels: Vec<&str> = entries.iter().map(|e| e.label.as_str()).collect();
        let ranked: Vec<usize> = fuzzy::rank(&self.query, &labels)
            .into_iter()
            .take(MAX_ROWS)
            .collect();
        if self.selected >= ranked.len() {
            self.selected = ranked.len().saturating_sub(1);
        }
        let (down, up, enter, escape) = ctx.input(|i| {
            (
                i.key_pressed(Key::ArrowDown),
                i.key_pressed(Key::ArrowUp),
                i.key_pressed(Key::Enter),
                i.key_pressed(Key::Escape),
            )
        });
        if down && !ranked.is_empty() {
            self.selected = (self.selected + 1) % ranked.len();
        }
        if up && !ranked.is_empty() {
            self.selected = (self.selected + ranked.len() - 1) % ranked.len();
        }
        let mut chosen = None;
        let width = 520.0_f32.min(ctx.content_rect().width() - 32.0);
        let area = egui::Area::new(egui::Id::unique("sanain-palette"))
            .order(egui::Order::Foreground)
            .anchor(egui::Align2::CENTER_TOP, egui::vec2(0.0, 56.0))
            .show(ctx, |ui| {
                egui::Frame::popup(ui.style())
                    .fill(p.surface)
                    .stroke(egui::Stroke::new(1.0, p.border))
                    .inner_margin(egui::Margin::same(8))
                    .show(ui, |ui| {
                        ui.set_width(width);
                        let field = egui::TextEdit::singleline(&mut self.query)
                            .id_salt("sanain-palette-query")
                            .hint_text(hint)
                            .desired_width(f32::INFINITY);
                        let r = ui.add(field);
                        if std::mem::take(&mut self.focus) {
                            r.request_focus();
                        }
                        if r.changed() {
                            self.selected = 0;
                        }
                        ui.add_space(4.0);
                        for (row, &i) in ranked.iter().enumerate() {
                            let e = &entries[i];
                            let active = row == self.selected;
                            let fill = if active {
                                p.accent.gamma_multiply(0.25)
                            } else {
                                egui::Color32::TRANSPARENT
                            };
                            let resp = egui::Frame::NONE
                                .fill(fill)
                                .corner_radius(4.0)
                                .inner_margin(egui::Margin::symmetric(8, 4))
                                .show(ui, |ui| {
                                    ui.set_width(ui.available_width());
                                    ui.horizontal(|ui| {
                                        let color = if e.enabled { p.text } else { p.muted };
                                        ui.label(egui::RichText::new(&e.label).color(color));
                                        ui.with_layout(
                                            egui::Layout::right_to_left(egui::Align::Center),
                                            |ui| {
                                                ui.label(
                                                    egui::RichText::new(&e.shortcut).color(p.muted),
                                                )
                                            },
                                        );
                                    });
                                })
                                .response
                                .interact(egui::Sense::click());
                            if resp.hovered() {
                                self.selected = row;
                            }
                            if resp.clicked() && e.enabled {
                                chosen = Some(e.action);
                            }
                        }
                    });
            });
        if enter
            && let Some(&i) = ranked.get(self.selected)
            && entries[i].enabled
        {
            chosen = Some(entries[i].action);
        }
        let clicked_outside = ctx.input(|i| i.pointer.any_pressed())
            && ctx
                .pointer_interact_pos()
                .is_some_and(|pos| !area.response.rect.contains(pos));
        if chosen.is_some() || escape || clicked_outside {
            self.open = false;
        }
        chosen
    }
}
