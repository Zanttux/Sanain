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
use crate::theme;
use crate::view::{EditorView, ViewOptions};
use sanain_app::settings::{FONT_SIZE_DEFAULT, Settings, ThemeChoice};

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
    syntax: SyntaxWorker,
    /// Latest highlight result and the revision requested last, per tab.
    spans: HashMap<TabId, (Revision, Vec<CharSpan>)>,
    requested: HashMap<TabId, Revision>,
    session_file: Option<std::path::PathBuf>,
    settings: Settings,
    settings_dir: Option<std::path::PathBuf>,
    settings_open: bool,
    palette: Palette,
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
            self.native_menu = menu::NativeMenu::install(&label, move || ctx.request_repaint());
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
        startup_errors: Vec<String>,
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
            syntax: SyntaxWorker::start(),
            spans: HashMap::new(),
            requested: HashMap::new(),
            session_file,
            settings,
            settings_dir,
            settings_open: false,
            palette: Palette::default(),
            applied_theme: None,
            quit_recorded: false,
            #[cfg(target_os = "macos")]
            native_menu: None,
        };
        if !startup_errors.is_empty() {
            shell.dialog = Some(Dialog::Error(startup_errors.join("\n")));
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
        ctx.set_theme(match self.settings.theme {
            ThemeChoice::System => egui::ThemePreference::System,
            ThemeChoice::Light => egui::ThemePreference::Light,
            ThemeChoice::Dark => egui::ThemePreference::Dark,
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
                            ] {
                                ui.radio_value(&mut s.theme, v, l(id));
                            }
                        });
                        ui.end_row();
                        ui.label("");
                        ui.checkbox(&mut s.restore_session, l("settings-restore-session"));
                        ui.end_row();
                        ui.label(l("settings-language"));
                        let current = match s.locale.as_deref() {
                            Some("fi-FI") => "Suomi".to_owned(),
                            Some("en-US") => "English".to_owned(),
                            _ => l("settings-language-system"),
                        };
                        egui::ComboBox::from_id_salt("settings-language")
                            .selected_text(current)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut s.locale,
                                    None,
                                    l("settings-language-system"),
                                );
                                // Language names in their own language.
                                ui.selectable_value(&mut s.locale, Some("en-US".into()), "English");
                                ui.selectable_value(&mut s.locale, Some("fi-FI".into()), "Suomi");
                            });
                        ui.end_row();
                    });
                ui.add_space(4.0);
                ui.weak(l("settings-language-note"));
            });
        if self.settings != before {
            self.save_settings();
        }
        if !open {
            self.settings_open = false;
            self.focus_active();
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
                _ => true,
            }
    }

    fn dispatch(&mut self, action: MenuAction, ctx: &egui::Context) {
        if self.dialog.is_some() {
            return;
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
            MenuAction::CommandPalette => {
                self.palette.open();
                return;
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
                let enc = match editor.encoding() {
                    TextEncoding::Utf8 => "encoding-utf8",
                    TextEncoding::Utf8Bom => "encoding-utf8-bom",
                    TextEncoding::Utf16LeBom => "encoding-utf16le",
                    TextEncoding::Utf16BeBom => "encoding-utf16be",
                };
                ui.colored_label(p.muted, self.t(enc, &[]));
                let le = match editor.line_ending() {
                    LineEnding::CrLf => "line-ending-crlf",
                    LineEnding::Cr => "line-ending-cr",
                    LineEnding::Mixed => "line-ending-mixed",
                    LineEnding::Lf | LineEnding::None => "line-ending-lf",
                };
                ui.colored_label(p.muted, self.t(le, &[]));
                let lang = match Self::language(editor) {
                    Some(l) => l.name().to_owned(),
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
        if !self.has_native_menu() && self.dialog.is_none() {
            for a in [
                MenuAction::SaveAs,
                MenuAction::Save,
                MenuAction::Open,
                MenuAction::New,
                MenuAction::CloseTab,
                MenuAction::ReopenTab,
                MenuAction::PrevTab,
                MenuAction::NextTab,
                MenuAction::Replace,
                MenuAction::CommandPalette,
                MenuAction::Settings,
                MenuAction::ZoomIn,
                MenuAction::ZoomOut,
                MenuAction::ZoomReset,
                MenuAction::Find,
                MenuAction::FindPrev,
                MenuAction::FindNext,
                MenuAction::Quit,
            ] {
                if let Some(s) = a.shortcut()
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
                if let Some(a) = menu::in_window(ui, &label, enabled) {
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
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(p.canvas))
            .show(ui, |ui| {
                let name = self.t("editor-accessible-name", &[]);
                let interactive = self.dialog.is_none();
                let id = self.ws.active().id;
                let view = self.views.entry(id).or_default();
                let editor = &mut self.ws.active_mut().editor;
                // Spans from an older revision are kept until fresh ones
                // arrive; a frame of slightly shifted colour beats flicker.
                let syntax: &[CharSpan] = self.spans.get(&id).map_or(&[], |(_, s)| s);
                let opts = ViewOptions {
                    font_size: self.settings.font_size,
                    indent: self.settings.indent(),
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
            });

        self.dialogs(&ctx);
        self.settings_window(&ctx);
        if self.palette.open && self.dialog.is_none() {
            let entries: Vec<Entry> = MenuAction::ALL
                .into_iter()
                .filter(|a| *a != MenuAction::CommandPalette)
                .map(|a| Entry {
                    action: a,
                    label: self.t(a.label_id(), &[]).trim_end_matches('…').to_owned(),
                    shortcut: a
                        .shortcut()
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
