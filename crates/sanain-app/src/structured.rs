//! Structured views: an editable CSV grid and read-only trees. The source
//! stays authoritative: grid edits come back as [`GridAction`]s that the
//! caller applies to the text as ordinary undoable edits.

use eframe::egui::{self, RichText};
use egui_extras::{Column, TableBuilder};
use sanain_tools::csv::Table;
use sanain_tools::json::Node;

use crate::theme::Palette;

/// JSON nesting shown before the rest is summarised.
const MAX_TREE_DEPTH: usize = 64;
/// Children listed per container before "… more".
const MAX_CHILDREN: usize = 1_000;

/// Selection and inline cell editor of one tab's CSV grid.
#[derive(Debug, Default)]
pub struct GridState {
    /// Selected cell as (table row, column); row 0 is the first line,
    /// shown as the header.
    pub selected: Option<(usize, usize)>,
    /// The cell being edited.
    pub editing: Option<CellEdit>,
    /// Bring the selected cell into view on the next frame.
    scroll: bool,
}

#[derive(Debug)]
pub struct CellEdit {
    pub row: usize,
    pub col: usize,
    pub text: String,
    focus: bool,
}

impl GridState {
    fn start_edit(&mut self, t: &Table, row: usize, col: usize) {
        let text = t
            .rows
            .get(row)
            .and_then(|r| r.get(col))
            .cloned()
            .unwrap_or_default();
        self.selected = Some((row, col));
        self.editing = Some(CellEdit {
            row,
            col,
            text,
            focus: true,
        });
    }

    /// Move the selection by whole cells, clamped to the grid; `wrap`
    /// continues on the next or previous row past a row's end (Tab).
    fn step(&mut self, rows: usize, cols: usize, dr: isize, dc: isize, wrap: bool) {
        let (r, c) = self.selected.unwrap_or((0, 0));
        let (mut r, mut c) = (r as isize + dr, c as isize + dc);
        if wrap {
            if c >= cols as isize && r + 1 < rows as isize {
                (r, c) = (r + 1, 0);
            } else if c < 0 && r > 0 {
                (r, c) = (r - 1, cols as isize - 1);
            }
        }
        let clamp = |v: isize, n: usize| v.clamp(0, n.saturating_sub(1) as isize) as usize;
        self.selected = Some((clamp(r, rows), clamp(c, cols)));
        self.scroll = true;
    }
}

/// A change the user made in the grid, for the caller to apply to the text
/// as one undoable edit. Rows are table rows (0 is the header line).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GridAction {
    SetCell {
        row: usize,
        col: usize,
        value: String,
    },
    InsertRowAfter(usize),
    DeleteRow(usize),
}

/// Localized labels of the grid's context menu.
pub struct GridLabels {
    pub edit_cell: String,
    pub insert_row_below: String,
    pub delete_row: String,
}

