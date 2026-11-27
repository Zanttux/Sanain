//! Read-only Markdown preview rendered from `sanain_markdown` blocks.
//!
//! Nothing here touches the network or runs document content. Clicking a
//! link only reports it; the shell decides what is allowed (confirmation
//! for web links, never for blocked schemes). Images show their alt text.

use std::ops::Range;

use eframe::egui::{
    self, Color32, CursorIcon, FontFamily, FontId, RichText, Sense, Stroke, text::LayoutJob,
    text::TextFormat,
};
use sanain_markdown::{Align, Block, BlockKind, Inline, MAX_DEPTH};

use crate::theme::Palette;

/// Documents larger than this are not previewed (bounded UI-thread work).
pub const MAX_PREVIEW_BYTES: usize = 2 * 1024 * 1024;

pub struct Style<'a> {
    pub p: &'a Palette,
    pub size: f32,
    pub untranslated: &'a dyn Fn(&str) -> String,
    /// Resolve and load an image by its Markdown URL.
    pub image: &'a dyn Fn(&str) -> crate::imagecache::ImageView,
}

/// What the user did in the preview this frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    LinkClicked(String),
    /// Allow images from this folder (for this session).
    AllowImages(std::path::PathBuf),
}

/// Byte ranges in a layout job's text that belong to a link target.
type Links = Vec<(Range<usize>, String)>;

/// Render `blocks`. `sync_line` (1-based source line) scrolls the block at
/// or before that line to the top when `scroll` is true. `anchor` scrolls
/// to the heading with that slug.
pub fn show(
    ui: &mut egui::Ui,
    blocks: &[Block],
    st: &Style<'_>,
    sync_line: usize,
    scroll: bool,
    anchor: Option<&str>,
) -> Option<Event> {
    let target = blocks.iter().rposition(|b| b.line <= sync_line);
    let mut event = None;
    for (i, b) in blocks.iter().enumerate() {
        let r = ui.scope(|ui| block(ui, b, st, 0, &mut event)).response;
        let anchored = anchor.is_some_and(|a| match &b.kind {
            BlockKind::Heading { inlines, .. } => slug(&sanain_markdown::inline_text(inlines)) == a,
            _ => false,
        });
        if anchored || (scroll && anchor.is_none() && Some(i) == target) {
            ui.scroll_to_rect(r.rect, Some(egui::Align::TOP));
        }
        ui.add_space(st.size * 0.6);
    }
    event
}

/// GitHub-style heading anchor: lowercase, spaces to '-', punctuation
/// removed.
pub fn slug(text: &str) -> String {
    text.trim()
        .to_lowercase()
        .chars()
        .filter_map(|c| match c {
            ' ' => Some('-'),
            c if c.is_alphanumeric() || c == '-' || c == '_' => Some(c),
            _ => None,
        })
        .collect()
}

fn block(ui: &mut egui::Ui, b: &Block, st: &Style<'_>, depth: usize, ev: &mut Option<Event>) {
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
            text(ui, inlines, st, st.size * scale, p.text, true, ev);
            if *level <= 2 {
                ui.add(egui::Separator::default().spacing(4.0));
            }
        }
        BlockKind::Paragraph(inlines) if is_image_paragraph(inlines) => {
            for i in inlines {
                if let Inline::Image { url, alt } = i {
                    image(ui, url, alt, st, ev);
                }
            }
        }
        BlockKind::Paragraph(inlines) => text(ui, inlines, st, st.size, body(p), false, ev),
        BlockKind::Code { lang, text: code } if lang.eq_ignore_ascii_case("mermaid") => {
            mermaid(ui, code, st);
        }
        BlockKind::Code { lang, text: code } => {
            code_frame_lang(ui, code.trim_end_matches('\n'), lang, st);
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
                        block(ui, c, st, depth + 1, ev);
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
                            block(ui, c, st, depth + 1, ev);
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
                            let mut cell =
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
                                        text(ui, inl, st, st.size, body(p), strong, ev);
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
        BlockKind::Html(raw) => code_frame(ui, raw.trim_end(), st),
        BlockKind::FrontMatter(fm) => code_frame(ui, fm, st),
    }
}

/// A wrapped text block whose link spans are clickable.
fn text(
    ui: &mut egui::Ui,
    inlines: &[Inline],
    st: &Style<'_>,
    size: f32,
    color: Color32,
    strong: bool,
    ev: &mut Option<Event>,
) {
    let (job, links) = job(inlines, st, size, color, strong);
    if links.is_empty() {
        ui.label(job);
        return;
    }
    let label = egui::Label::new(job).sense(Sense::click());
    let (pos, galley, response) = label.layout_in_ui(ui);
    ui.painter().galley(pos, galley.clone(), color);
    let Some(pointer) = response.hover_pos() else {
        return;
    };
    let cursor = galley.cursor_from_pos(pointer - pos);
    // Galley cursors count chars; links are byte ranges in the job text.
    let byte = galley
        .job
        .text
        .char_indices()
        .nth(cursor.index.0)
        .map_or(galley.job.text.len(), |(b, _)| b);
    let hit = links.iter().find(|(r, _)| r.contains(&byte));
    if let Some((_, url)) = hit {
        ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
        let response = response.on_hover_text(url);
        if response.clicked() {
            *ev = Some(Event::LinkClicked(url.clone()));
        }
    }
}

/// A paragraph holding only images (and line breaks) shows them as images;
/// images inside running text stay as their alt text.
fn is_image_paragraph(inlines: &[Inline]) -> bool {
    let mut any = false;
    for i in inlines {
        match i {
            Inline::Image { .. } => any = true,
            Inline::SoftBreak | Inline::LineBreak => {}
            Inline::Text(t) if t.trim().is_empty() => {}
            _ => return false,
        }
    }
    any
}

