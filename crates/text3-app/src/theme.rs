//! Colour themes applied to egui's visuals. The Text³ themes use the brand
//! tokens (assets/brand/tokens.json) and follow the system light/dark
//! preference; bundled and user themes (see [`themes`]) replace them.

use eframe::egui::{self, Color32, Theme};

/// Syntax colours of a theme (see [`syntax`]).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Syntax {
    pub keyword: Color32,
    pub string: Color32,
    pub escape: Color32,
    pub comment: Color32,
    pub number: Color32,
    pub function: Color32,
    pub type_: Color32,
    pub tag: Color32,
    pub attribute: Color32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Palette {
    /// Dark background (egui's dark style is used as the base).
    pub dark: bool,
    pub canvas: Color32,
    pub surface: Color32,
    pub text: Color32,
    pub muted: Color32,
    pub accent: Color32,
    pub border: Color32,
    pub error: Color32,
    pub warning: Color32,
    pub syntax: Syntax,
}

const fn hex(v: u32) -> Color32 {
    Color32::from_rgb((v >> 16) as u8, (v >> 8) as u8, v as u8)
}

/// Text³ dark syntax colours (WCAG AA on the dark canvas, 2026-11-25).
const TEXT3_DARK_SYNTAX: Syntax = Syntax {
    keyword: hex(0xE6A46A),
    string: hex(0x9DCC8F),
    escape: hex(0xE3D08A),
    comment: hex(0x84919B),
    number: hex(0xD9A8D2),
    function: hex(0x8FC6E8),
    type_: hex(0xE3D08A),
    tag: hex(0xE6A46A),
    attribute: hex(0xB9C6CF),
};

/// Text³ light syntax colours (WCAG AA on the light canvas).
const TEXT3_LIGHT_SYNTAX: Syntax = Syntax {
    keyword: hex(0x9A4A0B),
    string: hex(0x2F6F2A),
    escape: hex(0x7A5A00),
    comment: hex(0x5E6B74),
    number: hex(0x8A3D7A),
    function: hex(0x1F5E8C),
    type_: hex(0x7A5A00),
    tag: hex(0x9A4A0B),
    attribute: hex(0x3E4C55),
};

pub const LIGHT: Palette = Palette {
    dark: false,
    canvas: Color32::from_rgb(0xF7, 0xF9, 0xF8),
    surface: Color32::from_rgb(0xFF, 0xFF, 0xFF),
    text: Color32::from_rgb(0x18, 0x23, 0x2A),
    muted: Color32::from_rgb(0x53, 0x63, 0x6D),
    accent: Color32::from_rgb(0x08, 0x7A, 0x52),
    border: Color32::from_rgb(0xD4, 0xDD, 0xD9),
    error: Color32::from_rgb(0xB4, 0x23, 0x18),
    warning: Color32::from_rgb(0x8A, 0x52, 0x00),
    syntax: TEXT3_LIGHT_SYNTAX,
};

pub const DARK: Palette = Palette {
    dark: true,
    canvas: Color32::from_rgb(0x14, 0x17, 0x1B),
    surface: Color32::from_rgb(0x20, 0x25, 0x2C),
    text: Color32::from_rgb(0xE8, 0xED, 0xF0),
    muted: Color32::from_rgb(0xA7, 0xB4, 0xBE),
    accent: Color32::from_rgb(0x70, 0xDB, 0xA1),
    border: Color32::from_rgb(0x3B, 0x46, 0x50),
    error: Color32::from_rgb(0xFF, 0x91, 0x8A),
    warning: Color32::from_rgb(0xF6, 0xC6, 0x6D),
    syntax: TEXT3_DARK_SYNTAX,
};

/// High-contrast palette: pure black/white with a yellow accent.
pub const HIGH_CONTRAST: Palette = Palette {
    dark: true,
    canvas: Color32::from_rgb(0x00, 0x00, 0x00),
    surface: Color32::from_rgb(0x0A, 0x0A, 0x0A),
    text: Color32::from_rgb(0xFF, 0xFF, 0xFF),
    muted: Color32::from_rgb(0xD6, 0xD6, 0xD6),
    accent: Color32::from_rgb(0xFF, 0xD6, 0x0A),
    border: Color32::from_rgb(0xFF, 0xFF, 0xFF),
    error: Color32::from_rgb(0xFF, 0x8A, 0x8A),
    warning: Color32::from_rgb(0xFF, 0xD6, 0x0A),
    syntax: TEXT3_DARK_SYNTAX,
};

