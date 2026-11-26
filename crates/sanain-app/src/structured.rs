//! Structured views: an editable CSV grid, editable JSON and XML trees and
//! a read-only YAML tree. The source stays authoritative: edits come back
//! as [`GridAction`]s and [`TreeAction`]s that the caller applies to the
//! text as ordinary undoable edits.

use eframe::egui::{self, RichText};
use egui_extras::{Column, TableBuilder};
use sanain_tools::csv::Table;
use sanain_tools::json::{self, Node};
use sanain_tools::xml;

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

/// Selection and inline editor of one tab's JSON or XML tree. Nodes are
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

/// What the tree's keys asked for on the selected node.
enum KeyAct {
    Edit(Vec<usize>),
    Delete(Vec<usize>),
}

/// Shared start of a tree frame: drop the field when it cannot apply,
/// expand the parents of a pending node, then the keys while `id` has
/// focus: Up and Down move the selection, Enter or F2 edits and Delete
/// deletes (when `editable`).
fn tree_frame(
    ui: &mut egui::Ui,
    id: egui::Id,
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
        if editable && let Some(sel) = state.selected.clone() {
            if pressed(Key::Enter) || pressed(Key::F2) {
                act = Some(KeyAct::Edit(sel));
            } else if pressed(Key::Delete) && !sel.is_empty() {
                act = Some(KeyAct::Delete(sel));
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
    let resp = ui.add(
        egui::TextEdit::singleline(&mut ed.text)
            .id(editor_id)
            .font(egui::TextStyle::Monospace)
            .desired_width(width.clamp(120.0, 600.0)),
    );
    if ed.focus {
        resp.request_focus();
        ed.focus = false;
    }
    if let Some(e) = ed.error {
        ui.colored_label(p.error, message(e));
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
    ui.add(egui::Label::new(t).sense(egui::Sense::click()))
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
) -> Vec<TreeAction> {
    let mut actions = Vec::new();
    let exists = |path: &[usize]| doc.find(path).is_some();
    let open = |s: &mut TreeState, path: &[usize], f| {
        s.start_edit(doc, path, f);
    };
    let act = tree_frame(ui, id, editable, state, &exists, &open);
    match act {
        Some(KeyAct::Edit(sel)) => state.start_default_edit(doc, &sel),
        Some(KeyAct::Delete(sel)) => actions.push(TreeAction::Delete(sel)),
        None => {}
    }
    let mut visible = Vec::new();
    egui::ScrollArea::both().auto_shrink(false).show(ui, |ui| {
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
        };
        cx.node(ui, 0, &mut Vec::new(), 0, None);
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
        let header = state.show_header(ui, |ui| self.row(ui, i, path, index));
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
            self.field(ui, i);
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
            self.field(ui, i);
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
        r.context_menu(|ui| {
            if scalar && ui.button(&self.labels.edit_value).clicked() {
                self.state.start_edit(self.doc, path, Field::Value);
            }
            if it.key_span.is_some() && ui.button(&self.labels.rename_key).clicked() {
                self.state.start_edit(self.doc, path, Field::Key);
            }
            if it.parent.is_some() && ui.button(&self.labels.add_after).clicked() {
                self.actions.push(TreeAction::AddAfter(path.to_vec()));
            }
            if !scalar && ui.button(&self.labels.add_inside).clicked() {
                self.actions.push(TreeAction::AddInside(path.to_vec()));
            }
            if it.parent.is_some() {
                ui.separator();
                if ui.button(&self.labels.delete).clicked() {
                    self.actions.push(TreeAction::Delete(path.to_vec()));
                }
            }
        });
    }

    /// The inline field of the node being edited.
    fn field(&mut self, ui: &mut egui::Ui, i: usize) {
        let (doc, labels) = (self.doc, self.labels);
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
            edit_field(ui, self.id, self.p, self.state, &message, &check)
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
) -> Vec<TreeAction> {
    let mut actions = Vec::new();
    let exists = |path: &[usize]| doc.find(path).is_some();
    let open = |s: &mut TreeState, path: &[usize], f| {
        s.start_xml_edit(doc, path, f);
    };
    let act = tree_frame(ui, id, editable, state, &exists, &open);
    match act {
        Some(KeyAct::Edit(sel)) => state.start_default_xml_edit(doc, &sel),
        Some(KeyAct::Delete(sel)) if doc.find(&sel).is_some_and(|i| xml_deletable(doc, i)) => {
            actions.push(TreeAction::Delete(sel));
        }
        _ => {}
    }
    let mut visible = Vec::new();
    egui::ScrollArea::both().auto_shrink(false).show(ui, |ui| {
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
        };
        let mut path = Vec::new();
        for (k, &c) in doc.items[0].children.iter().enumerate() {
            path.push(k);
            cx.node(ui, c, &mut path, 0);
            path.pop();
        }
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
}

impl XmlCx<'_> {
    fn node(&mut self, ui: &mut egui::Ui, i: usize, path: &mut Vec<usize>, depth: usize) {
        self.visible.push(path.clone());
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
        let header = state.show_header(ui, |ui| self.row(ui, i, path));
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
        match it.kind {
            xml::Kind::Element => {
                if editing_key {
                    self.field(ui, i);
                } else {
                    let r = label(ui, format!("<{name}>"), p.accent, selected, p);
                    self.interact(ui, &r, i, path, Field::Key);
                }
            }
            xml::Kind::Attribute => {
                if editing_key {
                    self.field(ui, i);
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
            self.field(ui, i);
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
        r.context_menu(|ui| {
            let edit = match it.kind {
                xml::Kind::Attribute => Some(&labels.edit_value),
                xml::Kind::Text => Some(&labels.edit_text),
                _ => it.text_only.then_some(&labels.edit_text),
            };
            if let Some(edit) = edit
                && ui.button(edit).clicked()
            {
                self.state.start_xml_edit(doc, path, Field::Value);
            }
            if it.kind != xml::Kind::Text && ui.button(&labels.rename).clicked() {
                self.state.start_xml_edit(doc, path, Field::Key);
            }
            if it.kind != xml::Kind::Text && ui.button(&labels.add_attribute).clicked() {
                self.actions.push(TreeAction::AddAttribute(path.to_vec()));
            }
            if element && ui.button(&labels.add_child).clicked() {
                self.actions.push(TreeAction::AddInside(path.to_vec()));
            }
            if element && xml_deletable(doc, i) && ui.button(&labels.add_after).clicked() {
                self.actions.push(TreeAction::AddAfter(path.to_vec()));
            }
            if xml_deletable(doc, i) {
                ui.separator();
                if ui.button(&labels.delete).clicked() {
                    self.actions.push(TreeAction::Delete(path.to_vec()));
                }
            }
        });
    }

    fn field(&mut self, ui: &mut egui::Ui, i: usize) {
        let (doc, labels) = (self.doc, self.labels);
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
            edit_field(ui, self.id, self.p, self.state, &message, &check)
        {
            self.actions.push(match field {
                Field::Value => TreeAction::SetValue { path, text },
                Field::Key => TreeAction::RenameKey { path, name: text },
            });
        }
    }
}

/// A read-only tree (YAML).
pub fn tree(ui: &mut egui::Ui, root: &Node, p: &Palette, more: &dyn Fn(usize) -> String) {
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
