//! Application menus. macOS gets the native menu bar; other platforms use
//! an in-window menu bar until native menus are verified there.

use eframe::egui::{self, Key, KeyboardShortcut, Modifiers};
use sanain_app::keymap::{Binding, Chord, Keymap, Mods};

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
    MoveTabLeft,
    MoveTabRight,
    Find,
    FindNext,
    FindPrev,
    Replace,
    Settings,
    ZoomIn,
    ZoomOut,
    ZoomReset,
    CommandPalette,
    MarkdownSource,
    MarkdownSplit,
    MarkdownPreview,
    JsonValidate,
    JsonFormat,
    JsonMinify,
    SortLines,
    SortLinesDesc,
    UniqueLines,
    TrimTrailing,
    UpperCase,
    LowerCase,
    ToLf,
    ToCrLf,
    CompareTabs,
    StructuredView,
    ReopenWithEncoding,
    SaveWithEncoding,
    Fold,
    Unfold,
    UnfoldAll,
    FindInFolder,
    CheckUpdates,
    LocalHistory,
    ClearHistory,
    OpenLog,
    PinTab,
    UnpinTab,
    AddCursorAbove,
    AddCursorBelow,
    WordWrap,
    Outline,
    GoToLine,
    GoToHeading,
    SelectNextOccurrence,
    SkipOccurrence,
    RecordMacro,
    PlayMacro,
    PlayMacroTimes,
    CompareSaved,
    CompareClipboard,
    CompareFile,
    SplitEditor,
    About,
    XmlValidate,
    YamlValidate,
    OpenRecent,
    SaveAll,
    Revert,
    ToggleComment,
    IndentLines,
    OutdentLines,
    CloseOthers,
    CloseAll,
    CopyPath,
    Reveal,
}

