//! Read-only Markdown preview rendered from `sanain_markdown` blocks.
//!
//! Nothing here touches the network or runs document content. Links are
//! shown (with their target on hover) but not opened yet; images show their
//! alt text.

use eframe::egui::{
    self, Color32, FontFamily, FontId, RichText, Stroke, text::LayoutJob, text::TextFormat,
};
use sanain_markdown::{Align, Block, BlockKind, Inline, MAX_DEPTH};

use crate::theme::Palette;

/// Documents larger than this are not previewed (bounded UI-thread work).
pub const MAX_PREVIEW_BYTES: usize = 2 * 1024 * 1024;

pub struct Style<'a> {
    pub p: &'a Palette,
    pub size: f32,
    pub untranslated: &'a dyn Fn(&str) -> String,
}

/// Render `blocks`. `sync_line` (1-based source line) scrolls the block at
/// or before that line to the top when `scroll` is true.
pub fn show(ui: &mut egui::Ui, blocks: &[Block], st: &Style<'_>, sync_line: usize, scroll: bool) {
    let target = blocks.iter().rposition(|b| b.line <= sync_line);
    for (i, b) in blocks.iter().enumerate() {
        let r = ui.scope(|ui| block(ui, b, st, 0)).response;
        if scroll && Some(i) == target {
            ui.scroll_to_rect(r.rect, Some(egui::Align::TOP));
        }
        ui.add_space(st.size * 0.6);
    }
}

fn block(ui: &mut egui::Ui, b: &Block, st: &Style<'_>, depth: usize) {
    if depth > MAX_DEPTH {
        return;
    }
    let p = st.p;
    match &b.kind {
        BlockKind::Heading { level, inlines } => {
            let scale = match level {
                1 => 1.8,
                2 => 1.45,
                3 => 1.2,
                _ => 1.05,
            };
            ui.add_space(st.size * 0.4);
            ui.label(job(inlines, st, st.size * scale, p.text, true));
            if *level <= 2 {
                ui.add(egui::Separator::default().spacing(4.0));
            }
        }
        BlockKind::Paragraph(inlines) => {
            ui.label(job(inlines, st, st.size, body(p), false));
        }
        BlockKind::Code { lang, text } => {
            if lang.eq_ignore_ascii_case("mermaid") {
                ui.label(
                    RichText::new((st.untranslated)("preview-mermaid-unsupported"))
                        .color(p.muted)
                        .italics(),
                );
            }
            code_frame(ui, text.trim_end_matches('\n'), st);
        }
        BlockKind::Quote(children) => {
            let r = egui::Frame::NONE
                .inner_margin(egui::Margin {
                    left: 14,
                    right: 0,
                    top: 2,
                    bottom: 2,
                })
                .show(ui, |ui| {
                    for c in children {
                        block(ui, c, st, depth + 1);
                    }
                })
                .response;
            let x = r.rect.left() + 3.0;
            ui.painter()
                .vline(x, r.rect.y_range(), Stroke::new(3.0, p.border));
        }
        BlockKind::List { start, items } => {
            for (n, item) in items.iter().enumerate() {
                ui.horizontal_top(|ui| {
                    let marker = match (item.task, start) {
                        (Some(true), _) => "☑".to_owned(),
                        (Some(false), _) => "☐".to_owned(),
                        (None, Some(s)) => format!("{}.", s + n),
                        (None, None) => "•".to_owned(),
                    };
                    ui.add_sized(
                        [st.size * 1.6, st.size * 1.3],
                        egui::Label::new(RichText::new(marker).size(st.size).color(p.muted)),
                    );
                    ui.vertical(|ui| {
                        for c in &item.blocks {
                            block(ui, c, st, depth + 1);
                        }
                    });
                });
            }
        }
        BlockKind::Table {
            aligns,
            header,
            rows,
        } => {
            egui::ScrollArea::horizontal()
                .id_salt(("md-table", b.line))
                .show(ui, |ui| {
                    egui::Grid::new(("md-grid", b.line))
                        .striped(true)
                        .spacing([st.size, st.size * 0.4])
                        .show(ui, |ui| {
                            let cell =
                                |ui: &mut egui::Ui, inl: &[Inline], col: usize, strong: bool| {
                                    let layout = match aligns.get(col) {
                                        Some(Align::Right) => {
                                            egui::Layout::right_to_left(egui::Align::Center)
                                        }
                                        Some(Align::Center) => {
                                            egui::Layout::top_down(egui::Align::Center)
                                        }
                                        _ => egui::Layout::left_to_right(egui::Align::Center),
                                    };
                                    ui.with_layout(layout, |ui| {
                                        ui.label(job(inl, st, st.size, body(p), strong));
                                    });
                                };
                            for (c, h) in header.iter().enumerate() {
                                cell(ui, h, c, true);
                            }
                            ui.end_row();
                            for row in rows {
                                for (c, v) in row.iter().enumerate() {
                                    cell(ui, v, c, false);
                                }
                                ui.end_row();
                            }
                        });
                });
        }
        BlockKind::Rule => {
            ui.separator();
        }
        BlockKind::Html(raw) => {
            code_frame(ui, raw.trim_end(), st);
        }
        BlockKind::FrontMatter(fm) => {
            code_frame(ui, fm, st);
        }
    }
}

