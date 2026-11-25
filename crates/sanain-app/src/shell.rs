//! Window chrome: menus, tab bar, status bar, file dialogs and
//! confirmations.

use std::collections::HashMap;
use std::path::Path;

use eframe::egui::{self, ViewportCommand};
use sanain_app::syntax::{CharSpan, SyntaxWorker};
use sanain_app::workspace::{TabId, Workspace};
use sanain_app::{Command, Editor, SaveOutcome};
use sanain_core::LineEnding;
use sanain_core::Revision;
use sanain_i18n::{FluentValue, Localizer};
use sanain_io::{LoadError, TextEncoding};
use sanain_language::Lang;

use crate::find::{FindBar, FindEvent};
use crate::menu::{self, MenuAction};
use crate::palette::{Entry, Palette};
use crate::preview;
use crate::theme;
use crate::view::{EditorView, ViewOptions};
use sanain_app::settings::{FONT_SIZE_DEFAULT, Settings, ThemeChoice};
use sanain_app::update::{Status as UpdateStatus, UpdateError};

/// An open side-by-side comparison of two tabs.
struct Compare {
    left: TabId,
    right: TabId,
    revs: (Revision, Revision),
    diff: sanain_tools::diff::Diff,
}

/// The encoding picker's purpose.
enum EncodingPick {
    /// Reopen this file with the chosen encoding.
    Reopen {
        path: std::path::PathBuf,
        /// First bytes of the file, for the preview.
        head: Vec<u8>,
        selected: usize,
        error: Option<String>,
    },
    /// Save the active document with the chosen encoding.
    Save {
        selected: usize,
        error: Option<String>,
    },
}

/// Encodings offered: Unicode first, then legacy ones.
fn encoding_choices(reopen: bool) -> Vec<TextEncoding> {
    let mut v = vec![TextEncoding::Utf8];
    if !reopen {
        v.extend([
            TextEncoding::Utf8Bom,
            TextEncoding::Utf16LeBom,
            TextEncoding::Utf16BeBom,
        ]);
    }
    v.extend(sanain_io::LEGACY.iter().map(|e| TextEncoding::Legacy(e)));
    v
}

/// Parsed data behind a structured view, for one document revision.
enum Structured {
    Csv(sanain_tools::csv::Table),
    Json(Result<sanain_tools::json::Node, sanain_tools::json::JsonError>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DataKind {
    Csv,
    Tsv,
    Json,
}

/// How a Markdown tab is shown.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MdMode {
    Source,
    Split,
    Preview,
}

/// What to do once a tab's unsaved changes are resolved.
#[derive(Debug, Clone, Copy)]
enum Then {
    /// Close this tab.
    CloseTab(TabId),
    /// Quit: close dirty tabs one by one, then the window.
    Quit,
}

/// A dialog button's follow-up, run after the dialog closes.
#[derive(Debug, Clone, Copy)]
enum Action {
    SaveThen(Then),
    Proceed(Then),
    SaveAs,
    Overwrite,
}

#[derive(Debug, Clone)]
enum Dialog {
    Unsaved(Then),
    Conflict,
    ReadOnly,
    Error(String),
    /// Confirm opening a web link in the system browser.
    OpenLink(String),
    /// A newer release exists: (version, release page).
    Update(String, String),
}

pub struct Shell {
    ws: Workspace,
    views: HashMap<TabId, EditorView>,
    l10n: Localizer,
    dialog: Option<Dialog>,
    status: Option<(String, bool)>,
    title: String,
    allow_close: bool,
    find: FindBar,
    folder: crate::folderpanel::FolderPanel,
    syntax: SyntaxWorker,
    /// Latest highlight result and the revision requested last, per tab.
    spans: HashMap<TabId, (Revision, Vec<CharSpan>)>,
    requested: HashMap<TabId, Revision>,
    session_file: Option<std::path::PathBuf>,
    settings: Settings,
    settings_dir: Option<std::path::PathBuf>,
    settings_open: bool,
    /// A running update check and whether the user asked for it.
    update: Option<(
        std::sync::mpsc::Receiver<Result<UpdateStatus, UpdateError>>,
        bool,
    )>,
    /// Command whose new shortcut is being recorded in Settings.
    recording: Option<MenuAction>,
    /// Commands that lost their shortcut to the last recorded one.
    shortcut_moved: Vec<String>,
    palette: Palette,
    compare: Option<Compare>,
    encoding_pick: Option<EncodingPick>,
    structured: std::collections::HashSet<TabId>,
    structured_cache: HashMap<TabId, (Revision, Option<char>, Structured)>,
    csv_delimiter: HashMap<TabId, char>,
    compare_pick: bool,
    md_modes: HashMap<TabId, MdMode>,
    md_cache: HashMap<TabId, (Revision, std::sync::Arc<Vec<sanain_markdown::Block>>)>,
    /// Caret line the preview was last scrolled to, per tab.
    md_synced: HashMap<TabId, usize>,
    /// Heading anchor to scroll the preview to, per tab.
    md_anchor: HashMap<TabId, String>,
    images: std::cell::RefCell<crate::imagecache::ImageCache>,
    applied_theme: Option<ThemeChoice>,
    /// Set once the session was recorded for the current quit attempt.
    quit_recorded: bool,
    #[cfg(target_os = "macos")]
    native_menu: Option<menu::NativeMenu>,
}

impl Shell {
    /// Install the native menu bar where supported. Set
    /// `SANAIN_IN_WINDOW_MENU=1` to force the in-window menu.
    pub fn install_menu(&mut self, ctx: &egui::Context) {
        #[cfg(target_os = "macos")]
        if std::env::var_os("SANAIN_IN_WINDOW_MENU").is_none() {
            let ctx = ctx.clone();
            let label = |id: &str| self.t(id, &[]);
            self.native_menu =
                menu::NativeMenu::install(&label, &self.settings.keymap, move || {
                    ctx.request_repaint()
                });
        }
        #[cfg(not(target_os = "macos"))]
        let _ = ctx;
    }

    fn has_native_menu(&self) -> bool {
        #[cfg(target_os = "macos")]
        return self.native_menu.is_some();
        #[cfg(not(target_os = "macos"))]
        false
    }

    pub fn new(
        ws: Workspace,
        l10n: Localizer,
        load_errors: Vec<(std::path::PathBuf, LoadError)>,
        restored: usize,
        session_file: Option<std::path::PathBuf>,
        settings: Settings,
        settings_dir: Option<std::path::PathBuf>,
    ) -> Self {
        let status = (restored > 0).then(|| (l10n.get("status-recovered"), false));
        let mut shell = Self {
            ws,
            views: HashMap::new(),
            l10n,
            dialog: None,
            status,
            title: String::new(),
            allow_close: false,
            find: FindBar::default(),
            folder: Default::default(),
            syntax: SyntaxWorker::start(),
            spans: HashMap::new(),
            requested: HashMap::new(),
            session_file,
            settings,
            settings_dir,
            settings_open: false,
            update: None,
            recording: None,
            shortcut_moved: Vec::new(),
            palette: Palette::default(),
            compare: None,
            encoding_pick: None,
            structured: Default::default(),
            structured_cache: HashMap::new(),
            csv_delimiter: HashMap::new(),
            compare_pick: false,
            md_modes: HashMap::new(),
            md_cache: HashMap::new(),
            md_synced: HashMap::new(),
            md_anchor: HashMap::new(),
            images: Default::default(),
            applied_theme: None,
            quit_recorded: false,
            #[cfg(target_os = "macos")]
            native_menu: None,
        };
        // Files that are not UTF-8/UTF-16 go to the encoding picker; other
        // failures are reported together.
        let mut messages = Vec::new();
        for (path, e) in &load_errors {
            match e {
                LoadError::Decode { .. } if shell.encoding_pick.is_none() => {
                    shell.start_reopen(path)
                }
                _ => messages.push(describe_load_error(&shell.l10n, path, e)),
            }
        }
        if !messages.is_empty() {
            shell.dialog = Some(Dialog::Error(messages.join("\n")));
        }
        shell.focus_active();
        shell
    }