fn image(ui: &mut egui::Ui, url: &str, alt: &str, st: &Style<'_>, ev: &mut Option<Event>) {
    use crate::imagecache::ImageView;
    match (st.image)(url) {
        ImageView::Ready(tex) => {
            let natural = tex.size_vec2() / ui.ctx().pixels_per_point();
            let width = natural.x.min(ui.available_width());
            let size = egui::vec2(width, natural.y * width / natural.x.max(1.0));
            let mut img = egui::Image::new((tex.id(), size));
            if !alt.is_empty() {
                img = img.alt_text(alt);
            }
            ui.add(img).on_hover_text(alt);
        }
        ImageView::Loading => {
            ui.spinner();
        }
        ImageView::Ask(folder) => {
            let label = if alt.is_empty() { url } else { alt };
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    RichText::new(format!("[{label}] {}", (st.untranslated)("image-outside")))
                        .color(st.p.muted)
                        .italics(),
                );
                let button = (st.untranslated)("image-allow");
                if ui
                    .button(button)
                    .on_hover_text(folder.display().to_string())
                    .clicked()
                {
                    *ev = Some(Event::AllowImages(folder));
                }
            });
        }
        ImageView::Note(id) => {
            let label = if alt.is_empty() { url } else { alt };
            ui.label(
                RichText::new(format!("[{label}] {}", (st.untranslated)(id)))
                    .color(st.p.muted)
                    .italics(),
            );
        }
    }
}

/// Code blocks up to this size get syntax colours.
const HIGHLIGHT_MAX: usize = 64 * 1024;

/// Highlight spans per (language, code hash).
type SpanCache = std::collections::HashMap<
    (sanain_language::Lang, u64),
    std::rc::Rc<Vec<sanain_language::Span>>,
>;

thread_local! {
    static HIGHLIGHTER: std::cell::RefCell<sanain_language::Highlighter> =
        std::cell::RefCell::new(sanain_language::Highlighter::new());
    /// Spans per (language, code hash), so a block is parsed once, not per
    /// frame. Cleared when it grows past a few hundred blocks.
    static SPANS: std::cell::RefCell<SpanCache> =
        std::cell::RefCell::new(std::collections::HashMap::new());
}

fn spans_for(lang: sanain_language::Lang, code: &str) -> std::rc::Rc<Vec<sanain_language::Span>> {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    code.hash(&mut h);
    let key = (lang, h.finish());
    if let Some(s) = SPANS.with(|c| c.borrow().get(&key).cloned()) {
        return s;
    }
    let spans = std::rc::Rc::new(
        HIGHLIGHTER
            .with(|hl| hl.borrow_mut().highlight(lang, code.as_bytes(), None))
            .unwrap_or_default(),
    );
    SPANS.with(|c| {
        let mut c = c.borrow_mut();
        if c.len() > 512 {
            c.clear();
        }
        c.insert(key, spans.clone());
    });
    spans
}

/// Colour `code` as `lang`: plain text between spans, palette colours on them.
pub fn highlighted_job(
    code: &str,
    spans: &[sanain_language::Span],
    font: FontId,
    p: &Palette,
) -> LayoutJob {
    let mut job = LayoutJob::default();
    let fmt = |color| TextFormat {
        font_id: font.clone(),
        color,
        ..Default::default()
    };
    let mut at = 0;
    for s in spans {
        let (start, end) = (s.range.start.min(code.len()), s.range.end.min(code.len()));
        if start < at || !code.is_char_boundary(start) || !code.is_char_boundary(end) {
            continue;
        }
        if start > at {
            job.append(&code[at..start], 0.0, fmt(p.text));
        }
        job.append(
            &code[start..end],
            0.0,
            fmt(crate::theme::syntax(p, s.style)),
        );
        at = end;
    }
    if at < code.len() {
        job.append(&code[at..], 0.0, fmt(p.text));
    }
    job
}

/// A code block, coloured when the fence names a known language.
fn code_frame_lang(ui: &mut egui::Ui, code: &str, lang: &str, st: &Style<'_>) {
    let Some(l) = sanain_language::Lang::from_fence(lang).filter(|_| code.len() <= HIGHLIGHT_MAX)
    else {
        return code_frame(ui, code, st);
    };
    let spans = spans_for(l, code);
    let job = highlighted_job(code, &spans, FontId::monospace(st.size * 0.92), st.p);
    code_box(
        ui,
        code.len(),
        |ui| {
            ui.add(egui::Label::new(job).wrap_mode(egui::TextWrapMode::Extend));
        },
        st,
    );
}

fn code_box(ui: &mut egui::Ui, salt: usize, add: impl FnOnce(&mut egui::Ui), st: &Style<'_>) {
    egui::Frame::NONE
        .fill(st.p.surface)
        .stroke(Stroke::new(1.0, st.p.border))
        .corner_radius(4.0)
        .inner_margin(egui::Margin::same(8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            egui::ScrollArea::horizontal().id_salt(salt).show(ui, add);
        });
}

fn code_frame(ui: &mut egui::Ui, code: &str, st: &Style<'_>) {
    let text = RichText::new(code)
        .font(FontId::monospace(st.size * 0.92))
        .color(st.p.text);
    code_box(
        ui,
        code.len(),
        |ui| {
            ui.add(egui::Label::new(text).wrap_mode(egui::TextWrapMode::Extend));
        },
        st,
    );
}

/// A laid-out diagram.
enum Laid {
    Flow(
        sanain_markdown::mermaid::Flowchart,
        sanain_markdown::mermaid::Layout,
    ),
    Seq(
        sanain_markdown::mermaid::sequence::Sequence,
        sanain_markdown::mermaid::sequence::SeqLayout,
    ),
    State(
        sanain_markdown::mermaid::state::StateDiagram,
        sanain_markdown::mermaid::Layout,
    ),
    Class(
        sanain_markdown::mermaid::class::ClassDiagram,
        sanain_markdown::mermaid::class::ClassLayout,
    ),
}

