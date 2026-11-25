//! Read-only structured views: CSV grid and JSON tree. They never modify
//! the document; the source stays authoritative and editable.

use eframe::egui::{self, RichText};
use egui_extras::{Column, TableBuilder};
use sanain_tools::csv::Table;
use sanain_tools::json::Node;

use crate::theme::Palette;

/// JSON nesting shown before the rest is summarised.
const MAX_TREE_DEPTH: usize = 64;
/// Children listed per container before "… more".
const MAX_CHILDREN: usize = 1_000;

pub fn csv(ui: &mut egui::Ui, t: &Table, p: &Palette, size: f32, header: bool) {
    let cols = t.rows.iter().map(Vec::len).max().unwrap_or(0);
    if cols == 0 {
        return;
    }
    let row_h = size * 1.6;
    let (head, body) = if header && !t.rows.is_empty() {
        (Some(&t.rows[0]), &t.rows[1..])
    } else {
        (None, &t.rows[..])
    };
    egui::ScrollArea::horizontal().show(ui, |ui| {
        let mut builder = TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .column(Column::exact(size * 3.5));
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
                        let name = head
                            .and_then(|r| r.get(c))
                            .cloned()
                            .unwrap_or_else(|| (c + 1).to_string());
                        ui.label(RichText::new(name).strong().color(p.text));
                    });
                }
            })
            .body(|b| {
                b.rows(row_h, body.len(), |mut row| {
                    let i = row.index();
                    let r = &body[i];
                    row.col(|ui| {
                        ui.label(
                            RichText::new((i + 1).to_string())
                                .color(p.muted)
                                .monospace(),
                        );
                    });
                    for c in 0..cols {
                        row.col(|ui| {
                            let v = r.get(c).map_or("", String::as_str);
                            // Embedded line breaks shown as a symbol to keep rows aligned.
                            let one_line = v.replace("\r\n", "⏎").replace(['\n', '\r'], "⏎");
                            ui.add(
                                egui::Label::new(RichText::new(one_line).color(p.text)).truncate(),
                            )
                            .on_hover_text(v);
                        });
                    }
                });
            });
    });
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
