//! Structured views: an editable CSV grid and editable JSON, XML and YAML
//! trees. The source stays authoritative: edits come back as
//! [`GridAction`]s and [`TreeAction`]s that the caller applies to the text
//! as ordinary undoable edits.

use eframe::egui::{self, RichText};
use egui_extras::{Column, TableBuilder};
use text3_i18n::FluentValue;
use text3_tools::csv::Table;
use text3_tools::json;
use text3_tools::xml;
use text3_tools::yaml;

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
    /// The table's column resize handles on the last frame.
    resize_handles: Vec<egui::Id>,
    /// Tab past the last cell (`Some(true)`) or Shift+Tab before the
    /// first (`Some(false)`) this frame: the caller moves focus on.
    pub leave: Option<bool>,
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

/// Localizes accessibility text: a message id and its arguments.
pub type Tr<'a> = &'a dyn Fn(&str, &[(&str, FluentValue<'_>)]) -> String;

fn num(n: usize) -> FluentValue<'static> {
    FluentValue::from(n as i64)
}

/// Accessible name of a grid cell: its position counted as shown (body
/// rows from 1 under the header) and the column's header text.
fn cell_label(tr: Tr<'_>, row: usize, col: usize, header: Option<&str>) -> String {
    match header.map(str::trim).filter(|h| !h.is_empty()) {
        Some(name) => tr(
            "a11y-grid-cell",
            &[
                ("row", num(row)),
                ("column", num(col)),
                ("name", name.to_owned().into()),
            ],
        ),
        None => tr(
            "a11y-grid-cell-plain",
            &[("row", num(row)), ("column", num(col))],
        ),
    }
}

/// Whether Tab (`back`: Shift+Tab) at cell `at` leaves the grid instead of
/// moving to the next (previous) cell: past the last cell or before the
/// first.
fn tab_leaves(at: (usize, usize), rows: usize, cols: usize, back: bool) -> bool {
    if back {
        at == (0, 0)
    } else {
        at == (rows.saturating_sub(1), cols.saturating_sub(1))
    }
}

/// Whether the context menu key was pressed: Shift+F10 or the Menu key.
fn menu_key_pressed(ui: &egui::Ui) -> bool {
    ui.input_mut(|i| {
        i.consume_key(egui::Modifiers::SHIFT, egui::Key::F10)
            || i.consume_key(egui::Modifiers::NONE, egui::Key::Menu)
    })
}

/// A context menu on `resp` that the keyboard can also open (`by_key`):
/// then it opens next to the item instead of at the pointer and `add` gets
/// `true` to focus its first entry. When the menu closes and nothing else
/// took keyboard focus, focus returns to `return_to`.
fn item_menu(
    resp: &egui::Response,
    by_key: bool,
    return_to: egui::Id,
    add: impl FnOnce(&mut egui::Ui, &mut bool),
) {
    let ctx = &resp.ctx;
    let popup_id = egui::Popup::default_response_id(resp);
    let key_id = popup_id.with("opened-by-key");
    if by_key {
        egui::Popup::open_id(ctx, popup_id);
        ctx.data_mut(|d| d.insert_temp(key_id, true));
    }
    let was_open = egui::Popup::is_id_open(ctx, popup_id);
    let mut popup = egui::Popup::context_menu(resp);
    if was_open && egui::Popup::position_of_id(ctx, popup_id).is_none() {
        popup = popup.anchor(resp.rect);
    }
    popup.show(|ui| {
        let mut focus_first = ctx
            .data_mut(|d| d.remove_temp::<bool>(key_id))
            .unwrap_or(false);
        add(ui, &mut focus_first);
    });
    if was_open && !egui::Popup::is_id_open(ctx, popup_id) && ctx.memory(|m| m.focused()).is_none()
    {
        ctx.memory_mut(|m| m.request_focus(return_to));
    }
}

/// A menu entry; the first one takes keyboard focus when the menu was
/// opened from the keyboard.
fn menu_entry(ui: &mut egui::Ui, text: &str, focus_first: &mut bool) -> bool {
    let r = ui.button(text);
    if std::mem::take(focus_first) {
        r.request_focus();
    }
    r.clicked()
}

/// Keep a widget's own accessibility node out of the tree when an
/// enclosing node already says the same (a cell's text, a row's labels).
fn hide_from_accessibility(ui: &egui::Ui, id: egui::Id) {
    ui.ctx().accesskit_node_builder(id, |n| n.set_hidden());
}

/// Announce a message (an error under a field, an import result) when it
/// appears or changes.
pub fn announce(ui: &egui::Ui, id: egui::Id) {
    ui.ctx()
        .accesskit_node_builder(id, |n| n.set_live(egui::accesskit::Live::Polite));
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
/// focus) has keyboard focus, and Tab past the last cell (Shift+Tab before
/// the first) leaves the grid. Shift+F10 or the Menu key opens the
/// selected cell's menu. For screen readers the grid is a table whose
/// active cell follows the selection. Returns what to change in the text.
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
    tr: Tr<'_>,
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
    ui.ctx().accesskit_node_builder(id, |n| {
        n.set_role(egui::Role::Grid);
        n.set_label(tr("a11y-csv-grid", &[]));
        n.set_row_count(rows);
        n.set_column_count(cols);
    });
    if focus.has_focus() && state.selected.is_none() {
        state.selected = Some((0, 0));
        state.scroll = true;
    }
    let mut menu_key = false;
    let grid_focused = focus.has_focus() && state.editing.is_none();
    if grid_focused {
        // Tab moves between cells, except past the last cell (Shift+Tab:
        // before the first), where the caller moves focus out of the grid
        // (`state.leave`). The grid keeps Tab from egui, whose filter is
        // set a frame ahead and cannot know whether Shift comes with it.
        let at = state.selected.unwrap_or((0, 0));
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
        // The Tab that brought focus here is not a move inside the grid.
        let fresh = focus.gained_focus();
        if !fresh && ui.input_mut(|i| i.consume_key(egui::Modifiers::SHIFT, Key::Tab)) {
            if tab_leaves(at, rows, cols, true) {
                state.leave = Some(false);
            } else {
                state.step(rows, cols, 0, -1, true);
            }
        }
        if !fresh && pressed(Key::Tab) {
            if tab_leaves(at, rows, cols, false) {
                state.leave = Some(true);
            } else {
                state.step(rows, cols, 0, 1, true);
            }
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
            } else if menu_key_pressed(ui) {
                menu_key = true;
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
    let header_name = |c: usize| {
        header
            .then(|| t.rows.first().and_then(|row| row.get(c)))
            .flatten()
            .map(String::as_str)
    };
    let row_ops =
        |ui: &mut egui::Ui, actions: &mut Vec<GridAction>, r: usize, focus_first: &mut bool| {
            if menu_entry(ui, &labels.insert_row_below, focus_first) {
                actions.push(GridAction::InsertRowAfter(r));
                ui.memory_mut(|m| m.request_focus(id));
            }
            if menu_entry(ui, &labels.delete_row, focus_first) {
                actions.push(GridAction::DeleteRow(r));
                ui.memory_mut(|m| m.request_focus(id));
            }
        };
    // One cell; `c` is `None` for the row number.
    let mut cell =
        |ui: &mut egui::Ui, state: &mut GridState, r: usize, c: Option<usize>, strong: bool| {
            let Some(c) = c else {
                let shown = r + 1 - first_body;
                let number = ui.label(RichText::new(shown.to_string()).color(p.muted).monospace());
                hide_from_accessibility(ui, number.id);
                // Clicks only: the keys go to the grid's focus, not to
                // every cell in the Tab order.
                let resp = ui.interact(ui.max_rect(), id.with(("row", r)), egui::Sense::CLICK);
                resp.widget_info(|| {
                    egui::WidgetInfo::labeled(
                        egui::Role::RowHeader,
                        true,
                        tr("a11y-grid-row-header", &[("row", num(shown))]),
                    )
                });
                if resp.clicked() || resp.secondary_clicked() {
                    let c = state.selected.map_or(0, |(_, c)| c);
                    state.selected = Some((r, c));
                    ui.memory_mut(|m| m.request_focus(id));
                }
                if editable {
                    item_menu(&resp, false, id, |ui, first| {
                        row_ops(ui, &mut actions, r, first);
                    });
                }
                return;
            };
            let is_header = strong && r == 0 && header;
            let name = if is_header {
                tr("a11y-grid-column-header", &[("column", num(c + 1))])
            } else {
                cell_label(tr, r + 1 - first_body, c + 1, header_name(c))
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
                let resp = ui
                    .add(
                        egui::TextEdit::singleline(&mut ed.text)
                            .id(editor_id)
                            .lock_focus(true)
                            .desired_width(f32::INFINITY),
                    )
                    .accessible_name(tr("a11y-edit-field", &[("item", name.into())]));
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
                if grid_focused {
                    // The keyboard focus ring.
                    ui.painter().rect_stroke(
                        rect.expand(1.0),
                        2.0,
                        egui::Stroke::new(2.0, p.accent),
                        egui::StrokeKind::Inside,
                    );
                }
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
            let shown_label = ui.add(egui::Label::new(text).truncate());
            hide_from_accessibility(ui, shown_label.id);
            let resp = ui.interact(rect, id.with((r, c)), egui::Sense::CLICK);
            let resp = if v.is_empty() {
                resp
            } else {
                resp.on_hover_text(v)
            };
            let role = if is_header {
                egui::Role::ColumnHeader
            } else {
                egui::Role::Cell
            };
            resp.widget_info(|| egui::WidgetInfo::labeled(role, true, &name));
            ui.ctx().accesskit_node_builder(resp.id, |n| {
                n.set_value(v);
                n.set_row_index(r);
                n.set_column_index(c);
                n.set_selected(selected);
                // The tooltip repeats the value; the value says it already.
                n.clear_description();
            });
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
                item_menu(&resp, selected && menu_key, id, |ui, first| {
                    if menu_entry(ui, &labels.edit_cell, first) {
                        state.start_edit(t, r, c);
                    }
                    ui.separator();
                    row_ops(ui, &mut actions, r, first);
                });
            }
        };
    // The cells belong to the grid's node.
    // Tab onto a column resize handle (egui_extras makes them focusable)
    // goes on to the next widget; they do nothing from the keyboard.
    if ui
        .memory(|m| m.focused())
        .is_some_and(|f| state.resize_handles.contains(&f))
    {
        let back = ui.input(|i| i.modifiers.shift);
        ui.memory_mut(|m| {
            m.move_focus(if back {
                egui::FocusDirection::Previous
            } else {
                egui::FocusDirection::Next
            });
        });
        ui.ctx().request_repaint();
    }
    let mut resize_handles = Vec::new();
    ui.scope_builder(egui::UiBuilder::new().accessibility_parent(id), |ui| {
        egui::ScrollArea::horizontal().show(ui, |ui| {
            // The table's column resize handles, as egui_extras names them.
            let state_id = ui.scope_id().with(egui::IdSalt::new(id));
            resize_handles = (0..=cols)
                .map(|i| state_id.with("resize_column").with(i))
                .collect();
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
                        let corner = ui.label("");
                        hide_from_accessibility(ui, corner.id);
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
    });
    // The resize handles are for the pointer: keep them out of the
    // accessibility tree (Tab passes over them, above).
    for &h in &resize_handles {
        hide_from_accessibility(ui, h);
    }
    state.resize_handles = resize_handles;
    // Screen readers follow the selected cell while the grid has focus.
    if let Some((r, c)) = state.selected {
        let cell_id = id.with((r, c));
        if ui.ctx().has_accesskit_node(cell_id) {
            ui.ctx().accesskit_node_builder(id, |n| {
                n.set_active_descendant(cell_id.accesskit_id());
            });
        }
    }
    state.scroll = false;
    actions
}

/// Which part of a tree node the inline field edits: a JSON key or an XML
/// element or attribute name; a JSON scalar, an XML attribute value or
/// element text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Field {
    Key,
    Value,
}

/// Why the inline field's text was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldError {
    InvalidValue,
    DuplicateKey,
    InvalidName,
    DuplicateAttribute,
    NotWellFormed,
    /// YAML: the edit would change more than the node.
    NotPreserved,
    /// YAML: a key over 1024 characters.
    KeyTooLong,
}

impl From<json::EditError> for FieldError {
    fn from(e: json::EditError) -> Self {
        match e {
            json::EditError::DuplicateKey => Self::DuplicateKey,
            _ => Self::InvalidValue,
        }
    }
}

impl From<xml::EditError> for FieldError {
    fn from(e: xml::EditError) -> Self {
        match e {
            xml::EditError::InvalidName => Self::InvalidName,
            xml::EditError::DuplicateName => Self::DuplicateAttribute,
            _ => Self::NotWellFormed,
        }
    }
}

impl From<yaml::EditError> for FieldError {
    fn from(e: yaml::EditError) -> Self {
        match e {
            yaml::EditError::DuplicateKey => Self::DuplicateKey,
            yaml::EditError::InvalidValue => Self::KeyTooLong,
            _ => Self::NotPreserved,
        }
    }
}

/// Selection and inline editor of one tab's JSON, XML or YAML tree. Nodes are
/// named by their path of child positions, which survives re-parsing the
/// text.
#[derive(Debug, Default)]
pub struct TreeState {
    pub selected: Option<Vec<usize>>,
    /// The part of the selected node last clicked: what Enter or F2 edits.
    pub selected_field: Option<Field>,
    pub editing: Option<NodeEdit>,
    /// Open this field once the tree shows the node (after an add).
    pub pending: Option<(Vec<usize>, Field)>,
    /// Rows shown on the last frame, top to bottom, for the arrow keys.
    visible: Vec<Vec<usize>>,
    /// Bring the selected row into view on the next frame.
    scroll: bool,
}

#[derive(Debug)]
pub struct NodeEdit {
    pub path: Vec<usize>,
    pub field: Field,
    pub text: String,
    /// Why the last commit was refused.
    pub error: Option<FieldError>,
    /// Move keyboard focus into the field on the next frame.
    focus: bool,
    /// Reset the caret (a newly opened field).
    fresh: bool,
}

impl TreeState {
    fn open(&mut self, path: &[usize], field: Field, text: Option<String>) -> bool {
        let Some(text) = text else {
            return false;
        };
        self.selected = Some(path.to_vec());
        self.editing = Some(NodeEdit {
            path: path.to_vec(),
            field,
            text,
            error: None,
            focus: true,
            fresh: true,
        });
        true
    }

    /// Open the inline field on a node, if that part of it is editable:
    /// a scalar's value as JSON text, or a member's key decoded.
    pub fn start_edit(&mut self, doc: &json::Doc, path: &[usize], field: Field) -> bool {
        let Some(i) = doc.find(path) else {
            return false;
        };
        let text = match field {
            Field::Value => match doc.items[i].kind {
                json::Kind::String | json::Kind::Literal => doc.value_text(i).map(str::to_owned),
                _ => None,
            },
            Field::Key => doc.key(i),
        };
        self.open(path, field, text)
    }

    /// Edit what Enter or F2 means for a node: the part last clicked if
    /// editable, else a scalar's value, else its key.
    fn start_default_edit(&mut self, doc: &json::Doc, path: &[usize]) {
        if let Some(f) = self.selected_field
            && self.start_edit(doc, path, f)
        {
            return;
        }
        if !self.start_edit(doc, path, Field::Value) {
            self.start_edit(doc, path, Field::Key);
        }
    }

    /// Open the inline field on an XML node: an element's or attribute's
    /// name, an attribute's value or the text of a text-only element or a
    /// text node (decoded).
    pub fn start_xml_edit(&mut self, doc: &xml::Doc, path: &[usize], field: Field) -> bool {
        let Some(i) = doc.find(path) else {
            return false;
        };
        let text = match field {
            Field::Value => doc.value(i),
            Field::Key => doc.name(i).map(str::to_owned),
        };
        self.open(path, field, text)
    }

    fn start_default_xml_edit(&mut self, doc: &xml::Doc, path: &[usize]) {
        if let Some(f) = self.selected_field
            && self.start_xml_edit(doc, path, f)
        {
            return;
        }
        if !self.start_xml_edit(doc, path, Field::Value) {
            self.start_xml_edit(doc, path, Field::Key);
        }
    }

    /// Open the inline field on a YAML node: a scalar's value or an
    /// entry's key, decoded, when that part is editable.
    pub fn start_yaml_edit(&mut self, doc: &yaml::Doc, path: &[usize], field: Field) -> bool {
        let Some(i) = doc.find(path) else {
            return false;
        };
        let text = match field {
            Field::Value if doc.can_set_value(i) => doc.value(i),
            Field::Key if doc.can_rename(i) => doc.key(i),
            _ => None,
        };
        self.open(path, field, text)
    }

    fn start_default_yaml_edit(&mut self, doc: &yaml::Doc, path: &[usize]) {
        if let Some(f) = self.selected_field
            && self.start_yaml_edit(doc, path, f)
        {
            return;
        }
        if !self.start_yaml_edit(doc, path, Field::Value) {
            self.start_yaml_edit(doc, path, Field::Key);
        }
    }

    fn step(&mut self, d: isize) {
        let rows = &self.visible;
        if rows.is_empty() {
            return;
        }
        let at = self
            .selected
            .as_ref()
            .and_then(|s| rows.iter().position(|r| r == s));
        let next = match at {
            Some(k) => (k as isize + d).clamp(0, rows.len() as isize - 1) as usize,
            None => 0,
        };
        if at != Some(next) {
            self.selected_field = None;
        }
        self.selected = Some(rows[next].clone());
        self.scroll = true;
    }

    /// Select a shown row (the parent, on Left).
    fn select(&mut self, path: &[usize]) {
        if self.visible.iter().any(|r| r == path) {
            self.selected = Some(path.to_vec());
            self.selected_field = None;
            self.scroll = true;
        }
    }
}

/// A change made in the JSON or XML tree, for the caller to apply to the
/// text as one undoable edit through [`json::Doc`] or [`xml::Doc`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TreeAction {
    SetValue {
        path: Vec<usize>,
        text: String,
    },
    /// Rename a JSON key, or an XML element or attribute.
    RenameKey {
        path: Vec<usize>,
        name: String,
    },
    Delete(Vec<usize>),
    AddAfter(Vec<usize>),
    /// Add a JSON member or element, or an XML child element.
    AddInside(Vec<usize>),
    /// Add an XML attribute to the element (or the attribute's element).
    AddAttribute(Vec<usize>),
}

/// Localized labels of the JSON tree's menu and errors.
pub struct TreeLabels {
    pub edit_value: String,
    pub rename_key: String,
    pub add_after: String,
    pub add_inside: String,
    pub delete: String,
    pub invalid_value: String,
    pub duplicate_key: String,
}

/// Localized labels of the XML tree's menu and errors.
pub struct XmlLabels {
    pub edit_value: String,
    pub edit_text: String,
    pub rename: String,
    pub add_attribute: String,
    pub add_child: String,
    pub add_after: String,
    pub delete: String,
    pub invalid_name: String,
    pub duplicate_attribute: String,
    pub not_well_formed: String,
}

/// Localized labels of the YAML tree's menu, errors and read-only reasons.
pub struct YamlLabels {
    pub edit_value: String,
    pub rename_key: String,
    pub add_after: String,
    pub add_inside: String,
    pub delete: String,
    pub duplicate_key: String,
    pub key_too_long: String,
    pub not_preserved: String,
    /// Why a node is read-only, by [`yaml::Lock`].
    pub locks: Vec<(yaml::Lock, String)>,
}

impl YamlLabels {
    fn lock(&self, l: yaml::Lock) -> String {
        self.locks
            .iter()
            .find(|(k, _)| *k == l)
            .map(|(_, t)| t.clone())
            .unwrap_or_default()
    }
}

/// What the tree's keys asked for on the selected node.
enum KeyAct {
    Edit(Vec<usize>),
    Delete(Vec<usize>),
    /// Open its context menu.
    Menu,
}

/// Accessibility of one frame of a tree: the rows are tree items named by
/// what they show, and the tree's active item follows the selection.
struct TreeAx<'a> {
    tr: Tr<'a>,
    /// The tree itself has keyboard focus (not the inline field).
    focused: bool,
    /// Open the selected row's context menu.
    menu_key: bool,
    /// Bring the selected row into view.
    scroll: bool,
    /// Level (from 1) and open state of the row being drawn.
    level: usize,
    expanded: Option<bool>,
    /// Node of the selected row.
    selected_node: Option<egui::Id>,
    /// Accessible name of the last row drawn.
    last_name: String,
}

impl<'a> TreeAx<'a> {
    fn new(tr: Tr<'a>, ui: &egui::Ui, id: egui::Id, state: &mut TreeState, menu: bool) -> Self {
        let focused = ui.memory(|m| m.has_focus(id));
        Self {
            tr,
            focused: focused && state.editing.is_none(),
            menu_key: menu,
            scroll: std::mem::take(&mut state.scroll),
            level: 1,
            expanded: None,
            selected_node: None,
            last_name: String::new(),
        }
    }

    /// The expand/collapse button egui draws before a container's row
    /// (its id is the open state's): named by the row, with its state.
    fn toggle(&self, ctx: &egui::Context, open_id: egui::Id) {
        let expanded = self.expanded;
        let name = self.last_name.clone();
        ctx.accesskit_node_builder(open_id, |n| {
            n.set_role(egui::Role::DisclosureTriangle);
            n.set_label(name);
            if let Some(open) = expanded {
                n.set_expanded(open);
            }
        });
    }

    /// Before a row's widgets: its node, so they become its children.
    fn begin_row(ui: &egui::Ui) {
        ui.ctx()
            .accesskit_node_builder(ui.unique_id(), |n| n.set_role(egui::Role::TreeItem));
    }

    /// Whether this row's menu should open from the keyboard (once).
    fn take_menu(&mut self, selected: bool) -> bool {
        selected && std::mem::take(&mut self.menu_key)
    }

    /// After a row's widgets: name, value, level and state; the focus ring
    /// and scrolling for the selected row.
    fn finish_row(&mut self, ui: &egui::Ui, p: &Palette, selected: bool, name: &str, value: &str) {
        let node = ui.unique_id();
        let rect = ui.min_rect();
        let level = num(self.level);
        let description = match self.expanded {
            Some(true) => (self.tr)("a11y-tree-item-expanded", &[("level", level)]),
            Some(false) => (self.tr)("a11y-tree-item-collapsed", &[("level", level)]),
            None => (self.tr)("a11y-tree-item", &[("level", level)]),
        };
        let (level, expanded) = (self.level, self.expanded);
        self.last_name = if name.is_empty() { value } else { name }.to_owned();
        ui.ctx().accesskit_node_builder(node, |n| {
            n.set_bounds(egui::accesskit::Rect {
                x0: rect.min.x.into(),
                y0: rect.min.y.into(),
                x1: rect.max.x.into(),
                y1: rect.max.y.into(),
            });
            if name.is_empty() {
                n.set_label(value);
            } else {
                n.set_label(name);
                n.set_value(value);
            }
            n.set_level(level);
            if let Some(open) = expanded {
                n.set_expanded(open);
            }
            n.set_selected(selected);
            n.set_description(description);
        });
        if selected {
            self.selected_node = Some(node);
            if self.focused {
                ui.painter().rect_stroke(
                    rect.expand(1.0),
                    2.0,
                    egui::Stroke::new(1.5, p.accent),
                    egui::StrokeKind::Inside,
                );
            }
            if self.scroll {
                ui.scroll_to_rect(rect, None);
            }
        }
    }
}

/// The scrolling body of a tree, its rows under the tree's node `id`; then
/// the tree's active item is the selected row.
fn tree_body(ui: &mut egui::Ui, id: egui::Id, add: impl FnOnce(&mut egui::Ui) -> Option<egui::Id>) {
    let selected = ui
        .scope_builder(egui::UiBuilder::new().accessibility_parent(id), |ui| {
            egui::ScrollArea::both()
                .auto_shrink(false)
                .show(ui, add)
                .inner
        })
        .inner;
    if let Some(node) = selected
        && ui.ctx().has_accesskit_node(node)
    {
        ui.ctx()
            .accesskit_node_builder(id, |n| n.set_active_descendant(node.accesskit_id()));
    }
}

/// Shared start of a tree frame: drop the field when it cannot apply,
/// expand the parents of a pending node, then the keys while `id` has
/// focus: Up and Down move the selection, Right expands (or goes to the
/// first child), Left collapses (or goes to the parent), Enter or F2 edits,
/// Delete deletes and Shift+F10 or the Menu key opens the menu (when
/// `editable`). `name` is the tree's accessible name.
fn tree_frame(
    ui: &mut egui::Ui,
    id: egui::Id,
    name: String,
    editable: bool,
    state: &mut TreeState,
    exists: &dyn Fn(&[usize]) -> bool,
    open: &dyn Fn(&mut TreeState, &[usize], Field),
) -> Option<KeyAct> {
    if !editable {
        state.editing = None;
        state.pending = None;
    }
    if state.editing.as_ref().is_some_and(|e| !exists(&e.path)) {
        state.editing = None;
    }
    if let Some((path, field)) = state.pending.take() {
        // Open the containers above a new node so its field shows.
        for k in 0..path.len() {
            let open_id = id.with(("node", &path[..k]));
            let mut c = egui::collapsing_header::CollapsingState::load_with_default_open(
                ui.ctx(),
                open_id,
                k < 2,
            );
            c.set_open(true);
            c.store(ui.ctx());
        }
        open(state, &path, field);
    }
    let mut act = None;
    let focus = ui.interact(ui.max_rect(), id, egui::Sense::focusable_noninteractive());
    ui.ctx().accesskit_node_builder(id, |n| {
        n.set_role(egui::Role::Tree);
        n.set_label(name);
    });
    if focus.has_focus() && state.selected.is_none() {
        state.step(0);
    }
    if focus.has_focus() && state.editing.is_none() {
        ui.memory_mut(|m| {
            m.set_focus_lock_filter(
                id,
                egui::EventFilter {
                    tab: false,
                    horizontal_arrows: true,
                    vertical_arrows: true,
                    escape: false,
                },
            );
        });
        use egui::Key;
        let pressed = |k| ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, k));
        if pressed(Key::ArrowUp) {
            state.step(-1);
        }
        if pressed(Key::ArrowDown) {
            state.step(1);
        }
        let (right, left) = (pressed(Key::ArrowRight), pressed(Key::ArrowLeft));
        if (right || left)
            && let Some(sel) = state.selected.clone()
        {
            let open_id = id.with(("node", sel.as_slice()));
            let header = egui::collapsing_header::CollapsingState::load(ui.ctx(), open_id);
            match header {
                Some(mut h) if right != h.is_open() => {
                    h.set_open(right);
                    h.store(ui.ctx());
                }
                Some(_) if right => state.step(1),
                _ if left && !sel.is_empty() => state.select(&sel[..sel.len() - 1]),
                _ => {}
            }
        }
        if editable && let Some(sel) = state.selected.clone() {
            if pressed(Key::Enter) || pressed(Key::F2) {
                act = Some(KeyAct::Edit(sel));
            } else if pressed(Key::Delete) && !sel.is_empty() {
                act = Some(KeyAct::Delete(sel));
            } else if menu_key_pressed(ui) {
                act = Some(KeyAct::Menu);
            }
        }
    }
    act
}