/// A laid-out diagram, or why the source is shown instead.
type Diagram = Result<Laid, sanain_markdown::mermaid::Error>;

thread_local! {
    /// Diagrams per (code hash, font size), so a block is parsed and laid
    /// out once, not per frame.
    static DIAGRAMS: std::cell::RefCell<std::collections::HashMap<(u64, u32), std::rc::Rc<Diagram>>> =
        std::cell::RefCell::new(std::collections::HashMap::new());
}

/// Font for diagram labels at `size` points.
fn diagram_font(size: f32) -> FontId {
    FontId::proportional(size)
}

fn diagram_for(ui: &egui::Ui, code: &str, size: f32) -> std::rc::Rc<Diagram> {
    use sanain_markdown::mermaid::{self, Diagram as D};
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    code.hash(&mut h);
    let key = (h.finish(), size.to_bits());
    if let Some(d) = DIAGRAMS.with(|c| c.borrow().get(&key).cloned()) {
        return d;
    }
    let painter = ui.painter();
    let mut measure = |text: &str| {
        let g = painter.layout_no_wrap(text.to_owned(), diagram_font(size), Color32::WHITE);
        (g.size().x, g.size().y)
    };
    let sp = mermaid::Spacing::for_font(size);
    let d = std::rc::Rc::new(mermaid::parse_diagram(code).and_then(|d| match d {
        D::Flowchart(fc) => {
            let l = mermaid::layout(&fc, sp, &mut measure)?;
            Ok(Laid::Flow(fc, l))
        }
        D::Sequence(sq) => {
            let l = mermaid::sequence::layout(&sq, sp, &mut measure);
            Ok(Laid::Seq(sq, l))
        }
        D::State(sd) => {
            let l = mermaid::state::layout(&sd, sp, &mut measure)?;
            Ok(Laid::State(sd, l))
        }
        D::Class(cd) => {
            let l = mermaid::class::layout(&cd, sp, &mut measure)?;
            Ok(Laid::Class(cd, l))
        }
    }));
    DIAGRAMS.with(|c| {
        let mut c = c.borrow_mut();
        if c.len() > 128 {
            c.clear();
        }
        c.insert(key, d.clone());
    });
    d
}

/// A ```` ```mermaid ```` block: a native drawing of the supported subset,
/// otherwise a short note and the source. Nothing is fetched or run, and
/// `click` links are ignored by the parser.
fn mermaid(ui: &mut egui::Ui, code: &str, st: &Style<'_>) {
    use sanain_markdown::mermaid::Error;
    let size = (st.size * 0.9).round();
    match &*diagram_for(ui, code, size) {
        Ok(Laid::Flow(fc, l)) => draw_diagram(ui, fc, l, None, size, st),
        Ok(Laid::Seq(sq, l)) => draw_sequence(ui, sq, l, size, st),
        Ok(Laid::State(sd, l)) => {
            draw_diagram(ui, &sd.graph, l, Some((&sd.kinds, &sd.regions)), size, st);
        }
        Ok(Laid::Class(cd, l)) => draw_class(ui, cd, l, size, st),
        Err(e) => {
            let id = match e {
                Error::UnsupportedType => "preview-mermaid-unsupported-type",
                Error::Unsupported { .. } => "preview-mermaid-unsupported-syntax",
                Error::TooLarge => "preview-mermaid-too-large",
            };
            ui.label(
                RichText::new((st.untranslated)(id))
                    .color(st.p.muted)
                    .italics(),
            );
            code_frame(ui, code.trim_end_matches('\n'), st);
        }
    }
}

/// Canvas for a diagram `w` x `h` layout units: scaled down (never up) to
/// the preview width and centred. Returns the painter, the mapping from
/// layout units to screen and the scale, or `None` when off screen.
fn diagram_canvas(
    ui: &mut egui::Ui,
    w: f32,
    h: f32,
    margin: f32,
    p: &Palette,
) -> Option<(egui::Painter, egui::Pos2, f32)> {
    let avail = ui.available_width().max(1.0);
    let scale = ((avail - 2.0 * margin) / w.max(1.0)).clamp(0.05, 1.0);
    let desired = egui::vec2(avail, h * scale + 2.0 * margin);
    let (rect, _) = ui.allocate_exact_size(desired, Sense::hover());
    if !ui.is_rect_visible(rect) {
        return None;
    }
    let painter = ui.painter_at(rect);
    painter.rect(
        rect,
        4.0,
        p.canvas,
        Stroke::new(1.0, p.border),
        egui::StrokeKind::Inside,
    );
    let origin = egui::pos2(
        rect.left() + (rect.width() - w * scale) / 2.0,
        rect.top() + margin,
    );
    Some((painter, origin, scale))
}

/// Draw `text` anchored at `at` (`align` picks the anchor point).
fn diagram_text(
    painter: &egui::Painter,
    text: &str,
    font: &FontId,
    at: egui::Pos2,
    align: egui::Align2,
    color: Color32,
    bg: Option<Color32>,
) -> egui::Rect {
    let mut job = LayoutJob::simple(text.to_owned(), font.clone(), color, f32::INFINITY);
    job.halign = align.x();
    let g = painter.layout_job(job);
    let r = align.anchor_size(at, g.rect.size());
    if let Some(bg) = bg {
        painter.rect_filled(r.expand(2.0), 2.0, bg);
    }
    painter.galley(r.min - g.rect.min.to_vec2(), g, color);
    r
}

