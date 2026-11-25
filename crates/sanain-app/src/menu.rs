//! Application menus. macOS gets the native menu bar; other platforms use
//! an in-window menu bar until native menus are verified there.

use eframe::egui::{self, Key, KeyboardShortcut, Modifiers};

/// Everything a menu item can do. Both menu implementations emit these.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuAction {
    New,
    Open,
    Save,
    SaveAs,
    Quit,
    Undo,
    Redo,
    Cut,
    Copy,
    Paste,
    SelectAll,
    DuplicateLine,
    DeleteLine,
    MoveLineUp,
    MoveLineDown,
    CloseTab,
    ReopenTab,
    NextTab,
    PrevTab,
    Find,
    FindNext,
    FindPrev,
    Replace,
    Settings,
    ZoomIn,
    ZoomOut,
    ZoomReset,
    CommandPalette,
}

impl MenuAction {
    pub const ALL: [MenuAction; 28] = [
        Self::New,
        Self::Open,
        Self::Save,
        Self::SaveAs,
        Self::Quit,
        Self::Undo,
        Self::Redo,
        Self::Cut,
        Self::Copy,
        Self::Paste,
        Self::SelectAll,
        Self::DuplicateLine,
        Self::DeleteLine,
        Self::MoveLineUp,
        Self::MoveLineDown,
        Self::CloseTab,
        Self::ReopenTab,
        Self::NextTab,
        Self::PrevTab,
        Self::Find,
        Self::FindNext,
        Self::FindPrev,
        Self::Replace,
        Self::Settings,
        Self::ZoomIn,
        Self::ZoomOut,
        Self::ZoomReset,
        Self::CommandPalette,
    ];

    /// Stable identifier used for native menu item ids.
    fn id(self) -> &'static str {
        match self {
            Self::New => "file.new",
            Self::Open => "file.open",
            Self::Save => "file.save",
            Self::SaveAs => "file.save-as",
            Self::Quit => "app.quit",
            Self::Undo => "edit.undo",
            Self::Redo => "edit.redo",
            Self::Cut => "edit.cut",
            Self::Copy => "edit.copy",
            Self::Paste => "edit.paste",
            Self::SelectAll => "edit.select-all",
            Self::DuplicateLine => "edit.duplicate-line",
            Self::DeleteLine => "edit.delete-line",
            Self::MoveLineUp => "edit.move-line-up",
            Self::MoveLineDown => "edit.move-line-down",
            Self::CloseTab => "file.close-tab",
            Self::ReopenTab => "file.reopen-tab",
            Self::NextTab => "window.next-tab",
            Self::PrevTab => "window.prev-tab",
            Self::Find => "edit.find",
            Self::FindNext => "edit.find-next",
            Self::FindPrev => "edit.find-prev",
            Self::Replace => "edit.replace",
            Self::Settings => "app.settings",
            Self::ZoomIn => "view.zoom-in",
            Self::ZoomOut => "view.zoom-out",
            Self::ZoomReset => "view.zoom-reset",
            Self::CommandPalette => "view.command-palette",
        }
    }

    #[cfg_attr(not(target_os = "macos"), allow(dead_code))]
    fn from_id(id: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|a| a.id() == id)
    }

    /// Localization message ID for the item label.
    pub fn label_id(self) -> &'static str {
        match self {
            Self::New => "file-new",
            Self::Open => "file-open",
            Self::Save => "file-save",
            Self::SaveAs => "file-save-as",
            Self::Quit => "app-quit",
            Self::Undo => "edit-undo",
            Self::Redo => "edit-redo",
            Self::Cut => "edit-cut",
            Self::Copy => "edit-copy",
            Self::Paste => "edit-paste",
            Self::SelectAll => "edit-select-all",
            Self::DuplicateLine => "edit-duplicate-line",
            Self::DeleteLine => "edit-delete-line",
            Self::MoveLineUp => "edit-move-line-up",
            Self::MoveLineDown => "edit-move-line-down",
            Self::CloseTab => "file-close-tab",
            Self::ReopenTab => "file-reopen-tab",
            Self::NextTab => "window-next-tab",
            Self::PrevTab => "window-prev-tab",
            Self::Find => "edit-find",
            Self::FindNext => "edit-find-next",
            Self::FindPrev => "edit-find-prev",
            Self::Replace => "edit-replace",
            Self::Settings => "app-settings",
            Self::ZoomIn => "view-zoom-in",
            Self::ZoomOut => "view-zoom-out",
            Self::ZoomReset => "view-zoom-reset",
            Self::CommandPalette => "view-command-palette",
        }
    }

    pub fn shortcut(self) -> Option<KeyboardShortcut> {
        let cmd = Modifiers::COMMAND;
        let cmd_shift = Modifiers::COMMAND.plus(Modifiers::SHIFT);
        let (mods, key) = match self {
            Self::New => (cmd, Key::N),
            Self::Open => (cmd, Key::O),
            Self::Save => (cmd, Key::S),
            Self::SaveAs => (cmd_shift, Key::S),
            Self::Quit => (cmd, Key::Q),
            Self::Undo => (cmd, Key::Z),
            Self::Redo => (cmd_shift, Key::Z),
            Self::Cut => (cmd, Key::X),
            Self::Copy => (cmd, Key::C),
            Self::Paste => (cmd, Key::V),
            Self::SelectAll => (cmd, Key::A),
            Self::DuplicateLine => (Modifiers::ALT.plus(Modifiers::SHIFT), Key::ArrowDown),
            Self::DeleteLine => (cmd_shift, Key::K),
            Self::MoveLineUp => (Modifiers::ALT, Key::ArrowUp),
            Self::MoveLineDown => (Modifiers::ALT, Key::ArrowDown),
            Self::CloseTab => (cmd, Key::W),
            Self::ReopenTab => (cmd_shift, Key::T),
            Self::NextTab => (Modifiers::CTRL, Key::Tab),
            Self::PrevTab => (Modifiers::CTRL.plus(Modifiers::SHIFT), Key::Tab),
            Self::Find => (cmd, Key::F),
            Self::FindNext => (cmd, Key::G),
            Self::FindPrev => (cmd_shift, Key::G),
            Self::Replace if cfg!(target_os = "macos") => (cmd.plus(Modifiers::ALT), Key::F),
            Self::Replace => (cmd, Key::H),
            Self::Settings => (cmd, Key::Comma),
            Self::ZoomIn => (cmd, Key::Plus),
            Self::ZoomOut => (cmd, Key::Minus),
            Self::ZoomReset => (cmd, Key::Num0),
            Self::CommandPalette => (cmd_shift, Key::P),
        };
        Some(KeyboardShortcut::new(mods, key))
    }
}