    fn t(&self, id: &str, args: &[(&str, FluentValue<'_>)]) -> String {
        tr(&self.l10n, id, args)
    }

    /// Move to the next or previous match and select it.
    fn find_step(&mut self, forward: bool) {
        let id = self.ws.active().id;
        self.find.refresh(id, &self.ws.active().editor);
        let sel = self.editor().selections().primary().range();
        let from = if forward { sel.end } else { sel.start };
        if let Some(i) = self.find.matches.next_from(from, forward) {
            let r = self.find.matches.ranges[i].clone();
            self.editor_mut().select(r);
            self.views.entry(id).or_default().reveal();
        }
    }

    fn find_event(&mut self, event: FindEvent) {
        let id = self.ws.active().id;
        match event {
            FindEvent::Next => self.find_step(true),
            FindEvent::Prev => self.find_step(false),
            FindEvent::Close => {
                self.find.open = false;
                self.focus_active();
            }
            FindEvent::Replace => {
                self.find.refresh(id, &self.ws.active().editor);
                let with = self.find.replacement.clone();
                if let Some(m) = self.find.matcher().cloned() {
                    self.editor_mut().replace_selected(&m, &with);
                }
                self.find_step(true);
            }
            FindEvent::ReplaceAll => {
                self.find.refresh(id, &self.ws.active().editor);
                let with = self.find.replacement.clone();
                if let Some(m) = self.find.matcher().cloned() {
                    let n = self.editor_mut().replace_all(&m, &with) as i64;
                    let msg = self.t("status-replaced", &[("count", n.into())]);
                    self.status = Some((msg, false));
                }
            }
        }
    }

    fn editor(&self) -> &Editor {
        &self.ws.active().editor
    }

    fn editor_mut(&mut self) -> &mut Editor {
        &mut self.ws.active_mut().editor
    }

    fn name_of(&self, ed: &Editor) -> String {
        ed.file_name().unwrap_or_else(|| self.t("untitled", &[]))
    }

    fn display_name(&self) -> String {
        self.name_of(self.editor())
    }

    fn focus_active(&mut self) {
        let id = self.ws.active().id;
        self.views.entry(id).or_default().request_focus();
    }

    fn save(&mut self) -> bool {
        let outcome = self.editor_mut().save();
        self.handle_save(outcome)
    }

    fn save_as(&mut self) -> bool {
        let mut dialog = rfd::FileDialog::new().set_file_name(self.display_name());
        if let Some(dir) = self.editor().file().and_then(|f| f.path.parent()) {
            dialog = dialog.set_directory(dir);
        }
        match dialog.save_file() {
            Some(path) => {
                let outcome = self.editor_mut().save_as(&path);
                self.handle_save(outcome)
            }
            None => false,
        }
    }

    fn handle_save(&mut self, outcome: SaveOutcome) -> bool {
        let name = self.display_name();
        match outcome {
            SaveOutcome::Saved => {
                self.status = Some((self.t("status-saved", &[("name", name.into())]), false));
                return true;
            }
            SaveOutcome::NeedsPath => return self.save_as(),
            SaveOutcome::Conflict => self.dialog = Some(Dialog::Conflict),
            SaveOutcome::ReadOnly => self.dialog = Some(Dialog::ReadOnly),
            SaveOutcome::Failed(e) => {
                let msg = self.t(
                    "error-save",
                    &[("name", name.into()), ("reason", e.to_string().into())],
                );
                self.dialog = Some(Dialog::Error(msg));
            }
        }
        false
    }

    /// Close a tab, asking first if it has unsaved changes.
    fn request_close_tab(&mut self, id: TabId, ctx: &egui::Context) {
        let Some(i) = self.ws.position(id) else {
            return;
        };
        if self.ws.tabs()[i].editor.document().is_dirty() {
            self.ws.activate(i);
            self.dialog = Some(Dialog::Unsaved(Then::CloseTab(id)));
        } else {
            self.proceed(Then::CloseTab(id), ctx);
        }
    }

    /// Quit, asking about each unsaved tab in turn.
    fn request_quit(&mut self, ctx: &egui::Context) {
        // Record open files before the quit flow starts closing tabs.
        if !std::mem::replace(&mut self.quit_recorded, true)
            && let Some(f) = &self.session_file
            && let Err(e) = self.ws.session().save(f)
        {
            eprintln!("sanain: could not save session: {e}");
        }
        match self.ws.first_dirty() {
            Some(i) => {
                self.ws.activate(i);
                self.dialog = Some(Dialog::Unsaved(Then::Quit));
            }
            None => {
                self.ws.flush();
                self.allow_close = true;
                ctx.send_viewport_cmd(ViewportCommand::Close);
            }
        }
    }

    /// Continue after the active tab was saved or its changes declined.
    fn proceed(&mut self, then: Then, ctx: &egui::Context) {
        match then {
            Then::CloseTab(id) => {
                if let Some(i) = self.ws.position(id) {
                    self.ws.close(i);
                    self.views.remove(&id);
                    self.spans.remove(&id);
                    self.requested.remove(&id);
                    self.md_cache.remove(&id);
                    self.md_modes.remove(&id);
                    self.md_synced.remove(&id);
                    self.md_anchor.remove(&id);
                    self.structured.remove(&id);
                    self.structured_cache.remove(&id);
                    self.csv_delimiter.remove(&id);
                }
            }
            Then::Quit => {
                let id = self.ws.active().id;
                let i = self.ws.active_index();
                self.ws.close(i);
                self.views.remove(&id);
                self.request_quit(ctx);
                return;
            }
        }
        self.status = None;
        self.focus_active();
    }

    fn open_dialog(&mut self) {
        let Some(paths) = rfd::FileDialog::new().pick_files() else {
            return;
        };
        for p in paths {
            self.open(&p);
        }
    }

    fn open(&mut self, path: &Path) {
        match self.ws.open(path) {
            Ok(_) => {
                self.status = None;
                self.focus_active();
            }
            // Not valid UTF-8/UTF-16: let the user pick an encoding instead
            // of failing.
            Err(LoadError::Decode { .. }) => self.start_reopen(path),
            Err(e) => self.dialog = Some(Dialog::Error(describe_load_error(&self.l10n, path, &e))),
        }
    }

    fn save_settings(&mut self) {
        if let Some(dir) = &self.settings_dir
            && let Err(e) = self.settings.save(dir)
        {
            let msg = self.t("settings-save-failed", &[("reason", e.to_string().into())]);
            self.status = Some((msg, true));
        }
    }

    fn apply_theme(&mut self, ctx: &egui::Context) {
        if self.applied_theme == Some(self.settings.theme) {
            return;
        }
        theme::set_high_contrast(ctx, self.settings.theme == ThemeChoice::HighContrast);
        ctx.set_theme(match self.settings.theme {
            ThemeChoice::System => egui::ThemePreference::System,
            ThemeChoice::Light => egui::ThemePreference::Light,
            ThemeChoice::Dark | ThemeChoice::HighContrast => egui::ThemePreference::Dark,
        });
        self.applied_theme = Some(self.settings.theme);
    }

    fn settings_window(&mut self, ctx: &egui::Context) {
        if !self.settings_open {
            return;
        }
        let before = self.settings.clone();
        let mut open = true;
        let title = self.t("settings-title", &[]);
        let l = |id: &str| tr(&self.l10n, id, &[]);
        let s = &mut self.settings;
        let recording = &mut self.recording;
        let moved = &mut self.shortcut_moved;
        let was_recording = recording.is_some();
        egui::Window::new(title)
            .id(egui::Id::unique("sanain-settings"))
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .default_width(380.0)
            .show(ctx, |ui| {
                egui::Grid::new("settings-grid")
                    .num_columns(2)
                    .spacing([16.0, 10.0])
                    .show(ui, |ui| {
                        ui.label(l("settings-font-size"));
                        ui.add(
                            egui::DragValue::new(&mut s.font_size)
                                .range(
                                    sanain_app::settings::FONT_SIZE_MIN
                                        ..=sanain_app::settings::FONT_SIZE_MAX,
                                )
                                .speed(0.25)
                                .suffix(" pt"),
                        );
                        ui.end_row();
                        ui.label(l("settings-tab-width"));
                        ui.add(egui::DragValue::new(&mut s.tab_width).range(1..=16));
                        ui.end_row();
                        ui.label("");
                        ui.checkbox(&mut s.insert_spaces, l("settings-insert-spaces"));
                        ui.end_row();
                        ui.label(l("settings-theme"));
                        ui.horizontal(|ui| {
                            for (v, id) in [
                                (ThemeChoice::System, "settings-theme-system"),
                                (ThemeChoice::Light, "settings-theme-light"),
                                (ThemeChoice::Dark, "settings-theme-dark"),
                                (ThemeChoice::HighContrast, "settings-theme-high-contrast"),
                            ] {
                                ui.radio_value(&mut s.theme, v, l(id));
                            }
                        });
                        ui.end_row();
                        ui.label("");
                        ui.checkbox(&mut s.restore_session, l("settings-restore-session"));
                        ui.end_row();
                        ui.label("");
                        ui.checkbox(&mut s.check_updates, l("settings-check-updates"))
                            .on_hover_text(l("settings-check-updates-detail"));
                        ui.end_row();
                        ui.label(l("settings-language"));
                        let current = s
                            .locale
                            .as_deref()
                            .and_then(|t| sanain_i18n::LANGUAGES.iter().find(|(tag, _)| *tag == t))
                            .map_or_else(
                                || l("settings-language-system"),
                                |(_, name)| (*name).to_owned(),
                            );
                        egui::ComboBox::from_id_salt("settings-language")
                            .selected_text(current)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut s.locale,
                                    None,
                                    l("settings-language-system"),
                                );
                                // Language names in their own language.
                                for (tag, name) in sanain_i18n::LANGUAGES {
                                    ui.selectable_value(
                                        &mut s.locale,
                                        Some((*tag).to_owned()),
                                        *name,
                                    );
                                }
                            });
                        ui.end_row();
                    });
                ui.add_space(4.0);
                ui.weak(l("settings-language-note"));
                ui.add_space(8.0);
                egui::CollapsingHeader::new(l("settings-shortcuts"))
                    .id_salt("settings-shortcuts")
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(l("settings-keymap-preset"));
                            use sanain_app::keymap::Preset;
                            for (p, id) in [
                                (Preset::Default, "settings-keymap-default"),
                                (Preset::NotepadPlusPlus, "settings-keymap-notepadpp"),
                            ] {
                                ui.radio_value(&mut s.keymap.preset, p, l(id));
                            }
                        });
                        egui::ScrollArea::vertical()
                            .max_height(260.0)
                            .show(ui, |ui| {
                                egui::Grid::new("shortcut-grid")
                                    .num_columns(3)
                                    .striped(true)
                                    .show(ui, |ui| {
                                        for a in MenuAction::ALL {
                                            ui.label(
                                                l(a.label_id()).trim_end_matches('…').to_owned(),
                                            );
                                            let text = if *recording == Some(a) {
                                                l("settings-shortcut-press")
                                            } else {
                                                a.shortcut(&s.keymap)
                                                    .map(|k| ctx.format_shortcut(&k))
                                                    .unwrap_or_default()
                                            };
                                            ui.monospace(text);
                                            ui.horizontal(|ui| {
                                                if ui
                                                    .small_button(l("settings-shortcut-change"))
                                                    .clicked()
                                                {
                                                    *recording = Some(a);
                                                }
                                                if s.keymap.custom.contains_key(a.command_id())
                                                    && ui
                                                        .small_button(l("settings-shortcut-reset"))
                                                        .clicked()
                                                {
                                                    s.keymap.custom.remove(a.command_id());
                                                }
                                            });
                                            ui.end_row();
                                        }
                                    });
                            });
                    });
                if let Some(a) = *recording {
                    // Capture the next chord; Escape cancels.
                    let pressed = ui.input(|i| {
                        i.events.iter().find_map(|e| match e {
                            egui::Event::Key {
                                key,
                                pressed: true,
                                modifiers,
                                ..
                            } => Some((*key, *modifiers)),
                            _ => None,
                        })
                    });
                    match pressed {
                        Some((egui::Key::Escape, _)) => *recording = None,
                        Some((key, m)) => {
                            if let Some(chord) = menu::chord_from_press(key, m) {
                                let clash: Vec<String> = MenuAction::ALL
                                    .into_iter()
                                    .filter(|o| {
                                        *o != a
                                            && o.shortcut(&s.keymap) == menu::chord_to_egui(&chord)
                                    })
                                    .map(|o| l(o.label_id()))
                                    .collect();
                                // The other command loses this shortcut.
                                for o in MenuAction::ALL {
                                    if o != a
                                        && o.shortcut(&s.keymap) == menu::chord_to_egui(&chord)
                                    {
                                        s.keymap.custom.insert(o.command_id().to_owned(), None);
                                    }
                                }
                                s.keymap
                                    .custom
                                    .insert(a.command_id().to_owned(), Some(chord));
                                *recording = None;
                                *moved = clash;
                            }
                        }
                        None => {}
                    }
                }
                if !moved.is_empty() {
                    ui.colored_label(
                        theme::palette(ui.ctx()).warning,
                        format!("{} {}", l("settings-shortcut-moved"), moved.join(", ")),
                    );
                }
            });
        if self.settings != before {
            self.save_settings();
        }
        // Keep the native menu's shortcuts in step; clear them while
        // recording so the menu does not swallow the key press.
        #[cfg(target_os = "macos")]
        if let Some(m) = &self.native_menu {
            if self.recording.is_some() && !was_recording {
                m.clear_shortcuts();
            } else if (was_recording && self.recording.is_none())
                || self.settings.keymap != before.keymap
            {
                m.apply_keymap(&self.settings.keymap);
            }
        }
        #[cfg(not(target_os = "macos"))]
        let _ = was_recording;
        if !open {
            self.settings_open = false;
            self.focus_active();
        }
    }

    /// Run a text tool on the selection (or whole document) as one edit.
    fn run_tool(&mut self, action: MenuAction) {
        use sanain_tools::{Newline, json};
        let indent = self.settings.indent();
        let newline = match self.editor().line_ending() {
            LineEnding::CrLf => "\r\n",
            LineEnding::Cr => "\r",
            _ => "\n",
        };
        let ok = |s: String| Ok::<_, json::JsonError>(s);
        let result = match action {
            MenuAction::JsonValidate => {
                let range = self.editor().tool_range();
                let text: String = self.editor().document().rope().slice(range.clone()).into();
                json::validate_json(&text).map(|()| false)
            }
            MenuAction::JsonFormat => self
                .editor_mut()
                .transform(|t| json::format(t, &indent, newline)),
            MenuAction::JsonMinify => self.editor_mut().transform(json::minify),
            MenuAction::SortLines => self
                .editor_mut()
                .transform(|t| ok(sanain_tools::sort_lines(t, false))),
            MenuAction::SortLinesDesc => self
                .editor_mut()
                .transform(|t| ok(sanain_tools::sort_lines(t, true))),
            MenuAction::UniqueLines => self
                .editor_mut()
                .transform(|t| ok(sanain_tools::unique_lines(t))),
            MenuAction::TrimTrailing => self
                .editor_mut()
                .transform(|t| ok(sanain_tools::trim_trailing_whitespace(t))),
            MenuAction::UpperCase => self
                .editor_mut()
                .transform(|t| ok(sanain_tools::upper_case(t))),
            MenuAction::LowerCase => self
                .editor_mut()
                .transform(|t| ok(sanain_tools::lower_case(t))),
            MenuAction::ToLf => self
                .editor_mut()
                .transform(|t| ok(sanain_tools::convert_newlines(t, Newline::Lf))),
            MenuAction::ToCrLf => self
                .editor_mut()
                .transform(|t| ok(sanain_tools::convert_newlines(t, Newline::CrLf))),
            _ => return,
        };
        match result {
            Ok(changed) => {
                let id = match (action, changed) {
                    (MenuAction::JsonValidate, _) => "tools-json-valid",
                    (_, true) => "tools-done",
                    (_, false) => "tools-no-change",
                };
                self.status = Some((self.t(id, &[]), false));
            }
            Err(e) => {
                // Put the caret on the problem.
                let range = self.editor().tool_range();
                let text: String = self.editor().document().rope().slice(range.clone()).into();
                let at = range.start + text[..e.offset.min(text.len())].chars().count();
                self.editor_mut().select(at..at);
                let id = self.ws.active().id;
                self.views.entry(id).or_default().reveal();
                let msg = self.t(
                    "tools-json-error",
                    &[
                        ("line", (e.line as i64).into()),
                        ("column", (e.column as i64).into()),
                        ("reason", self.t(&json_reason_id(e.message), &[]).into()),
                    ],
                );
                self.status = Some((msg, true));
            }
        }
    }

    fn start_reopen(&mut self, path: &std::path::Path) {
        use std::io::Read;
        let mut head = Vec::new();
        let read = std::fs::File::open(path).and_then(|f| f.take(64 * 1024).read_to_end(&mut head));
        match read {
            Ok(_) => {
                self.encoding_pick = Some(EncodingPick::Reopen {
                    path: path.to_owned(),
                    head,
                    selected: 1, // Windows-1252, the most common legacy case
                    error: None,
                })
            }
            Err(e) => self.dialog = Some(Dialog::Error(e.to_string())),
        }
    }

    fn encoding_label(&self, e: TextEncoding) -> String {
        match e {
            TextEncoding::Legacy(enc) => {
                let id = format!("enc-{}", enc.name().to_ascii_lowercase());
                format!("{} ({})", self.t(&id, &[]), enc.name())
            }
            other => other.name().to_owned(),
        }
    }

    fn encoding_window(&mut self, ctx: &egui::Context) {
        let Some(pick) = &self.encoding_pick else {
            return;
        };
        let reopen = matches!(pick, EncodingPick::Reopen { .. });
        let choices = encoding_choices(reopen);
        let labels: Vec<String> = choices.iter().map(|e| self.encoding_label(*e)).collect();
        let title = if reopen {
            self.t("encoding-reopen-title", &[])
        } else {
            self.t("encoding-save-title", &[])
        };
        let (ok_label, cancel_label) = (
            self.t(if reopen { "button-open" } else { "button-save" }, &[]),
            self.t("button-cancel", &[]),
        );
        let p = theme::palette(ctx);
        let mut open = true;
        let mut confirm = false;
        let mut cancel = false;
        let Some(pick) = &self.encoding_pick else {
            return;
        };
        let preview = match pick {
            EncodingPick::Reopen { head, selected, .. } => {
                let enc = choices
                    .get(*selected)
                    .copied()
                    .unwrap_or(TextEncoding::Utf8);
                let text = match enc {
                    TextEncoding::Legacy(e) => e.decode_without_bom_handling(head).0.into_owned(),
                    _ => String::from_utf8_lossy(head).into_owned(),
                };
                Some(text.chars().take(1200).collect::<String>())
            }
            EncodingPick::Save { .. } => None,
        };
        let (selected, error) = match &mut self.encoding_pick {
            Some(
                EncodingPick::Reopen {
                    selected, error, ..
                }
                | EncodingPick::Save { selected, error },
            ) => (selected, error),
            None => return,
        };
        egui::Window::new(title)
            .id(egui::Id::unique("sanain-encoding"))
            .open(&mut open)
            .collapsible(false)
            .default_width(520.0)
            .show(ctx, |ui| {
                egui::ComboBox::from_id_salt("encoding-choice")
                    .width(360.0)
                    .selected_text(labels.get(*selected).cloned().unwrap_or_default())
                    .show_ui(ui, |ui| {
                        for (i, l) in labels.iter().enumerate() {
                            if ui.selectable_value(selected, i, l).changed() {
                                *error = None;
                            }
                        }
                    });
                if let Some(text) = &preview {
                    ui.add_space(6.0);
                    egui::Frame::NONE
                        .fill(p.canvas)
                        .stroke(egui::Stroke::new(1.0, p.border))
                        .inner_margin(egui::Margin::same(8))
                        .show(ui, |ui| {
                            egui::ScrollArea::vertical()
                                .max_height(220.0)
                                .show(ui, |ui| {
                                    ui.set_width(ui.available_width());
                                    ui.label(egui::RichText::new(text).monospace());
                                });
                        });
                }
                if let Some(err) = error.as_ref() {
                    ui.colored_label(p.error, err);
                }
                ui.add_space(6.0);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(&ok_label).clicked() {
                        confirm = true;
                    }
                    if ui.button(&cancel_label).clicked() {
                        cancel = true;
                    }
                });
            });
        if !open || cancel {
            self.encoding_pick = None;
            return;
        }
        if !confirm {
            return;
        }
        let Some(pick) = self.encoding_pick.take() else {
            return;
        };
        match pick {
            EncodingPick::Reopen {
                path,
                head,
                selected,
                ..
            } => {
                let enc = choices.get(selected).copied().unwrap_or(TextEncoding::Utf8);
                match self.ws.open_as(&path, enc) {
                    Ok(_) => self.focus_active(),
                    Err(LoadError::Decode { source, .. }) => {
                        let msg = self.t(
                            "encoding-not-exact",
                            &[("reason", source.to_string().into())],
                        );
                        self.encoding_pick = Some(EncodingPick::Reopen {
                            path,
                            head,
                            selected,
                            error: Some(msg),
                        });
                    }
                    Err(e) => {
                        self.dialog =
                            Some(Dialog::Error(describe_load_error(&self.l10n, &path, &e)))
                    }
                }
            }
            EncodingPick::Save { selected, .. } => {
                let enc = choices.get(selected).copied().unwrap_or(TextEncoding::Utf8);
                match self.editor_mut().save_with_encoding(enc) {
                    SaveOutcome::Failed(sanain_io::SaveError::Unencodable { source, .. }) => {
                        let msg = self.t(
                            "encoding-unencodable",
                            &[
                                ("character", source.character.to_string().into()),
                                ("encoding", source.encoding.into()),
                            ],
                        );
                        // Show where the problem is.
                        self.editor_mut().select(source.index..source.index + 1);
                        self.encoding_pick = Some(EncodingPick::Save {
                            selected,
                            error: Some(msg),
                        });
                    }
                    outcome => {
                        self.handle_save(outcome);
                    }
                }
            }
        }
    }

    /// Ask GitHub for the latest release in the background. `manual`
    /// checks report every outcome; automatic ones only a new version.
    fn start_update_check(&mut self, ctx: &egui::Context, manual: bool) {
        if self.update.is_some() {
            return;
        }
        let (tx, rx) = std::sync::mpsc::channel();
        let ctx = ctx.clone();
        let spawned = std::thread::Builder::new()
            .name("sanain-update".into())
            .spawn(move || {
                let _ = tx.send(sanain_app::update::check(env!("CARGO_PKG_VERSION")));
                ctx.request_repaint();
            });
        if spawned.is_ok() {
            self.update = Some((rx, manual));
            if manual {
                self.status = Some((self.t("update-checking", &[]), false));
            }
        }
    }

    fn poll_update(&mut self) {
        let Some((rx, manual)) = &self.update else {
            return;
        };
        let Ok(result) = rx.try_recv() else { return };
        let manual = *manual;
        self.update = None;
        match result {
            Ok(UpdateStatus::Available(r)) => {
                let version = format!(
                    "{}.{}.{}",
                    r.version.major, r.version.minor, r.version.patch
                );
                self.dialog = Some(Dialog::Update(version, r.page));
            }
            Ok(UpdateStatus::UpToDate) if manual => {
                let msg = self.t(
                    "update-up-to-date",
                    &[("version", env!("CARGO_PKG_VERSION").into())],
                );
                self.status = Some((msg, false));
            }
            Ok(UpdateStatus::NoReleases) if manual => {
                self.status = Some((self.t("update-no-releases", &[]), false));
            }
            Err(e) if manual => {
                let reason = match e {
                    UpdateError::Network(m) => m,
                    UpdateError::Invalid => self.t("update-invalid", &[]),
                };
                self.status = Some((self.t("update-failed", &[("reason", reason.into())]), true));
            }
            _ => {}
        }
    }

    /// Run the daily automatic check if the user enabled it.
    pub fn auto_update_check(&mut self, ctx: &egui::Context) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_secs());
        if self.settings.check_updates
            && now.saturating_sub(self.settings.last_update_check) >= 24 * 3600
        {
            self.settings.last_update_check = now;
            self.save_settings();
            self.start_update_check(ctx, false);
        }
    }

    fn data_kind(ed: &Editor) -> Option<DataKind> {
        let ext = ed.file()?.path.extension()?.to_str()?.to_ascii_lowercase();
        match ext.as_str() {
            "csv" => Some(DataKind::Csv),
            "tsv" | "tab" => Some(DataKind::Tsv),
            "json" | "geojson" => Some(DataKind::Json),
            _ => None,
        }
    }

    /// Draw the structured view if enabled for the active tab.
    fn structured_area(&mut self, ui: &mut egui::Ui) -> bool {
        let id = self.ws.active().id;
        let Some(kind) = Self::data_kind(self.editor()).filter(|_| self.structured.contains(&id))
        else {
            return false;
        };
        let rev = self.editor().document().revision();
        let override_delim = self.csv_delimiter.get(&id).copied();
        let fresh = self
            .structured_cache
            .get(&id)
            .is_some_and(|(r, d, _)| *r == rev && *d == override_delim);
        if !fresh {
            let text = self.editor().document().text();
            let data = match kind {
                DataKind::Json => Structured::Json(sanain_tools::json::tree(&text)),
                DataKind::Csv | DataKind::Tsv => {
                    let hint = override_delim.or((kind == DataKind::Tsv).then_some('\t'));
                    let d = sanain_tools::csv::detect_delimiter(&text, hint);
                    Structured::Csv(sanain_tools::csv::parse(&text, d))
                }
            };
            self.structured_cache
                .insert(id, (rev, override_delim, data));
        }
        let p = theme::palette(ui.ctx());
        let size = self.settings.font_size;
        let l10n = &self.l10n;
        let Some((_, _, data)) = self.structured_cache.get(&id) else {
            return false;
        };
        let mut pick = None;
        match data {
            Structured::Csv(table) => {
                egui::Frame::NONE
                    .fill(p.surface)
                    .inner_margin(egui::Margin::symmetric(8, 6))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(tr(l10n, "csv-delimiter", &[]));
                            for (c, name) in [(',', ","), (';', ";"), ('\t', "Tab"), ('|', "|")] {
                                if ui.selectable_label(table.delimiter == c, name).clicked() {
                                    pick = Some(c);
                                }
                            }
                            // The first row is shown as the header.
                            let rows = table.rows.len().saturating_sub(1) as i64;
                            ui.colored_label(
                                p.muted,
                                tr(l10n, "csv-rows", &[("count", rows.into())]),
                            );
                            if let Some(line) = table.unterminated_quote {
                                let msg =
                                    tr(l10n, "csv-unterminated", &[("line", (line as i64).into())]);
                                ui.colored_label(p.error, msg);
                            }
                            if table.truncated {
                                ui.colored_label(p.muted, tr(l10n, "csv-truncated", &[]));
                            }
                        });
                    });
                crate::structured::csv(ui, table, p, size, true);
            }
            Structured::Json(Ok(root)) => {
                let more = |n: usize| tr(l10n, "json-more", &[("count", (n as i64).into())]);
                crate::structured::json(ui, root, p, &more);
            }
            Structured::Json(Err(e)) => {
                let msg = tr(
                    l10n,
                    "tools-json-error",
                    &[
                        ("line", (e.line as i64).into()),
                        ("column", (e.column as i64).into()),
                        ("reason", tr(l10n, &json_reason_id(e.message), &[]).into()),
                    ],
                );
                ui.colored_label(p.error, msg);
            }
        }
        if let Some(c) = pick {
            self.csv_delimiter.insert(id, c);
        }
        true
    }

    fn start_compare(&mut self, right: TabId) {
        self.compare = Some(Compare {
            left: self.ws.active().id,
            right,
            revs: (Revision(u64::MAX), Revision(u64::MAX)),
            diff: Default::default(),
        });
    }

    fn tab_text(&self, id: TabId) -> Option<(Revision, String)> {
        let i = self.ws.position(id)?;
        let d = self.ws.tabs()[i].editor.document();
        Some((d.revision(), d.text()))
    }

    fn compare_picker(&mut self, ctx: &egui::Context) {
        if !self.compare_pick {
            return;
        }
        let active = self.ws.active().id;
        let others: Vec<(TabId, String)> = self
            .ws
            .tabs()
            .iter()
            .filter(|t| t.id != active)
            .map(|t| (t.id, self.name_of(&t.editor)))
            .collect();
        let mut open = true;
        let mut chosen = None;
        egui::Window::new(self.t(
            "compare-pick-title",
            &[("name", self.display_name().into())],
        ))
        .id(egui::Id::unique("sanain-compare-pick"))
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            for (id, name) in &others {
                if ui.button(name).clicked() {
                    chosen = Some(*id);
                }
            }
        });
        if let Some(id) = chosen {
            self.start_compare(id);
            self.compare_pick = false;
        } else if !open {
            self.compare_pick = false;
        }
    }

    /// Draw the comparison if the active tab is its left side. Returns
    /// whether it was drawn.
    fn compare_area(&mut self, ui: &mut egui::Ui) -> bool {
        let Some(c) = &self.compare else { return false };
        if c.left != self.ws.active().id {
            return false;
        }
        let (left, right) = (c.left, c.right);
        let (Some((lrev, ltext)), Some((rrev, rtext))) =
            (self.tab_text(left), self.tab_text(right))
        else {
            self.compare = None;
            return false;
        };
        if let Some(c) = &mut self.compare
            && c.revs != (lrev, rrev)
        {
            c.diff = sanain_tools::diff::diff_lines(&ltext, &rtext);
            c.revs = (lrev, rrev);
        }
        let Some(c) = &self.compare else { return false };
        let p = theme::palette(ui.ctx());
        let lname = self.name_of(&self.ws.tabs()[self.ws.position(left).unwrap_or(0)].editor);
        let rname = self.name_of(&self.ws.tabs()[self.ws.position(right).unwrap_or(0)].editor);
        let changes = c.diff.hunks.len() as i64;
        let mut close = false;
        egui::Frame::NONE
            .fill(p.surface)
            .inner_margin(egui::Margin::symmetric(8, 6))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(format!("{lname}  ↔  {rname}"));
                    ui.colored_label(
                        p.muted,
                        self.t("compare-count", &[("count", changes.into())]),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(self.t("compare-close", &[])).clicked() {
                            close = true;
                        }
                    });
                });
            });
        let labels = crate::diffview::Labels {
            to_left: self.t("compare-to-left", &[]),
            to_right: self.t("compare-to-right", &[]),
        };
        let action = crate::diffview::show(
            ui,
            &c.diff,
            &ltext,
            &rtext,
            p,
            self.settings.font_size,
            &labels,
        );
        if close || ui.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.compare = None;
            return true;
        }
        if let Some(a) = action {
            self.apply_hunk(a, &ltext, &rtext);
        }
        true
    }

    /// Copy one hunk across, unless either side changed since the diff was
    /// computed (then the comparison refreshes instead).
    fn apply_hunk(&mut self, a: crate::diffview::ApplyDir, ltext: &str, rtext: &str) {
        use crate::diffview::ApplyDir;
        let Some(c) = &self.compare else { return };
        let (left, right) = (c.left, c.right);
        let now = (
            self.tab_text(left).map(|t| t.0),
            self.tab_text(right).map(|t| t.0),
        );
        if now != (Some(c.revs.0), Some(c.revs.1)) {
            self.status = Some((self.t("compare-stale", &[]), true));
            return;
        }
        let (target, bytes, text, target_text) = match a {
            ApplyDir::ToLeft(h) => {
                let Some(h) = c.diff.hunks.get(h) else { return };
                (
                    left,
                    h.left_bytes.clone(),
                    &rtext[h.right_bytes.clone()],
                    ltext,
                )
            }
            ApplyDir::ToRight(h) => {
                let Some(h) = c.diff.hunks.get(h) else { return };
                (
                    right,
                    h.right_bytes.clone(),
                    &ltext[h.left_bytes.clone()],
                    rtext,
                )
            }
        };
        let start = target_text[..bytes.start].chars().count();
        let end = start + target_text[bytes.clone()].chars().count();
        let text = text.to_owned();
        if let Some(i) = self.ws.position(target) {
            let current = self.ws.active_index();
            self.ws.activate(i);
            self.editor_mut().replace_range(start..end, &text);
            self.ws.activate(current);
        }
    }

    fn is_markdown(ed: &Editor) -> bool {
        ed.file()
            .and_then(|f| f.path.extension())
            .and_then(|e| e.to_str())
            .is_some_and(|e| {
                matches!(
                    e.to_ascii_lowercase().as_str(),
                    "md" | "markdown" | "mdown" | "mkd"
                )
            })
    }

    fn md_mode(&self) -> MdMode {
        if Self::is_markdown(self.editor()) {
            self.md_modes
                .get(&self.ws.active().id)
                .copied()
                .unwrap_or(MdMode::Source)
        } else {
            MdMode::Source
        }
    }

    fn editor_area(&mut self, ui: &mut egui::Ui) {
        let name = self.t("editor-accessible-name", &[]);
        let interactive = self.dialog.is_none();
        let id = self.ws.active().id;
        let view = self.views.entry(id).or_default();
        let editor = &mut self.ws.active_mut().editor;
        // Spans from an older revision are kept until fresh ones arrive;
        // a frame of slightly shifted colour beats flicker.
        let syntax: &[CharSpan] = self.spans.get(&id).map_or(&[], |(_, s)| s);
        let opts = ViewOptions {
            font_size: self.settings.font_size,
            indent: self.settings.indent(),
            tab_width: usize::from(self.settings.tab_width),
        };
        let highlights: &[std::ops::Range<usize>] = if self.find.open {
            &self.find.matches.ranges
        } else {
            &[]
        };
        ui.push_id(id, |ui| {
            ui.add_enabled_ui(interactive, |ui| {
                if view.show(ui, editor, &name, highlights, syntax, &opts) {
                    self.status = None;
                }
            });
        });
    }

    fn preview_area(&mut self, ui: &mut egui::Ui) {
        let tab = self.ws.active();
        let (id, rev) = (tab.id, tab.editor.document().revision());
        let doc = tab.editor.document();
        let p = theme::palette(ui.ctx());
        if doc.rope().len_bytes() > preview::MAX_PREVIEW_BYTES {
            ui.colored_label(p.muted, self.t("preview-too-large", &[]));
            return;
        }
        let fresh = self.md_cache.get(&id).is_some_and(|(r, _)| *r == rev);
        if !fresh {
            let blocks = sanain_markdown::parse(&doc.text());
            self.md_cache.insert(id, (rev, std::sync::Arc::new(blocks)));
        }
        let Some((_, blocks)) = self.md_cache.get(&id) else {
            return;
        };
        let blocks = blocks.clone();
        let head = self.editor().selections().primary().head;
        let caret_line =
            sanain_core::selection::line_col(self.editor().document().rope(), head).0 + 1;
        let scroll = self.md_synced.get(&id) != Some(&caret_line);
        // An anchor click scrolls once, on the next frame.
        let anchor = self.md_anchor.remove(&id);
        let mut event = None;
        self.md_synced.insert(id, caret_line);
        let l10n = &self.l10n;
        let untranslated = |key: &str| tr(l10n, key, &[]);
        let doc_dir = self
            .editor()
            .file()
            .and_then(|f| f.path.parent().map(std::path::Path::to_path_buf));
        let ctx = ui.ctx().clone();
        let images = &self.images;
        let image = |url: &str| {
            use crate::imagecache::ImageView;
            use sanain_app::images::ImageSource;
            match sanain_app::images::resolve(url, doc_dir.as_deref()) {
                ImageSource::Local(path) => images.borrow_mut().get(&ctx, &path),
                ImageSource::Remote => ImageView::Note("image-remote"),
                ImageSource::Blocked => ImageView::Note("image-blocked"),
            }
        };
        self.images.borrow_mut().poll(&ctx);
        let style = preview::Style {
            p,
            size: self.settings.font_size + 1.0,
            untranslated: &untranslated,
            image: &image,
        };
        egui::Frame::NONE
            .fill(p.canvas)
            .inner_margin(egui::Margin::symmetric(24, 16))
            .show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .id_salt(("md-preview", id))
                    .auto_shrink(false)
                    .show(ui, |ui| {
                        ui.set_max_width(760.0_f32.min(ui.available_width()));
                        event = preview::show(
                            ui,
                            &blocks,
                            &style,
                            caret_line,
                            scroll,
                            anchor.as_deref(),
                        );
                    });
            });
        if let Some(preview::Event::LinkClicked(url)) = event {
            self.follow_link(&url);
        }
    }

    /// Act on a link clicked in the preview. Web links need confirmation;
    /// local files open in a tab; anchors scroll; everything else is refused.
    fn follow_link(&mut self, url: &str) {
        use sanain_markdown::LinkKind;
        match sanain_markdown::classify_link(url) {
            LinkKind::External => self.dialog = Some(Dialog::OpenLink(url.trim().to_owned())),
            LinkKind::Anchor => {
                let id = self.ws.active().id;
                self.md_anchor
                    .insert(id, url.trim().trim_start_matches('#').to_owned());
            }
            LinkKind::Relative => {
                let target = url
                    .split(['#', '?'])
                    .next()
                    .unwrap_or("")
                    .replace("%20", " ");
                let base = self
                    .editor()
                    .file()
                    .and_then(|f| f.path.parent().map(std::path::Path::to_path_buf));
                match base.map(|b| b.join(&target)).filter(|p| p.is_file()) {
                    Some(path) => self.open(&path),
                    None => {
                        let msg = self.t("link-not-found", &[("target", target.into())]);
                        self.status = Some((msg, true));
                    }
                }
            }
            LinkKind::Blocked => self.status = Some((self.t("link-blocked", &[]), true)),
        }
    }

    fn language(ed: &Editor) -> Option<Lang> {
        ed.file().and_then(|f| Lang::from_path(&f.path))
    }

    /// Ask for highlighting of the active tab if its text changed, and
    /// collect finished results.
    fn update_syntax(&mut self, ctx: &egui::Context) {
        for h in self.syntax.poll() {
            self.spans.insert(h.tab, (h.revision, h.spans));
        }
        let tab = self.ws.active();
        let rev = tab.editor.document().revision();
        match Self::language(&tab.editor) {
            Some(lang) if self.requested.get(&tab.id) != Some(&rev) => {
                self.syntax
                    .submit(tab.id, rev, lang, tab.editor.document().text());
                self.requested.insert(tab.id, rev);
            }
            None => {
                self.spans.remove(&tab.id);
            }
            _ => {}
        }
        // Results arrive from another thread; check back shortly while
        // the active tab is waiting for fresh spans.
        let fresh = self.spans.get(&tab.id).is_some_and(|(r, _)| *r == rev);
        if Self::language(&tab.editor).is_some() && !fresh {
            ctx.request_repaint_after(std::time::Duration::from_millis(16));
        }
    }

    fn enabled(&self, action: MenuAction) -> bool {
        let doc = self.editor().document();
        self.dialog.is_none()
            && match action {
                MenuAction::Undo => doc.can_undo(),
                MenuAction::Redo => doc.can_redo(),
                MenuAction::Cut | MenuAction::Copy => self.editor().copy().is_some(),
                MenuAction::NextTab | MenuAction::PrevTab => self.ws.len() > 1,
                MenuAction::MarkdownSource
                | MenuAction::MarkdownSplit
                | MenuAction::MarkdownPreview => Self::is_markdown(self.editor()),
                MenuAction::StructuredView => Self::data_kind(self.editor()).is_some(),
                MenuAction::ReopenWithEncoding => self.editor().file().is_some(),
                _ => true,
            }
    }

    /// Whether a text field other than the editor has keyboard focus.
    fn text_field_focused(&self, ctx: &egui::Context) -> bool {
        let editor = self
            .views
            .get(&self.ws.active().id)
            .and_then(|v| v.widget_id);
        ctx.memory(|m| m.focused())
            .is_some_and(|f| Some(f) != editor)
    }

    fn dispatch(&mut self, action: MenuAction, ctx: &egui::Context) {
        if self.dialog.is_some() {
            return;
        }
        // Menu shortcuts reach us even when a field like the find bar has
        // focus: hand clipboard and undo there instead of to the document.
        if self.text_field_focused(ctx) {
            use egui::{Event, Key, Modifiers};
            let key = |key, modifiers| Event::Key {
                key,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers,
            };
            let event = match action {
                MenuAction::Copy => Some(Event::Copy),
                MenuAction::Cut => Some(Event::Cut),
                MenuAction::Paste => arboard::Clipboard::new()
                    .and_then(|mut c| c.get_text())
                    .ok()
                    .map(Event::Paste),
                MenuAction::SelectAll => Some(key(Key::A, Modifiers::COMMAND)),
                MenuAction::Undo => Some(key(Key::Z, Modifiers::COMMAND)),
                MenuAction::Redo => Some(key(Key::Z, Modifiers::COMMAND.plus(Modifiers::SHIFT))),
                _ => None,
            };
            if let Some(e) = event {
                ctx.input_mut(|i| i.events.push(e));
                return;
            }
        }
        let editor_cmd = |cmd| Some(cmd);
        let cmd = match action {
            MenuAction::New => {
                self.ws.new_tab();
                None
            }
            MenuAction::Open => {
                self.open_dialog();
                None
            }
            MenuAction::Save => {
                self.save();
                None
            }
            MenuAction::SaveAs => {
                self.save_as();
                None
            }
            MenuAction::CloseTab => {
                let id = self.ws.active().id;
                self.request_close_tab(id, ctx);
                None
            }
            MenuAction::ReopenTab => {
                if let Some(Err(e)) = self.ws.reopen_closed() {
                    let path = match &e {
                        LoadError::Io { path, .. } | LoadError::Decode { path, .. } => path.clone(),
                    };
                    self.dialog = Some(Dialog::Error(describe_load_error(&self.l10n, &path, &e)));
                } else {
                    self.focus_active();
                }
                None
            }
            MenuAction::NextTab => {
                self.ws.cycle(true);
                None
            }
            MenuAction::PrevTab => {
                self.ws.cycle(false);
                None
            }
            // Goes through close_requested, which guards unsaved changes.
            MenuAction::Quit => {
                ctx.send_viewport_cmd(ViewportCommand::Close);
                None
            }
            MenuAction::Find | MenuAction::Replace => {
                let seed = self.editor().copy();
                self.find.open(action == MenuAction::Replace, seed);
                return;
            }
            MenuAction::FindNext | MenuAction::FindPrev => {
                if !self.find.open {
                    self.find.open(false, None);
                }
                self.find_step(action == MenuAction::FindNext);
                return;
            }
            MenuAction::MarkdownSource
            | MenuAction::MarkdownSplit
            | MenuAction::MarkdownPreview => {
                if Self::is_markdown(self.editor()) {
                    let mode = match action {
                        MenuAction::MarkdownSplit => MdMode::Split,
                        MenuAction::MarkdownPreview => MdMode::Preview,
                        _ => MdMode::Source,
                    };
                    self.md_modes.insert(self.ws.active().id, mode);
                }
                None
            }
            MenuAction::CommandPalette => {
                self.palette.open();
                return;
            }
            MenuAction::CheckUpdates => {
                self.start_update_check(ctx, true);
                return;
            }
            MenuAction::FindInFolder => {
                let dir = self
                    .editor()
                    .file()
                    .and_then(|f| f.path.parent().map(std::path::Path::to_path_buf));
                self.folder.show_panel(dir);
                if self.folder.root.is_none() {
                    self.folder.root = rfd::FileDialog::new().pick_folder();
                }
                return;
            }
            MenuAction::Fold => {
                let tab = usize::from(self.settings.tab_width);
                self.editor_mut().fold_at_caret(tab);
                None
            }
            MenuAction::Unfold => {
                self.editor_mut().unfold_at_caret();
                None
            }
            MenuAction::UnfoldAll => {
                self.editor_mut().unfold_all();
                None
            }
            MenuAction::ReopenWithEncoding => {
                match self.editor().file().map(|f| f.path.clone()) {
                    Some(_) if self.editor().document().is_dirty() => {
                        self.status = Some((self.t("encoding-save-first", &[]), true));
                    }
                    Some(path) => self.start_reopen(&path),
                    None => {}
                }
                None
            }
            MenuAction::SaveWithEncoding => {
                let current = self.editor().encoding();
                let selected = encoding_choices(false)
                    .iter()
                    .position(|e| *e == current)
                    .unwrap_or(0);
                self.encoding_pick = Some(EncodingPick::Save {
                    selected,
                    error: None,
                });
                None
            }
            MenuAction::StructuredView => {
                if Self::data_kind(self.editor()).is_some() {
                    let id = self.ws.active().id;
                    if !self.structured.remove(&id) {
                        self.structured.insert(id);
                    }
                }
                None
            }
            MenuAction::CompareTabs => {
                if self.compare.take().is_none() {
                    match self.ws.len() {
                        0 | 1 => self.status = Some((self.t("compare-need-two", &[]), false)),
                        2 => {
                            let other = self.ws.tabs()[1 - self.ws.active_index()].id;
                            self.start_compare(other);
                        }
                        _ => self.compare_pick = true,
                    }
                }
                None
            }
            MenuAction::JsonValidate
            | MenuAction::JsonFormat
            | MenuAction::JsonMinify
            | MenuAction::SortLines
            | MenuAction::SortLinesDesc
            | MenuAction::UniqueLines
            | MenuAction::TrimTrailing
            | MenuAction::UpperCase
            | MenuAction::LowerCase
            | MenuAction::ToLf
            | MenuAction::ToCrLf => {
                self.run_tool(action);
                None
            }
            MenuAction::Settings => {
                self.settings_open = true;
                return;
            }
            MenuAction::ZoomIn | MenuAction::ZoomOut | MenuAction::ZoomReset => {
                match action {
                    MenuAction::ZoomIn => self.settings.zoom(1.0),
                    MenuAction::ZoomOut => self.settings.zoom(-1.0),
                    _ => self.settings.font_size = FONT_SIZE_DEFAULT,
                }
                self.save_settings();
                return;
            }
            MenuAction::Undo => editor_cmd(Command::Undo),
            MenuAction::Redo => editor_cmd(Command::Redo),
            MenuAction::SelectAll => editor_cmd(Command::SelectAll),
            MenuAction::DuplicateLine => editor_cmd(Command::DuplicateLine),
            MenuAction::DeleteLine => editor_cmd(Command::DeleteLine),
            MenuAction::MoveLineUp => editor_cmd(Command::MoveLineUp),
            MenuAction::MoveLineDown => editor_cmd(Command::MoveLineDown),
            MenuAction::Copy => {
                if let Some(t) = self.editor().copy() {
                    ctx.copy_text(t);
                }
                None
            }
            MenuAction::Cut => {
                if let Some(t) = self.editor_mut().execute(Command::Cut) {
                    ctx.copy_text(t);
                }
                None
            }
            MenuAction::Paste => arboard::Clipboard::new()
                .and_then(|mut c| c.get_text())
                .ok()
                .filter(|t| !t.is_empty())
                .map(Command::Insert),
        };
        if let Some(cmd) = cmd {
            self.editor_mut().execute(cmd);
        }
        self.focus_active();
    }

    fn tab_bar(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let p = theme::palette(ctx);
        let mut activate = None;
        let mut close = None;
        egui::ScrollArea::horizontal()
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    let active = self.ws.active_index();
                    for (i, tab) in self.ws.tabs().iter().enumerate() {
                        let is_active = i == active;
                        let dirty = tab.editor.document().is_dirty();
                        let name = self.name_of(&tab.editor);
                        let frame = egui::Frame::NONE
                            .fill(if is_active { p.canvas } else { p.surface })
                            .inner_margin(egui::Margin {
                                left: 12,
                                right: 6,
                                top: 5,
                                bottom: 5,
                            });
                        let r = frame.show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing.x = 6.0;
                                let color = if is_active { p.text } else { p.muted };
                                let label = ui.add(
                                    egui::Label::new(egui::RichText::new(&name).color(color))
                                        .selectable(false)
                                        .sense(egui::Sense::click()),
                                );
                                if label.clicked() {
                                    activate = Some(i);
                                }
                                if label.middle_clicked() {
                                    close = Some(tab.id);
                                }
                                // Unsaved marker doubles as the close button,
                                // as in most native editors.
                                let glyph = if dirty { "●" } else { "×" };
                                let btn = ui
                                    .add(
                                        egui::Button::new(
                                            egui::RichText::new(glyph).color(p.muted),
                                        )
                                        .frame(false)
                                        .min_size(egui::vec2(16.0, 16.0)),
                                    )
                                    .on_hover_text(self.t("tab-close", &[]));
                                if btn.clicked() {
                                    close = Some(tab.id);
                                }
                            });
                        });
                        let rect = r.response.rect;
                        let painter = ui.painter();
                        painter.vline(
                            rect.right(),
                            rect.y_range(),
                            egui::Stroke::new(1.0, p.border),
                        );
                        if is_active {
                            let top =
                                egui::Rect::from_min_size(rect.min, egui::vec2(rect.width(), 2.0));
                            painter.rect_filled(top, 0.0, p.accent);
                        } else {
                            painter.hline(
                                rect.x_range(),
                                rect.bottom() - 0.5,
                                egui::Stroke::new(1.0, p.border),
                            );
                        }
                        let r = r.response.interact(egui::Sense::click());
                        if r.clicked() {
                            activate = Some(i);
                        }
                        if r.middle_clicked() {
                            close = Some(tab.id);
                        }
                    }
                });
            });
        if let Some(i) = activate {
            self.ws.activate(i);
            self.focus_active();
        }
        if let Some(id) = close {
            self.request_close_tab(id, ctx);
        }
    }

    fn status_bar(&self, ui: &mut egui::Ui) {
        let p = theme::palette(ui.ctx());
        let editor = self.editor();
        ui.horizontal(|ui| {
            if let Some((msg, is_error)) = &self.status {
                ui.colored_label(if *is_error { p.error } else { p.muted }, msg);
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.colored_label(p.muted, editor.encoding().name());
                let le = match editor.line_ending() {
                    LineEnding::CrLf => "line-ending-crlf",
                    LineEnding::Cr => "line-ending-cr",
                    LineEnding::Mixed => "line-ending-mixed",
                    LineEnding::Lf | LineEnding::None => "line-ending-lf",
                };
                ui.colored_label(p.muted, self.t(le, &[]));
                let lang = match Self::language(editor) {
                    Some(l) => l.name().to_owned(),
                    None if Self::is_markdown(editor) => "Markdown".to_owned(),
                    None => self.t("language-plain", &[]),
                };
                ui.colored_label(p.muted, lang);
                let sels = editor.selections();
                if sels.len() > 1 {
                    let n = sels.len() as i64;
                    ui.colored_label(p.muted, self.t("status-selections", &[("count", n.into())]));
                }
                let (line, col) =
                    sanain_core::selection::line_col(editor.document().rope(), sels.primary().head);
                let pos = self.t(
                    "status-position",
                    &[
                        ("line", (line as i64 + 1).into()),
                        ("column", (col as i64 + 1).into()),
                    ],
                );
                ui.colored_label(p.muted, pos);
            });
        });
    }

    fn dialogs(&mut self, ctx: &egui::Context) {
        let Some(dialog) = self.dialog.clone() else {
            return;
        };
        let name = self.display_name();
        let mut close = false;
        let mut after: Option<Action> = None;
        egui::Modal::new(egui::Id::unique("sanain-dialog")).show(ctx, |ui| {
            ui.set_max_width(420.0);
            let (title, body) = match &dialog {
                Dialog::Unsaved(_) => (
                    self.t("dialog-unsaved-title", &[("name", name.clone().into())]),
                    self.t("dialog-unsaved-body", &[]),
                ),
                Dialog::Conflict => (
                    self.t("dialog-conflict-title", &[("name", name.clone().into())]),
                    self.t("dialog-conflict-body", &[]),
                ),
                Dialog::ReadOnly => (
                    self.t("dialog-readonly-title", &[("name", name.clone().into())]),
                    self.t("dialog-readonly-body", &[]),
                ),
                Dialog::Error(msg) => (msg.clone(), String::new()),
                Dialog::OpenLink(url) => (self.t("dialog-open-link-title", &[]), url.clone()),
                Dialog::Update(version, _) => (
                    self.t(
                        "update-available-title",
                        &[("version", version.clone().into())],
                    ),
                    self.t("update-available-body", &[]),
                ),
            };
            ui.strong(title);
            if !body.is_empty() {
                ui.label(body);
            }
            ui.add_space(8.0);
            ui.with_layout(
                egui::Layout::right_to_left(egui::Align::Center),
                |ui| match &dialog {
                    Dialog::Unsaved(then) => {
                        if ui.button(self.t("button-save", &[])).clicked() {
                            close = true;
                            after = Some(Action::SaveThen(*then));
                        }
                        if ui.button(self.t("button-dont-save", &[])).clicked() {
                            close = true;
                            after = Some(Action::Proceed(*then));
                        }
                        if ui.button(self.t("button-cancel", &[])).clicked() {
                            close = true;
                        }
                    }
                    Dialog::Conflict => {
                        if ui.button(self.t("button-save-as", &[])).clicked() {
                            close = true;
                            after = Some(Action::SaveAs);
                        }
                        if ui.button(self.t("button-overwrite", &[])).clicked() {
                            close = true;
                            after = Some(Action::Overwrite);
                        }
                        if ui.button(self.t("button-cancel", &[])).clicked() {
                            close = true;
                        }
                    }
                    Dialog::ReadOnly => {
                        if ui.button(self.t("button-save-as", &[])).clicked() {
                            close = true;
                            after = Some(Action::SaveAs);
                        }
                        if ui.button(self.t("button-cancel", &[])).clicked() {
                            close = true;
                        }
                    }
                    Dialog::Update(_, page) => {
                        if ui.button(self.t("update-open-page", &[])).clicked() {
                            ui.ctx().open_url(egui::OpenUrl::new_tab(page));
                            close = true;
                        }
                        if ui.button(self.t("update-later", &[])).clicked() {
                            close = true;
                        }
                    }
                    Dialog::OpenLink(url) => {
                        if ui.button(self.t("button-open", &[])).clicked() {
                            ui.ctx().open_url(egui::OpenUrl::new_tab(url));
                            close = true;
                        }
                        if ui.button(self.t("button-cancel", &[])).clicked() {
                            close = true;
                        }
                    }
                    Dialog::Error(_) => {
                        if ui.button(self.t("button-ok", &[])).clicked() {
                            close = true;
                        }
                    }
                },
            );
        });
        if close {
            self.dialog = None;
            if after.is_none() {
                // Cancelled: a later quit records the session afresh.
                self.quit_recorded = false;
            }
            self.focus_active();
            match after {
                Some(Action::SaveThen(then)) => {
                    if self.save() {
                        self.proceed(then, ctx);
                    }
                }
                Some(Action::Proceed(then)) => self.proceed(then, ctx),
                Some(Action::SaveAs) => {
                    self.save_as();
                }
                Some(Action::Overwrite) => {
                    let o = self.editor_mut().save_overwrite();
                    self.handle_save(o);
                }
                None => {}
            }
        }
    }
}