/// Selection and double-click editing of a label: returns whether a
/// double-click asked to edit `field`.
fn select_on_click(
    ui: &egui::Ui,
    id: egui::Id,
    state: &mut TreeState,
    r: &egui::Response,
    path: &[usize],
    field: Field,
) -> bool {
    if r.clicked() || r.secondary_clicked() {
        state.selected = Some(path.to_vec());
        state.selected_field = Some(field);
        ui.memory_mut(|m| m.request_focus(id));
    }
    r.double_clicked()
}

/// The inline field of the node being edited. `check` tells whether the
/// text changes anything (or why it is refused); a commit that changes
/// something comes back as (path, field, text).
fn edit_field(
    ui: &mut egui::Ui,
    id: egui::Id,
    p: &Palette,
    state: &mut TreeState,
    name: String,
    message: &dyn Fn(FieldError) -> String,
    check: &dyn Fn(Field, &str) -> Result<bool, FieldError>,
) -> Option<(Vec<usize>, Field, String)> {
    let ed = state.editing.as_mut()?;
    let editor_id = id.with("node-editor");
    if ed.fresh {
        egui::text_edit::TextEditState::default().store(ui.ctx(), editor_id);
        ed.fresh = false;
    }
    let width = (ed.text.chars().count() as f32 + 4.0) * 9.0;
    let resp = ui
        .add(
            egui::TextEdit::singleline(&mut ed.text)
                .id(editor_id)
                .font(egui::TextStyle::Monospace)
                .desired_width(width.clamp(120.0, 600.0)),
        )
        .accessible_name(name);
    if ed.focus {
        resp.request_focus();
        ed.focus = false;
    }
    if let Some(e) = ed.error {
        let r = ui.colored_label(p.error, message(e));
        announce(ui, r.id);
    }
    if !resp.lost_focus() {
        return None;
    }
    let esc = ui.input(|i| i.key_pressed(egui::Key::Escape));
    let enter = ui.input(|i| i.key_pressed(egui::Key::Enter));
    if esc {
        state.editing = None;
        ui.memory_mut(|m| m.request_focus(id));
        return None;
    }
    let mut out = None;
    match check(ed.field, &ed.text) {
        Ok(changed) => {
            if let (true, Some(ed)) = (changed, state.editing.take()) {
                out = Some((ed.path, ed.field, ed.text));
            } else {
                state.editing = None;
            }
            if enter {
                ui.memory_mut(|m| m.request_focus(id));
            }
        }
        Err(e) => {
            // Keep the field and say why; Enter keeps typing in it.
            ed.error = Some(e);
            ed.focus = enter;
        }
    }
    out
}