fn code_frame(ui: &mut egui::Ui, text: &str, st: &Style<'_>) {
    egui::Frame::NONE
        .fill(st.p.surface)
        .stroke(Stroke::new(1.0, st.p.border))
        .corner_radius(4.0)
        .inner_margin(egui::Margin::same(8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            egui::ScrollArea::horizontal()
                .id_salt(text.len())
                .show(ui, |ui| {
                    ui.add(
                        egui::Label::new(
                            RichText::new(text)
                                .font(FontId::monospace(st.size * 0.92))
                                .color(st.p.text),
                        )
                        .wrap_mode(egui::TextWrapMode::Extend),
                    );
                });
        });
}

/// Lay out inline content as one wrapped text job.
fn job(inlines: &[Inline], st: &Style<'_>, size: f32, color: Color32, strong: bool) -> LayoutJob {
    let mut j = LayoutJob::default();
    let base = TextFormat {
        font_id: FontId::new(size, FontFamily::Proportional),
        color,
        ..Default::default()
    };
    append(&mut j, inlines, st, &base, strong, 0);
    j
}

fn append(
    j: &mut LayoutJob,
    inlines: &[Inline],
    st: &Style<'_>,
    fmt: &TextFormat,
    strong: bool,
    depth: usize,
) {
    if depth > MAX_DEPTH {
        return;
    }
    let p = st.p;
    for i in inlines {
        match i {
            Inline::Text(t) => {
                let mut f = fmt.clone();
                // egui ships no bold face: body text uses a softer colour so
                // strong text (full contrast) stands out.
                if strong {
                    f.color = p.text;
                }
                j.append(t, 0.0, f);
            }
            Inline::Code(t) => {
                let mut f = fmt.clone();
                f.font_id = FontId::monospace(fmt.font_id.size * 0.92);
                f.background = p.surface;
                j.append(t, 0.0, f);
            }
            Inline::Strong(c) => append(j, c, st, fmt, true, depth + 1),
            Inline::Emph(c) => {
                let mut f = fmt.clone();
                f.italics = true;
                append(j, c, st, &f, strong, depth + 1);
            }
            Inline::Strike(c) => {
                let mut f = fmt.clone();
                f.strikethrough = Stroke::new(1.0, fmt.color);
                append(j, c, st, &f, strong, depth + 1);
            }
            Inline::Link { children, .. } => {
                let mut f = fmt.clone();
                f.color = p.accent;
                f.underline = Stroke::new(1.0, p.accent);
                append(j, children, st, &f, strong, depth + 1);
            }
            Inline::Image { alt, .. } => {
                let mut f = fmt.clone();
                f.color = p.muted;
                f.italics = true;
                let label = if alt.is_empty() {
                    "image".to_owned()
                } else {
                    alt.clone()
                };
                j.append(&format!("[{label}]"), 0.0, f);
            }
            Inline::Html(h) => {
                let mut f = fmt.clone();
                f.font_id = FontId::monospace(fmt.font_id.size * 0.92);
                f.color = p.muted;
                j.append(h, 0.0, f);
            }
            Inline::SoftBreak => j.append(" ", 0.0, fmt.clone()),
            Inline::LineBreak => j.append("\n", 0.0, fmt.clone()),
        }
    }
}

fn body(p: &Palette) -> Color32 {
    p.text.lerp_to_gamma(p.muted, 0.3)
}