impl eframe::App for Shell {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();

        if ctx.input(|i| i.viewport().close_requested()) && !self.allow_close {
            ctx.send_viewport_cmd(ViewportCommand::CancelClose);
            if self.dialog.is_none() {
                self.request_quit(&ctx);
            }
        }

        let mut actions = Vec::new();
        // Cmd/Ctrl+= also zooms in (US layouts), whatever the menu shows.
        if self.dialog.is_none() {
            let eq = egui::KeyboardShortcut::new(egui::Modifiers::COMMAND, egui::Key::Equals);
            let plus = egui::KeyboardShortcut::new(egui::Modifiers::COMMAND, egui::Key::Plus);
            if ctx.input_mut(|i| i.consume_shortcut(&eq) || i.consume_shortcut(&plus)) {
                actions.push(MenuAction::ZoomIn);
            }
        }
        #[cfg(target_os = "macos")]
        if let Some(m) = &self.native_menu {
            actions.extend(m.poll());
        }
        // Without a native menu, file, tab and quit shortcuts are handled
        // here; editing shortcuts arrive as key events in the view.
        if !self.has_native_menu() && self.dialog.is_none() && self.recording.is_none() {
            let text_field = self.text_field_focused(&ctx);
            for a in MenuAction::ALL {
                // Clipboard shortcuts arrive as clipboard events; editing
                // commands belong to the focused text field if it is not
                // the editor.
                let skip = matches!(a, MenuAction::Cut | MenuAction::Copy | MenuAction::Paste)
                    || (text_field && a.edits_text());
                if skip {
                    continue;
                }
                if let Some(s) = a.shortcut(&self.settings.keymap)
                    && ctx.input_mut(|i| i.consume_shortcut(&s))
                {
                    actions.push(a);
                }
            }
        }