/// A clickable monospace label, highlighted when its row is selected.
fn label(
    ui: &mut egui::Ui,
    text: String,
    color: egui::Color32,
    selected: bool,
    p: &Palette,
) -> egui::Response {
    let mut t = RichText::new(text).monospace().color(color);
    if selected {
        t = t.background_color(p.accent.gamma_multiply(0.25));
    }
    // Clicks only: the tree's focus takes the keys, so its labels stay out
    // of the Tab order.
    ui.add(egui::Label::new(t).sense(egui::Sense::CLICK))
}

/// The editable JSON tree. With `editable`, double-clicking a key or a
/// scalar value (or Enter/F2 on the selected node) edits it inline: Enter
/// commits, Escape cancels, and a refused value shows why without changing
/// anything. Up and Down move the selection while `id` has focus; Delete
/// removes the selected node; right-click offers every operation.
#[allow(clippy::too_many_arguments)]
pub fn json(
    ui: &mut egui::Ui,
    id: egui::Id,
    doc: &json::Doc,
    p: &Palette,
    editable: bool,
    state: &mut TreeState,
    labels: &TreeLabels,
    more: &dyn Fn(usize) -> String,
    tr: Tr<'_>,
) -> Vec<TreeAction> {
    let mut actions = Vec::new();
    let exists = |path: &[usize]| doc.find(path).is_some();
    let open = |s: &mut TreeState, path: &[usize], f| {
        s.start_edit(doc, path, f);
    };
    let name = tr("a11y-json-tree", &[]);
    let act = tree_frame(ui, id, name, editable, state, &exists, &open);
    let mut menu = false;
    match act {
        Some(KeyAct::Edit(sel)) => state.start_default_edit(doc, &sel),
        Some(KeyAct::Delete(sel)) => actions.push(TreeAction::Delete(sel)),
        Some(KeyAct::Menu) => menu = true,
        None => {}
    }
    let mut visible = Vec::new();
    let ax = TreeAx::new(tr, ui, id, state, menu);
    tree_body(ui, id, |ui| {
        let mut cx = TreeCx {
            doc,
            p,
            id,
            editable,
            state,
            labels,
            more,
            actions: &mut actions,
            visible: &mut visible,
            ax,
        };
        cx.node(ui, 0, &mut Vec::new(), 0, None);
        cx.ax.selected_node
    });
    state.visible = visible;
    actions
}