impl MenuAction {
    pub const ALL: [MenuAction; 86] = [
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
        Self::MoveTabLeft,
        Self::MoveTabRight,
        Self::Find,
        Self::FindNext,
        Self::FindPrev,
        Self::Replace,
        Self::Settings,
        Self::ZoomIn,
        Self::ZoomOut,
        Self::ZoomReset,
        Self::CommandPalette,
        Self::MarkdownSource,
        Self::MarkdownSplit,
        Self::MarkdownPreview,
        Self::JsonValidate,
        Self::JsonFormat,
        Self::JsonMinify,
        Self::SortLines,
        Self::SortLinesDesc,
        Self::UniqueLines,
        Self::TrimTrailing,
        Self::UpperCase,
        Self::LowerCase,
        Self::ToLf,
        Self::ToCrLf,
        Self::CompareTabs,
        Self::StructuredView,
        Self::ReopenWithEncoding,
        Self::SaveWithEncoding,
        Self::Fold,
        Self::Unfold,
        Self::UnfoldAll,
        Self::FindInFolder,
        Self::CheckUpdates,
        Self::LocalHistory,
        Self::ClearHistory,
        Self::OpenLog,
        Self::PinTab,
        Self::UnpinTab,
        Self::AddCursorAbove,
        Self::AddCursorBelow,
        Self::WordWrap,
        Self::Outline,
        Self::GoToLine,
        Self::GoToHeading,
        Self::SelectNextOccurrence,
        Self::SkipOccurrence,
        Self::RecordMacro,
        Self::PlayMacro,
        Self::PlayMacroTimes,
        Self::CompareSaved,
        Self::CompareClipboard,
        Self::CompareFile,
        Self::SplitEditor,
        Self::About,
        Self::XmlValidate,
        Self::YamlValidate,
        Self::OpenRecent,
        Self::SaveAll,
        Self::Revert,
        Self::ToggleComment,
        Self::IndentLines,
        Self::OutdentLines,
        Self::CloseOthers,
        Self::CloseAll,
        Self::CopyPath,
        Self::Reveal,
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
            Self::MoveTabLeft => "window.move-tab-left",
            Self::MoveTabRight => "window.move-tab-right",
            Self::Find => "edit.find",
            Self::FindNext => "edit.find-next",
            Self::FindPrev => "edit.find-prev",
            Self::Replace => "edit.replace",
            Self::Settings => "app.settings",
            Self::ZoomIn => "view.zoom-in",
            Self::ZoomOut => "view.zoom-out",
            Self::ZoomReset => "view.zoom-reset",
            Self::CommandPalette => "view.command-palette",
            Self::MarkdownSource => "view.markdown-source",
            Self::MarkdownSplit => "view.markdown-split",
            Self::MarkdownPreview => "view.markdown-preview",
            Self::JsonValidate => "tools.json-validate",
            Self::JsonFormat => "tools.json-format",
            Self::JsonMinify => "tools.json-minify",
            Self::SortLines => "tools.sort-lines",
            Self::SortLinesDesc => "tools.sort-lines-desc",
            Self::UniqueLines => "tools.unique-lines",
            Self::TrimTrailing => "tools.trim-trailing",
            Self::UpperCase => "tools.upper-case",
            Self::LowerCase => "tools.lower-case",
            Self::ToLf => "tools.to-lf",
            Self::ToCrLf => "tools.to-crlf",
            Self::CompareTabs => "tools.compare",
            Self::StructuredView => "view.structured",
            Self::ReopenWithEncoding => "file.reopen-encoding",
            Self::SaveWithEncoding => "file.save-encoding",
            Self::Fold => "view.fold",
            Self::Unfold => "view.unfold",
            Self::UnfoldAll => "view.unfold-all",
            Self::FindInFolder => "edit.find-in-folder",
            Self::CheckUpdates => "app.check-updates",
            Self::LocalHistory => "file.local-history",
            Self::ClearHistory => "file.clear-history",
            Self::OpenLog => "file.open-log",
            Self::PinTab => "window.pin-tab",
            Self::UnpinTab => "window.unpin-tab",
            Self::AddCursorAbove => "edit.add-cursor-above",
            Self::AddCursorBelow => "edit.add-cursor-below",
            Self::WordWrap => "view.word-wrap",
            Self::Outline => "view.outline",
            Self::GoToLine => "edit.go-to-line",
            Self::GoToHeading => "edit.go-to-heading",
            Self::SelectNextOccurrence => "edit.select-next-occurrence",
            Self::SkipOccurrence => "edit.skip-occurrence",
            Self::RecordMacro => "tools.record-macro",
            Self::PlayMacro => "tools.play-macro",
            Self::PlayMacroTimes => "tools.play-macro-times",
            Self::CompareSaved => "tools.compare-saved",
            Self::CompareClipboard => "tools.compare-clipboard",
            Self::CompareFile => "tools.compare-file",
            Self::SplitEditor => "view.split-editor",
            Self::About => "app.about",
            Self::XmlValidate => "tools.xml-validate",
            Self::YamlValidate => "tools.yaml-validate",
            Self::OpenRecent => "file.open-recent",
            Self::SaveAll => "file.save-all",
            Self::Revert => "file.revert",
            Self::ToggleComment => "edit.toggle-comment",
            Self::IndentLines => "edit.indent",
            Self::OutdentLines => "edit.outdent",
            Self::CloseOthers => "file.close-others",
            Self::CloseAll => "file.close-all",
            Self::CopyPath => "file.copy-path",
            Self::Reveal => "file.reveal",
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
            Self::MoveTabLeft => "window-move-tab-left",
            Self::MoveTabRight => "window-move-tab-right",
            Self::Find => "edit-find",
            Self::FindNext => "edit-find-next",
            Self::FindPrev => "edit-find-prev",
            Self::Replace => "edit-replace",
            Self::Settings => "app-settings",
            Self::ZoomIn => "view-zoom-in",
            Self::ZoomOut => "view-zoom-out",
            Self::ZoomReset => "view-zoom-reset",
            Self::CommandPalette => "view-command-palette",
            Self::MarkdownSource => "view-markdown-source",
            Self::MarkdownSplit => "view-markdown-split",
            Self::MarkdownPreview => "view-markdown-preview",
            Self::JsonValidate => "tools-json-validate",
            Self::JsonFormat => "tools-json-format",
            Self::JsonMinify => "tools-json-minify",
            Self::SortLines => "tools-sort-lines",
            Self::SortLinesDesc => "tools-sort-lines-desc",
            Self::UniqueLines => "tools-unique-lines",
            Self::TrimTrailing => "tools-trim-trailing",
            Self::UpperCase => "tools-upper-case",
            Self::LowerCase => "tools-lower-case",
            Self::ToLf => "tools-to-lf",
            Self::ToCrLf => "tools-to-crlf",
            Self::CompareTabs => "tools-compare",
            Self::StructuredView => "view-structured",
            Self::ReopenWithEncoding => "file-reopen-encoding",
            Self::SaveWithEncoding => "file-save-encoding",
            Self::Fold => "view-fold",
            Self::Unfold => "view-unfold",
            Self::UnfoldAll => "view-unfold-all",
            Self::FindInFolder => "edit-find-in-folder",
            Self::CheckUpdates => "app-check-updates",
            Self::LocalHistory => "file-local-history",
            Self::ClearHistory => "file-clear-history",
            Self::OpenLog => "file-open-log",
            Self::PinTab => "tab-pin",
            Self::UnpinTab => "tab-unpin",
            Self::AddCursorAbove => "edit-add-cursor-above",
            Self::AddCursorBelow => "edit-add-cursor-below",
            Self::WordWrap => "view-word-wrap",
            Self::Outline => "view-outline",
            Self::GoToLine => "edit-go-to-line",
            Self::GoToHeading => "edit-go-to-heading",
            Self::SelectNextOccurrence => "edit-select-next-occurrence",
            Self::SkipOccurrence => "edit-skip-occurrence",
            Self::RecordMacro => "tools-record-macro",
            Self::PlayMacro => "tools-play-macro",
            Self::PlayMacroTimes => "tools-play-macro-times",
            Self::CompareSaved => "tools-compare-saved",
            Self::CompareClipboard => "tools-compare-clipboard",
            Self::CompareFile => "tools-compare-file",
            Self::SplitEditor => "view-split-editor",
            Self::About => "app-about",
            Self::XmlValidate => "tools-xml-validate",
            Self::YamlValidate => "tools-yaml-validate",
            Self::OpenRecent => "file-open-recent",
            Self::SaveAll => "file-save-all",
            Self::Revert => "file-revert",
            Self::ToggleComment => "edit-toggle-comment",
            Self::IndentLines => "edit-indent",
            Self::OutdentLines => "edit-outdent",
            Self::CloseOthers => "tab-close-others",
            Self::CloseAll => "tab-close-all",
            Self::CopyPath => "file-copy-path",
            Self::Reveal => REVEAL_LABEL,
        }
    }

    /// The shortcut for this command under `keymap`.
    pub fn shortcut(self, keymap: &Keymap) -> Option<KeyboardShortcut> {
        match keymap.binding(self.id()) {
            Binding::Default => self.default_shortcut(),
            Binding::None => None,
            Binding::Chord(c) => chord_to_egui(&c),
        }
    }

    /// Commands that edit text: in a focused text field other than the
    /// editor they belong to that field.
    pub fn edits_text(self) -> bool {
        matches!(
            self,
            Self::Undo | Self::Redo | Self::SelectAll | Self::Cut | Self::Copy | Self::Paste
        )
    }

    /// Stable command id, also used as the keymap key.
    pub fn command_id(self) -> &'static str {
        self.id()
    }

    fn default_shortcut(self) -> Option<KeyboardShortcut> {
        let cmd = Modifiers::COMMAND;
        let cmd_shift = Modifiers::COMMAND.plus(Modifiers::SHIFT);
        let (mods, key) = match self {
            Self::New => (cmd, Key::N),
            Self::Open => (cmd, Key::O),
            Self::Save => (cmd, Key::S),
            Self::SaveAs => (cmd_shift, Key::S),
            Self::SaveAll => (cmd.plus(Modifiers::ALT), Key::S),
            Self::OpenRecent => (cmd_shift, Key::E),
            Self::Quit => (cmd, Key::Q),
            Self::Undo => (cmd, Key::Z),
            Self::Redo => (cmd_shift, Key::Z),
            Self::Cut => (cmd, Key::X),
            Self::Copy => (cmd, Key::C),
            Self::Paste => (cmd, Key::V),
            Self::SelectAll => (cmd, Key::A),
            Self::DuplicateLine => (Modifiers::ALT.plus(Modifiers::SHIFT), Key::ArrowDown),
            Self::AddCursorAbove => (cmd.plus(Modifiers::ALT), Key::ArrowUp),
            Self::AddCursorBelow => (cmd.plus(Modifiers::ALT), Key::ArrowDown),
            Self::DeleteLine => (cmd_shift, Key::K),
            Self::GoToLine => (cmd, Key::L),
            Self::SplitEditor => (cmd, Key::Backslash),
            Self::GoToHeading => (cmd_shift, Key::O),
            Self::SelectNextOccurrence => (cmd, Key::D),
            Self::SkipOccurrence => (cmd_shift, Key::D),
            Self::ToggleComment => (cmd, Key::Slash),
            Self::IndentLines => (cmd, Key::CloseBracket),
            Self::OutdentLines => (cmd, Key::OpenBracket),
            Self::RecordMacro => (cmd_shift, Key::R),
            Self::PlayMacro => (cmd_shift, Key::M),
            Self::MoveLineUp => (Modifiers::ALT, Key::ArrowUp),
            Self::MoveLineDown => (Modifiers::ALT, Key::ArrowDown),
            Self::CloseTab => (cmd, Key::W),
            Self::ReopenTab => (cmd_shift, Key::T),
            Self::NextTab => (Modifiers::CTRL, Key::Tab),
            Self::PrevTab => (Modifiers::CTRL.plus(Modifiers::SHIFT), Key::Tab),
            Self::MoveTabLeft => (Modifiers::CTRL.plus(Modifiers::SHIFT), Key::PageUp),
            Self::MoveTabRight => (Modifiers::CTRL.plus(Modifiers::SHIFT), Key::PageDown),
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
            Self::FindInFolder => (cmd_shift, Key::F),
            Self::MarkdownSplit => (cmd_shift, Key::V),
            Self::MarkdownSource | Self::MarkdownPreview => return None,
            Self::JsonValidate
            | Self::JsonFormat
            | Self::JsonMinify
            | Self::SortLines
            | Self::SortLinesDesc
            | Self::UniqueLines
            | Self::TrimTrailing
            | Self::UpperCase
            | Self::LowerCase
            | Self::ToLf
            | Self::ToCrLf
            | Self::CompareTabs
            | Self::StructuredView
            | Self::ReopenWithEncoding
            | Self::SaveWithEncoding
            | Self::Fold
            | Self::Unfold
            | Self::UnfoldAll
            | Self::CheckUpdates
            | Self::LocalHistory
            | Self::ClearHistory
            | Self::OpenLog
            | Self::PinTab
            | Self::UnpinTab
            | Self::WordWrap
            | Self::About
            | Self::XmlValidate
            | Self::YamlValidate
            | Self::Revert
            | Self::CloseOthers
            | Self::CloseAll
            | Self::CopyPath
            | Self::Reveal
            | Self::CompareSaved
            | Self::CompareClipboard
            | Self::CompareFile
            | Self::PlayMacroTimes
            | Self::Outline => return None,
        };
        Some(KeyboardShortcut::new(mods, key))
    }
}