        if !self.has_native_menu() {
            let p = theme::palette(&ctx);
            let frame = egui::Frame::NONE
                .fill(p.surface)
                .inner_margin(egui::Margin::symmetric(6, 2));
            egui::Panel::top("menu").frame(frame).show(ui, |ui| {
                let label = |id: &str| self.t(id, &[]);
                let enabled = |a| self.enabled(a);
                if let Some(a) = menu::in_window(ui, &label, enabled, &self.settings.keymap) {
                    actions.push(a);
                }
            });
        }
        for a in actions {
            self.dispatch(a, &ctx);
        }
        #[cfg(target_os = "macos")]
        if let Some(m) = &self.native_menu {
            m.set_enabled(|a| self.enabled(a));
        }

        let p = theme::palette(&ctx);
        egui::Panel::top("tabs")
            .frame(egui::Frame::NONE.fill(p.surface))
            .show(ui, |ui| self.tab_bar(ui, &ctx));
        if self.find.open {
            let tab = self.ws.active().id;
            self.find.refresh(tab, &self.ws.active().editor);
            let current = self
                .find
                .matches
                .index_of(&self.editor().selections().primary().range());
            let l10n = &self.l10n;
            let t = |id: &str, a: Option<(&str, i64)>, b: Option<(&str, i64)>| {
                let args: Vec<(&str, FluentValue<'_>)> = [a, b]
                    .into_iter()
                    .flatten()
                    .map(|(k, v)| (k, v.into()))
                    .collect();
                tr(l10n, id, &args)
            };
            let frame = egui::Frame::NONE
                .fill(p.surface)
                .inner_margin(egui::Margin::symmetric(8, 6));
            let mut event = None;
            egui::Panel::top("find").frame(frame).show(ui, |ui| {
                event = self.find.show(ui, &t, current);
            });
            if let Some(e) = event {
                self.find_event(e);
            }
        }
        self.update_syntax(&ctx);
        egui::Panel::bottom("status").show(ui, |ui| self.status_bar(ui));
        if self.folder.open {
            let l10n = &self.l10n;
            let t = |id: &str, a: Option<(&str, i64)>, b: Option<(&str, i64)>| {
                let args: Vec<(&str, FluentValue<'_>)> = [a, b]
                    .into_iter()
                    .flatten()
                    .map(|(k, v)| (k, v.into()))
                    .collect();
                tr(l10n, id, &args)
            };
            let mut event = None;
            egui::Panel::bottom("folder-search")
                .resizable(true)
                .default_size(240.0)
                .frame(
                    egui::Frame::NONE
                        .fill(p.surface)
                        .inner_margin(egui::Margin::same(8)),
                )
                .show(ui, |ui| event = self.folder.ui(ui, &t));
            use crate::folderpanel::PanelEvent;
            match event {
                Some(PanelEvent::Close) => {
                    self.folder.open = false;
                    self.focus_active();
                }
                Some(PanelEvent::ChooseFolder) => {
                    if let Some(dir) = rfd::FileDialog::new().pick_folder() {
                        self.folder.root = Some(dir);
                    }
                }
                Some(PanelEvent::OpenHit(hit)) => {
                    self.open(&hit.path);
                    if self.editor().file().is_some_and(|f| f.path == hit.path) {
                        self.editor_mut().select(hit.chars.clone());
                        let id = self.ws.active().id;
                        self.views.entry(id).or_default().reveal();
                        self.focus_active();
                    }
                }
                None => {}
            }
        }
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(p.canvas))
            .show(ui, |ui| {
                if self.compare_area(ui) || self.structured_area(ui) {
                    return;
                }
                match self.md_mode() {
                    MdMode::Source => self.editor_area(ui),
                    MdMode::Preview => self.preview_area(ui),
                    MdMode::Split => {
                        ui.columns(2, |cols| {
                            self.editor_area(&mut cols[0]);
                            self.preview_area(&mut cols[1]);
                        });
                    }
                }
            });

        self.poll_update();
        self.dialogs(&ctx);
        self.settings_window(&ctx);
        self.compare_picker(&ctx);
        self.encoding_window(&ctx);
        if self.palette.open && self.dialog.is_none() {
            let entries: Vec<Entry> = MenuAction::ALL
                .into_iter()
                .filter(|a| *a != MenuAction::CommandPalette)
                .map(|a| Entry {
                    action: a,
                    label: self.t(a.label_id(), &[]).trim_end_matches('…').to_owned(),
                    shortcut: a
                        .shortcut(&self.settings.keymap)
                        .map(|s| ctx.format_shortcut(&s))
                        .unwrap_or_default(),
                    enabled: self.enabled(a),
                })
                .collect();
            let hint = self.t("palette-placeholder", &[]);
            if let Some(a) = self.palette.show(&ctx, &entries, &hint) {
                self.dispatch(a, &ctx);
            } else if !self.palette.open {
                self.focus_active();
            }
        }
        self.apply_theme(&ctx);

        let (wait, error) = self.ws.tick(std::time::Instant::now());
        if let Some(wait) = wait {
            ctx.request_repaint_after(wait);
        }
        if let Some(reason) = error {
            let msg = self.t("status-recovery-failed", &[("reason", reason.into())]);
            self.status = Some((msg, true));
        }

        let dirty = if self.editor().document().is_dirty() {
            "yes"
        } else {
            "no"
        };
        let title = self.t(
            "window-title",
            &[
                ("name", self.display_name().into()),
                ("dirty", dirty.into()),
            ],
        );
        if title != self.title {
            ctx.send_viewport_cmd(ViewportCommand::Title(title.clone()));
            self.title = title;
        }
    }
}