/// The grid. With `editable`, a double-click or Enter on the selected cell
/// edits it inline: Enter commits, Escape cancels, Tab commits and edits
/// the next cell; arrow keys move the selection while `id` (the grid's
/// focus) has keyboard focus. Returns what to change in the text.
#[allow(clippy::too_many_arguments)]
pub fn csv(
    ui: &mut egui::Ui,
    id: egui::Id,
    t: &Table,
    p: &Palette,
    size: f32,
    header: bool,
    editable: bool,
    state: &mut GridState,
    labels: &GridLabels,
) -> Vec<GridAction> {
    let mut actions = Vec::new();
    let cols = t.rows.iter().map(Vec::len).max().unwrap_or(0);
    if cols == 0 {
        return actions;
    }
    let rows = t.rows.len();
    if state.selected.is_some_and(|(r, c)| r >= rows || c >= cols) {
        state.selected = Some((0, 0));
    }
    if !editable || state.editing.as_ref().is_some_and(|e| e.row >= rows) {
        state.editing = None;
    }
    // The grid's own focus target: a widget that takes keyboard focus but
    // no clicks, under the cells.
    let focus = ui.interact(ui.max_rect(), id, egui::Sense::focusable_noninteractive());
    if focus.has_focus() && state.editing.is_none() {
        ui.memory_mut(|m| {
            m.set_focus_lock_filter(
                id,
                egui::EventFilter {
                    tab: true,
                    horizontal_arrows: true,
                    vertical_arrows: true,
                    escape: false,
                },
            );
        });
        use egui::Key;
        let pressed = |k| ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, k));
        if pressed(Key::ArrowUp) {
            state.step(rows, cols, -1, 0, false);
        }
        if pressed(Key::ArrowDown) {
            state.step(rows, cols, 1, 0, false);
        }
        if pressed(Key::ArrowLeft) {
            state.step(rows, cols, 0, -1, false);
        }
        if pressed(Key::ArrowRight) {
            state.step(rows, cols, 0, 1, false);
        }
        if ui.input_mut(|i| i.consume_key(egui::Modifiers::SHIFT, Key::Tab)) {
            state.step(rows, cols, 0, -1, true);
        }
        if pressed(Key::Tab) {
            state.step(rows, cols, 0, 1, true);
        }
        if editable && let Some((r, c)) = state.selected {
            if pressed(Key::Enter) || pressed(Key::F2) {
                state.start_edit(t, r, c);
            } else if pressed(Key::Delete) || pressed(Key::Backspace) {
                actions.push(GridAction::SetCell {
                    row: r,
                    col: c,
                    value: String::new(),
                });
            }
        }
    }
    let row_h = size * 1.6;
    let first_body = usize::from(header);
    let scroll_row = state
        .scroll
        .then_some(state.selected)
        .flatten()
        .and_then(|(r, _)| r.checked_sub(first_body));
    let row_ops = |ui: &mut egui::Ui, actions: &mut Vec<GridAction>, r: usize| {
        if ui.button(&labels.insert_row_below).clicked() {
            actions.push(GridAction::InsertRowAfter(r));
        }
        if ui.button(&labels.delete_row).clicked() {
            actions.push(GridAction::DeleteRow(r));
        }
    };
    // One cell; `c` is `None` for the row number.
    let mut cell =
        |ui: &mut egui::Ui, state: &mut GridState, r: usize, c: Option<usize>, strong: bool| {
            let Some(c) = c else {
                ui.label(
                    RichText::new((r + 1 - first_body).to_string())
                        .color(p.muted)
                        .monospace(),
                );
                let resp = ui.interact(ui.max_rect(), id.with(("row", r)), egui::Sense::click());
                if resp.clicked() || resp.secondary_clicked() {
                    let c = state.selected.map_or(0, |(_, c)| c);
                    state.selected = Some((r, c));
                    ui.memory_mut(|m| m.request_focus(id));
                }
                if editable {
                    resp.context_menu(|ui| row_ops(ui, &mut actions, r));
                }
                return;
            };
            let editing_here = state
                .editing
                .as_ref()
                .is_some_and(|e| (e.row, e.col) == (r, c));
            if editing_here && let Some(ed) = &mut state.editing {
                let editor_id = id.with("cell-editor");
                if ed.focus {
                    // A fresh state: the caret goes to the end of this value,
                    // not where it was in the previous cell.
                    egui::text_edit::TextEditState::default().store(ui.ctx(), editor_id);
                }
                // Tab stays in the field (no focus traversal); handled below.
                let resp = ui.add(
                    egui::TextEdit::singleline(&mut ed.text)
                        .id(editor_id)
                        .lock_focus(true)
                        .desired_width(f32::INFINITY),
                );
                if ed.focus {
                    resp.request_focus();
                    ed.focus = false;
                }
                let tab = if resp.has_focus() {
                    ui.input_mut(|i| {
                        if i.consume_key(egui::Modifiers::SHIFT, egui::Key::Tab) {
                            Some(-1)
                        } else if i.consume_key(egui::Modifiers::NONE, egui::Key::Tab) {
                            Some(1)
                        } else {
                            None
                        }
                    })
                } else {
                    None
                };
                if let Some(dc) = tab {
                    if let Some(ed) = state.editing.take() {
                        actions.push(GridAction::SetCell {
                            row: ed.row,
                            col: ed.col,
                            value: ed.text,
                        });
                    }
                    state.step(rows, cols, 0, dc, true);
                    if let Some((nr, nc)) = state.selected {
                        state.start_edit(t, nr, nc);
                    }
                } else if resp.lost_focus() {
                    let esc = ui.input(|i| i.key_pressed(egui::Key::Escape));
                    let ed = state.editing.take();
                    if !esc && let Some(ed) = ed {
                        actions.push(GridAction::SetCell {
                            row: ed.row,
                            col: ed.col,
                            value: ed.text,
                        });
                    }
                    // Enter or Escape: back to moving around the grid. A
                    // click elsewhere keeps the focus it moved to.
                    if esc || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        ui.memory_mut(|m| m.request_focus(id));
                    }
                }
                return;
            }
            let rect = ui.max_rect();
            let selected = state.selected == Some((r, c));
            if selected {
                ui.painter()
                    .rect_filled(rect.expand(1.0), 2.0, p.accent.gamma_multiply(0.25));
            }
            let v = t
                .rows
                .get(r)
                .and_then(|row| row.get(c))
                .map_or("", String::as_str);
            let shown = if strong && v.is_empty() && r == 0 && header {
                (c + 1).to_string()
            } else {
                // Embedded line breaks shown as a symbol to keep rows aligned.
                v.replace("\r\n", "⏎").replace(['\n', '\r'], "⏎")
            };
            let mut text = RichText::new(shown).color(p.text);
            if strong {
                text = text.strong();
            }
            ui.add(egui::Label::new(text).truncate());
            let resp = ui.interact(rect, id.with((r, c)), egui::Sense::click());
            let resp = if v.is_empty() {
                resp
            } else {
                resp.on_hover_text(v)
            };
            if selected && state.scroll {
                resp.scroll_to_me(None);
            }
            if resp.clicked() || resp.secondary_clicked() {
                state.selected = Some((r, c));
                ui.memory_mut(|m| m.request_focus(id));
            }
            if editable && resp.double_clicked() {
                state.start_edit(t, r, c);
            }
            if editable {
                resp.context_menu(|ui| {
                    if ui.button(&labels.edit_cell).clicked() {
                        state.start_edit(t, r, c);
                    }
                    ui.separator();
                    row_ops(ui, &mut actions, r);
                });
            }
        };
    egui::ScrollArea::horizontal().show(ui, |ui| {
        let mut builder = TableBuilder::new(ui)
            .id_salt(id)
            .striped(true)
            .resizable(true)
            .column(Column::exact(size * 3.5));
        if let Some(r) = scroll_row {
            builder = builder.scroll_to_row(r, None);
        }
        for _ in 0..cols {
            builder = builder.column(Column::initial(140.0).at_least(40.0).clip(true));
        }
        builder
            .header(row_h, |mut h| {
                h.col(|ui| {
                    ui.label("");
                });
                for c in 0..cols {
                    h.col(|ui| {
                        if header {
                            cell(ui, state, 0, Some(c), true);
                        } else {
                            ui.label(RichText::new((c + 1).to_string()).strong().color(p.text));
                        }
                    });
                }
            })
            .body(|b| {
                b.rows(row_h, rows - first_body, |mut row| {
                    let r = row.index() + first_body;
                    row.col(|ui| cell(ui, state, r, None, false));
                    for c in 0..cols {
                        row.col(|ui| cell(ui, state, r, Some(c), false));
                    }
                });
            });
    });
    state.scroll = false;
    actions
}

