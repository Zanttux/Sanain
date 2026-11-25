//! The text editing surface: renders only visible lines from the rope and
//! translates input into editor commands.

use eframe::egui::{
    self, Align2, CursorIcon, Event, EventFilter, FontId, Galley, ImeEvent, Key, Pos2, Rect, Sense,
    Stroke, Ui, Vec2, pos2, text::CCursor, vec2,
};
use sanain_app::syntax::CharSpan;
use sanain_app::{Command, Editor};
use sanain_core::selection::line_content_end;
use std::sync::Arc;

use crate::theme;

/// Per-frame options from user settings.
#[derive(Debug, Clone)]
pub struct ViewOptions {
    pub font_size: f32,
    /// Text inserted by the Tab key.
    pub indent: String,
    /// Tab width in columns, for indentation-based folding.
    pub tab_width: usize,
}

#[derive(Debug, Default)]
pub struct EditorView {
    /// IME composition text, shown at the caret until committed.
    preedit: String,
    /// Widest line seen so far, for the horizontal scroll extent.
    max_width: f32,
    focus_requested: bool,
    reveal_requested: bool,
    /// Where an Alt+drag block selection started (relative to the text
    /// area's origin).
    block_from: Option<Vec2>,
    /// The editor widget's id, to tell it apart from other focused fields.
    pub widget_id: Option<egui::Id>,
}

impl EditorView {
    pub fn request_focus(&mut self) {
        self.focus_requested = true;
    }

    /// Scroll the primary caret into view on the next frame.
    pub fn reveal(&mut self) {
        self.reveal_requested = true;
    }