struct TreeCx<'a> {
    doc: &'a json::Doc,
    p: &'a Palette,
    id: egui::Id,
    editable: bool,
    state: &'a mut TreeState,
    labels: &'a TreeLabels,
    more: &'a dyn Fn(usize) -> String,
    actions: &'a mut Vec<TreeAction>,
    visible: &'a mut Vec<Vec<usize>>,
    ax: TreeAx<'a>,
}

impl TreeCx<'_> {
    fn node(
        &mut self,
        ui: &mut egui::Ui,
        i: usize,
        path: &mut Vec<usize>,
        depth: usize,
        index: Option<usize>,
    ) {
        let it = &self.doc.items[i];
        self.visible.push(path.clone());
        self.ax.level = depth + 1;
        self.ax.expanded = None;
        let container = matches!(it.kind, json::Kind::Object | json::Kind::Array);
        if !container {
            ui.horizontal(|ui| self.row(ui, i, path, index));
            return;
        }
        if depth >= MAX_TREE_DEPTH {
            ui.horizontal(|ui| self.row(ui, i, path, index));
            return;
        }
        let open_id = self.id.with(("node", path.as_slice()));
        let state = egui::collapsing_header::CollapsingState::load_with_default_open(
            ui.ctx(),
            open_id,
            depth < 2,
        );
        self.ax.expanded = Some(state.is_open());
        let ctx = ui.ctx().clone();
        let header = state.show_header(ui, |ui| self.row(ui, i, path, index));
        self.ax.toggle(&ctx, open_id);
        header.body(|ui| {
            let children = &self.doc.items[i].children;
            let is_array = self.doc.items[i].kind == json::Kind::Array;
            for (k, &c) in children.iter().take(MAX_CHILDREN).enumerate() {
                path.push(k);
                self.node(ui, c, path, depth + 1, is_array.then_some(k));
                path.pop();
            }
            if children.len() > MAX_CHILDREN {
                ui.label(
                    RichText::new((self.more)(children.len() - MAX_CHILDREN)).color(self.p.muted),
                );
            }
        });
    }

    /// One line of the tree: key (or array index), then value or summary.
    fn row(&mut self, ui: &mut egui::Ui, i: usize, path: &[usize], index: Option<usize>) {
        let (doc, p) = (self.doc, self.p);
        let it = &doc.items[i];
        let selected = self.state.selected.as_deref() == Some(path);
        TreeAx::begin_row(ui);
        let tr = self.ax.tr;
        let name = doc
            .key(i)
            .or_else(|| index.map(|n| n.to_string()))
            .unwrap_or_default();
        let count = num(it.children.len());
        let value = match it.kind {
            json::Kind::Object => tr("a11y-tree-object", &[("count", count)]),
            json::Kind::Array => tr("a11y-tree-list", &[("count", count)]),
            _ => doc.value_text(i).unwrap_or_default().to_owned(),
        };
        let editing = |f: Field| {
            self.state
                .editing
                .as_ref()
                .is_some_and(|e| e.field == f && e.path == path)
        };
        let editing_key = editing(Field::Key);
        let editing_value = editing(Field::Value);
        // The key part.
        if editing_key {
            self.field(ui, i, &name);
            ui.label(RichText::new(":").monospace().color(p.muted));
        } else if let Some(k) = doc.key_text(i) {
            let r = label(ui, format!("{k}:"), p.muted, selected, p);
            self.interact(ui, &r, i, path, Field::Key);
        } else if let Some(n) = index {
            let r = label(ui, format!("{n}:"), p.muted, selected, p);
            self.interact(ui, &r, i, path, Field::Value);
        }
        // The value part.
        let summary = match it.kind {
            json::Kind::Object => Some(format!("{{{}}}", it.children.len())),
            json::Kind::Array => Some(format!("[{}]", it.children.len())),
            _ => None,
        };
        if editing_value {
            self.field(ui, i, &name);
        } else {
            let text = summary.unwrap_or_else(|| doc.value_text(i).unwrap_or_default().to_owned());
            let color = if it.children.is_empty()
                && matches!(it.kind, json::Kind::Object | json::Kind::Array)
            {
                p.muted
            } else {
                p.text
            };
            let r = label(ui, text, color, selected, p);
            self.interact(ui, &r, i, path, Field::Value);
        }
        self.ax.finish_row(ui, p, selected, &name, &value);
    }

    /// Selection, double-click editing and the context menu of a label.
    fn interact(
        &mut self,
        ui: &egui::Ui,
        r: &egui::Response,
        i: usize,
        path: &[usize],
        field: Field,
    ) {
        let double = select_on_click(ui, self.id, self.state, r, path, field);
        if self.editable && double && !self.state.start_edit(self.doc, path, field) {
            self.state.start_default_edit(self.doc, path);
        }
        if !self.editable {
            return;
        }
        let it = &self.doc.items[i];
        let scalar = matches!(it.kind, json::Kind::String | json::Kind::Literal);
        let selected = self.state.selected.as_deref() == Some(path);
        let by_key = self.ax.take_menu(selected);
        let id = self.id;
        item_menu(r, by_key, id, |ui, first| {
            let back = |ui: &egui::Ui| ui.memory_mut(|m| m.request_focus(id));
            if scalar && menu_entry(ui, &self.labels.edit_value, first) {
                self.state.start_edit(self.doc, path, Field::Value);
            }
            if it.key_span.is_some() && menu_entry(ui, &self.labels.rename_key, first) {
                self.state.start_edit(self.doc, path, Field::Key);
            }
            if it.parent.is_some() && menu_entry(ui, &self.labels.add_after, first) {
                self.actions.push(TreeAction::AddAfter(path.to_vec()));
                back(ui);
            }
            if !scalar && menu_entry(ui, &self.labels.add_inside, first) {
                self.actions.push(TreeAction::AddInside(path.to_vec()));
                back(ui);
            }
            if it.parent.is_some() {
                ui.separator();
                if menu_entry(ui, &self.labels.delete, first) {
                    self.actions.push(TreeAction::Delete(path.to_vec()));
                    back(ui);
                }
            }
        });
    }

    /// The inline field of the node being edited; `name` names its row.
    fn field(&mut self, ui: &mut egui::Ui, i: usize, name: &str) {
        let (doc, labels) = (self.doc, self.labels);
        let name = (self.ax.tr)("a11y-edit-field", &[("item", name.to_owned().into())]);
        let message = |e: FieldError| match e {
            FieldError::DuplicateKey => labels.duplicate_key.clone(),
            _ => labels.invalid_value.clone(),
        };
        let check = |f: Field, text: &str| {
            let r = match f {
                Field::Value => doc.check_value(i, text),
                Field::Key => doc.check_key(i, text),
            };
            r.map(|c| c.is_some()).map_err(FieldError::from)
        };
        if let Some((path, field, text)) =
            edit_field(ui, self.id, self.p, self.state, name, &message, &check)
        {
            self.actions.push(match field {
                Field::Value => TreeAction::SetValue { path, text },
                Field::Key => TreeAction::RenameKey { path, name: text },
            });
        }
    }
}

