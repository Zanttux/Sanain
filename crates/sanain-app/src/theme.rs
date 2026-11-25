//! Brand tokens (assets/brand/tokens.json) applied to egui's visuals.
//! The app follows the system light/dark preference.

use eframe::egui::{self, Color32, Theme};

pub struct Palette {
    pub canvas: Color32,
    pub surface: Color32,
    pub text: Color32,
    pub muted: Color32,
    pub accent: Color32,
    pub border: Color32,
    pub error: Color32,
    pub warning: Color32,
}

pub const LIGHT: Palette = Palette {
    canvas: Color32::from_rgb(0xF7, 0xF9, 0xF8),
    surface: Color32::from_rgb(0xFF, 0xFF, 0xFF),
    text: Color32::from_rgb(0x18, 0x23, 0x2A),
    muted: Color32::from_rgb(0x53, 0x63, 0x6D),
    accent: Color32::from_rgb(0x08, 0x7A, 0x52),
    border: Color32::from_rgb(0xD4, 0xDD, 0xD9),
    error: Color32::from_rgb(0xB4, 0x23, 0x18),
    warning: Color32::from_rgb(0x8A, 0x52, 0x00),
};

pub const DARK: Palette = Palette {
    canvas: Color32::from_rgb(0x14, 0x17, 0x1B),
    surface: Color32::from_rgb(0x20, 0x25, 0x2C),
    text: Color32::from_rgb(0xE8, 0xED, 0xF0),
    muted: Color32::from_rgb(0xA7, 0xB4, 0xBE),
    accent: Color32::from_rgb(0x70, 0xDB, 0xA1),
    border: Color32::from_rgb(0x3B, 0x46, 0x50),
    error: Color32::from_rgb(0xFF, 0x91, 0x8A),
    warning: Color32::from_rgb(0xF6, 0xC6, 0x6D),
};

/// High-contrast palette: pure black/white with a yellow accent.
pub const HIGH_CONTRAST: Palette = Palette {
    canvas: Color32::from_rgb(0x00, 0x00, 0x00),
    surface: Color32::from_rgb(0x0A, 0x0A, 0x0A),
    text: Color32::from_rgb(0xFF, 0xFF, 0xFF),
    muted: Color32::from_rgb(0xD6, 0xD6, 0xD6),
    accent: Color32::from_rgb(0xFF, 0xD6, 0x0A),
    border: Color32::from_rgb(0xFF, 0xFF, 0xFF),
    error: Color32::from_rgb(0xFF, 0x8A, 0x8A),
    warning: Color32::from_rgb(0xFF, 0xD6, 0x0A),
};

static HIGH_CONTRAST_ON: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

pub fn palette(ctx: &egui::Context) -> &'static Palette {
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

/// Syntax colours, kept apart from brand colours (docs/BRAND.md). Each
/// foreground meets WCAG 2.2 AA (>= 4.5:1) on the editor canvas: dark
/// 5.6-15.2:1, light 5.2-15.1:1 (computed 2026-11-25). Selection and
/// search-highlight backgrounds are not yet verified.
pub fn syntax(p: &Palette, style: sanain_language::Style) -> Color32 {
    use sanain_language::Style as S;
    let dark = p.canvas == DARK.canvas || p.canvas == HIGH_CONTRAST.canvas;
    let rgb = |d: [u8; 3], l: [u8; 3]| {
        let c = if dark { d } else { l };
        Color32::from_rgb(c[0], c[1], c[2])
    };
    match style {
        S::Keyword => rgb([0xE6, 0xA4, 0x6A], [0x9A, 0x4A, 0x0B]),
        S::String => rgb([0x9D, 0xCC, 0x8F], [0x2F, 0x6F, 0x2A]),
        S::Escape => rgb([0xE3, 0xD0, 0x8A], [0x7A, 0x5A, 0x00]),
        S::Comment => rgb([0x84, 0x91, 0x9B], [0x5E, 0x6B, 0x74]),
        S::Number | S::Constant => rgb([0xD9, 0xA8, 0xD2], [0x8A, 0x3D, 0x7A]),
        S::Function => rgb([0x8F, 0xC6, 0xE8], [0x1F, 0x5E, 0x8C]),
        S::Type => rgb([0xE3, 0xD0, 0x8A], [0x7A, 0x5A, 0x00]),
        S::Tag => rgb([0xE6, 0xA4, 0x6A], [0x9A, 0x4A, 0x0B]),
        S::Attribute | S::Property => rgb([0xB9, 0xC6, 0xCF], [0x3E, 0x4C, 0x55]),
        S::Variable => p.text,
        S::Operator | S::Punctuation => p.muted,
    }
}