/// Labels resolved by the caller (keeps this module free of l10n state).
pub type Label<'a> = &'a dyn Fn(&str) -> String;

const FILE_ITEMS: &[Option<MenuAction>] = &[
    Some(MenuAction::New),
    Some(MenuAction::Open),
    None,
    Some(MenuAction::Save),
    Some(MenuAction::SaveAs),
    None,
    Some(MenuAction::CloseTab),
    Some(MenuAction::ReopenTab),
];
const WINDOW_ITEMS: &[Option<MenuAction>] = &[Some(MenuAction::NextTab), Some(MenuAction::PrevTab)];
const VIEW_ITEMS: &[Option<MenuAction>] = &[
    Some(MenuAction::CommandPalette),
    None,
    Some(MenuAction::ZoomIn),
    Some(MenuAction::ZoomOut),
    Some(MenuAction::ZoomReset),
];
const EDIT_ITEMS: &[Option<MenuAction>] = &[
    Some(MenuAction::Undo),
    Some(MenuAction::Redo),
    None,
    Some(MenuAction::Cut),
    Some(MenuAction::Copy),
    Some(MenuAction::Paste),
    None,
    Some(MenuAction::SelectAll),
    None,
    Some(MenuAction::Find),
    Some(MenuAction::FindNext),
    Some(MenuAction::FindPrev),
    Some(MenuAction::Replace),
    None,
    Some(MenuAction::DuplicateLine),
    Some(MenuAction::DeleteLine),
    Some(MenuAction::MoveLineUp),
    Some(MenuAction::MoveLineDown),
];