/// Localization ID for a JSON error reason (see locales: `json-reason-*`).
fn json_reason_id(message: &str) -> String {
    let mut id = String::from("json-reason-");
    let mut dash = false;
    for c in message.chars().flat_map(char::to_lowercase) {
        if c.is_ascii_alphanumeric() {
            if dash && !id.ends_with('-') {
                id.push('-');
            }
            id.push(c);
            dash = false;
        } else {
            dash = true;
        }
    }
    id
}

/// Format a localized message. Fluent's isolation marks are stripped
/// because egui renders them as visible boxes.
fn tr(l10n: &Localizer, id: &str, args: &[(&str, FluentValue<'_>)]) -> String {
    let mut fa = sanain_i18n::FluentArgs::new();
    for (k, v) in args {
        fa.set(*k, v.clone());
    }
    l10n.format(id, Some(&fa))
        .replace(['\u{2068}', '\u{2069}'], "")
}

pub fn describe_load_error(l10n: &Localizer, path: &Path, e: &LoadError) -> String {
    let name = path.file_name().map_or_else(
        || path.display().to_string(),
        |n| n.to_string_lossy().into_owned(),
    );
    let mut args = sanain_i18n::FluentArgs::new();
    args.set("name", name);
    let id = match e {
        LoadError::Decode { .. } => "error-open-encoding",
        LoadError::Io { .. } => "error-open",
    };
    l10n.format(id, Some(&args))
        .replace(['\u{2068}', '\u{2069}'], "")
}

#[cfg(test)]
mod tests {
    #[test]
    fn json_reason_ids_exist_in_english() {
        let en = include_str!("../../../locales/en-US/main.ftl");
        for m in [
            "unterminated string",
            "invalid \\u escape",
            "expected ':'",
            "expected ',' or a closing bracket",
            "too deeply nested to format",
        ] {
            let id = super::json_reason_id(m);
            assert!(en.contains(&format!("{id} = ")), "{id} missing");
        }
    }
}
