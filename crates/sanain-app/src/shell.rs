//! Window chrome: menus, status bar, file dialogs and confirmations.

use std::path::Path;

use eframe::egui::{self, ViewportCommand};
use sanain_app::{Editor, SaveOutcome};
use sanain_core::LineEnding;
use sanain_i18n::{FluentValue, Localizer};
use sanain_io::{LoadError, TextEncoding};

use sanain_app::recovery::Recovery;

use crate::menu::{self, MenuAction};
use crate::theme;
use crate::view::EditorView;

/// What to do once unsaved changes are resolved.
#[derive(Debug, Clone)]
enum Then {
    Close,
    New,
    Open,
}

/// A dialog button's follow-up, run after the dialog closes.
#[derive(Debug, Clone)]
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
    editor: Editor,
    l10n: Localizer,
    view: EditorView,
    dialog: Option<Dialog>,
    status: Option<(String, bool)>,
    title: String,
    allow_close: bool,
    recovery: Option<Recovery>,
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
        editor: Editor,
        l10n: Localizer,
        startup_error: Option<String>,
        recovery: Option<Recovery>,
        recovered: bool,
    ) -> Self {
        let mut view = EditorView::default();
        view.request_focus();
        let status = recovered.then(|| (l10n.get("status-recovered"), false));
        Self {
            editor,
            l10n,
            view,
            dialog: startup_error.map(Dialog::Error),
            status,
            recovery,
            title: String::new(),
            allow_close: false,
            #[cfg(target_os = "macos")]
            native_menu: None,
        }
    }

    fn t(&self, id: &str, args: &[(&str, FluentValue<'_>)]) -> String {
        let mut fa = sanain_i18n::FluentArgs::new();
        for (k, v) in args {
            fa.set(*k, v.clone());
        }
        // Isolation marks would show as boxes in egui's text rendering.
        self.l10n
            .format(id, Some(&fa))
            .replace(['\u{2068}', '\u{2069}'], "")
    }

    fn display_name(&self) -> String {
        self.editor
            .file_name()
            .unwrap_or_else(|| self.t("untitled", &[]))
    }

    fn save(&mut self) -> bool {
        let outcome = self.editor.save();
        self.handle_save(outcome)
    }

    fn save_as(&mut self) -> bool {
        let mut dialog = rfd::FileDialog::new().set_file_name(self.display_name());
        if let Some(dir) = self.editor.file().and_then(|f| f.path.parent()) {
            dialog = dialog.set_directory(dir);
        }
        match dialog.save_file() {
            Some(path) => {
                let outcome = self.editor.save_as(&path);
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

    /// Run `then` now if there is nothing to lose, otherwise ask first.
    fn guard(&mut self, then: Then, ctx: &egui::Context) {
        if self.editor.document().is_dirty() {
            self.dialog = Some(Dialog::Unsaved(then));
        } else {
            self.proceed(then, ctx);
        }
    }

    fn proceed(&mut self, then: Then, ctx: &egui::Context) {
        // The document is either saved or its changes were declined: the
        // checkpoint for it is no longer needed.
        if let Some(r) = &mut self.recovery {
            r.discard();
        }
        match then {
            Then::Close => {
                if let Some(r) = &self.recovery {
                    r.flush(std::time::Duration::from_secs(2));
                }
                self.allow_close = true;
                ctx.send_viewport_cmd(ViewportCommand::Close);
            }
            Then::New => {
                self.editor = Editor::new();
                self.status = None;
                self.view = EditorView::default();
                self.view.request_focus();
            }
            Then::Open => {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.open(&path);
                }
            }
        }
    }

    fn open(&mut self, path: &Path) {
        match Editor::open(path) {
            Ok(ed) => {
                self.editor = ed;
                self.status = None;
                self.view = EditorView::default();
                self.view.request_focus();
            }
            Err(e) => self.dialog = Some(Dialog::Error(describe_load_error(&self.l10n, path, &e))),
        }
    }

    fn enabled(&self, action: MenuAction) -> bool {
        let doc = self.editor.document();
        let has_selection = self.editor.copy().is_some();
        self.dialog.is_none()
            && match action {
                MenuAction::Undo => doc.can_undo(),
                MenuAction::Redo => doc.can_redo(),
                MenuAction::Cut | MenuAction::Copy => has_selection,
                _ => true,
            }
    }

    fn dispatch(&mut self, action: MenuAction, ctx: &egui::Context) {
        if self.dialog.is_some() {
            return;
        }
        use sanain_app::Command;
        match action {
            MenuAction::New => self.guard(Then::New, ctx),
            MenuAction::Open => self.guard(Then::Open, ctx),
            MenuAction::Save => {
                self.save();
            }
            MenuAction::SaveAs => {
                self.save_as();
            }
            // Goes through close_requested, which guards unsaved changes.
            MenuAction::Quit => ctx.send_viewport_cmd(ViewportCommand::Close),
            MenuAction::Undo => {
                self.editor.execute(Command::Undo);
            }
            MenuAction::Redo => {
                self.editor.execute(Command::Redo);
            }
            MenuAction::SelectAll => {
                self.editor.execute(Command::SelectAll);
            }
            MenuAction::Copy => {
                if let Some(t) = self.editor.copy() {
                    ctx.copy_text(t);
                }
            }
            MenuAction::Cut => {
                if let Some(t) = self.editor.execute(Command::Cut) {
                    ctx.copy_text(t);
                }
            }
            MenuAction::Paste => {
                let text = arboard::Clipboard::new().and_then(|mut c| c.get_text());
                if let Ok(t) = text
                    && !t.is_empty()
                {
                    self.editor.execute(Command::Insert(t));
                }
            }
        }
        self.view.request_focus();
    }

    fn status_bar(&self, ui: &mut egui::Ui) {
        let p = theme::palette(ui.ctx());
        ui.horizontal(|ui| {
            if let Some((msg, is_error)) = &self.status {
                ui.colored_label(if *is_error { p.error } else { p.muted }, msg);
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let enc = match self.editor.encoding() {
                    TextEncoding::Utf8 => "encoding-utf8",
                    TextEncoding::Utf8Bom => "encoding-utf8-bom",
                    TextEncoding::Utf16LeBom => "encoding-utf16le",
                    TextEncoding::Utf16BeBom => "encoding-utf16be",
                };
                ui.colored_label(p.muted, self.t(enc, &[]));
                let le = match self.editor.line_ending() {
                    LineEnding::CrLf => "line-ending-crlf",
                    LineEnding::Cr => "line-ending-cr",
                    LineEnding::Mixed => "line-ending-mixed",
                    LineEnding::Lf | LineEnding::None => "line-ending-lf",
                };
                ui.colored_label(p.muted, self.t(le, &[]));
                let sels = self.editor.selections();
                if sels.len() > 1 {
                    let n = sels.len() as i64;
                    ui.colored_label(p.muted, self.t("status-selections", &[("count", n.into())]));
                }
                let (line, col) = sanain_core::selection::line_col(
                    self.editor.document().rope(),
                    sels.primary().head,
                );
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
                            after = Some(Action::SaveThen(then.clone()));
                        }
                        if ui.button(self.t("button-dont-save", &[])).clicked() {
                            close = true;
                            after = Some(Action::Proceed(then.clone()));
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
            self.view.request_focus();
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
                    let o = self.editor.save_overwrite();
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

        if ctx.input(|i| i.viewport().close_requested())
            && !self.allow_close
            && self.editor.document().is_dirty()
        {
            ctx.send_viewport_cmd(ViewportCommand::CancelClose);
            self.dialog = Some(Dialog::Unsaved(Then::Close));
        }

        let mut actions = Vec::new();
        #[cfg(target_os = "macos")]
        if let Some(m) = &self.native_menu {
            actions.extend(m.poll());
        }
        // Without a native menu, file and quit shortcuts are handled here;
        // editing shortcuts arrive as key and clipboard events in the view.
        if !self.has_native_menu() && self.dialog.is_none() {
            for a in [
                MenuAction::SaveAs,
                MenuAction::Save,
                MenuAction::Open,
                MenuAction::New,
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
                .inner_margin(egui::Margin::symmetric(6, 2))
                .stroke(egui::Stroke::NONE);
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
        egui::Panel::bottom("status").show(ui, |ui| self.status_bar(ui));
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(theme::palette(&ctx).canvas))
            .show(ui, |ui| {
                let name = self.t("editor-accessible-name", &[]);
                let interactive = self.dialog.is_none();
                ui.add_enabled_ui(interactive, |ui| {
                    if self.view.show(ui, &mut self.editor, &name) {
                        self.status = None;
                    }
                });
            });

        self.dialogs(&ctx);

        let mut recovery_error = None;
        if let Some(r) = &mut self.recovery {
            if let Some(wait) = r.tick(&self.editor, std::time::Instant::now()) {
                ctx.request_repaint_after(wait);
            }
            recovery_error = r.take_error();
            // A clean window closing without a dialog: let pending
            // removals reach disk before the process exits.
            if ctx.input(|i| i.viewport().close_requested()) && !self.editor.document().is_dirty() {
                r.flush(std::time::Duration::from_secs(2));
            }
        }
        if let Some(reason) = recovery_error {
            let msg = self.t("status-recovery-failed", &[("reason", reason.into())]);
            self.status = Some((msg, true));
        }

        let dirty = if self.editor.document().is_dirty() {
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
