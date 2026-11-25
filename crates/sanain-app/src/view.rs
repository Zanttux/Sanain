//! The text editing surface: renders only visible lines from the rope and
//! translates input into editor commands.

use eframe::egui::{
    self, Align2, CursorIcon, Event, EventFilter, FontId, Galley, ImeEvent, Key, Pos2, Rect, Sense,
    Stroke, Ui, Vec2, pos2, text::CCursor, vec2,
};
use sanain_app::{Command, Editor};
use sanain_core::selection::line_content_end;
use std::sync::Arc;

use crate::theme;

const FONT_SIZE: f32 = 14.0;

#[derive(Debug, Default)]
pub struct EditorView {
    /// IME composition text, shown at the caret until committed.
    preedit: String,
    /// Widest line seen so far, for the horizontal scroll extent.
    max_width: f32,
    focus_requested: bool,
}

impl EditorView {
    pub fn request_focus(&mut self) {
        self.focus_requested = true;
    }

    /// Returns true when the document or selection changed.
    pub fn show(&mut self, ui: &mut Ui, editor: &mut Editor, accessible_name: &str) -> bool {
        let font = FontId::monospace(FONT_SIZE);
        let (row_h, char_w) = ui.fonts_mut(|f| (f.row_height(&font), f.glyph_width(&font, '0')));
        let palette = theme::palette(ui.ctx());
        let id = ui.make_persistent_id("sanain-editor");
        let mut changed = false;

        egui::ScrollArea::both()
            .auto_shrink(false)
            .show_viewport(ui, |ui, viewport| {
                let line_count = editor.document().rope().len_lines();
                let digits = line_count.to_string().len().max(3) as f32;
                let gutter_w = (digits + 2.0) * char_w;
                let size = vec2(
                    (gutter_w + self.max_width + 4.0 * char_w).max(ui.available_width()),
                    (line_count as f32 * row_h + row_h * 4.0).max(ui.available_height()),
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
                                escape: false,
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
                    ui.fonts_mut(|f| f.layout_no_wrap(text, font.clone(), palette.text))
                };
                let hit = |ui: &Ui, pos: Pos2, editor: &Editor| -> usize {
                    let rope = editor.document().rope();
                    let line = (((pos.y - origin.y) / row_h).floor().max(0.0) as usize)
                        .min(rope.len_lines().saturating_sub(1));
                    let galley = layout(ui, line, editor);
                    let c = galley.cursor_from_pos(vec2(pos.x - text_x, row_h / 2.0));
                    rope.line_to_char(line) + c.index.0
                };
                if let Some(pos) = response.interact_pointer_pos() {
                    let shift = ui.input(|i| i.modifiers.shift);
                    if response.drag_started() || response.clicked() {
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

                // Keyboard and text input.
                let mut moved_by_keyboard = false;
                if response.has_focus() {
                    let events = ui.input(|i| i.events.clone());
                    for event in events {
                        if let Some(done) = self.handle_event(ui, editor, &event) {
                            changed = true;
                            moved_by_keyboard |= done;
                        }
                    }
                }

                // Paint visible lines.
                let painter = ui.painter_at(viewport.translate(origin.to_vec2()).intersect(rect));
                let first = (viewport.min.y / row_h).floor().max(0.0) as usize;
                let last = ((viewport.max.y / row_h).ceil() as usize + 1).min(line_count);
                let rope = editor.document().rope();
                let sels: Vec<_> = editor.selections().iter().copied().collect();
                let primary_head = editor.selections().primary().head;
                let mut caret_rect = None;
                for line in first..last {
                    let y = origin.y + line as f32 * row_h;
                    let galley = layout(ui, line, editor);
                    self.max_width = self.max_width.max(galley.size().x);
                    let start = rope.line_to_char(line);
                    let content_end = line_content_end(rope, line);
                    let line_end = start + rope.line(line).len_chars();
                    let x_of = |idx: usize| -> f32 {
                        let col = idx.clamp(start, content_end) - start;
                        text_x + galley.pos_from_cursor(CCursor::new(col)).min.x
                    };

                    for s in &sels {
                        let r = s.range();
                        if r.is_empty() || r.end <= start || r.start > line_end {
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
                        pos2(text_x - char_w, y),
                        Align2::RIGHT_TOP,
                        (line + 1).to_string(),
                        font.clone(),
                        palette.muted,
                    );
                    painter.galley(pos2(text_x, y), galley.clone(), palette.text);

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
                    Rect::from_min_size(pos2(x, origin.y + line as f32 * row_h), vec2(2.0, row_h))
                });
                if moved_by_keyboard {
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
            });
        changed
    }

    /// Handle one input event. Returns `Some(scroll)` if it changed state,
    /// where `scroll` asks to bring the caret into view.
    fn handle_event(&mut self, ui: &Ui, editor: &mut Editor, event: &Event) -> Option<bool> {
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
                match key {
                    Key::Enter if !m.command => Command::NewLine,
                    Key::Tab if !m.command && !m.shift => Command::Insert("\t".into()),
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
