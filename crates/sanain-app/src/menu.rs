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
}

impl MenuAction {
    const ALL: [MenuAction; 23] = [
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
        file.extend([None, Some(MenuAction::Quit)]);
        for (title, items) in [
            ("menu-file", file.as_slice()),
            ("menu-edit", EDIT_ITEMS),
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

    use muda::accelerator::{Accelerator, Code, Modifiers};
    use muda::{Menu, MenuEvent, MenuItem, PredefinedMenuItem, Submenu};

    use super::{EDIT_ITEMS, FILE_ITEMS, Label, MenuAction, WINDOW_ITEMS};

    /// The macOS menu bar. Keep alive for the lifetime of the app.
    pub struct NativeMenu {
        _menu: Menu,
        items: Vec<(MenuAction, MenuItem)>,
        rx: Receiver<MenuAction>,
    }

    fn accelerator(action: MenuAction) -> Option<Accelerator> {
        let cmd = Modifiers::META;
        let cmd_shift = Modifiers::META | Modifiers::SHIFT;
        let (m, k) = match action {
            MenuAction::New => (cmd, Code::KeyN),
            MenuAction::Open => (cmd, Code::KeyO),
            MenuAction::Save => (cmd, Code::KeyS),
            MenuAction::SaveAs => (cmd_shift, Code::KeyS),
            MenuAction::Quit => (cmd, Code::KeyQ),
            MenuAction::Undo => (cmd, Code::KeyZ),
            MenuAction::Redo => (cmd_shift, Code::KeyZ),
            MenuAction::Cut => (cmd, Code::KeyX),
            MenuAction::Copy => (cmd, Code::KeyC),
            MenuAction::Paste => (cmd, Code::KeyV),
            MenuAction::SelectAll => (cmd, Code::KeyA),
            MenuAction::DuplicateLine => (Modifiers::ALT | Modifiers::SHIFT, Code::ArrowDown),
            MenuAction::DeleteLine => (cmd_shift, Code::KeyK),
            MenuAction::MoveLineUp => (Modifiers::ALT, Code::ArrowUp),
            MenuAction::MoveLineDown => (Modifiers::ALT, Code::ArrowDown),
            MenuAction::CloseTab => (cmd, Code::KeyW),
            MenuAction::ReopenTab => (cmd_shift, Code::KeyT),
            MenuAction::NextTab => (Modifiers::CONTROL, Code::Tab),
            MenuAction::PrevTab => (Modifiers::CONTROL | Modifiers::SHIFT, Code::Tab),
            MenuAction::Find => (cmd, Code::KeyF),
            MenuAction::FindNext => (cmd, Code::KeyG),
            MenuAction::FindPrev => (cmd_shift, Code::KeyG),
            MenuAction::Replace => (cmd | Modifiers::ALT, Code::KeyF),
        };
        Some(Accelerator::new(m, k))
    }

    impl NativeMenu {
        /// Build and install the menu bar. Call once the app has started.
        /// `wake` is called from the menu callback to repaint the UI.
        pub fn install(label: Label<'_>, wake: impl Fn() + Send + Sync + 'static) -> Option<Self> {
            let mut items = Vec::new();
            let mut make = |action: MenuAction| {
                let item = MenuItem::with_id(
                    action.id(),
                    label(action.label_id()),
                    true,
                    accelerator(action),
                );
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
            let app = Submenu::with_items(
                "Sanain",
                true,
                &[
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
            let minimize = label("window-minimize");
            let zoom = label("window-zoom");
            let mut window_items = section(WINDOW_ITEMS, &mut make);
            window_items.push(Box::new(PredefinedMenuItem::separator()));
            window_items.push(Box::new(PredefinedMenuItem::minimize(Some(&minimize))));
            window_items.push(Box::new(PredefinedMenuItem::maximize(Some(&zoom))));
            let window_refs: Vec<&dyn muda::IsMenuItem> =
                window_items.iter().map(|b| b.as_ref()).collect();
            let window = Submenu::with_items(label("menu-window"), true, &window_refs).ok()?;
            let menu = Menu::with_items(&[&app, &file, &edit, &window]).ok()?;
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
