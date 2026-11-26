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
        BlockKind::Code { lang, text: code } => {
            if lang.eq_ignore_ascii_case("mermaid") {
                ui.label(
                    RichText::new((st.untranslated)("preview-mermaid-unsupported"))
                        .color(p.muted)
                        .italics(),
                );
            }
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
    fn heading_slugs() {
        assert_eq!(super::slug("Getting Started!"), "getting-started");
        assert_eq!(super::slug("  Ääkköset & more "), "ääkköset--more");
    }
}