    /// Returns true when the document or selection changed.
    /// `highlights` are ascending, non-overlapping char ranges (search
    /// matches) painted behind the text.
    pub fn show(
        &mut self,
        ui: &mut Ui,
        editor: &mut Editor,
        accessible_name: &str,
        highlights: &[std::ops::Range<usize>],
        syntax: &[CharSpan],
        opts: &ViewOptions,
    ) -> bool {
        let font = FontId::monospace(opts.font_size);
        let (row_h, char_w) = ui.fonts_mut(|f| (f.row_height(&font), f.glyph_width(&font, '0')));
        let palette = theme::palette(ui.ctx());
        let id = ui.make_persistent_id("sanain-editor");
        self.widget_id = Some(id);
        let mut changed = false;

        egui::ScrollArea::both()
            .auto_shrink(false)
            .show_viewport(ui, |ui, viewport| {
                let line_count = editor.document().rope().len_lines();
                let tab = opts.tab_width.max(1);
                let folds = editor.fold_map(tab).clone();
                let digits = line_count.to_string().len().max(3) as f32;
                // Line numbers, then a fold marker column.
                let gutter_w = (digits + 3.5) * char_w;
                let rows = folds.rows(line_count);
                let size = vec2(
                    (gutter_w + self.max_width + 4.0 * char_w).max(ui.available_width()),
                    (rows as f32 * row_h + row_h * 4.0).max(ui.available_height()),
                );
                let (_, rect) = ui.allocate_space(size);
                let response = ui.interact(rect, id, Sense::click_and_drag());
                let response = response.on_hover_cursor(CursorIcon::Text);
                response.widget_info(|| {
                    egui::WidgetInfo::labeled(egui::Role::MultilineTextInput, true, accessible_name)
                });
                let origin = rect.min;
                let text_x = origin.x + gutter_w;

                if self.focus_requested {
                    response.request_focus();
                    self.focus_requested = false;
                }
                if response.has_focus() {
                    ui.memory_mut(|m| {
                        m.set_focus_lock_filter(
                            id,
                            EventFilter {
                                tab: true,
                                horizontal_arrows: true,
                                vertical_arrows: true,
                                // Esc collapses multiple cursors.
                                escape: editor.selections().len() > 1,
                            },
                        );
                    });
                }

                // Pointer: click places the caret, Shift+click and drag extend.
                let layout = |ui: &Ui, line: usize, editor: &Editor| -> Arc<Galley> {
                    let rope = editor.document().rope();
                    let start = rope.line_to_char(line);
                    let end = line_content_end(rope, line);
                    let text = rope.slice(start..end).to_string();
                    let job = line_job(text, start, syntax, &font, palette);
                    ui.fonts_mut(|f| f.layout_job(job))
                };
                let line_at = |pos: Pos2| -> usize {
                    let row = ((pos.y - origin.y) / row_h).floor().max(0.0) as usize;
                    folds.line_of_row(row).min(line_count.saturating_sub(1))
                };
                let hit_in = |ui: &Ui, line: usize, x: f32, editor: &Editor| -> usize {
                    let galley = layout(ui, line, editor);
                    let c = galley.cursor_from_pos(vec2(x - text_x, row_h / 2.0));
                    editor.document().rope().line_to_char(line) + c.index.0
                };
                let hit = |ui: &Ui, pos: Pos2, editor: &Editor| -> usize {
                    hit_in(ui, line_at(pos), pos.x, editor)
                };
                // A click on the fold marker column toggles the fold.
                let marker_x = (text_x - 1.6 * char_w)..(text_x - 0.2 * char_w);
                let mut toggled = false;
                if response.clicked()
                    && let Some(pos) = response.interact_pointer_pos()
                    && marker_x.contains(&pos.x)
                {
                    toggled = editor.toggle_fold(line_at(pos), tab);
                    changed |= toggled;
                }
                if let Some(pos) = response.interact_pointer_pos().filter(|_| !toggled) {
                    let shift = ui.input(|i| i.modifiers.shift);
                    let alt = ui.input(|i| i.modifiers.alt);
                    if alt && response.drag_started() {
                        self.block_from = Some(pos - origin);
                    }
                    if alt && response.clicked() {
                        // Alt+click adds (or removes) a caret.
                        response.request_focus();
                        let at = hit(ui, pos, editor);
                        editor.execute(Command::AddCaret { at });
                        changed = true;
                    } else if let Some(from) = self.block_from.filter(|_| response.dragged()) {
                        // Alt+drag: one selection per visible line, between
                        // the start and current x positions.
                        response.request_focus();
                        let from = origin + from;
                        let (r0, r1) = (
                            folds.row_of_line(line_at(from)),
                            folds.row_of_line(line_at(pos)),
                        );
                        let rows: Vec<usize> = if r0 <= r1 {
                            (r0..=r1).collect()
                        } else {
                            (r1..=r0).rev().collect()
                        };
                        let pairs = rows
                            .into_iter()
                            .map(|r| folds.line_of_row(r).min(line_count.saturating_sub(1)))
                            .map(|l| (hit_in(ui, l, from.x, editor), hit_in(ui, l, pos.x, editor)))
                            .collect();
                        editor.execute(Command::SetSelections(pairs));
                        changed = true;
                    } else if response.drag_started() || response.clicked() {
                        response.request_focus();
                        let at = hit(ui, pos, editor);
                        editor.execute(Command::PlaceCaret { at, extend: shift });
                        changed = true;
                    } else if response.dragged() {
                        let at = hit(ui, pos, editor);
                        editor.execute(Command::PlaceCaret { at, extend: true });
                        changed = true;
                    }
                }

                if !response.dragged() {
                    self.block_from = None;
                }

                // Keyboard and text input.
                let mut moved_by_keyboard = false;
                if response.has_focus() {
                    let events = ui.input(|i| i.events.clone());
                    for event in events {
                        if let Some(done) = self.handle_event(ui, editor, &event, &opts.indent) {
                            changed = true;
                            moved_by_keyboard |= done;
                        }
                    }
                }

                // Paint visible lines. Folds may have changed this frame.
                let folds = editor.fold_map(tab).clone();
                let line_count = editor.document().rope().len_lines();
                let painter = ui.painter_at(viewport.translate(origin.to_vec2()).intersect(rect));
                let first = (viewport.min.y / row_h).floor().max(0.0) as usize;
                let last =
                    ((viewport.max.y / row_h).ceil() as usize + 1).min(folds.rows(line_count));
                let rope = editor.document().rope();
                let sels: Vec<_> = editor.selections().iter().copied().collect();
                let primary_head = editor.selections().primary().head;
                let mut caret_rect = None;
                for row in first..last {
                    let line = folds.line_of_row(row);
                    let y = origin.y + row as f32 * row_h;
                    let galley = layout(ui, line, editor);
                    self.max_width = self.max_width.max(galley.size().x);
                    let start = rope.line_to_char(line);
                    let content_end = line_content_end(rope, line);
                    let line_end = start + rope.line(line).len_chars();
                    let x_of = |idx: usize| -> f32 {
                        let col = idx.clamp(start, content_end) - start;
                        text_x + galley.pos_from_cursor(CCursor::new(col)).min.x
                    };

                    let first_hit = highlights.partition_point(|h| h.end <= start);
                    for h in highlights[first_hit..]
                        .iter()
                        .take_while(|h| h.start < line_end)
                    {
                        let x0 = x_of(h.start.max(start));
                        let x1 = x_of(h.end.min(content_end)).max(x0 + 2.0);
                        painter.rect_filled(
                            Rect::from_min_max(pos2(x0, y), pos2(x1, y + row_h)),
                            2.0,
                            palette.warning.gamma_multiply(0.35),
                        );
                    }

                    for s in &sels {
                        let r = s.range();
                        // A selection starting at the next line's first char
                        // does not touch this line's break.
                        let starts_after =
                            r.start > line_end || (r.start == line_end && line_end > content_end);
                        if r.is_empty() || r.end <= start || starts_after {
                            continue;
                        }
                        let x0 = x_of(r.start.max(start));
                        let mut x1 = x_of(r.end.min(content_end));
                        if r.end > content_end && line_end > content_end {
                            x1 += char_w * 0.6; // show that the line break is selected
                        }
                        painter.rect_filled(
                            Rect::from_min_max(pos2(x0, y), pos2(x1, y + row_h)),
                            0.0,
                            palette.accent.gamma_multiply(0.3),
                        );
                    }

                    painter.text(
                        pos2(text_x - 2.0 * char_w, y),
                        Align2::RIGHT_TOP,
                        (line + 1).to_string(),
                        font.clone(),
                        palette.muted,
                    );
                    let folded = editor.is_folded(line);
                    if folded || sanain_core::folding::is_fold_start(rope, line, tab) {
                        painter.text(
                            pos2(text_x - 0.9 * char_w, y),
                            Align2::CENTER_TOP,
                            if folded { "▸" } else { "▾" },
                            font.clone(),
                            palette.muted,
                        );
                    }
                    painter.galley(pos2(text_x, y), galley.clone(), palette.text);
                    if folded {
                        let x = text_x + galley.size().x + char_w;
                        let r =
                            Rect::from_min_size(pos2(x, y + 2.0), vec2(char_w * 3.0, row_h - 4.0));
                        painter.rect_filled(r, 3.0, palette.surface);
                        painter.text(
                            r.center(),
                            Align2::CENTER_CENTER,
                            "…",
                            font.clone(),
                            palette.muted,
                        );
                    }

                    for s in &sels {
                        if s.head < start || s.head > content_end {
                            continue;
                        }
                        let x = x_of(s.head);
                        let caret = Rect::from_min_size(pos2(x, y), vec2(2.0, row_h));
                        if s.head == primary_head {
                            caret_rect = Some(caret);
                            if !self.preedit.is_empty() {
                                let g = ui.fonts_mut(|f| {
                                    f.layout_no_wrap(
                                        self.preedit.clone(),
                                        font.clone(),
                                        palette.text,
                                    )
                                });
                                let bg = Rect::from_min_size(pos2(x, y), g.size());
                                painter.rect_filled(bg, 0.0, palette.surface);
                                painter.galley(pos2(x, y), g.clone(), palette.text);
                                painter.line_segment(
                                    [
                                        pos2(x, y + row_h - 1.0),
                                        pos2(x + g.size().x, y + row_h - 1.0),
                                    ],
                                    Stroke::new(1.0, palette.text),
                                );
                            }
                        }
                        painter.rect_filled(caret, 0.0, palette.text);
                    }
                }

                // Caret rect for IME placement and scrolling, even off-screen.
                let caret_rect = caret_rect.unwrap_or_else(|| {
                    let (line, _) = sanain_core::selection::line_col(rope, primary_head);
                    let galley = layout(ui, line, editor);
                    let col = primary_head - rope.line_to_char(line);
                    let x = text_x + galley.pos_from_cursor(CCursor::new(col)).min.x;
                    let row = folds.row_of_line(line);
                    Rect::from_min_size(pos2(x, origin.y + row as f32 * row_h), vec2(2.0, row_h))
                });
                if moved_by_keyboard || std::mem::take(&mut self.reveal_requested) {
                    ui.scroll_to_rect(caret_rect.expand2(Vec2::new(char_w * 4.0, row_h)), None);
                }
                if response.has_focus() {
                    ui.output_mut(|o| {
                        o.ime = Some(egui::output::IMEOutput {
                            purpose: egui::IMEPurpose::Normal,
                            rect,
                            cursor_rect: caret_rect,
                            should_interrupt_composition: false,
                        });
                    });
                }
                // Screen readers: expose the caret's line as text with the
                // caret/selection inside it. egui only builds this when an
                // accessibility client is active.
                if response.has_focus() {
                    let sel = editor.selections().primary();
                    let (line, col) = sanain_core::selection::line_col(rope, sel.head);
                    let start = rope.line_to_char(line);
                    let end = line_content_end(rope, line);
                    let galley = layout(ui, line, editor);
                    let head = CCursor::new(col.min(end - start));
                    let anchor = if (start..=end).contains(&sel.anchor) {
                        CCursor::new(sel.anchor - start)
                    } else if sel.anchor < start {
                        CCursor::new(0)
                    } else {
                        CCursor::new(end - start)
                    };
                    let range = egui::text_selection::CCursorRange::two(anchor, head);
                    let origin = vec2(text_x, origin.y + folds.row_of_line(line) as f32 * row_h);
                    egui::text_selection::accesskit_text::update_accesskit_for_text_widget(
                        ui.ctx(),
                        id,
                        Some(range),
                        egui::emath::TSTransform::from_translation(origin),
                        &galley,
                    );
                    let text = galley.text().to_owned();
                    ui.ctx()
                        .accesskit_node_builder(id, |node| node.set_value(text));
                }
            });
        changed
    }