/// Flowchart link end: arrow, circle or cross at `tip`, coming from `from`.
fn link_head(
    painter: &egui::Painter,
    head: sanain_markdown::mermaid::Head,
    tip: egui::Pos2,
    from: egui::Pos2,
    color: Color32,
    scale: f32,
    width: f32,
) {
    use sanain_markdown::mermaid::Head;
    let dir = (tip - from).normalized();
    let len = 8.0 * scale.max(0.5) + width;
    match head {
        Head::None => {}
        Head::Arrow => {
            let base = tip - dir * len;
            let side = dir.rot90() * len * 0.45;
            painter.add(egui::Shape::convex_polygon(
                vec![tip, base + side, base - side],
                color,
                Stroke::NONE,
            ));
        }
        Head::Circle => {
            let r = len * 0.4;
            painter.circle_filled(tip - dir * r, r, color);
        }
        Head::Cross => {
            let r = len * 0.4;
            let c = tip - dir * r;
            let (a, b) = (
                (dir + dir.rot90()) * r * 0.75,
                (dir - dir.rot90()) * r * 0.75,
            );
            let s = Stroke::new(width.max(1.2), color);
            painter.line_segment([c - a, c + a], s);
            painter.line_segment([c - b, c + b], s);
        }
    }
}

/// Subgraph, composite-state or namespace frames, outer first (parents
/// come before children). `dashed` frames (concurrent regions) have no
/// fill and no title.
#[allow(clippy::too_many_arguments)]
fn draw_frames(
    painter: &egui::Painter,
    fc: &sanain_markdown::mermaid::Flowchart,
    l: &sanain_markdown::mermaid::Layout,
    dashed: &dyn Fn(usize) -> bool,
    at: &dyn Fn(f32, f32) -> egui::Pos2,
    scale: f32,
    font: &FontId,
    pad: f32,
    p: &Palette,
) {
    use egui::{Align2, StrokeKind, pos2, vec2};
    for (k, (sg, f)) in fc.subgraphs.iter().zip(&l.frames).enumerate() {
        let r = egui::Rect::from_center_size(at(f.x, f.y), vec2(f.w, f.h) * scale);
        let line = Stroke::new(1.0 * scale.max(0.6), p.border.lerp_to_gamma(p.muted, 0.4));
        if dashed(k) {
            let dash = 4.0 * scale.max(0.5);
            let pts = [
                r.left_top(),
                r.right_top(),
                r.right_bottom(),
                r.left_bottom(),
                r.left_top(),
            ];
            painter.extend(egui::Shape::dashed_line(&pts, line, dash, dash));
            continue;
        }
        let fill = p.canvas.lerp_to_gamma(p.surface, 0.45);
        painter.rect(r, 6.0 * scale, fill, line, StrokeKind::Inside);
        let band = l.title_band.get(k).copied().unwrap_or(0.0) * scale;
        let clip = painter.with_clip_rect(r.intersect(painter.clip_rect()));
        diagram_text(
            &clip,
            &sg.title,
            font,
            pos2(r.left() + pad, r.top() + pad * 0.5 + band / 2.0),
            Align2::LEFT_CENTER,
            body(p),
            None,
        );
    }
}

/// Note colours shared by sequence, state and class diagrams.
fn note_style(p: &Palette, scale: f32) -> (Color32, Stroke) {
    (
        p.surface.lerp_to_gamma(p.warning, 0.18),
        Stroke::new(1.0 * scale.max(0.6), p.warning.lerp_to_gamma(p.border, 0.5)),
    )
}