static HIGH_CONTRAST_ON: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// A bundled or user theme replacing the Text³ themes, if chosen.
static OVERRIDE: std::sync::RwLock<Option<&'static Palette>> = std::sync::RwLock::new(None);

pub fn palette(ctx: &egui::Context) -> &'static Palette {
    if let Some(p) = OVERRIDE.read().ok().and_then(|o| *o) {
        return p;
    }
    match ctx.theme() {
        Theme::Dark if HIGH_CONTRAST_ON.load(std::sync::atomic::Ordering::Relaxed) => {
            &HIGH_CONTRAST
        }
        Theme::Dark => &DARK,
        Theme::Light => &LIGHT,
    }
}

/// Switch the dark theme between the normal and high-contrast palette.
pub fn set_high_contrast(ctx: &egui::Context, on: bool) {
    HIGH_CONTRAST_ON.store(on, std::sync::atomic::Ordering::Relaxed);
    apply(ctx, Theme::Dark, if on { &HIGH_CONTRAST } else { &DARK });
    ctx.style_mut_of(Theme::Dark, |s| {
        let w = &mut s.visuals.widgets;
        let stroke = if on { 1.5 } else { 1.0 };
        for v in [&mut w.inactive, &mut w.hovered, &mut w.active] {
            v.bg_stroke.width = stroke;
        }
        // A clearly visible keyboard focus ring.
        s.visuals.selection.stroke.width = if on { 2.0 } else { 1.0 };
    });
}

/// Use `p` (a bundled or user theme) instead of the Text³ themes, or go
/// back to them with `None`.
pub fn set_override(ctx: &egui::Context, p: Option<&'static Palette>) {
    if let Ok(mut o) = OVERRIDE.write() {
        *o = p;
    }
    match p {
        Some(p) => {
            let t = if p.dark { Theme::Dark } else { Theme::Light };
            apply(ctx, t, p);
            ctx.set_theme(if p.dark {
                egui::ThemePreference::Dark
            } else {
                egui::ThemePreference::Light
            });
        }
        None => install(ctx),
    }
}

pub fn install(ctx: &egui::Context) {
    for (theme, p) in [(Theme::Light, &LIGHT), (Theme::Dark, &DARK)] {
        apply(ctx, theme, p);
    }
}

fn apply(ctx: &egui::Context, theme: Theme, p: &Palette) {
    {
        ctx.style_mut_of(theme, |style| {
            let v = &mut style.visuals;
            v.panel_fill = p.surface;
            v.window_fill = p.surface;
            v.extreme_bg_color = p.canvas;
            v.override_text_color = Some(p.text);
            v.selection.bg_fill = p.accent.gamma_multiply(0.35);
            v.selection.stroke.color = p.text;
            v.hyperlink_color = p.accent;
            v.error_fg_color = p.error;
            v.widgets.noninteractive.bg_stroke.color = p.border;
            v.window_stroke.color = p.border;
            v.window_shadow = egui::epaint::Shadow::NONE;
            v.popup_shadow = egui::epaint::Shadow::NONE;
            style.spacing.item_spacing = egui::vec2(8.0, 4.0);
            style.spacing.button_padding = egui::vec2(8.0, 4.0);
            style.animation_time = 0.0;
        });
    }
}

/// Syntax colour for `style` in palette `p`. Variables use the text
/// colour, operators and punctuation the muted colour.
pub fn syntax(p: &Palette, style: text3_language::Style) -> Color32 {
    use text3_language::Style as S;
    let c = &p.syntax;
    match style {
        S::Keyword => c.keyword,
        S::String => c.string,
        S::Escape => c.escape,
        S::Comment => c.comment,
        S::Number | S::Constant => c.number,
        S::Function => c.function,
        S::Type => c.type_,
        S::Tag => c.tag,
        S::Attribute | S::Property => c.attribute,
        S::Variable => p.text,
        S::Operator | S::Punctuation => p.muted,
    }
}
