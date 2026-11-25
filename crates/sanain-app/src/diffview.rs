//! Side-by-side comparison of two tabs.

use std::ops::Range;

use eframe::egui::{self, Align2, FontId, Rect, Sense, Stroke, pos2, vec2};
use sanain_tools::diff::{Diff, Row, split_lines};

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
    pub to_right: String,
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
                let side = |r: Rect,
                            line: Option<usize>,
                            lines: &[&str],
                            bg: Option<egui::Color32>,
                            mark: &str| {
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
                        let text = lines
                            .get(n)
                            .map_or("", |l| l.trim_end_matches(['\n', '\r']));
                        painter.text(
                            pos2(r.min.x + gutter, r.center().y),
                            Align2::LEFT_CENTER,
                            text.replace('\t', "    "),
                            font.clone(),
                            p.text,
                        );
                    }
                };
                side(lrect, lline, &ll, lbg, lmark);
                side(rrect, rline, &rl, rbg, rmark);
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
                    if ui
                        .put(half_m2, egui::Button::new("›").frame(false).small())
                        .on_hover_text(&labels.to_right)
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