/// A flowchart, or a state diagram when `state` gives the node kinds and
/// which frames are concurrent regions.
fn draw_diagram(
    ui: &mut egui::Ui,
    fc: &sanain_markdown::mermaid::Flowchart,
    l: &sanain_markdown::mermaid::Layout,
    state: Option<(&[sanain_markdown::mermaid::state::Kind], &[bool])>,
    size: f32,
    st: &Style<'_>,
) {
    use egui::{Align2, Pos2, Shape as S, StrokeKind, pos2, vec2};
    use sanain_markdown::mermaid::{Shape, Stroke as Line};
    let p = st.p;
    let Some((painter, origin, scale)) = diagram_canvas(ui, l.width, l.height, size, p) else {
        return;
    };
    let at = |x: f32, y: f32| pos2(origin.x + x * scale, origin.y + y * scale);
    let font = diagram_font((size * scale).max(4.0));
    let label = |text: &str, centre: Pos2, color: Color32, bg: Option<Color32>| {
        diagram_text(
            &painter,
            text,
            &font,
            centre,
            Align2::CENTER_CENTER,
            color,
            bg,
        );
    };
    use sanain_markdown::mermaid::state::Kind;
    let pad = sanain_markdown::mermaid::Spacing::for_font(size).pad * scale;
    let regions = state.map_or(&[][..], |s| s.1);
    let dashed = |k: usize| regions.get(k).copied().unwrap_or(false);
    draw_frames(&painter, fc, l, &dashed, &at, scale, &font, pad, p);
    let edge_color = p.text.lerp_to_gamma(p.muted, 0.5);
    // Edges under nodes.
    for (e, path) in fc.edges.iter().zip(&l.edges) {
        if e.stroke == Line::Invisible || path.points.len() < 2 {
            continue;
        }
        let pts: Vec<Pos2> = path.points.iter().map(|&(x, y)| at(x, y)).collect();
        let width = if e.stroke == Line::Thick { 2.6 } else { 1.3 };
        let stroke = Stroke::new(width * scale.max(0.6), edge_color);
        if e.stroke == Line::Dotted {
            let dash = 4.0 * scale.max(0.5);
            painter.extend(S::dashed_line(&pts, stroke, dash, dash));
        } else {
            painter.add(S::line(pts.clone(), stroke));
        }
        let k = pts.len() - 1;
        link_head(
            &painter,
            e.head,
            pts[k],
            pts[k - 1],
            edge_color,
            scale,
            width,
        );
        link_head(&painter, e.tail, pts[0], pts[1], edge_color, scale, width);
    }
    let (note_fill, note_stroke) = note_style(p, scale);
    for (i, (node, b)) in fc.nodes.iter().zip(&l.nodes).enumerate() {
        let c = at(b.x, b.y);
        let half = vec2(b.w, b.h) * scale / 2.0;
        let r = egui::Rect::from_center_size(c, half * 2.0);
        let stroke = Stroke::new(1.3 * scale.max(0.6), p.accent);
        let kind = state.and_then(|s| s.0.get(i).copied());
        match kind {
            Some(Kind::Start) => {
                painter.circle_filled(c, half.x.min(half.y), p.text);
                continue;
            }
            Some(Kind::End) => {
                let outer = half.x.min(half.y);
                painter.circle_stroke(c, outer, Stroke::new(1.4 * scale.max(0.6), p.text));
                painter.circle_filled(c, outer * 0.6, p.text);
                continue;
            }
            Some(Kind::Fork | Kind::Join) => {
                painter.rect_filled(r, 1.5 * scale, p.text);
                continue;
            }
            Some(Kind::Note) => {
                painter.rect(r, 2.0, note_fill, note_stroke, StrokeKind::Inside);
                label(&node.label, c, p.text, None);
                continue;
            }
            _ => {}
        }
        let poly = |pts: Vec<Pos2>| {
            painter.add(S::convex_polygon(pts, p.surface, stroke));
        };
        // Slant inset for hexagons, parallelograms and trapezoids.
        let i = r.height() / 2.0;
        let (lt, rt, lb, rb) = (
            r.left_top(),
            r.right_top(),
            r.left_bottom(),
            r.right_bottom(),
        );
        let dx = vec2(i, 0.0);
        match node.shape {
            Shape::Rect => {
                painter.rect(r, 2.0, p.surface, stroke, StrokeKind::Inside);
            }
            Shape::Round => {
                painter.rect(r, 7.0 * scale, p.surface, stroke, StrokeKind::Inside);
            }
            Shape::Stadium => {
                painter.rect(r, r.height() / 2.0, p.surface, stroke, StrokeKind::Inside);
            }
            Shape::Circle => {
                painter.circle(c, half.x.min(half.y), p.surface, stroke);
            }
            Shape::DoubleCircle => {
                let outer = half.x.min(half.y);
                painter.circle(c, outer, p.surface, stroke);
                painter.circle_stroke(c, outer - 3.5 * scale.max(0.5), stroke);
            }
            Shape::Diamond => poly(vec![
                c - vec2(0.0, half.y),
                c + vec2(half.x, 0.0),
                c + vec2(0.0, half.y),
                c - vec2(half.x, 0.0),
            ]),
            Shape::Hexagon => poly(vec![
                lt + dx,
                rt - dx,
                pos2(r.right(), c.y),
                rb - dx,
                lb + dx,
                pos2(r.left(), c.y),
            ]),
            Shape::Parallelogram => poly(vec![lt + dx, rt, rb - dx, lb]),
            Shape::ParallelogramAlt => poly(vec![lt, rt - dx, rb, lb + dx]),
            Shape::Trapezoid => poly(vec![lt + dx, rt - dx, rb, lb]),
            Shape::TrapezoidAlt => poly(vec![lt, rt, rb - dx, lb + dx]),
            Shape::Cylinder => {
                // Body with elliptical lids h/6 high (the layout adds h/2).
                let ry = r.height() / 6.0;
                let rx = r.width() / 2.0;
                let ellipse = |cy: f32, from: f32, to: f32| -> Vec<Pos2> {
                    (0..=16)
                        .map(|k| {
                            let t = from + (to - from) * k as f32 / 16.0;
                            pos2(c.x + rx * t.cos(), cy + ry * t.sin())
                        })
                        .collect()
                };
                let pi = std::f32::consts::PI;
                let (top, bottom) = (r.top() + ry, r.bottom() - ry);
                // Left side down, front of the bottom lid, right side up,
                // back of the top lid.
                let mut pts = vec![pos2(r.left(), top)];
                pts.extend(ellipse(bottom, pi, 0.0));
                pts.extend(ellipse(top, 0.0, -pi).into_iter().skip(1));
                pts.pop();
                poly(pts);
                painter.add(S::line(ellipse(top, 0.0, pi), stroke));
            }
        }
        label(&node.label, c, p.text, None);
    }
    // Edge labels on top, on the canvas colour so lines do not cross text.
    for (e, path) in fc.edges.iter().zip(&l.edges) {
        if let (Some(text), Some((x, y))) = (&e.label, path.label_at)
            && e.stroke != Line::Invisible
        {
            label(text, at(x, y), body(p), Some(p.canvas));
        }
    }
}

/// Class relationship end at `tip`, coming from `from`: open arrow,
/// hollow triangle, or filled or hollow diamond.
fn class_end(
    painter: &egui::Painter,
    end: sanain_markdown::mermaid::class::End,
    tip: egui::Pos2,
    from: egui::Pos2,
    color: Color32,
    hollow: Color32,
    scale: f32,
) {
    use sanain_markdown::mermaid::class::End;
    let dir = (tip - from).normalized();
    let len = 11.0 * scale.max(0.5);
    let side = dir.rot90() * len * 0.45;
    let stroke = Stroke::new(1.3 * scale.max(0.6), color);
    match end {
        End::None => {}
        End::Arrow => {
            let base = tip - dir * len * 0.8;
            painter.add(egui::Shape::line(
                vec![base + side, tip, base - side],
                stroke,
            ));
        }
        End::Triangle => {
            let base = tip - dir * len;
            painter.add(egui::Shape::convex_polygon(
                vec![tip, base + side, base - side],
                hollow,
                stroke,
            ));
        }
        End::Diamond | End::DiamondOpen => {
            let mid = tip - dir * len * 0.65;
            let side = side * 0.85;
            let fill = if end == End::Diamond { color } else { hollow };
            painter.add(egui::Shape::convex_polygon(
                vec![tip, mid + side, tip - dir * len * 1.3, mid - side],
                fill,
                stroke,
            ));
        }
    }
}