pub fn chord_to_egui(c: &Chord) -> Option<KeyboardShortcut> {
    let mut m = Modifiers::NONE;
    if c.mods.command {
        m = m.plus(Modifiers::COMMAND);
    }
    if c.mods.ctrl {
        m = m.plus(Modifiers::CTRL);
    }
    if c.mods.alt {
        m = m.plus(Modifiers::ALT);
    }
    if c.mods.shift {
        m = m.plus(Modifiers::SHIFT);
    }
    Some(KeyboardShortcut::new(m, Key::from_name(&c.key)?))
}

/// The chord for a key press, if it makes a usable shortcut (a modifier or
/// a function key is required so plain typing is never captured).
pub fn chord_from_press(key: Key, m: Modifiers) -> Option<Chord> {
    let mods = Mods {
        command: m.command,
        ctrl: m.ctrl && cfg!(target_os = "macos"),
        alt: m.alt,
        shift: m.shift,
    };
    let name = key.name();
    let function_key =
        name.len() > 1 && name.starts_with('F') && name[1..].chars().all(|c| c.is_ascii_digit());
    if !(mods.command || mods.ctrl || mods.alt || function_key) {
        return None;
    }
    Chord::parse(
        &Chord {
            mods,
            key: name.to_owned(),
        }
        .format(),
    )
}

