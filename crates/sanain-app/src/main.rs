//! Sanain desktop entry point.

mod menu;
mod shell;
mod theme;
mod view;

use std::path::PathBuf;

use sanain_app::workspace::Workspace;
use sanain_i18n::Localizer;
use sanain_io::recovery::RecoveryStore;

fn main() -> eframe::Result {
    let paths: Vec<PathBuf> = std::env::args_os().skip(1).map(PathBuf::from).collect();

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

    // Crash recovery. SANAIN_RECOVERY_DIR overrides the location.
    let store = std::env::var_os("SANAIN_RECOVERY_DIR")
        .map(PathBuf::from)
        .or_else(RecoveryStore::default_dir)
        .and_then(|dir| match RecoveryStore::open(&dir) {
            Ok(s) => Some(s),
            Err(e) => {
                eprintln!("sanain: recovery disabled ({}): {e}", dir.display());
                None
            }
        });
    let snapshots = store
        .as_ref()
        .and_then(|s| s.list().ok())
        .map(|l| l.into_iter().filter_map(Result::ok).collect())
        .unwrap_or_default();
    let (workspace, load_errors, restored) = Workspace::startup(store, snapshots, &paths);
    let startup_errors: Vec<String> = load_errors
        .iter()
        .map(|(p, e)| shell::describe_load_error(&l10n, p, e))
        .collect();

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
            let mut shell = shell::Shell::new(workspace, l10n, startup_errors, restored);
            shell.install_menu(&cc.egui_ctx);
            Ok(Box::new(shell))
        }),
    )
}