    /// Handle one input event. Returns `Some(scroll)` if it changed state,
    /// where `scroll` asks to bring the caret into view.
    fn handle_event(
        &mut self,
        ui: &Ui,
        editor: &mut Editor,
        event: &Event,
        indent: &str,
    ) -> Option<bool> {
        let cmd = match event {
            Event::Text(t) if !t.is_empty() => Command::Insert(t.clone()),
            Event::Paste(t) if !t.is_empty() => Command::Insert(t.clone()),
            Event::Ime(ImeEvent::Preedit { text: t, .. }) => {
                self.preedit = t.clone();
                return Some(false);
            }
            Event::Ime(ImeEvent::Commit(t)) => {
                self.preedit.clear();
                if t.is_empty() {
                    return Some(false);
                }
                Command::Insert(t.clone())
            }
            Event::Ime(ImeEvent::DeleteSurrounding {
                before_chars,
                after_chars,
            }) => {
                let head = editor.selections().primary().head;
                let len = editor.document().len_chars();
                editor.execute(Command::PlaceCaret {
                    at: head.saturating_sub(*before_chars),
                    extend: false,
                });
                editor.execute(Command::PlaceCaret {
                    at: (head + after_chars).min(len),
                    extend: true,
                });
                Command::Insert(String::new())
            }
            Event::Copy => {
                if let Some(t) = editor.copy() {
                    ui.ctx().copy_text(t);
                }
                return None;
            }
            Event::Cut => {
                let t = editor.execute(Command::Cut)?;
                ui.ctx().copy_text(t);
                return Some(true);
            }
            Event::Key {
                key,
                pressed: true,
                modifiers: m,
                ..
            } => {
                // While composing, the IME owns navigation keys.
                if !self.preedit.is_empty() {
                    return None;
                }
                let extend = m.shift;
                // Word navigation: Option on macOS, Ctrl elsewhere.
                let word = if cfg!(target_os = "macos") {
                    m.alt
                } else {
                    m.ctrl
                };
                match key {
                    Key::Enter if !m.command => Command::NewLine,
                    Key::Escape if editor.selections().len() > 1 => Command::SingleSelection,
                    Key::Tab if !m.command && !m.shift => Command::Insert(indent.to_owned()),
                    Key::ArrowUp if m.alt && m.shift => Command::DuplicateLine,
                    Key::ArrowDown if m.alt && m.shift => Command::DuplicateLine,
                    Key::ArrowUp if m.alt && !m.command => Command::MoveLineUp,
                    Key::ArrowDown if m.alt && !m.command => Command::MoveLineDown,
                    Key::K if m.command && m.shift => Command::DeleteLine,
                    Key::Backspace if word => Command::DeleteWordBackward,
                    Key::Delete if word => Command::DeleteWordForward,
                    Key::ArrowLeft if word => Command::WordLeft { extend },
                    Key::ArrowRight if word => Command::WordRight { extend },
                    Key::Backspace => Command::Backspace,
                    Key::Delete => Command::Delete,
                    Key::ArrowLeft if m.mac_cmd => Command::LineStart { extend },
                    Key::ArrowRight if m.mac_cmd => Command::LineEnd { extend },
                    Key::ArrowLeft => Command::Left { extend },
                    Key::ArrowRight => Command::Right { extend },
                    Key::ArrowUp => Command::Up { extend },
                    Key::ArrowDown => Command::Down { extend },
                    Key::Home => Command::LineStart { extend },
                    Key::End => Command::LineEnd { extend },
                    Key::A if m.command => Command::SelectAll,
                    Key::Z if m.command && m.shift => Command::Redo,
                    Key::Z if m.command => Command::Undo,
                    Key::Y if m.command && !m.mac_cmd => Command::Redo,
                    _ => return None,
                }
            }
            _ => return None,
        };
        editor.execute(cmd);
        Some(true)
    }
}

