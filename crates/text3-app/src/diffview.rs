//! Side-by-side comparison of two tabs.

use std::ops::Range;

use eframe::egui::{self, Align2, FontId, Rect, Sense, Stroke, pos2, vec2};
use text3_tools::diff::{Diff, Row, intraline, split_lines};

use crate::theme::Palette;

/// A request to copy one hunk across.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplyDir {
    /// Make the left side match the right in this hunk.
    ToLeft(usize),
    /// Make the right side match the left in this hunk.
    ToRight(usize),
}

pub struct Labels {
    pub to_left: String,
    /// `None` when the right side is read-only (saved file, clipboard).
    pub to_right: Option<String>,
}

/// Draw the comparison. Returns a hunk to apply, if one was clicked.
pub fn show(
    ui: &mut egui::Ui,
    diff: &Diff,
    left: &str,
    right: &str,
    p: &Palette,
    font_size: f32,
    labels: &Labels,
) -> Option<ApplyDir> {
    let font = FontId::monospace(font_size);
    let row_h = ui.fonts_mut(|f| f.row_height(&font)) + 2.0;
    let char_w = ui.fonts_mut(|f| f.glyph_width(&font, '0'));
    let (ll, rl) = (split_lines(left), split_lines(right));
    let digits = ll.len().max(rl.len()).max(1).to_string().len() as f32;
    let gutter = (digits + 3.0) * char_w;
    let hunk_starts: Vec<(usize, usize)> = diff
        .hunks
        .iter()
        .enumerate()
        .map(|(i, h)| (h.rows.start, i))
        .collect();
    let mut action = None;

    let del_bg = p.error.gamma_multiply(0.18);
    let ins_bg = p.accent.gamma_multiply(0.18);
    let del_strong = p.error.gamma_multiply(0.45);
    let ins_strong = p.accent.gamma_multiply(0.45);

    egui::ScrollArea::vertical().auto_shrink(false).show_rows(
        ui,
        row_h,
        diff.rows.len(),
        |ui, range: Range<usize>| {
            for idx in range {
                let row = diff.rows[idx];
                let (rect, _) =
                    ui.allocate_exact_size(vec2(ui.available_width(), row_h), Sense::hover());
                let mid = 28.0;
                let half = (rect.width() - mid) / 2.0;
                let lrect = Rect::from_min_size(rect.min, vec2(half, row_h));
                let mrect =
                    Rect::from_min_size(pos2(rect.min.x + half, rect.min.y), vec2(mid, row_h));
                let rrect = Rect::from_min_size(
                    pos2(rect.min.x + half + mid, rect.min.y),
                    vec2(half, row_h),
                );
                let (lline, rline, lbg, rbg, lmark, rmark) = match row {
                    Row::Same { left, right } => (Some(left), Some(right), None, None, " ", " "),
                    Row::Changed { left, right } => (
                        Some(left),
                        Some(right),
                        Some(del_bg),
                        Some(ins_bg),
                        "−",
                        "+",
                    ),
                    Row::LeftOnly(l) => (Some(l), None, Some(del_bg), None, "−", " "),
                    Row::RightOnly(r) => (None, Some(r), None, Some(ins_bg), " ", "+"),
                };
                let line_text = |lines: &[&str], n: Option<usize>| -> String {
                    n.and_then(|n| lines.get(n))
                        .map_or("", |l| l.trim_end_matches(['\n', '\r']))
                        .to_owned()
                };
                // Changed characters within a changed pair of lines.
                let (lspans, rspans) = match row {
                    Row::Changed { left, right } => {
                        intraline(&line_text(&ll, Some(left)), &line_text(&rl, Some(right)))
                    }
                    _ => (Vec::new(), Vec::new()),
                };
                let side = |r: Rect,
                            line: Option<usize>,
                            lines: &[&str],
                            bg: Option<egui::Color32>,
                            mark: &str,
                            spans: &[Range<usize>],
                            strong: egui::Color32| {
                    let painter = ui.painter_at(r);
                    if let Some(bg) = bg {
                        painter.rect_filled(r, 0.0, bg);
                    }
                    if let Some(n) = line {
                        painter.text(
                            pos2(r.min.x + gutter - 2.0 * char_w, r.center().y),
                            Align2::RIGHT_CENTER,
                            (n + 1).to_string(),
                            font.clone(),
                            p.muted,
                        );
                        painter.text(
                            pos2(r.min.x + gutter - 1.2 * char_w, r.center().y),
                            Align2::LEFT_CENTER,
                            mark,
                            font.clone(),
                            p.muted,
                        );
                        let raw = line_text(lines, Some(n));
                        // Tabs are drawn as four spaces; map char indices.
                        let shown = |idx: usize| -> usize {
                            idx + 3 * raw.chars().take(idx).filter(|c| *c == '\t').count()
                        };
                        let galley = ui.fonts_mut(|f| {
                            f.layout_no_wrap(raw.replace('\t', "    "), font.clone(), p.text)
                        });
                        let origin = pos2(r.min.x + gutter, r.center().y - galley.size().y / 2.0);
                        for span in spans {
                            let x0 = galley
                                .pos_from_cursor(egui::text::CCursor::new(shown(span.start)))
                                .min
                                .x;
                            let x1 = galley
                                .pos_from_cursor(egui::text::CCursor::new(shown(span.end)))
                                .min
                                .x;
                            painter.rect_filled(
                                Rect::from_min_max(
                                    pos2(origin.x + x0, r.min.y + 1.0),
                                    pos2(origin.x + x1.max(x0 + 2.0), r.max.y - 1.0),
                                ),
                                2.0,
                                strong,
                            );
                        }
                        painter.galley(origin, galley, p.text);
                    }
                };
                side(lrect, lline, &ll, lbg, lmark, &lspans, del_strong);
                side(rrect, rline, &rl, rbg, rmark, &rspans, ins_strong);
                ui.painter().vline(
                    mrect.center().x,
                    mrect.y_range(),
                    Stroke::new(1.0, p.border),
                );
                if let Some(&(_, h)) = hunk_starts.iter().find(|(s, _)| *s == idx) {
                    let half_m = Rect::from_min_size(mrect.min, vec2(mid / 2.0, row_h));
                    let half_m2 = Rect::from_min_size(
                        pos2(mrect.min.x + mid / 2.0, mrect.min.y),
                        vec2(mid / 2.0, row_h),
                    );
                    if ui
                        .put(half_m, egui::Button::new("‹").frame(false).small())
                        .on_hover_text(&labels.to_left)
                        .clicked()
                    {
                        action = Some(ApplyDir::ToLeft(h));
                    }
                    if let Some(tip) = &labels.to_right
                        && ui
                            .put(half_m2, egui::Button::new("›").frame(false).small())
                            .on_hover_text(tip)
                            .clicked()
                    {
                        action = Some(ApplyDir::ToRight(h));
                    }
                }
            }
        },
    );
    action
}