/// Draw the in-window menu bar. Returns the chosen action, if any.
pub fn in_window(
    ui: &mut egui::Ui,
    label: Label<'_>,
    enabled: impl Fn(MenuAction) -> bool,
) -> Option<MenuAction> {
    let mut chosen = None;
    let ctx = ui.ctx().clone();
    egui::MenuBar::new().ui(ui, |ui| {
        let mut file: Vec<Option<MenuAction>> = FILE_ITEMS.to_vec();
        file.extend([
            None,
            Some(MenuAction::Settings),
            None,
            Some(MenuAction::Quit),
        ]);
        for (title, items) in [
            ("menu-file", file.as_slice()),
            ("menu-edit", EDIT_ITEMS),
            ("menu-view", VIEW_ITEMS),
            ("menu-window", WINDOW_ITEMS),
        ] {
            ui.menu_button(label(title), |ui| {
                ui.set_min_width(220.0);
                for item in items {
                    let Some(action) = item else {
                        ui.separator();
                        continue;
                    };
                    let mut button = egui::Button::new(label(action.label_id()));
                    if let Some(s) = action.shortcut() {
                        button = button.shortcut_text(ctx.format_shortcut(&s));
                    }
                    if ui.add_enabled(enabled(*action), button).clicked() {
                        chosen = Some(*action);
                        ui.close();
                    }
                }
            });
        }
    });
    chosen
}

#[cfg(target_os = "macos")]
pub use native::NativeMenu;

#[cfg(target_os = "macos")]
mod native {
    use std::sync::mpsc::{Receiver, channel};

    use muda::accelerator::{Key, KeyAccelerator, Modifiers, NamedKey};
    use muda::{Menu, MenuEvent, MenuItem, PredefinedMenuItem, Submenu};

    use super::{EDIT_ITEMS, FILE_ITEMS, Label, MenuAction, VIEW_ITEMS, WINDOW_ITEMS};

    /// The macOS menu bar. Keep alive for the lifetime of the app.
    pub struct NativeMenu {
        _menu: Menu,
        items: Vec<(MenuAction, MenuItem)>,
        rx: Receiver<MenuAction>,
    }

    /// Shortcuts as logical keys (the character a key produces), as macOS
    /// expects: physical key codes break punctuation shortcuts on non-US
    /// layouts (e.g. "=" is Shift+0 on Finnish keyboards).
    fn accelerator(action: MenuAction) -> KeyAccelerator {
        let cmd = Modifiers::META;
        let cmd_shift = Modifiers::META | Modifiers::SHIFT;
        let ch = |c: &str| Key::Character(c.into());
        let (m, k) = match action {
            MenuAction::New => (cmd, ch("n")),
            MenuAction::Open => (cmd, ch("o")),
            MenuAction::Save => (cmd, ch("s")),
            MenuAction::SaveAs => (cmd_shift, ch("s")),
            MenuAction::Quit => (cmd, ch("q")),
            MenuAction::Undo => (cmd, ch("z")),
            MenuAction::Redo => (cmd_shift, ch("z")),
            MenuAction::Cut => (cmd, ch("x")),
            MenuAction::Copy => (cmd, ch("c")),
            MenuAction::Paste => (cmd, ch("v")),
            MenuAction::SelectAll => (cmd, ch("a")),
            MenuAction::DuplicateLine => (
                Modifiers::ALT | Modifiers::SHIFT,
                Key::Named(NamedKey::ArrowDown),
            ),
            MenuAction::DeleteLine => (cmd_shift, ch("k")),
            MenuAction::MoveLineUp => (Modifiers::ALT, Key::Named(NamedKey::ArrowUp)),
            MenuAction::MoveLineDown => (Modifiers::ALT, Key::Named(NamedKey::ArrowDown)),
            MenuAction::CloseTab => (cmd, ch("w")),
            MenuAction::ReopenTab => (cmd_shift, ch("t")),
            MenuAction::NextTab => (Modifiers::CONTROL, Key::Named(NamedKey::Tab)),
            MenuAction::PrevTab => (
                Modifiers::CONTROL | Modifiers::SHIFT,
                Key::Named(NamedKey::Tab),
            ),
            MenuAction::Find => (cmd, ch("f")),
            MenuAction::FindNext => (cmd, ch("g")),
            MenuAction::FindPrev => (cmd_shift, ch("g")),
            MenuAction::Replace => (cmd | Modifiers::ALT, ch("f")),
            MenuAction::Settings => (cmd, ch(",")),
            MenuAction::ZoomIn => (cmd, ch("+")),
            MenuAction::ZoomOut => (cmd, ch("-")),
            MenuAction::ZoomReset => (cmd, ch("0")),
            MenuAction::CommandPalette => (cmd_shift, ch("p")),
        };
        KeyAccelerator::new(m, k)
    }