fn draw_class(
    ui: &mut egui::Ui,
    cd: &sanain_markdown::mermaid::class::ClassDiagram,
    l: &sanain_markdown::mermaid::class::ClassLayout,
    size: f32,
    st: &Style<'_>,
) {
    use egui::{Align2, Pos2, Shape as S, StrokeKind, pos2, vec2};
    use sanain_markdown::mermaid::class::{Classifier, annotation_text};
    let p = st.p;
    let base = &l.base;
    let Some((painter, origin, scale)) = diagram_canvas(ui, base.width, base.height, size, p)
    else {
        return;
    };
    let at = |x: f32, y: f32| pos2(origin.x + x * scale, origin.y + y * scale);
    let font = diagram_font((size * scale).max(4.0));
    let pad = sanain_markdown::mermaid::Spacing::for_font(size).pad * scale;
    draw_frames(
        &painter,
        &cd.graph,
        base,
        &|_| false,
        &at,
        scale,
        &font,
        pad,
        p,
    );
    let edge_color = p.text.lerp_to_gamma(p.muted, 0.5);
    let dash = 4.0 * scale.max(0.5);
    let thin = Stroke::new(1.3 * scale.max(0.6), edge_color);
    let line = |pts: &[Pos2], dashed: bool| {
        if dashed {
            painter.extend(S::dashed_line(pts, thin, dash, dash));
        } else {
            painter.add(S::line(pts.to_vec(), thin));
        }
    };
    // Relations, then note links (dotted, no ends).
    for (i, path) in base.edges.iter().enumerate() {
        if path.points.len() < 2 {
            continue;
        }
        let pts: Vec<Pos2> = path.points.iter().map(|&(x, y)| at(x, y)).collect();
        match cd.relations.get(i) {
            Some(r) => line(&pts, r.dashed),
            None => line(&pts, true),
        }
    }
    // Ends above the lines, so hollow ends hide the line under them.
    for (r, path) in cd.relations.iter().zip(&base.edges) {
        if path.points.len() < 2 {
            continue;
        }
        let pts: Vec<Pos2> = path.points.iter().map(|&(x, y)| at(x, y)).collect();
        let k = pts.len() - 1;
        class_end(
            &painter,
            r.to_end,
            pts[k],
            pts[k - 1],
            edge_color,
            p.canvas,
            scale,
        );
        class_end(
            &painter, r.from_end, pts[0], pts[1], edge_color, p.canvas, scale,
        );
    }
    let stroke = Stroke::new(1.3 * scale.max(0.6), p.accent);
    for ((c, b), bx) in cd.classes.iter().zip(&base.nodes).zip(&l.boxes) {
        let r = egui::Rect::from_center_size(at(b.x, b.y), vec2(b.w, b.h) * scale);
        painter.rect(r, 2.0, p.surface, stroke, StrokeKind::Inside);
        for y in [bx.header, bx.attributes] {
            let y = r.top() + y * scale;
            painter.line_segment([pos2(r.left(), y), pos2(r.right(), y)], stroke);
        }
        let mut rows = bx.rows.iter().map(|y| r.top() + y * scale);
        let mut centred = |t: &str, color: Color32| {
            if let Some(y) = rows.next() {
                diagram_text(
                    &painter,
                    t,
                    &font,
                    pos2(r.center().x, y),
                    Align2::CENTER_CENTER,
                    color,
                    None,
                );
            }
        };
        if let Some(a) = &c.annotation {
            centred(&annotation_text(a), body(p));
        }
        centred(&c.name, p.text);
        for m in c.attributes.iter().chain(&c.methods) {
            let Some(y) = rows.next() else { break };
            let mut job = LayoutJob::default();
            job.append(
                &m.text,
                0.0,
                TextFormat {
                    font_id: font.clone(),
                    color: p.text,
                    italics: m.classifier == Classifier::Abstract,
                    underline: if m.classifier == Classifier::Static {
                        Stroke::new(1.0, p.text)
                    } else {
                        Stroke::NONE
                    },
                    ..Default::default()
                },
            );
            let g = painter.layout_job(job);
            let tr = Align2::LEFT_CENTER.anchor_size(pos2(r.left() + pad, y), g.rect.size());
            painter.galley(tr.min - g.rect.min.to_vec2(), g, p.text);
        }
    }
    let (note_fill, note_stroke) = note_style(p, scale);
    for (text, b) in cd.notes.iter().zip(&base.nodes[cd.classes.len()..]) {
        let r = egui::Rect::from_center_size(at(b.x, b.y), vec2(b.w, b.h) * scale);
        painter.rect(r, 2.0, note_fill, note_stroke, StrokeKind::Inside);
        diagram_text(
            &painter,
            text,
            &font,
            r.center(),
            Align2::CENTER_CENTER,
            p.text,
            None,
        );
    }
    // Labels and cardinalities on top, on the canvas colour.
    let label = |t: &str, (x, y): (f32, f32)| {
        diagram_text(
            &painter,
            t,
            &font,
            at(x, y),
            Align2::CENTER_CENTER,
            body(p),
            Some(p.canvas),
        );
    };
    for ((r, path), cards) in cd.relations.iter().zip(&base.edges).zip(&l.cards) {
        if let (Some(t), Some(pt)) = (&r.label, path.label_at) {
            label(t, pt);
        }
        if let (Some(t), Some(pt)) = (&r.from_card, cards[0]) {
            label(t, pt);
        }
        if let (Some(t), Some(pt)) = (&r.to_card, cards[1]) {
            label(t, pt);
        }
    }
}