/// Whether the tree offers Delete for an XML node: attributes and elements
/// other than the root.
fn xml_deletable(doc: &xml::Doc, i: usize) -> bool {
    let it = &doc.items[i];
    match it.kind {
        xml::Kind::Attribute => true,
        xml::Kind::Element => it.parent.is_some_and(|p| p != 0),
        _ => false,
    }
}

/// The editable XML tree: elements with their attributes (`@name`), text
/// (of text-only elements, or `#text` nodes in mixed content) and child
/// elements. Interaction is as in [`json`]: double-click a name, value or
/// text (or Enter/F2) to edit it inline, Up/Down/Delete keys, and a
/// context menu with every operation. Comments, processing instructions
/// and the declaration are not shown and never change.
#[allow(clippy::too_many_arguments)]
pub fn xml(
    ui: &mut egui::Ui,
    id: egui::Id,
    doc: &xml::Doc,
    p: &Palette,
    editable: bool,
    state: &mut TreeState,
    labels: &XmlLabels,
    more: &dyn Fn(usize) -> String,
    tr: Tr<'_>,
) -> Vec<TreeAction> {
    let mut actions = Vec::new();
    let exists = |path: &[usize]| doc.find(path).is_some();
    let open = |s: &mut TreeState, path: &[usize], f| {
        s.start_xml_edit(doc, path, f);
    };
    let name = tr("a11y-xml-tree", &[]);
    let act = tree_frame(ui, id, name, editable, state, &exists, &open);
    let mut menu = false;
    match act {
        Some(KeyAct::Edit(sel)) => state.start_default_xml_edit(doc, &sel),
        Some(KeyAct::Delete(sel)) if doc.find(&sel).is_some_and(|i| xml_deletable(doc, i)) => {
            actions.push(TreeAction::Delete(sel));
        }
        Some(KeyAct::Menu) => menu = true,
        _ => {}
    }
    let mut visible = Vec::new();
    let ax = TreeAx::new(tr, ui, id, state, menu);
    tree_body(ui, id, |ui| {
        let mut cx = XmlCx {
            doc,
            p,
            id,
            editable,
            state,
            labels,
            more,
            actions: &mut actions,
            visible: &mut visible,
            ax,
        };
        let mut path = Vec::new();
        for (k, &c) in doc.items[0].children.iter().enumerate() {
            path.push(k);
            cx.node(ui, c, &mut path, 0);
            path.pop();
        }
        cx.ax.selected_node
    });
    state.visible = visible;
    actions
}

struct XmlCx<'a> {
    doc: &'a xml::Doc,
    p: &'a Palette,
    id: egui::Id,
    editable: bool,
    state: &'a mut TreeState,
    labels: &'a XmlLabels,
    more: &'a dyn Fn(usize) -> String,
    actions: &'a mut Vec<TreeAction>,
    visible: &'a mut Vec<Vec<usize>>,
    ax: TreeAx<'a>,
}

