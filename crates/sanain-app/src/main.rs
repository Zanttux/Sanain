//! Sanain desktop entry point.

mod diffview;
mod find;
mod folderpanel;
mod imagecache;
mod logview;
mod menu;
mod palette;
mod preview;
mod shell;
mod structured;
mod theme;
mod view;

use std::path::PathBuf;

use sanain_app::session::Session;
use sanain_app::settings::Settings;
use sanain_app::workspace::Workspace;
use sanain_i18n::Localizer;
use sanain_io::recovery::RecoveryStore;

fn main() -> eframe::Result {
    let (large, paths): (Vec<PathBuf>, Vec<PathBuf>) = std::env::args_os()
        .skip(1)
        .map(PathBuf::from)
        .partition(|p| shell::Shell::is_large(p));

    let settings_dir = Settings::default_dir();
    let settings = settings_dir
        .as_deref()
        .map(Settings::load)
        .unwrap_or_default();

    // Language: SANAIN_LOCALE, then the user's choice, then the system.
    let requested: Vec<String> = std::env::var("SANAIN_LOCALE")
        .ok()
        .into_iter()
        .chain(settings.locale.clone())
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
    let recovery_dir = std::env::var_os("SANAIN_RECOVERY_DIR")
        .map(PathBuf::from)
        .or_else(RecoveryStore::default_dir);
    // The session file lives next to the recovery directory.
    let session_file = recovery_dir
        .as_ref()
        .and_then(|d| d.parent())
        .map(|d| d.join("session"));
    let history = recovery_dir
        .as_ref()
        .and_then(|d| d.parent())
        .and_then(|d| sanain_io::history::HistoryStore::open(d.join("History")).ok());
    let store = recovery_dir.and_then(|dir| match RecoveryStore::open(&dir) {
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
    let (mut workspace, load_errors, restored) = Workspace::startup(store, snapshots, &paths);
    if paths.is_empty()
        && settings.restore_session
        && std::env::var_os("SANAIN_NO_SESSION").is_none()
        && let Some(session) = session_file.as_deref().and_then(Session::load)
    {
        workspace.restore_session(&session);
    }

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
            let mut shell = shell::Shell::new(
                workspace,
                l10n,
                load_errors,
                restored,
                session_file,
                settings,
                settings_dir,
            );
            shell.set_history(history);
            shell.init_recent();
            // One log viewer at a time: the last large file named wins.
            if let Some(p) = large.last() {
                shell.open_large(p, &cc.egui_ctx);
            }
            shell.install_menu(&cc.egui_ctx);
            shell.auto_update_check(&cc.egui_ctx);
            Ok(Box::new(shell))
        }),
    )
}