fn draw_sequence(
    ui: &mut egui::Ui,
    sq: &sanain_markdown::mermaid::sequence::Sequence,
    l: &sanain_markdown::mermaid::sequence::SeqLayout,
    size: f32,
    st: &Style<'_>,
) {
    use egui::{Align2, Pos2, Shape as S, StrokeKind, pos2, vec2};
    use sanain_markdown::mermaid::sequence::{Item, Line, Rect, Tip, bracketed};
    let p = st.p;
    let Some((painter, origin, scale)) = diagram_canvas(ui, l.width, l.height, size, p) else {
        return;
    };
    let at = |x: f32, y: f32| pos2(origin.x + x * scale, origin.y + y * scale);
    let rect = |r: &Rect| egui::Rect::from_min_size(at(r.x, r.y), vec2(r.w, r.h) * scale);
    let font = diagram_font((size * scale).max(4.0));
    let small = diagram_font((size * 0.8 * scale).max(4.0));
    let text = |t: &str, pos: Pos2, align: Align2, color: Color32| {
        diagram_text(&painter, t, &font, pos, align, color, None);
    };
    let pad = sanain_markdown::mermaid::Spacing::for_font(size).pad * scale;
    let line_color = p.text.lerp_to_gamma(p.muted, 0.5);
    let thin = Stroke::new(1.2 * scale.max(0.6), line_color);
    let box_stroke = Stroke::new(1.3 * scale.max(0.6), p.accent);
    let dash = 4.0 * scale.max(0.5);
    // Lifelines.
    for i in 0..l.heads.len() {
        let x = origin.x + l.center(i) * scale;
        let (y0, y1) = (at(0.0, l.lifeline.0).y, at(0.0, l.lifeline.1).y);
        painter.extend(S::dashed_line(
            &[pos2(x, y0), pos2(x, y1)],
            Stroke::new(1.0 * scale.max(0.6), p.border.lerp_to_gamma(p.muted, 0.3)),
            dash,
            dash,
        ));
    }
    for b in &l.activations {
        painter.rect(rect(b), 0.0, p.surface, box_stroke, StrokeKind::Inside);
    }
    // Block frames: outlines and section lines here, labels on top later.
    let frame_stroke = Stroke::new(1.0 * scale.max(0.6), p.muted);
    for f in &l.frames {
        let r = rect(&f.rect);
        painter.rect_stroke(r, 3.0 * scale, frame_stroke, StrokeKind::Inside);
        for (y, _) in &f.branches {
            let y = at(0.0, *y).y;
            painter.extend(S::dashed_line(
                &[pos2(r.left(), y), pos2(r.right(), y)],
                frame_stroke,
                dash,
                dash,
            ));
        }
    }
    // Messages.
    let tip = |end: Pos2, dir: egui::Vec2, kind: Tip| {
        let len = 8.0 * scale.max(0.5);
        let base = end - dir * len;
        let side = dir.rot90() * len * 0.45;
        match kind {
            Tip::None => {}
            Tip::Arrow => {
                painter.add(S::convex_polygon(
                    vec![end, base + side, base - side],
                    line_color,
                    Stroke::NONE,
                ));
            }
            Tip::Open => {
                painter.add(S::line(vec![base + side, end, base - side], thin));
            }
            Tip::Cross => {
                let c = end - dir * len * 0.5;
                let (a, b) = (
                    (dir + dir.rot90()) * len * 0.35,
                    (dir - dir.rot90()) * len * 0.35,
                );
                let s = Stroke::new(1.6 * scale.max(0.6), line_color);
                painter.line_segment([c - a, c + a], s);
                painter.line_segment([c - b, c + b], s);
            }
        }
    };
    for m in &l.messages {
        let Some(Item::Message {
            text: label,
            line,
            tip: kind,
            number,
            ..
        }) = sq.items.get(m.item)
        else {
            continue;
        };
        let pts: Vec<Pos2> = match m.loop_size {
            Some((w, h)) => vec![
                at(m.x1, m.y),
                at(m.x1 + w, m.y),
                at(m.x1 + w, m.y + h),
                at(m.x1, m.y + h),
            ],
            None => vec![at(m.x1, m.y), at(m.x2, m.y)],
        };
        if *line == Line::Dotted {
            painter.extend(S::dashed_line(&pts, thin, dash, dash));
        } else {
            painter.add(S::line(pts.clone(), thin));
        }
        let k = pts.len() - 1;
        let dir = (pts[k] - pts[k - 1]).normalized();
        tip(pts[k], dir, *kind);
        text(
            label,
            at(m.text_at.0, m.text_at.1),
            Align2::CENTER_CENTER,
            body(p),
        );
        if let (Some(n), Some((x, y))) = (number, m.number_at) {
            let c = at(x, y);
            painter.circle_filled(c, size * 0.6 * scale, p.accent);
            diagram_text(
                &painter,
                &n.to_string(),
                &small,
                c,
                Align2::CENTER_CENTER,
                p.canvas,
                None,
            );
        }
    }
    // Notes.
    let (note_fill, note_stroke) = note_style(p, scale);
    for nb in &l.notes {
        let Some(Item::Note { text: t, .. }) = sq.items.get(nb.item) else {
            continue;
        };
        let r = rect(&nb.rect);
        painter.rect(r, 2.0, note_fill, note_stroke, StrokeKind::Inside);
        text(t, r.center(), Align2::CENTER_CENTER, p.text);
    }
    // Frame labels above bars and lines.
    let labelled = |t: &str, pos: Pos2, align: Align2| {
        diagram_text(&painter, t, &font, pos, align, body(p), Some(p.canvas));
    };
    for f in &l.frames {
        let r = rect(&f.rect);
        let tab = rect(&f.tab);
        painter.rect(tab, 0.0, p.surface, frame_stroke, StrokeKind::Inside);
        text(
            f.kind.keyword(),
            tab.center(),
            Align2::CENTER_CENTER,
            p.text,
        );
        if !f.label.is_empty() {
            labelled(
                &bracketed(&f.label),
                pos2(tab.right() + pad * 0.5, tab.center().y),
                Align2::LEFT_CENTER,
            );
        }
        for (y, label) in &f.branches {
            if !label.is_empty() {
                let y = at(0.0, *y).y;
                labelled(
                    &bracketed(label),
                    pos2(r.center().x, y + pad * 0.4),
                    Align2::CENTER_TOP,
                );
            }
        }
    }
    // Participant boxes, top and bottom.
    for (i, part) in sq.participants.iter().enumerate() {
        for b in [&l.heads[i], &l.feet[i]] {
            let r = rect(b);
            if part.actor {
                let lr = diagram_text(
                    &painter,
                    &part.label,
                    &font,
                    pos2(r.center().x, r.bottom()),
                    Align2::CENTER_BOTTOM,
                    p.text,
                    None,
                );
                // Stick figure above the label.
                let fig =
                    egui::Rect::from_min_max(r.left_top(), pos2(r.right(), lr.top() - pad * 0.25));
                let fh = fig.height();
                let cx = fig.center().x;
                let s = Stroke::new(1.4 * scale.max(0.6), p.accent);
                let head_r = fh * 0.16;
                let neck = fig.top() + head_r * 2.0;
                painter.circle(pos2(cx, fig.top() + head_r), head_r, p.surface, s);
                let hip = fig.top() + fh * 0.66;
                painter.line_segment([pos2(cx, neck), pos2(cx, hip)], s);
                let arm_y = neck + fh * 0.12;
                painter.line_segment(
                    [pos2(cx - fh * 0.28, arm_y), pos2(cx + fh * 0.28, arm_y)],
                    s,
                );
                painter.line_segment([pos2(cx, hip), pos2(cx - fh * 0.22, fig.bottom())], s);
                painter.line_segment([pos2(cx, hip), pos2(cx + fh * 0.22, fig.bottom())], s);
            } else {
                painter.rect(r, 3.0 * scale, p.surface, box_stroke, StrokeKind::Inside);
                text(&part.label, r.center(), Align2::CENTER_CENTER, p.text);
            }
        }
    }
}