    impl NativeMenu {
        /// Build and install the menu bar. Call once the app has started.
        /// `wake` is called from the menu callback to repaint the UI.
        pub fn install(label: Label<'_>, wake: impl Fn() + Send + Sync + 'static) -> Option<Self> {
            let mut items = Vec::new();
            let mut make = |action: MenuAction| {
                let item = MenuItem::with_id(action.id(), label(action.label_id()), true, None);
                let _ = item.set_key_accelerator(Some(accelerator(action)));
                items.push((action, item.clone()));
                item
            };
            let section = |list: &[Option<MenuAction>],
                           make: &mut dyn FnMut(MenuAction) -> MenuItem| {
                list.iter()
                    .map(|i| -> Box<dyn muda::IsMenuItem> {
                        match i {
                            Some(a) => Box::new(make(*a)),
                            None => Box::new(PredefinedMenuItem::separator()),
                        }
                    })
                    .collect::<Vec<_>>()
            };

            let hide = label("app-hide");
            let hide_others = label("app-hide-others");
            let show_all = label("app-show-all");
            let quit = make(MenuAction::Quit);
            let settings = make(MenuAction::Settings);
            let app = Submenu::with_items(
                "Sanain",
                true,
                &[
                    &settings,
                    &PredefinedMenuItem::separator(),
                    &PredefinedMenuItem::hide(Some(&hide)),
                    &PredefinedMenuItem::hide_others(Some(&hide_others)),
                    &PredefinedMenuItem::show_all(Some(&show_all)),
                    &PredefinedMenuItem::separator(),
                    &quit,
                ],
            )
            .ok()?;
            let file_items = section(FILE_ITEMS, &mut make);
            let file_refs: Vec<&dyn muda::IsMenuItem> =
                file_items.iter().map(|b| b.as_ref()).collect();
            let file = Submenu::with_items(label("menu-file"), true, &file_refs).ok()?;
            let edit_items = section(EDIT_ITEMS, &mut make);
            let edit_refs: Vec<&dyn muda::IsMenuItem> =
                edit_items.iter().map(|b| b.as_ref()).collect();
            let edit = Submenu::with_items(label("menu-edit"), true, &edit_refs).ok()?;
            let view_items = section(VIEW_ITEMS, &mut make);
            let view_refs: Vec<&dyn muda::IsMenuItem> =
                view_items.iter().map(|b| b.as_ref()).collect();
            let view = Submenu::with_items(label("menu-view"), true, &view_refs).ok()?;
            let minimize = label("window-minimize");
            let zoom = label("window-zoom");
            let mut window_items = section(WINDOW_ITEMS, &mut make);
            window_items.push(Box::new(PredefinedMenuItem::separator()));
            window_items.push(Box::new(PredefinedMenuItem::minimize(Some(&minimize))));
            window_items.push(Box::new(PredefinedMenuItem::maximize(Some(&zoom))));
            let window_refs: Vec<&dyn muda::IsMenuItem> =
                window_items.iter().map(|b| b.as_ref()).collect();
            let window = Submenu::with_items(label("menu-window"), true, &window_refs).ok()?;
            let menu = Menu::with_items(&[&app, &file, &edit, &view, &window]).ok()?;
            menu.init_for_nsapp();
            window.set_as_windows_menu_for_nsapp();

            let (tx, rx) = channel();
            MenuEvent::set_event_handler(Some(std::sync::Arc::new(move |e: MenuEvent| {
                if let Some(a) = MenuAction::from_id(e.id().as_ref()) {
                    let _ = tx.send(a);
                    wake();
                }
            })));
            Some(Self {
                _menu: menu,
                items,
                rx,
            })
        }

        pub fn poll(&self) -> Vec<MenuAction> {
            self.rx.try_iter().collect()
        }

        pub fn set_enabled(&self, enabled: impl Fn(MenuAction) -> bool) {
            for (action, item) in &self.items {
                let want = enabled(*action);
                if item.is_enabled() != want {
                    item.set_enabled(want);
                }
            }
        }
    }
}