pub fn json(ui: &mut egui::Ui, root: &Node, p: &Palette, more: &dyn Fn(usize) -> String) {
    egui::ScrollArea::both().auto_shrink(false).show(ui, |ui| {
        node(ui, None, root, p, 0, "$".to_owned(), more);
    });
}

fn node(
    ui: &mut egui::Ui,
    key: Option<&str>,
    n: &Node,
    p: &Palette,
    depth: usize,
    path: String,
    more: &dyn Fn(usize) -> String,
) {
    let label = |summary: String| {
        let mut s = String::new();
        if let Some(k) = key {
            s.push_str(k);
            s.push_str(": ");
        }
        s.push_str(&summary);
        s
    };
    match n {
        Node::Scalar(v) => {
            ui.horizontal(|ui| {
                if let Some(k) = key {
                    ui.label(RichText::new(format!("{k}:")).monospace().color(p.muted));
                }
                ui.label(RichText::new(v).monospace().color(p.text));
            });
        }
        Node::Object(_) | Node::Array(_) if depth >= MAX_TREE_DEPTH => {
            ui.label(RichText::new(label("…".into())).monospace().color(p.muted));
        }
        Node::Object(members) => {
            egui::CollapsingHeader::new(
                RichText::new(label(format!("{{{}}}", members.len()))).monospace(),
            )
            .id_salt(&path)
            .default_open(depth < 2)
            .show(ui, |ui| {
                for (i, (k, v)) in members.iter().take(MAX_CHILDREN).enumerate() {
                    node(ui, Some(k), v, p, depth + 1, format!("{path}.{i}"), more);
                }
                if members.len() > MAX_CHILDREN {
                    ui.label(RichText::new(more(members.len() - MAX_CHILDREN)).color(p.muted));
                }
            });
        }
        Node::Array(items) => {
            egui::CollapsingHeader::new(
                RichText::new(label(format!("[{}]", items.len()))).monospace(),
            )
            .id_salt(&path)
            .default_open(depth < 2)
            .show(ui, |ui| {
                for (i, v) in items.iter().take(MAX_CHILDREN).enumerate() {
                    node(
                        ui,
                        Some(&i.to_string()),
                        v,
                        p,
                        depth + 1,
                        format!("{path}[{i}]"),
                        more,
                    );
                }
                if items.len() > MAX_CHILDREN {
                    ui.label(RichText::new(more(items.len() - MAX_CHILDREN)).color(p.muted));
                }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arrows_clamp_and_tab_wraps() {
        let mut g = GridState::default();
        g.step(3, 2, -1, -1, false);
        assert_eq!(g.selected, Some((0, 0)), "clamped at the top left");
        g.step(3, 2, 0, 5, false);
        assert_eq!(g.selected, Some((0, 1)));
        g.step(3, 2, 0, 1, true);
        assert_eq!(g.selected, Some((1, 0)), "Tab past the end: next row");
        g.step(3, 2, 0, -1, true);
        assert_eq!(g.selected, Some((0, 1)), "Shift+Tab: previous row");
        g.step(3, 2, 9, 0, false);
        assert_eq!(g.selected, Some((2, 1)));
        g.step(3, 2, 0, 1, true);
        assert_eq!(g.selected, Some((2, 1)), "the last cell stays");
    }

    #[test]
    fn editing_starts_with_the_cell_value() {
        let t = sanain_tools::csv::parse("a,b\nc\n", ',');
        let mut g = GridState::default();
        g.start_edit(&t, 0, 1);
        assert_eq!(g.editing.as_ref().map(|e| e.text.as_str()), Some("b"));
        g.start_edit(&t, 1, 1);
        assert_eq!(
            g.editing.as_ref().map(|e| e.text.as_str()),
            Some(""),
            "a missing ragged cell edits as empty"
        );
        assert_eq!(g.selected, Some((1, 1)));
    }
}
