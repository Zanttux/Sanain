//! Text³ desktop entry point.
//!
//! Release builds on Windows use the GUI subsystem, so starting Text³ does
//! not open a console window. CLI commands that print and exit attach to
//! the parent's console. Their output may appear after the shell prompt
//! because the shell does not wait for GUI-subsystem programs; this is
//! normal Windows behaviour. Debug builds keep the console for development.

#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod diffview;
mod find;
mod focus;
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

use text3_app::session::Session;
use text3_app::settings::Settings;
use text3_app::workspace::Workspace;
use text3_i18n::Localizer;
use text3_io::recovery::RecoveryStore;

fn main() -> eframe::Result {
    let parsed = text3_app::cli::parse(std::env::args_os().skip(1));
    match &parsed {
        text3_app::cli::Parsed::Version
        | text3_app::cli::Parsed::Help
        | text3_app::cli::Parsed::Unknown(_)
        | text3_app::cli::Parsed::DiffNeedsTwo
        | text3_app::cli::Parsed::Register
        | text3_app::cli::Parsed::Unregister => {
            text3_platform::attach_parent_console();
        }
        text3_app::cli::Parsed::Run(_) => {}
    }
    if parsed == text3_app::cli::Parsed::Version {
        println!("Text³ {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    let cli = match &parsed {
        text3_app::cli::Parsed::Run(c) => Some(c),
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

    // First start after the rename: copy the Sanain folders (once, never
    // overwriting, old folders untouched; text3_app::migrate).
    let migrated = text3_app::migrate::run(&text3_app::migrate::default_pairs());

    let settings_dir = Settings::default_dir();
    let settings = settings_dir
        .as_deref()
        .map(Settings::load)
        .unwrap_or_default();

    // Language: TEXT3_LOCALE (or SANAIN_LOCALE), then the user's choice,
    // then the system.
    let requested: Vec<String> = text3_app::migrate::var("LOCALE")
        .into_iter()
        .chain(settings.locale.clone())
        .chain(sys_locale::get_locales())
        .collect();
    let refs: Vec<&str> = requested.iter().map(String::as_str).collect();
    let l10n = match Localizer::new(&refs) {
        Ok(l) => l,
        Err(e) => {
            // Embedded resources are checked by tests; this is a build bug.
            eprintln!("text3: localization resources are invalid: {e:?}");
            std::process::exit(2);
        }
    };

    // Help and usage errors need the localizer; print and exit.
    match &parsed {
        text3_app::cli::Parsed::Help => {
            println!(
                "{}\n{}",
                l10n.get("cli-help"),
                l10n.get("cli-help-integration")
            );
            return Ok(());
        }
        text3_app::cli::Parsed::Register | text3_app::cli::Parsed::Unregister => {
            let register = parsed == text3_app::cli::Parsed::Register;
            std::process::exit(shell_registration(&l10n, register));
        }
        text3_app::cli::Parsed::Unknown(arg) => {
            let mut args = text3_i18n::FluentArgs::new();
            args.set("option", arg.as_str());
            // Bidi isolation marks are for GUI text; terminals show them.
            let msg = l10n
                .format("cli-unknown-option", Some(&args))
                .replace(['\u{2068}', '\u{2069}'], "");
            eprintln!("text3: {msg}\n\n{}", l10n.get("cli-help"));
            std::process::exit(2);
        }
        text3_app::cli::Parsed::DiffNeedsTwo => {
            eprintln!(
                "text3: {}\n\n{}",
                l10n.get("cli-diff-needs-two"),
                l10n.get("cli-help")
            );
            std::process::exit(2);
        }
        _ => {}
    }

    // Crash recovery. TEXT3_RECOVERY_DIR (or SANAIN_RECOVERY_DIR) overrides
    // the location.
    let recovery_dir = text3_app::migrate::var_os("RECOVERY_DIR")
        .map(PathBuf::from)
        .or_else(|| text3_app::settings::portable_root().map(|r| r.join("recovery")))
        .or_else(RecoveryStore::default_dir);
    // The session file lives next to the recovery directory.
    let session_file = recovery_dir
        .as_ref()
        .and_then(|d| d.parent())
        .map(|d| d.join("session"));
    let history = recovery_dir
        .as_ref()
        .and_then(|d| d.parent())
        .and_then(|d| text3_io::history::HistoryStore::open(d.join("History")).ok());
    let store = recovery_dir.and_then(|dir| match RecoveryStore::open(&dir) {
        Ok(s) => Some(s),
        Err(e) => {
            eprintln!("text3: recovery disabled ({}): {e}", dir.display());
            None
        }
    });
    let snapshots = store
        .as_ref()
        .and_then(|s| s.list().ok())
        .map(|l| l.into_iter().filter_map(Result::ok).collect())
        .unwrap_or_default();
    let (mut workspace, load_errors, restored) = Workspace::startup(store, snapshots, &paths);
    let mut split = None;
    if paths.is_empty()
        && settings.restore_session
        && text3_app::migrate::var_os("NO_SESSION").is_none()
        && let Some(session) = session_file.as_deref().and_then(Session::load)
    {
        split = workspace.restore_session(&session);
    }

    // Before the event loop starts, so a file opened from Finder at launch
    // is not missed; the app wakes up itself once it is running.
    text3_platform::prepare_open_documents();

    let options = eframe::NativeOptions {
        renderer: renderer(),
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("Text³")
            .with_app_id("text3")
            .with_inner_size([960.0, 680.0])
            .with_min_inner_size([360.0, 240.0])
            .with_icon(app_icon()),
        ..Default::default()
    };
    eframe::run_native(
        "Text³",
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
            shell.set_split(split);
            shell.start_editor_font(&cc.egui_ctx);
            let wake = cc.egui_ctx.clone();
            text3_platform::on_opened_files(move || wake.request_repaint());
            shell.init_recent();
            if migrated > 0 {
                shell.notify(shell.tr("migration-copied"));
            }
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
    use text3_app::shellreg;
    let plain = |s: String| s.replace(['\u{2068}', '\u{2069}'], "");
    let fmt = |id: &str, k: &str, v: String| {
        let mut a = text3_i18n::FluentArgs::new();
        a.set(k, v);
        plain(l10n.format(id, Some(&a)))
    };
    if cfg!(target_os = "macos") {
        println!("{}", l10n.get("cli-register-macos"));
        return 0;
    }
    let exe = match std::env::current_exe().and_then(|p| p.canonicalize()) {
        // Windows canonical paths start with \\?\, which Explorer cannot
        // run (#115).
        Ok(p) => shellreg::plain_path(&p),
        Err(e) => {
            eprintln!(
                "text3: {}",
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
                "text3: {}",
                fmt("cli-register-failed", "reason", e.to_string())
            );
            1
        }
    }
}

/// Window and taskbar icon (packaging/icons, rendered from assets/brand/text3-mascot.svg).
fn app_icon() -> std::sync::Arc<eframe::egui::IconData> {
    let png = include_bytes!("../../../packaging/icons/text3-256.png");
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
/// moved on some drivers (#95). `TEXT3_RENDERER=wgpu` or `glow` (also as
/// `SANAIN_RENDERER`) overrides
/// it where both are built in.
fn renderer() -> eframe::Renderer {
    #[cfg(windows)]
    {
        if text3_app::migrate::var("RENDERER").as_deref() == Some("wgpu") {
            return eframe::Renderer::Wgpu;
        }
        eframe::Renderer::Glow
    }
    #[cfg(not(windows))]
    {
        eframe::Renderer::Wgpu
    }
}
