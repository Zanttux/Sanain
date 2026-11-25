//! Sanain desktop entry point.

mod shell;
mod theme;
mod view;

use std::path::PathBuf;

use sanain_app::Editor;
use sanain_i18n::Localizer;

fn main() -> eframe::Result {
    let path = std::env::args_os().nth(1).map(PathBuf::from);

    let requested: Vec<String> = std::env::var("SANAIN_LOCALE")
        .ok()
        .into_iter()
        .chain(sys_locale::get_locales())
        .collect();
    let refs: Vec<&str> = requested.iter().map(String::as_str).collect();
    let l10n = match Localizer::new(&refs) {
        Ok(l) => l,
        Err(e) => {
            // Embedded resources are checked by tests; this is a build bug.
            eprintln!("sanain: localization resources are invalid: {e:?}");
            std::process::exit(2);
        }
    };

    let (editor, startup_error) = match &path {
        Some(p) => match Editor::open(p) {
            Ok(ed) => (ed, None),
            Err(e) => (
                Editor::new(),
                Some(shell::describe_load_error(&l10n, p, &e)),
            ),
        },
        None => (Editor::new(), None),
    };

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("Sanain")
            .with_app_id("sanain")
            .with_inner_size([960.0, 680.0])
            .with_min_inner_size([360.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Sanain",
        options,
        Box::new(move |cc| {
            theme::install(&cc.egui_ctx);
            Ok(Box::new(shell::Shell::new(editor, l10n, startup_error)))
        }),
    )
}