/// Build a layout job for one line's text (starting at document char
/// `start`), colouring the parts covered by syntax spans.
fn line_job(
    text: String,
    start: usize,
    syntax: &[CharSpan],
    font: &FontId,
    palette: &theme::Palette,
) -> egui::text::LayoutJob {
    use egui::text::{LayoutJob, TextFormat};
    let plain = TextFormat::simple(font.clone(), palette.text);
    let end = start + text.chars().count();
    let first = syntax.partition_point(|s| s.range.end <= start);
    let spans = syntax[first..].iter().take_while(|s| s.range.start < end);
    // Spans ascend, so walk char → byte offsets incrementally.
    let (mut chr, mut byte) = (0usize, 0usize);
    let advance = |chr: &mut usize, byte: &mut usize, to: usize| -> usize {
        while *chr < to {
            match text[*byte..].chars().next() {
                Some(c) => *byte += c.len_utf8(),
                None => break,
            }
            *chr += 1;
        }
        *byte
    };
    let mut job = LayoutJob::default();
    let mut cursor = 0usize; // byte offset in `text` already appended
    for s in spans {
        let from = s.range.start.max(start) - start;
        let to = s.range.end.min(end) - start;
        if from < chr {
            continue; // overlapping span; keep the first
        }
        let a = advance(&mut chr, &mut byte, from);
        let b = advance(&mut chr, &mut byte, to);
        if a >= b {
            continue;
        }
        if a > cursor {
            job.append(&text[cursor..a], 0.0, plain.clone());
        }
        let fmt = TextFormat::simple(font.clone(), theme::syntax(palette, s.style));
        job.append(&text[a..b], 0.0, fmt);
        cursor = b;
    }
    if cursor < text.len() || job.sections.is_empty() {
        job.append(&text[cursor..], 0.0, plain);
    }
    job
}