/// Labels resolved by the caller (keeps this module free of l10n state).
pub type Label<'a> = &'a dyn Fn(&str) -> String;

/// "Show in Finder/Explorer/File Manager", by platform.
pub const REVEAL_LABEL: &str = if cfg!(target_os = "macos") {
    "file-reveal-macos"
} else if cfg!(windows) {
    "file-reveal-windows"
} else {
    "file-reveal-other"
};

const FILE_ITEMS: &[Option<MenuAction>] = &[
    Some(MenuAction::New),
    Some(MenuAction::Open),
    Some(MenuAction::OpenRecent),
    Some(MenuAction::OpenLog),
    None,
    Some(MenuAction::Save),
    Some(MenuAction::SaveAs),
    Some(MenuAction::SaveAll),
    Some(MenuAction::Revert),
    Some(MenuAction::SaveWithEncoding),
    Some(MenuAction::ReopenWithEncoding),
    None,
    Some(MenuAction::LocalHistory),
    Some(MenuAction::ClearHistory),
    None,
    Some(MenuAction::CloseTab),
    Some(MenuAction::CloseOthers),
    Some(MenuAction::CloseAll),
    Some(MenuAction::ReopenTab),
    None,
    Some(MenuAction::CopyPath),
    Some(MenuAction::Reveal),
];
const WINDOW_ITEMS: &[Option<MenuAction>] = &[
    Some(MenuAction::NextTab),
    Some(MenuAction::PrevTab),
    Some(MenuAction::MoveTabLeft),
    Some(MenuAction::MoveTabRight),
    None,
    Some(MenuAction::PinTab),
    Some(MenuAction::UnpinTab),
];
const TOOLS_ITEMS: &[Option<MenuAction>] = &[
    Some(MenuAction::CompareTabs),
    Some(MenuAction::CompareSaved),
    Some(MenuAction::CompareClipboard),
    Some(MenuAction::CompareFile),
    None,
    Some(MenuAction::RecordMacro),
    Some(MenuAction::PlayMacro),
    Some(MenuAction::PlayMacroTimes),
    None,
    Some(MenuAction::JsonValidate),
    Some(MenuAction::JsonFormat),
    Some(MenuAction::JsonMinify),
    Some(MenuAction::XmlValidate),
    Some(MenuAction::YamlValidate),
    None,
    Some(MenuAction::SortLines),
    Some(MenuAction::SortLinesDesc),
    Some(MenuAction::UniqueLines),
    Some(MenuAction::TrimTrailing),
    None,
    Some(MenuAction::UpperCase),
    Some(MenuAction::LowerCase),
    None,
    Some(MenuAction::ToLf),
    Some(MenuAction::ToCrLf),
];
const VIEW_ITEMS: &[Option<MenuAction>] = &[
    Some(MenuAction::CommandPalette),
    None,
    Some(MenuAction::MarkdownSource),
    Some(MenuAction::MarkdownSplit),
    Some(MenuAction::MarkdownPreview),
    Some(MenuAction::StructuredView),
    None,
    Some(MenuAction::Fold),
    Some(MenuAction::Unfold),
    Some(MenuAction::UnfoldAll),
    Some(MenuAction::WordWrap),
    Some(MenuAction::Outline),
    Some(MenuAction::SplitEditor),
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
    Some(MenuAction::FindInFolder),
    None,
    Some(MenuAction::GoToLine),
    Some(MenuAction::GoToHeading),
    None,
    Some(MenuAction::ToggleComment),
    Some(MenuAction::IndentLines),
    Some(MenuAction::OutdentLines),
    Some(MenuAction::DuplicateLine),
    Some(MenuAction::DeleteLine),
    Some(MenuAction::MoveLineUp),
    Some(MenuAction::MoveLineDown),
    None,
    Some(MenuAction::AddCursorAbove),
    Some(MenuAction::AddCursorBelow),
    Some(MenuAction::SelectNextOccurrence),
    Some(MenuAction::SkipOccurrence),
];