impl XmlCx<'_> {
    fn node(&mut self, ui: &mut egui::Ui, i: usize, path: &mut Vec<usize>, depth: usize) {
        self.visible.push(path.clone());
        self.ax.level = depth + 1;
        self.ax.expanded = None;
        if self.doc.items[i].children.is_empty() || depth >= MAX_TREE_DEPTH {
            ui.horizontal(|ui| self.row(ui, i, path));
            return;
        }
        let open_id = self.id.with(("node", path.as_slice()));
        let state = egui::collapsing_header::CollapsingState::load_with_default_open(
            ui.ctx(),
            open_id,
            depth < 2,
        );
        self.ax.expanded = Some(state.is_open());
        let ctx = ui.ctx().clone();
        let header = state.show_header(ui, |ui| self.row(ui, i, path));
        self.ax.toggle(&ctx, open_id);
        header.body(|ui| {
            let children = &self.doc.items[i].children;
            for (k, &c) in children.iter().take(MAX_CHILDREN).enumerate() {
                path.push(k);
                self.node(ui, c, path, depth + 1);
                path.pop();
            }
            if children.len() > MAX_CHILDREN {
                ui.label(
                    RichText::new((self.more)(children.len() - MAX_CHILDREN)).color(self.p.muted),
                );
            }
        });
    }

    /// One line: `<name>` and its text, `@name = "value"`, or `#text`.
    fn row(&mut self, ui: &mut egui::Ui, i: usize, path: &[usize]) {
        let (doc, p) = (self.doc, self.p);
        let it = &doc.items[i];
        let selected = self.state.selected.as_deref() == Some(path);
        let editing = |f: Field| {
            self.state
                .editing
                .as_ref()
                .is_some_and(|e| e.field == f && e.path == path)
        };
        let (editing_key, editing_value) = (editing(Field::Key), editing(Field::Value));
        let name = doc.name(i).unwrap_or_default();
        TreeAx::begin_row(ui);
        let tr = self.ax.tr;
        let has_value = it.kind != xml::Kind::Element || it.text_only;
        let ax_name = match it.kind {
            xml::Kind::Element => tr("a11y-tree-element", &[("name", name.to_owned().into())]),
            xml::Kind::Attribute => tr("a11y-tree-attribute", &[("name", name.to_owned().into())]),
            _ => tr("a11y-tree-text", &[]),
        };
        let ax_value = if has_value {
            doc.value(i).unwrap_or_default()
        } else {
            String::new()
        };
        self.row_parts(
            ui,
            i,
            path,
            selected,
            (editing_key, editing_value),
            &ax_name,
        );
        self.ax.finish_row(ui, p, selected, &ax_name, &ax_value);
    }

    /// The widgets of a row; `ax_name` names it for the inline field.
    fn row_parts(
        &mut self,
        ui: &mut egui::Ui,
        i: usize,
        path: &[usize],
        selected: bool,
        (editing_key, editing_value): (bool, bool),
        ax_name: &str,
    ) {
        let (doc, p) = (self.doc, self.p);
        let it = &doc.items[i];
        let name = doc.name(i).unwrap_or_default();
        match it.kind {
            xml::Kind::Element => {
                if editing_key {
                    self.field(ui, i, ax_name);
                } else {
                    let r = label(ui, format!("<{name}>"), p.accent, selected, p);
                    self.interact(ui, &r, i, path, Field::Key);
                }
            }
            xml::Kind::Attribute => {
                if editing_key {
                    self.field(ui, i, ax_name);
                } else {
                    let r = label(ui, format!("@{name}"), p.muted, selected, p);
                    self.interact(ui, &r, i, path, Field::Key);
                }
                ui.label(RichText::new("=").monospace().color(p.muted));
            }
            _ => {
                let r = label(ui, "#text".into(), p.muted, selected, p);
                self.interact(ui, &r, i, path, Field::Value);
            }
        }
        let has_value = it.kind != xml::Kind::Element || it.text_only;
        if !has_value {
            return;
        }
        if editing_value {
            self.field(ui, i, ax_name);
            return;
        }
        let value = doc.value(i).unwrap_or_default();
        let text = if it.kind == xml::Kind::Attribute {
            format!("\"{value}\"")
        } else {
            value
        };
        let r = label(ui, text, p.text, selected, p);
        self.interact(ui, &r, i, path, Field::Value);
    }

    fn interact(
        &mut self,
        ui: &egui::Ui,
        r: &egui::Response,
        i: usize,
        path: &[usize],
        field: Field,
    ) {
        let double = select_on_click(ui, self.id, self.state, r, path, field);
        if self.editable && double && !self.state.start_xml_edit(self.doc, path, field) {
            self.state.start_default_xml_edit(self.doc, path);
        }
        if !self.editable {
            return;
        }
        let (doc, labels) = (self.doc, self.labels);
        let it = &doc.items[i];
        let element = it.kind == xml::Kind::Element;
        let selected = self.state.selected.as_deref() == Some(path);
        let by_key = self.ax.take_menu(selected);
        let id = self.id;
        item_menu(r, by_key, id, |ui, first| {
            let back = |ui: &egui::Ui| ui.memory_mut(|m| m.request_focus(id));
            let edit = match it.kind {
                xml::Kind::Attribute => Some(&labels.edit_value),
                xml::Kind::Text => Some(&labels.edit_text),
                _ => it.text_only.then_some(&labels.edit_text),
            };
            if let Some(edit) = edit
                && menu_entry(ui, edit, first)
            {
                self.state.start_xml_edit(doc, path, Field::Value);
            }
            if it.kind != xml::Kind::Text && menu_entry(ui, &labels.rename, first) {
                self.state.start_xml_edit(doc, path, Field::Key);
            }
            if it.kind != xml::Kind::Text && menu_entry(ui, &labels.add_attribute, first) {
                self.actions.push(TreeAction::AddAttribute(path.to_vec()));
                back(ui);
            }
            if element && menu_entry(ui, &labels.add_child, first) {
                self.actions.push(TreeAction::AddInside(path.to_vec()));
                back(ui);
            }
            if element && xml_deletable(doc, i) && menu_entry(ui, &labels.add_after, first) {
                self.actions.push(TreeAction::AddAfter(path.to_vec()));
                back(ui);
            }
            if xml_deletable(doc, i) {
                ui.separator();
                if menu_entry(ui, &labels.delete, first) {
                    self.actions.push(TreeAction::Delete(path.to_vec()));
                    back(ui);
                }
            }
        });
    }

    fn field(&mut self, ui: &mut egui::Ui, i: usize, name: &str) {
        let (doc, labels) = (self.doc, self.labels);
        let name = (self.ax.tr)("a11y-edit-field", &[("item", name.to_owned().into())]);
        let message = |e: FieldError| match e {
            FieldError::InvalidName => labels.invalid_name.clone(),
            FieldError::DuplicateAttribute => labels.duplicate_attribute.clone(),
            _ => labels.not_well_formed.clone(),
        };
        let check = |f: Field, text: &str| {
            let r = match f {
                Field::Value => doc.set_value(i, text),
                Field::Key => doc.rename(i, text),
            };
            r.map(|c| c.is_some()).map_err(FieldError::from)
        };
        if let Some((path, field, text)) =
            edit_field(ui, self.id, self.p, self.state, name, &message, &check)
        {
            self.actions.push(match field {
                Field::Value => TreeAction::SetValue { path, text },
                Field::Key => TreeAction::RenameKey { path, name: text },
            });
        }
    }
}

/// The editable YAML tree: mappings (`{n}`), sequences (`[n]`) and scalars
/// as written. Interaction is as in [`json`]. Read-only parts (flow
/// collections, anchors, tags, aliases, block scalars, complex keys, later
/// documents) show why on hover, and their menu entries are disabled with
/// the same reason. Comments are not shown and never change.
#[allow(clippy::too_many_arguments)]
pub fn yaml(
    ui: &mut egui::Ui,
    id: egui::Id,
    doc: &yaml::Doc,
    p: &Palette,
    editable: bool,
    state: &mut TreeState,
    labels: &YamlLabels,
    more: &dyn Fn(usize) -> String,
    tr: Tr<'_>,
) -> Vec<TreeAction> {
    let mut actions = Vec::new();
    let exists = |path: &[usize]| doc.find(path).is_some();
    let open = |s: &mut TreeState, path: &[usize], f| {
        s.start_yaml_edit(doc, path, f);
    };
    let name = tr("a11y-yaml-tree", &[]);
    let act = tree_frame(ui, id, name, editable, state, &exists, &open);
    let mut menu = false;
    match act {
        Some(KeyAct::Edit(sel)) => state.start_default_yaml_edit(doc, &sel),
        Some(KeyAct::Delete(sel)) if doc.find(&sel).is_some_and(|i| doc.can_delete(i)) => {
            actions.push(TreeAction::Delete(sel));
        }
        Some(KeyAct::Menu) => menu = true,
        _ => {}
    }
    let mut visible = Vec::new();
    let ax = TreeAx::new(tr, ui, id, state, menu);
    tree_body(ui, id, |ui| {
        let mut cx = YamlCx {
            doc,
            p,
            id,
            editable,
            state,
            labels,
            more,
            actions: &mut actions,
            visible: &mut visible,
            ax,
        };
        let docs = &doc.items[0].children;
        let many = docs.len() > 1;
        let mut path = Vec::new();
        for (k, &c) in docs.iter().enumerate() {
            if many {
                ui.label(
                    RichText::new(format!("--- {}", k + 1))
                        .monospace()
                        .color(p.muted),
                );
            }
            path.push(k);
            cx.node(ui, c, &mut path, 0);
            path.pop();
        }
        cx.ax.selected_node
    });
    state.visible = visible;
    actions
}

struct YamlCx<'a> {
    doc: &'a yaml::Doc,
    p: &'a Palette,
    id: egui::Id,
    editable: bool,
    state: &'a mut TreeState,
    labels: &'a YamlLabels,
    more: &'a dyn Fn(usize) -> String,
    actions: &'a mut Vec<TreeAction>,
    visible: &'a mut Vec<Vec<usize>>,
    ax: TreeAx<'a>,
}

/// Text shown for a scalar, alias or flow collection: as written, on one
/// line.
fn one_line(s: &str) -> String {
    const MAX: usize = 200;
    let mut out: String = s
        .chars()
        .take(MAX)
        .map(|c| if c == '\n' || c == '\r' { '⏎' } else { c })
        .collect();
    if s.chars().count() > MAX {
        out.push('…');
    }
    out
}

