//! Sanain desktop entry point.
//!
//! Release builds on Windows use the GUI subsystem, so starting Sanain does
//! not open a console window. Command-line output (`--help`, `--version`,
//! `--register`) is then not shown in a console on Windows release builds;
//! debug builds keep the console for development.

#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

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
mod themes;
mod view;

use std::path::PathBuf;

use sanain_app::session::Session;
use sanain_app::settings::Settings;
use sanain_app::workspace::Workspace;
use sanain_i18n::Localizer;
use sanain_io::recovery::RecoveryStore;

fn main() -> eframe::Result {
    let parsed = sanain_app::cli::parse(std::env::args_os().skip(1));
    if parsed == sanain_app::cli::Parsed::Version {
        println!("Sanain {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    let cli = match &parsed {
        sanain_app::cli::Parsed::Run(c) => Some(c),
        _ => None,
    };
    let diff = cli.and_then(|c| c.diff.clone());
    let (large, mut paths): (Vec<PathBuf>, Vec<PathBuf>) = cli
        .map(|c| c.paths.clone())
        .unwrap_or_default()
        .into_iter()
        .partition(|p| shell::Shell::is_large(p));
    // Both sides of --diff open as tabs (so either can be edited).
    if let Some((l, r)) = &diff {
        paths.push(r.clone());
        paths.push(l.clone());
    }

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

    // Help and usage errors need the localizer; print and exit.
    match &parsed {
        sanain_app::cli::Parsed::Help => {
            println!(
                "{}\n{}",
                l10n.get("cli-help"),
                l10n.get("cli-help-integration")
            );
            return Ok(());
        }
        sanain_app::cli::Parsed::Register | sanain_app::cli::Parsed::Unregister => {
            let register = parsed == sanain_app::cli::Parsed::Register;
            std::process::exit(shell_registration(&l10n, register));
        }
        sanain_app::cli::Parsed::Unknown(arg) => {
            let mut args = sanain_i18n::FluentArgs::new();
            args.set("option", arg.as_str());
            // Bidi isolation marks are for GUI text; terminals show them.
            let msg = l10n
                .format("cli-unknown-option", Some(&args))
                .replace(['\u{2068}', '\u{2069}'], "");
            eprintln!("sanain: {msg}\n\n{}", l10n.get("cli-help"));
            std::process::exit(2);
        }
        sanain_app::cli::Parsed::DiffNeedsTwo => {
            eprintln!(
                "sanain: {}\n\n{}",
                l10n.get("cli-diff-needs-two"),
                l10n.get("cli-help")
            );
            std::process::exit(2);
        }
        _ => {}
    }

    // Crash recovery. SANAIN_RECOVERY_DIR overrides the location.
    let recovery_dir = std::env::var_os("SANAIN_RECOVERY_DIR")
        .map(PathBuf::from)
        .or_else(|| sanain_app::settings::portable_root().map(|r| r.join("recovery")))
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
        renderer: renderer(),
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("Sanain")
            .with_app_id("sanain")
            .with_inner_size([960.0, 680.0])
            .with_min_inner_size([360.0, 240.0])
            .with_icon(app_icon()),
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
            shell.start_editor_font(&cc.egui_ctx);
            shell.init_recent();
            // One log viewer at a time: the last large file named wins.
            if let Some(p) = large.last() {
                shell.open_large(p, &cc.egui_ctx);
            }
            if let Some((l, r)) = &diff {
                shell.compare_files(l, r);
            }
            shell.install_menu(&cc.egui_ctx);
            shell.auto_update_check(&cc.egui_ctx);
            Ok(Box::new(shell))
        }),
    )
}

/// `--register` / `--unregister`: prints the outcome, returns the exit code.
fn shell_registration(l10n: &Localizer, register: bool) -> i32 {
    use sanain_app::shellreg;
    let plain = |s: String| s.replace(['\u{2068}', '\u{2069}'], "");
    let fmt = |id: &str, k: &str, v: String| {
        let mut a = sanain_i18n::FluentArgs::new();
        a.set(k, v);
        plain(l10n.format(id, Some(&a)))
    };
    if cfg!(target_os = "macos") {
        println!("{}", l10n.get("cli-register-macos"));
        return 0;
    }
    let exe = match std::env::current_exe().and_then(|p| p.canonicalize()) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "sanain: {}",
                fmt("cli-register-failed", "reason", e.to_string())
            );
            return 1;
        }
    };
    let result: std::io::Result<String> = if cfg!(windows) {
        shellreg::windows_apply(&exe, register).map(|()| String::new())
    } else if register {
        shellreg::register_freedesktop(&exe, None).map(|p| p.display().to_string())
    } else {
        shellreg::unregister_freedesktop(None).map(|_| String::new())
    };
    match result {
        Ok(place) if register => {
            println!("{}", fmt("cli-registered", "place", place));
            0
        }
        Ok(_) => {
            println!("{}", l10n.get("cli-unregistered"));
            0
        }
        Err(e) => {
            eprintln!(
                "sanain: {}",
                fmt("cli-register-failed", "reason", e.to_string())
            );
            1
        }
    }
}

/// Window and taskbar icon (packaging/icons, generated from the brand SVG).
fn app_icon() -> std::sync::Arc<eframe::egui::IconData> {
    let png = include_bytes!("../../../packaging/icons/sanain-256.png");
    let icon = image::load_from_memory(png)
        .map(|i| i.into_rgba8())
        .map_or_else(
            |_| eframe::egui::IconData::default(),
            |rgba| eframe::egui::IconData {
                width: rgba.width(),
                height: rgba.height(),
                rgba: rgba.into_raw(),
            },
        );
    std::sync::Arc::new(icon)
}

/// Windows uses OpenGL (glow): wgpu flickers and lags while the window is
/// moved on some drivers (#95). `SANAIN_RENDERER=wgpu` or `glow` overrides
/// it where both are built in.
fn renderer() -> eframe::Renderer {
    #[cfg(windows)]
    {
        if std::env::var("SANAIN_RENDERER").as_deref() == Ok("wgpu") {
            return eframe::Renderer::Wgpu;
        }
        eframe::Renderer::Glow
    }
    #[cfg(not(windows))]
    {
        eframe::Renderer::Wgpu
    }
}