/// Draw the in-window menu bar. Returns the chosen action, if any.
pub fn in_window(
    ui: &mut egui::Ui,
    label: Label<'_>,
    enabled: impl Fn(MenuAction) -> bool,
    keymap: &Keymap,
) -> Option<MenuAction> {
    let mut chosen = None;
    let ctx = ui.ctx().clone();
    egui::MenuBar::new().ui(ui, |ui| {
        let mut file: Vec<Option<MenuAction>> = FILE_ITEMS.to_vec();
        file.extend([
            None,
            Some(MenuAction::Settings),
            Some(MenuAction::CheckUpdates),
            None,
            Some(MenuAction::Quit),
        ]);
        for (title, items) in [
            ("menu-file", file.as_slice()),
            ("menu-edit", EDIT_ITEMS),
            ("menu-view", VIEW_ITEMS),
            ("menu-tools", TOOLS_ITEMS),
            ("menu-window", WINDOW_ITEMS),
            ("menu-help", &[Some(MenuAction::About)]),
        ] {
            ui.menu_button(label(title), |ui| {
                ui.set_min_width(220.0);
                for item in items {
                    let Some(action) = item else {
                        ui.separator();
                        continue;
                    };
                    let mut button = egui::Button::new(label(action.label_id()));
                    if let Some(s) = action.shortcut(keymap) {
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

    use super::{EDIT_ITEMS, FILE_ITEMS, Label, MenuAction, TOOLS_ITEMS, VIEW_ITEMS, WINDOW_ITEMS};

    /// The macOS menu bar. Keep alive for the lifetime of the app.
    pub struct NativeMenu {
        _menu: Menu,
        items: Vec<(MenuAction, MenuItem)>,
        rx: Receiver<MenuAction>,
    }

    /// Shortcuts as logical keys (the character a key produces), as macOS
    /// expects: physical key codes break punctuation shortcuts on non-US
    /// layouts (e.g. "=" is Shift+0 on Finnish keyboards).
    fn accelerator(action: MenuAction, keymap: &super::Keymap) -> Option<KeyAccelerator> {
        match keymap.binding(action.id()) {
            super::Binding::Default => default_accelerator(action),
            super::Binding::None => None,
            super::Binding::Chord(c) => chord_accelerator(&c),
        }
    }

    fn chord_accelerator(c: &super::Chord) -> Option<KeyAccelerator> {
        let mut m = Modifiers::empty();
        if c.mods.command {
            m |= Modifiers::META;
        }
        if c.mods.ctrl {
            m |= Modifiers::CONTROL;
        }
        if c.mods.alt {
            m |= Modifiers::ALT;
        }
        if c.mods.shift {
            m |= Modifiers::SHIFT;
        }
        let named = |n| Some(Key::Named(n));
        let key = match c.key.as_str() {
            "ArrowUp" => named(NamedKey::ArrowUp),
            "ArrowDown" => named(NamedKey::ArrowDown),
            "ArrowLeft" => named(NamedKey::ArrowLeft),
            "ArrowRight" => named(NamedKey::ArrowRight),
            "Tab" => named(NamedKey::Tab),
            "Enter" => named(NamedKey::Enter),
            "Escape" => named(NamedKey::Escape),
            "Backspace" => named(NamedKey::Backspace),
            "Delete" => named(NamedKey::Delete),
            "Home" => named(NamedKey::Home),
            "End" => named(NamedKey::End),
            "PageUp" => named(NamedKey::PageUp),
            "PageDown" => named(NamedKey::PageDown),
            "F1" => named(NamedKey::F1),
            "F2" => named(NamedKey::F2),
            "F3" => named(NamedKey::F3),
            "F4" => named(NamedKey::F4),
            "F5" => named(NamedKey::F5),
            "F6" => named(NamedKey::F6),
            "F7" => named(NamedKey::F7),
            "F8" => named(NamedKey::F8),
            "F9" => named(NamedKey::F9),
            "F10" => named(NamedKey::F10),
            "F11" => named(NamedKey::F11),
            "F12" => named(NamedKey::F12),
            "Plus" => Some(Key::Character("+".into())),
            "Minus" => Some(Key::Character("-".into())),
            "Equals" => Some(Key::Character("=".into())),
            "Comma" => Some(Key::Character(",".into())),
            "Period" => Some(Key::Character(".".into())),
            "Slash" => Some(Key::Character("/".into())),
            "Semicolon" => Some(Key::Character(";".into())),
            "OpenBracket" => Some(Key::Character("[".into())),
            "CloseBracket" => Some(Key::Character("]".into())),
            "Space" => Some(Key::Character(" ".into())),
            k if k.len() == 1 => Some(Key::Character(k.to_ascii_lowercase())),
            _ => None,
        }?;
        Some(KeyAccelerator::new(m, key))
    }

    fn default_accelerator(action: MenuAction) -> Option<KeyAccelerator> {
        let cmd = Modifiers::META;
        let cmd_shift = Modifiers::META | Modifiers::SHIFT;
        let ch = |c: &str| Key::Character(c.into());
        let (m, k) = match action {
            MenuAction::New => (cmd, ch("n")),
            MenuAction::Open => (cmd, ch("o")),
            MenuAction::Save => (cmd, ch("s")),
            MenuAction::SaveAs => (cmd_shift, ch("s")),
            MenuAction::SaveAll => (cmd | Modifiers::ALT, ch("s")),
            MenuAction::OpenRecent => (cmd_shift, ch("e")),
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
            MenuAction::GoToLine => (cmd, ch("l")),
            MenuAction::SplitEditor => (cmd, ch("\\")),
            MenuAction::GoToHeading => (cmd_shift, ch("o")),
            MenuAction::SelectNextOccurrence => (cmd, ch("d")),
            MenuAction::SkipOccurrence => (cmd_shift, ch("d")),
            MenuAction::ToggleComment => (cmd, ch("/")),
            MenuAction::IndentLines => (cmd, ch("]")),
            MenuAction::OutdentLines => (cmd, ch("[")),
            MenuAction::RecordMacro => (cmd_shift, ch("r")),
            MenuAction::PlayMacro => (cmd_shift, ch("m")),
            MenuAction::AddCursorAbove => (cmd | Modifiers::ALT, Key::Named(NamedKey::ArrowUp)),
            MenuAction::AddCursorBelow => (cmd | Modifiers::ALT, Key::Named(NamedKey::ArrowDown)),
            MenuAction::MoveLineUp => (Modifiers::ALT, Key::Named(NamedKey::ArrowUp)),
            MenuAction::MoveLineDown => (Modifiers::ALT, Key::Named(NamedKey::ArrowDown)),
            MenuAction::CloseTab => (cmd, ch("w")),
            MenuAction::ReopenTab => (cmd_shift, ch("t")),
            MenuAction::NextTab => (Modifiers::CONTROL, Key::Named(NamedKey::Tab)),
            MenuAction::PrevTab => (
                Modifiers::CONTROL | Modifiers::SHIFT,
                Key::Named(NamedKey::Tab),
            ),
            MenuAction::MoveTabLeft => (
                Modifiers::CONTROL | Modifiers::SHIFT,
                Key::Named(NamedKey::PageUp),
            ),
            MenuAction::MoveTabRight => (
                Modifiers::CONTROL | Modifiers::SHIFT,
                Key::Named(NamedKey::PageDown),
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
            MenuAction::FindInFolder => (cmd_shift, ch("f")),
            MenuAction::MarkdownSplit => (cmd_shift, ch("v")),
            MenuAction::MarkdownSource | MenuAction::MarkdownPreview => return None,
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
            | MenuAction::ToCrLf
            | MenuAction::CompareTabs
            | MenuAction::StructuredView
            | MenuAction::ReopenWithEncoding
            | MenuAction::SaveWithEncoding
            | MenuAction::Fold
            | MenuAction::Unfold
            | MenuAction::UnfoldAll
            | MenuAction::CheckUpdates
            | MenuAction::LocalHistory
            | MenuAction::ClearHistory
            | MenuAction::OpenLog
            | MenuAction::PinTab
            | MenuAction::UnpinTab
            | MenuAction::WordWrap
            | MenuAction::About
            | MenuAction::XmlValidate
            | MenuAction::YamlValidate
            | MenuAction::Revert
            | MenuAction::CloseOthers
            | MenuAction::CloseAll
            | MenuAction::CopyPath
            | MenuAction::Reveal
            | MenuAction::CompareSaved
            | MenuAction::CompareClipboard
            | MenuAction::CompareFile
            | MenuAction::PlayMacroTimes
            | MenuAction::Outline => return None,
        };
        Some(KeyAccelerator::new(m, k))
    }

    impl NativeMenu {
        /// Build and install the menu bar. Call once the app has started.
        /// `wake` is called from the menu callback to repaint the UI.
        pub fn install(
            label: Label<'_>,
            keymap: &super::Keymap,
            wake: impl Fn() + Send + Sync + 'static,
        ) -> Option<Self> {
            let mut items = Vec::new();
            let mut make = |action: MenuAction| {
                let item = MenuItem::with_id(action.id(), label(action.label_id()), true, None);
                let _ = item.set_key_accelerator(accelerator(action, keymap));
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
            let updates = make(MenuAction::CheckUpdates);
            let about = make(MenuAction::About);
            let app = Submenu::with_items(
                "Sanain",
                true,
                &[
                    &about,
                    &PredefinedMenuItem::separator(),
                    &settings,
                    &updates,
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
            let tools_items = section(TOOLS_ITEMS, &mut make);
            let tools_refs: Vec<&dyn muda::IsMenuItem> =
                tools_items.iter().map(|b| b.as_ref()).collect();
            let tools = Submenu::with_items(label("menu-tools"), true, &tools_refs).ok()?;
            let minimize = label("window-minimize");
            let zoom = label("window-zoom");
            let mut window_items = section(WINDOW_ITEMS, &mut make);
            window_items.push(Box::new(PredefinedMenuItem::separator()));
            window_items.push(Box::new(PredefinedMenuItem::minimize(Some(&minimize))));
            window_items.push(Box::new(PredefinedMenuItem::maximize(Some(&zoom))));
            let window_refs: Vec<&dyn muda::IsMenuItem> =
                window_items.iter().map(|b| b.as_ref()).collect();
            let window = Submenu::with_items(label("menu-window"), true, &window_refs).ok()?;
            let menu = Menu::with_items(&[&app, &file, &edit, &view, &tools, &window]).ok()?;
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

        /// Apply a (changed) keymap to the menu items' shortcuts.
        pub fn apply_keymap(&self, keymap: &super::Keymap) {
            for (action, item) in &self.items {
                let _ = item.set_key_accelerator(accelerator(*action, keymap));
            }
        }

        /// Remove all shortcuts (while the user records a new one).
        pub fn clear_shortcuts(&self) {
            for (_, item) in &self.items {
                let _ = item.set_key_accelerator(None);
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use sanain_app::keymap::{Keymap, Preset};

    /// No two commands share a default shortcut in any preset, on the
    /// platform the tests run on (CI covers macOS, Windows and Linux).
    #[test]
    fn default_shortcuts_are_unique() {
        for preset in [Preset::Default, Preset::NotepadPlusPlus] {
            let keymap = Keymap {
                preset,
                ..Keymap::default()
            };
            let mut seen: Vec<(KeyboardShortcut, MenuAction)> = Vec::new();
            for a in MenuAction::ALL {
                if let Some(s) = a.shortcut(&keymap) {
                    if let Some((_, other)) = seen.iter().find(|(k, _)| *k == s) {
                        panic!("{preset:?}: {a:?} and {other:?} share {s:?}");
                    }
                    seen.push((s, a));
                }
            }
        }
    }
}