impl YamlCx<'_> {
    fn node(&mut self, ui: &mut egui::Ui, i: usize, path: &mut Vec<usize>, depth: usize) {
        let it = &self.doc.items[i];
        self.visible.push(path.clone());
        self.ax.level = depth + 1;
        self.ax.expanded = None;
        // Flow collections are shown as written, on their row.
        let open = matches!(it.kind, yaml::Kind::Mapping | yaml::Kind::Sequence) && !it.flow;
        if !open || depth >= MAX_TREE_DEPTH {
            ui.horizontal(|ui| self.row(ui, i, path));
            return;
        }
        let open_id = self.id.with(("node", path.as_slice()));
        let state = egui::collapsing_header::CollapsingState::load_with_default_open(
            ui.ctx(),
            open_id,
            depth < 2,
        );
        self.ax.expanded = Some(state.is_open());
        let ctx = ui.ctx().clone();
        let header = state.show_header(ui, |ui| self.row(ui, i, path));
        self.ax.toggle(&ctx, open_id);
        header.body(|ui| {
            let children = &self.doc.items[i].children;
            for (k, &c) in children.iter().take(MAX_CHILDREN).enumerate() {
                path.push(k);
                self.node(ui, c, path, depth + 1);
                path.pop();
            }
            if children.len() > MAX_CHILDREN {
                ui.label(
                    RichText::new((self.more)(children.len() - MAX_CHILDREN)).color(self.p.muted),
                );
            }
        });
    }

    /// One line: key (or sequence index), then the value or a summary.
    fn row(&mut self, ui: &mut egui::Ui, i: usize, path: &[usize]) {
        let (doc, p) = (self.doc, self.p);
        let it = &doc.items[i];
        let selected = self.state.selected.as_deref() == Some(path);
        let in_seq = it
            .parent
            .is_some_and(|q| doc.items[q].kind == yaml::Kind::Sequence);
        TreeAx::begin_row(ui);
        let ax_name = if let Some(k) = doc.key_text(i) {
            one_line(k)
        } else if it.key_lock.is_some() {
            "?".to_owned()
        } else if in_seq {
            path.last().copied().unwrap_or(0).to_string()
        } else {
            String::new()
        };
        let value = self.row_parts(ui, i, path, selected, in_seq, &ax_name);
        self.ax.finish_row(ui, p, selected, &ax_name, &value);
    }

    /// The widgets of a row; returns its value or summary as spoken.
    fn row_parts(
        &mut self,
        ui: &mut egui::Ui,
        i: usize,
        path: &[usize],
        selected: bool,
        in_seq: bool,
        ax_name: &str,
    ) -> String {
        let (doc, p) = (self.doc, self.p);
        let it = &doc.items[i];
        let editing = |f: Field| {
            self.state
                .editing
                .as_ref()
                .is_some_and(|e| e.field == f && e.path == path)
        };
        let (editing_key, editing_value) = (editing(Field::Key), editing(Field::Value));
        let tr = self.ax.tr;
        let count = num(it.children.len());
        let spoken = match it.kind {
            yaml::Kind::Mapping if !it.flow => tr("a11y-tree-object", &[("count", count)]),
            yaml::Kind::Sequence if !it.flow => tr("a11y-tree-list", &[("count", count)]),
            _ => doc
                .value(i)
                .unwrap_or_else(|| one_line(doc.value_text(i).unwrap_or_default())),
        };
        if editing_key {
            self.field(ui, i, ax_name);
            ui.label(RichText::new(":").monospace().color(p.muted));
        } else if let Some(k) = doc.key_text(i) {
            let r = label(ui, format!("{}:", one_line(k)), p.muted, selected, p);
            self.interact(ui, &r, i, path, Field::Key);
        } else if it.key_lock.is_some() {
            let r = label(ui, "?:".into(), p.muted, selected, p);
            self.interact(ui, &r, i, path, Field::Key);
        } else if in_seq {
            let n = path.last().copied().unwrap_or(0);
            let r = label(ui, format!("{n}:"), p.muted, selected, p);
            self.interact(ui, &r, i, path, Field::Value);
        }
        if editing_value {
            self.field(ui, i, ax_name);
            return spoken;
        }
        let (text, color) = match it.kind {
            yaml::Kind::Mapping | yaml::Kind::Sequence if !it.flow => {
                let n = it.children.len();
                let t = if it.kind == yaml::Kind::Mapping {
                    format!("{{{n}}}")
                } else {
                    format!("[{n}]")
                };
                (t, p.muted)
            }
            _ => {
                let raw = doc.value_text(i).unwrap_or_default();
                let shown = match it.style {
                    Some(yaml::Style::Literal | yaml::Style::Folded) => {
                        doc.value(i).unwrap_or_default()
                    }
                    _ => raw.to_owned(),
                };
                (one_line(&shown), p.text)
            }
        };
        let r = label(ui, text, color, selected, p);
        self.interact(ui, &r, i, path, Field::Value);
        spoken
    }

    fn interact(
        &mut self,
        ui: &egui::Ui,
        r: &egui::Response,
        i: usize,
        path: &[usize],
        field: Field,
    ) {
        let (doc, labels) = (self.doc, self.labels);
        let r = match doc.lock(i) {
            Some(l) => r.clone().on_hover_text(labels.lock(l)),
            None => r.clone(),
        };
        let double = select_on_click(ui, self.id, self.state, &r, path, field);
        if self.editable && double && !self.state.start_yaml_edit(doc, path, field) {
            self.state.start_default_yaml_edit(doc, path);
        }
        if !self.editable {
            return;
        }
        let it = &doc.items[i];
        let reason = |l: Option<yaml::Lock>| l.map(|l| labels.lock(l)).unwrap_or_default();
        // An entry with a disabled reason, when the operation applies;
        // the first enabled one takes focus when the keyboard opened the menu.
        let entry =
            |ui: &mut egui::Ui, text: &str, on: bool, why: Option<yaml::Lock>, first: &mut bool| {
                let b = ui.add_enabled(on, egui::Button::new(text));
                let b = if on {
                    if std::mem::take(first) {
                        b.request_focus();
                    }
                    b
                } else {
                    b.on_disabled_hover_text(reason(why))
                };
                b.clicked()
            };
        let scalar = matches!(it.kind, yaml::Kind::Scalar | yaml::Kind::Alias);
        let has_key = it.spans.key.is_some() || it.key_lock.is_some();
        let container = matches!(it.kind, yaml::Kind::Mapping | yaml::Kind::Sequence);
        let member = it.parent.is_some_and(|q| q != 0);
        let selected = self.state.selected.as_deref() == Some(path);
        let by_key = self.ax.take_menu(selected);
        let id = self.id;
        item_menu(&r, by_key, id, |ui, first| {
            let back = |ui: &egui::Ui| ui.memory_mut(|m| m.request_focus(id));
            if scalar
                && entry(
                    ui,
                    &labels.edit_value,
                    doc.can_set_value(i),
                    it.value_lock,
                    first,
                )
            {
                self.state.start_yaml_edit(doc, path, Field::Value);
            }
            if has_key
                && entry(
                    ui,
                    &labels.rename_key,
                    doc.can_rename(i),
                    it.key_lock,
                    first,
                )
            {
                self.state.start_yaml_edit(doc, path, Field::Key);
            }
            if member
                && entry(
                    ui,
                    &labels.add_after,
                    doc.can_add_after(i),
                    it.add_lock,
                    first,
                )
            {
                self.actions.push(TreeAction::AddAfter(path.to_vec()));
                back(ui);
            }
            if container
                && entry(
                    ui,
                    &labels.add_inside,
                    doc.can_add_inside(i),
                    doc.add_inside_lock(i),
                    first,
                )
            {
                self.actions.push(TreeAction::AddInside(path.to_vec()));
                back(ui);
            }
            if member {
                ui.separator();
                if entry(ui, &labels.delete, doc.can_delete(i), it.delete_lock, first) {
                    self.actions.push(TreeAction::Delete(path.to_vec()));
                    back(ui);
                }
            }
            if let Some(l) = doc.lock(i) {
                ui.separator();
                ui.label(RichText::new(labels.lock(l)).color(self.p.muted).small());
            }
        });
    }

    fn field(&mut self, ui: &mut egui::Ui, i: usize, name: &str) {
        let (doc, labels) = (self.doc, self.labels);
        let name = (self.ax.tr)("a11y-edit-field", &[("item", name.to_owned().into())]);
        let message = |e: FieldError| match e {
            FieldError::DuplicateKey => labels.duplicate_key.clone(),
            FieldError::KeyTooLong => labels.key_too_long.clone(),
            _ => labels.not_preserved.clone(),
        };
        let check = |f: Field, text: &str| {
            let r = match f {
                Field::Value => doc.check_value(i, text),
                Field::Key => doc.check_key(i, text),
            };
            r.map_err(FieldError::from)
        };
        if let Some((path, field, text)) =
            edit_field(ui, self.id, self.p, self.state, name, &message, &check)
        {
            self.actions.push(match field {
                Field::Value => TreeAction::SetValue { path, text },
                Field::Key => TreeAction::RenameKey { path, name: text },
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn localizer(tag: &str) -> text3_i18n::Localizer {
        text3_i18n::Localizer::new(&[tag]).expect("locale")
    }

    fn format(l: &text3_i18n::Localizer, id: &str, args: &[(&str, FluentValue<'_>)]) -> String {
        let mut fa = text3_i18n::FluentArgs::new();
        for (k, v) in args {
            fa.set(*k, v.clone());
        }
        l.format(id, Some(&fa))
            .replace(['\u{2068}', '\u{2069}'], "")
    }

    #[test]
    fn cell_labels_name_position_and_header() {
        let en = localizer("en-US");
        let tr = |id: &str, args: &[(&str, FluentValue<'_>)]| format(&en, id, args);
        assert_eq!(cell_label(&tr, 3, 2, Some("age")), "Row 3, column 2: age");
        assert_eq!(cell_label(&tr, 1, 1, Some("  ")), "Row 1, column 1");
        assert_eq!(cell_label(&tr, 1, 4, None), "Row 1, column 4");
        let fi = localizer("fi-FI");
        let tr = |id: &str, args: &[(&str, FluentValue<'_>)]| format(&fi, id, args);
        assert_eq!(cell_label(&tr, 3, 2, Some("ikä")), "Rivi 3, sarake 2: ikä");
    }

    #[test]
    fn tab_leaves_the_grid_only_at_its_ends() {
        assert!(!tab_leaves((0, 0), 3, 2, false));
        assert!(tab_leaves((2, 1), 3, 2, false));
        assert!(!tab_leaves((2, 1), 3, 2, true));
        assert!(tab_leaves((0, 0), 3, 2, true));
        assert!(!tab_leaves((1, 0), 3, 2, true));
    }

    #[test]
    fn left_selects_the_parent_only_when_shown() {
        let mut t = TreeState {
            visible: vec![vec![], vec![0], vec![0, 1]],
            ..TreeState::default()
        };
        t.select(&[0]);
        assert_eq!(t.selected.as_deref(), Some(&[0][..]));
        t.select(&[5]);
        assert_eq!(t.selected.as_deref(), Some(&[0][..]), "not shown: kept");
        t.step(1);
        assert_eq!(t.selected.as_deref(), Some(&[0, 1][..]));
        assert!(t.scroll, "keyboard moves bring the row into view");
    }

    #[test]
    fn accessibility_strings_exist_in_every_locale() {
        for (tag, _) in text3_i18n::RESOURCES {
            let l = localizer(tag);
            for id in A11Y_IDS {
                let text = format(
                    &l,
                    id,
                    &[
                        ("row", 2.into()),
                        ("column", 3.into()),
                        ("name", "x".into()),
                        ("item", "x".into()),
                        ("level", 1.into()),
                        ("count", 4.into()),
                        ("pane", 1.into()),
                        ("command", "x".into()),
                        ("keys", "x".into()),
                        ("nodes", 1.into()),
                        ("links", 1.into()),
                        ("participants", 1.into()),
                        ("messages", 1.into()),
                        ("states", 1.into()),
                        ("transitions", 1.into()),
                        ("classes", 1.into()),
                        ("relations", 1.into()),
                    ],
                );
                assert!(!text.is_empty() && text != *id, "{id} in {tag}");
            }
        }
    }

    const A11Y_IDS: [&str; 27] = [
        "a11y-csv-grid",
        "a11y-grid-cell",
        "a11y-grid-cell-plain",
        "a11y-grid-column-header",
        "a11y-grid-row-header",
        "a11y-edit-field",
        "a11y-json-tree",
        "a11y-xml-tree",
        "a11y-yaml-tree",
        "a11y-tree-item",
        "a11y-tree-item-expanded",
        "a11y-tree-item-collapsed",
        "a11y-tree-object",
        "a11y-tree-list",
        "a11y-tree-element",
        "a11y-tree-attribute",
        "a11y-tree-text",
        "a11y-pane",
        "a11y-pane-active",
        "a11y-pane-editor",
        "a11y-mermaid-flowchart",
        "a11y-mermaid-sequence",
        "a11y-mermaid-state",
        "a11y-mermaid-class",
        "a11y-shortcut-change",
        "a11y-shortcut-change-current",
        "a11y-shortcut-reset",
    ];

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
        let t = text3_tools::csv::parse("a,b\nc\n", ',');
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

    #[test]
    fn tree_fields_start_with_json_text_or_decoded_key() {
        let doc = json::doc(r#"{"\u00e4": "x\n", "n": [1e3, {}]}"#).unwrap();
        let mut t = TreeState::default();
        assert!(t.start_edit(&doc, &[0], Field::Value));
        assert_eq!(
            t.editing.as_ref().map(|e| e.text.as_str()),
            Some("\"x\\n\"")
        );
        assert!(t.start_edit(&doc, &[0], Field::Key));
        assert_eq!(t.editing.as_ref().map(|e| e.text.as_str()), Some("ä"));
        assert!(
            !t.start_edit(&doc, &[1], Field::Value),
            "containers have no value field"
        );
        assert!(
            !t.start_edit(&doc, &[1, 0], Field::Key),
            "array elements have no key"
        );
        t.start_default_edit(&doc, &[1]);
        assert_eq!(t.editing.as_ref().map(|e| e.field), Some(Field::Key));
        t.start_default_edit(&doc, &[1, 0]);
        assert_eq!(
            t.editing.as_ref().map(|e| (e.field, e.text.as_str())),
            Some((Field::Value, "1e3"))
        );
        assert_eq!(t.selected.as_deref(), Some(&[1, 0][..]));
        t.selected_field = Some(Field::Key);
        t.start_default_edit(&doc, &[0]);
        assert_eq!(
            t.editing.as_ref().map(|e| e.field),
            Some(Field::Key),
            "the clicked key, not the value"
        );
        t.visible = vec![vec![], vec![0], vec![1]];
        t.selected = Some(vec![0]);
        t.step(1);
        t.step(1);
        assert_eq!(t.selected, Some(vec![1]), "clamped at the end");
        t.step(-5);
        assert_eq!(t.selected, Some(vec![]));
    }

    #[test]
    fn xml_fields_start_with_names_and_decoded_values() {
        let doc = xml::doc("<r a=\"x &amp; y\"><t>1 &lt; 2</t><m>t <b/></m><e/></r>").unwrap();
        let mut t = TreeState::default();
        assert!(t.start_xml_edit(&doc, &[0, 0], Field::Value));
        assert_eq!(t.editing.as_ref().map(|e| e.text.as_str()), Some("x & y"));
        assert!(t.start_xml_edit(&doc, &[0, 0], Field::Key));
        assert_eq!(t.editing.as_ref().map(|e| e.text.as_str()), Some("a"));
        t.selected_field = None;
        t.start_default_xml_edit(&doc, &[0, 1]);
        assert_eq!(
            t.editing.as_ref().map(|e| (e.field, e.text.as_str())),
            Some((Field::Value, "1 < 2")),
            "a text-only element edits its text"
        );
        t.start_default_xml_edit(&doc, &[0, 2]);
        assert_eq!(
            t.editing.as_ref().map(|e| (e.field, e.text.as_str())),
            Some((Field::Key, "m")),
            "mixed content: the name"
        );
        assert!(t.start_xml_edit(&doc, &[0, 2, 0], Field::Value), "#text");
        assert!(!t.start_xml_edit(&doc, &[0, 2, 0], Field::Key));
        assert!(t.start_xml_edit(&doc, &[0, 3], Field::Value), "<e/> text");
        assert!(xml_deletable(&doc, doc.find(&[0, 3]).unwrap()));
        assert!(!xml_deletable(&doc, doc.find(&[0]).unwrap()), "the root");
        assert!(!xml_deletable(&doc, doc.find(&[0, 2, 0]).unwrap()));
    }

    #[test]
    fn yaml_fields_start_with_decoded_text_where_editable() {
        let doc =
            yaml::doc("'a b': 'it''s' # c\nn: 5\nf: [1]\nl:\n  - x\n  - &r y\n  - *r\n").unwrap();
        let mut t = TreeState::default();
        assert!(t.start_yaml_edit(&doc, &[0, 0], Field::Value));
        assert_eq!(t.editing.as_ref().map(|e| e.text.as_str()), Some("it's"));
        assert!(t.start_yaml_edit(&doc, &[0, 0], Field::Key));
        assert_eq!(t.editing.as_ref().map(|e| e.text.as_str()), Some("a b"));
        t.selected_field = None;
        t.start_default_yaml_edit(&doc, &[0, 1]);
        assert_eq!(
            t.editing.as_ref().map(|e| (e.field, e.text.as_str())),
            Some((Field::Value, "5"))
        );
        t.start_default_yaml_edit(&doc, &[0, 2]);
        assert_eq!(
            t.editing.as_ref().map(|e| e.field),
            Some(Field::Key),
            "a flow collection: its key"
        );
        assert!(
            !t.start_yaml_edit(&doc, &[0, 2, 0], Field::Value),
            "inside flow"
        );
        assert!(t.start_yaml_edit(&doc, &[0, 3, 0], Field::Value));
        assert!(!t.start_yaml_edit(&doc, &[0, 3, 0], Field::Key), "no key");
        assert!(!t.start_yaml_edit(&doc, &[0, 3, 1], Field::Value), "anchor");
        assert!(!t.start_yaml_edit(&doc, &[0, 3, 2], Field::Value), "alias");
        assert_eq!(one_line("a\r\nb"), "a⏎⏎b");
    }

    #[test]
    fn yaml_tree_strings_exist_in_every_locale() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../locales");
        let mut seen = 0;
        for entry in std::fs::read_dir(dir).expect("locales") {
            let path = entry.expect("entry").path().join("main.ftl");
            let ftl = std::fs::read_to_string(&path).expect("main.ftl");
            for id in YAML_IDS {
                assert!(ftl.contains(&format!("\n{id} = ")), "{id} in {path:?}");
            }
            seen += 1;
        }
        assert_eq!(seen, 26);
    }

    const YAML_IDS: [&str; 11] = [
        "yaml-not-preserved",
        "yaml-key-too-long",
        "yaml-read-only-flow",
        "yaml-read-only-anchor",
        "yaml-read-only-tag",
        "yaml-read-only-alias",
        "yaml-read-only-block-scalar",
        "yaml-read-only-complex-key",
        "yaml-read-only-document",
        "yaml-read-only-layout",
        "yaml-read-only-only-child",
    ];

    #[test]
    fn xml_tree_strings_exist_in_every_locale() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../locales");
        let mut seen = 0;
        for entry in std::fs::read_dir(dir).expect("locales") {
            let path = entry.expect("entry").path().join("main.ftl");
            let ftl = std::fs::read_to_string(&path).expect("main.ftl");
            for id in [
                "xml-edit-value",
                "xml-edit-text",
                "xml-rename",
                "xml-add-attribute",
                "xml-add-child",
                "xml-add-after",
                "xml-delete",
                "xml-invalid-name",
                "xml-duplicate-attribute",
                "xml-not-well-formed",
            ] {
                assert!(ftl.contains(&format!("\n{id} = ")), "{id} in {path:?}");
            }
            seen += 1;
        }
        assert_eq!(seen, 26);
    }
}
