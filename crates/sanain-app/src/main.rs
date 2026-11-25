//! Sanain desktop entry point.

mod menu;
mod shell;
mod theme;
mod view;

use std::path::PathBuf;

use sanain_app::Editor;
use sanain_app::recovery::{self, Recovery};
use sanain_i18n::Localizer;
use sanain_io::recovery::RecoveryStore;

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
    let restored = store.as_ref().and_then(|s| {
        let entries = s.list().ok()?.into_iter().filter_map(Result::ok).collect();
        recovery::pick(entries, path.as_deref())
    });

    let mut recovered = false;
    let (editor, startup_error, recovery_id) = match (&restored, &path) {
        (Some(entry), _) => {
            let ed = Editor::restore(&entry.snapshot);
            recovered = ed.document().is_dirty();
            (ed, None, Some(entry.id.clone()))
        }
        (None, Some(p)) => match Editor::open(p) {
            Ok(ed) => (ed, None, None),
            Err(e) => (
                Editor::new(),
                Some(shell::describe_load_error(&l10n, p, &e)),
                None,
            ),
        },
        (None, None) => (Editor::new(), None, None),
    };
    // A restored snapshot identical to disk is stale: drop it.
    if let (Some(s), Some(id), false) = (&store, &recovery_id, recovered) {
        let _ = s.remove(id);
    }
    let recovery = store.map(|s| Recovery::start(s, recovery_id.filter(|_| recovered)));

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
            let mut shell = shell::Shell::new(editor, l10n, startup_error, recovery, recovered);
            shell.install_menu(&cc.egui_ctx);
            Ok(Box::new(shell))
        }),
    )
}
