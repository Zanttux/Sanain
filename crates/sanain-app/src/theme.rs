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
}

pub const LIGHT: Palette = Palette {
    canvas: Color32::from_rgb(0xF7, 0xF9, 0xF8),
    surface: Color32::from_rgb(0xFF, 0xFF, 0xFF),
    text: Color32::from_rgb(0x18, 0x23, 0x2A),
    muted: Color32::from_rgb(0x53, 0x63, 0x6D),
    accent: Color32::from_rgb(0x08, 0x7A, 0x52),
    border: Color32::from_rgb(0xD4, 0xDD, 0xD9),
    error: Color32::from_rgb(0xB4, 0x23, 0x18),
};

pub const DARK: Palette = Palette {
    canvas: Color32::from_rgb(0x14, 0x17, 0x1B),
    surface: Color32::from_rgb(0x20, 0x25, 0x2C),
    text: Color32::from_rgb(0xE8, 0xED, 0xF0),
    muted: Color32::from_rgb(0xA7, 0xB4, 0xBE),
    accent: Color32::from_rgb(0x70, 0xDB, 0xA1),
    border: Color32::from_rgb(0x3B, 0x46, 0x50),
    error: Color32::from_rgb(0xFF, 0x91, 0x8A),
};

pub fn palette(ctx: &egui::Context) -> &'static Palette {
    match ctx.theme() {
        Theme::Dark => &DARK,
        Theme::Light => &LIGHT,
    }
}

pub fn install(ctx: &egui::Context) {
    for (theme, p) in [(Theme::Light, &LIGHT), (Theme::Dark, &DARK)] {
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