/// Lay out inline content as one wrapped text job, collecting link spans.
fn job(
    inlines: &[Inline],
    st: &Style<'_>,
    size: f32,
    color: Color32,
    strong: bool,
) -> (LayoutJob, Links) {
    let mut j = LayoutJob::default();
    let mut links = Vec::new();
    let base = TextFormat {
        font_id: FontId::new(size, FontFamily::Proportional),
        color,
        ..Default::default()
    };
    append(&mut j, &mut links, None, inlines, st, &base, strong, 0);
    (j, links)
}

#[allow(clippy::too_many_arguments)]
fn append(
    j: &mut LayoutJob,
    links: &mut Links,
    link: Option<&str>,
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
    let push = |j: &mut LayoutJob, links: &mut Links, s: &str, f: TextFormat| {
        let start = j.text.len();
        j.append(s, 0.0, f);
        if let Some(url) = link {
            links.push((start..j.text.len(), url.to_owned()));
        }
    };
    for i in inlines {
        match i {
            Inline::Text(t) => {
                let mut f = fmt.clone();
                // egui ships no bold face: body text uses a softer colour so
                // strong text (full contrast) stands out.
                if strong && link.is_none() {
                    f.color = p.text;
                }
                push(j, links, t, f);
            }
            Inline::Code(t) => {
                let mut f = fmt.clone();
                f.font_id = FontId::monospace(fmt.font_id.size * 0.92);
                f.background = p.surface;
                push(j, links, t, f);
            }
            Inline::Strong(c) => append(j, links, link, c, st, fmt, true, depth + 1),
            Inline::Emph(c) => {
                let mut f = fmt.clone();
                f.italics = true;
                append(j, links, link, c, st, &f, strong, depth + 1);
            }
            Inline::Strike(c) => {
                let mut f = fmt.clone();
                f.strikethrough = Stroke::new(1.0, fmt.color);
                append(j, links, link, c, st, &f, strong, depth + 1);
            }
            Inline::Link { url, children } => {
                let mut f = fmt.clone();
                f.color = p.accent;
                f.underline = Stroke::new(1.0, p.accent);
                append(j, links, Some(url), children, st, &f, strong, depth + 1);
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
                push(j, links, &format!("[{label}]"), f);
            }
            Inline::Html(h) => {
                let mut f = fmt.clone();
                f.font_id = FontId::monospace(fmt.font_id.size * 0.92);
                f.color = p.muted;
                push(j, links, h, f);
            }
            Inline::SoftBreak => push(j, links, " ", fmt.clone()),
            Inline::LineBreak => push(j, links, "\n", fmt.clone()),
        }
    }
}

fn body(p: &Palette) -> Color32 {
    p.text.lerp_to_gamma(p.muted, 0.3)
}

#[cfg(test)]
mod tests {

    #[test]
    fn code_block_colours() {
        let p = crate::theme::DARK;
        let code = "fn main() { let s = \"ä\"; }";
        let spans = super::spans_for(sanain_language::Lang::Rust, code);
        assert!(!spans.is_empty());
        let job = super::highlighted_job(code, &spans, super::FontId::monospace(12.0), &p);
        assert_eq!(job.text, code, "text unchanged");
        let colours: std::collections::HashSet<_> =
            job.sections.iter().map(|s| s.format.color).collect();
        assert!(colours.len() >= 3, "keyword, string and plain text differ");
    }
    #[test]
    fn mermaid_notes_exist_in_every_locale() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../locales");
        let mut seen = 0;
        for entry in std::fs::read_dir(dir).expect("locales") {
            let path = entry.expect("entry").path().join("main.ftl");
            let ftl = std::fs::read_to_string(&path).expect("main.ftl");
            for id in [
                "preview-mermaid-unsupported-type",
                "preview-mermaid-unsupported-syntax",
                "preview-mermaid-too-large",
            ] {
                assert!(ftl.contains(&format!("\n{id} = ")), "{id} in {path:?}");
            }
            seen += 1;
        }
        assert_eq!(seen, 26);
    }
    #[test]
    fn heading_slugs() {
        assert_eq!(super::slug("Getting Started!"), "getting-started");
        assert_eq!(super::slug("  Ääkköset & more "), "ääkköset--more");
    }
}
