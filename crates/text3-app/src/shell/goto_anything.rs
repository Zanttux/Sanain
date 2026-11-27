//! Goto Anything (Cmd/Ctrl+P): the palette popup listing open tabs,
//! recent files and the project folder's files, with `:line`, `@symbol`
//! and `#text` modes. Parsing, ranking and the file listing live in
//! [`text3_app::goto`].

use std::path::PathBuf;

use text3_app::filesearch::WalkEnd;
use text3_app::fuzzy;
use text3_app::goto::{self, Candidate, LineHit, Query, Source};

use super::{Entry, Lang, Revision, Shell, TabId, egui, short_dir};
use crate::palette::Mode;

/// Symbols of the active tab: (depth, name, 0-based line).
type Symbols = Vec<(usize, String, usize)>;

#[derive(Default)]
pub(super) struct GotoState {
    scan: Option<goto::Scan>,
    cands: Vec<Candidate>,
    /// The scan generation `cands` were built for.
    cands_gen: Option<u64>,
    /// The query `ranked` is for, and the ranking.
    ranked: Option<(String, Vec<usize>)>,
    symbols: Option<((TabId, Revision), Symbols)>,
    highlighter: Option<text3_language::Highlighter>,
    hits: Option<((TabId, Revision, String), Vec<LineHit>)>,
}

/// What a row does when chosen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pick {
    /// A candidate file, by index.
    File(usize),
    Line(usize, usize),
    /// A symbol or heading on this 0-based line.
    At(usize),
    /// A `#` match, by index.
    Hit(usize),
}

impl Shell {
    /// The folder Goto Anything lists: the Find in Folder folder, else
    /// the active file's folder.
    fn goto_root(&self) -> Option<PathBuf> {
        self.folder.root.clone().filter(|r| r.is_dir()).or_else(|| {
            self.editor()
                .file()
                .and_then(|f| f.path.parent().map(std::path::Path::to_path_buf))
        })
    }

    /// Open the popup and start listing the project folder afresh.
    pub(super) fn start_goto(&mut self) {
        let highlighter = self.goto.highlighter.take();
        self.goto = GotoState {
            highlighter,
            ..GotoState::default()
        };
        if let Some(root) = self.goto_root() {
            let ctx = self.ctx.clone();
            self.goto.scan = Some(goto::Scan::start(&root, goto::MAX_FILES, move || {
                ctx.request_repaint()
            }));
        }
        self.palette.open(Mode::Anything);
    }

    /// One frame of the popup.
    pub(super) fn goto_anything(&mut self, ctx: &egui::Context) {
        let query = self.palette.query().to_owned();
        let parsed = goto::parse(&query);
        let (entries, note) = match parsed {
            Query::Line(target) => self.goto_line_entries(target),
            Query::Files { text, .. } => self.goto_file_entries(text),
            Query::Symbol(text) => self.goto_symbol_entries(text),
            Query::Text(text) => self.goto_text_entries(text),
        };
        let ranked: Vec<usize> = (0..entries.len()).collect();
        let hint = self.t("goto-anything-hint", &[]);
        let picked = self
            .palette
            .show_ranked(ctx, &entries, &ranked, &hint, note.as_deref());
        if let Some(pick) = picked {
            let line = match parsed {
                Query::Files { line, .. } => line,
                _ => None,
            };
            let other_pane = self.palette.command_enter;
            self.goto_pick(pick, line, other_pane);
        }
        if !self.palette.open {
            self.end_goto();
        }
    }

    /// Stop listing and free the lists (the popup closed, or the palette
    /// switched to another mode).
    pub(super) fn end_goto(&mut self) {
        if self.goto.scan.is_none() && self.goto.cands.is_empty() {
            return;
        }
        let highlighter = self.goto.highlighter.take();
        self.goto = GotoState {
            highlighter,
            ..GotoState::default()
        };
    }

    fn goto_line_entries(
        &mut self,
        target: Option<(usize, usize)>,
    ) -> (Vec<Entry<Pick>>, Option<String>) {
        let lines = match &self.log {
            Some(log) => log.line_count() as i64,
            None => self.editor().document().rope().len_lines() as i64,
        };
        let note = self.t("goto-line-note", &[("count", lines.into())]);
        let entries = target
            .map(|(line, col)| Entry {
                action: Pick::Line(line, col),
                label: self.t("goto-anything-line", &[("line", (line as i64 + 1).into())]),
                shortcut: String::new(),
                enabled: true,
            })
            .into_iter()
            .collect();
        (entries, Some(note))
    }

    fn goto_file_entries(&mut self, text: &str) -> (Vec<Entry<Pick>>, Option<String>) {
        if let Some(scan) = &mut self.goto.scan {
            scan.poll();
        }
        let generation = self.goto.scan.as_ref().map_or(0, |s| s.generation);
        if self.goto.cands_gen != Some(generation) {
            let tabs: Vec<(TabId, Option<PathBuf>, String)> = self
                .ws
                .tabs()
                .iter()
                .map(|t| {
                    (
                        t.id,
                        t.editor.file().map(|f| f.path.clone()),
                        self.name_of(&t.editor),
                    )
                })
                .collect();
            let recent = self.recent.existing();
            let (root, files) = match &self.goto.scan {
                Some(s) => (Some(s.root.as_path()), s.files.as_slice()),
                None => (None, &[][..]),
            };
            self.goto.cands = goto::candidates(&tabs, &recent, root, files);
            self.goto.cands_gen = Some(generation);
            self.goto.ranked = None;
        }
        if self.goto.ranked.as_ref().is_none_or(|(q, _)| q != text) {
            let ranked = goto::rank(text, &self.goto.cands);
            self.goto.ranked = Some((text.to_owned(), ranked));
        }
        let open = self.t("goto-anything-open", &[]);
        let entries = self
            .goto
            .ranked
            .as_ref()
            .map(|(_, r)| r.as_slice())
            .unwrap_or_default()
            .iter()
            .filter_map(|&i| {
                let c = self.goto.cands.get(i)?;
                let dir = if std::path::Path::new(&c.rel).is_absolute() {
                    c.path
                        .as_deref()
                        .and_then(std::path::Path::parent)
                        .map(short_dir)
                        .unwrap_or_default()
                } else {
                    c.dir().to_owned()
                };
                let shortcut = match (c.source, dir.is_empty()) {
                    (Source::Tab(_), true) => open.clone(),
                    (Source::Tab(_), false) => format!("{dir} · {open}"),
                    _ => dir,
                };
                Some(Entry {
                    action: Pick::File(i),
                    label: c.name.clone(),
                    shortcut,
                    enabled: true,
                })
            })
            .collect();
        let note = match &self.goto.scan {
            None => Some(self.t("goto-anything-no-folder", &[])),
            Some(s) => {
                let folder = short_dir(&s.root);
                match s.end {
                    None => Some(self.t("goto-anything-scanning", &[("folder", folder.into())])),
                    Some(WalkEnd::Limit) => Some(self.t(
                        "goto-anything-limit",
                        &[
                            ("max", (goto::MAX_FILES as i64).into()),
                            ("folder", folder.into()),
                        ],
                    )),
                    Some(_) => Some(self.t(
                        "goto-anything-scanned",
                        &[
                            ("count", (s.files.len() as i64).into()),
                            ("folder", folder.into()),
                        ],
                    )),
                }
            }
        };
        (entries, note)
    }

    /// Symbols of the active tab (Markdown headings, or Tree-sitter
    /// functions and types), once per revision.
    fn goto_symbols(&mut self) -> Symbols {
        let id = self.ws.active().id;
        let key = (id, self.editor().document().revision());
        if let Some((k, s)) = &self.goto.symbols
            && *k == key
        {
            return s.clone();
        }
        let lang = self.language(id, self.editor());
        let symbols: Symbols = if Self::is_markdown(self.editor()) || lang == Some(Lang::Markdown) {
            self.outline()
                .into_iter()
                .map(|(level, title, line)| {
                    (
                        usize::from(level.saturating_sub(1)),
                        title,
                        line.saturating_sub(1),
                    )
                })
                .collect()
        } else if let Some(lang) = lang
            && self.editor().document().rope().len_bytes() <= text3_app::syntax::MAX_BYTES
        {
            let text = self.editor().document().text();
            self.goto
                .highlighter
                .get_or_insert_with(text3_language::Highlighter::new)
                .symbols(lang, text.as_bytes(), None)
                .unwrap_or_default()
                .into_iter()
                .map(|s| (s.depth, s.name, s.line))
                .collect()
        } else {
            Vec::new()
        };
        self.goto.symbols = Some((key, symbols.clone()));
        symbols
    }

    fn goto_symbol_entries(&mut self, text: &str) -> (Vec<Entry<Pick>>, Option<String>) {
        if self.log.is_some() {
            return (Vec::new(), Some(self.t("goto-anything-symbols-none", &[])));
        }
        let symbols = self.goto_symbols();
        if symbols.is_empty() {
            return (Vec::new(), Some(self.t("goto-anything-symbols-none", &[])));
        }
        let names: Vec<&str> = symbols.iter().map(|(_, n, _)| n.as_str()).collect();
        let mut ranked = fuzzy::rank(text, &names);
        ranked.truncate(goto::MAX_RESULTS);
        // Indent by nesting only while the list is in document order.
        let indent = text.is_empty();
        let entries = ranked
            .into_iter()
            .map(|i| {
                let (depth, name, line) = &symbols[i];
                Entry {
                    action: Pick::At(*line),
                    label: if indent {
                        format!("{}{name}", "    ".repeat(*depth))
                    } else {
                        name.clone()
                    },
                    shortcut: format!("{}", line + 1),
                    enabled: true,
                }
            })
            .collect();
        (entries, None)
    }

    fn goto_text_entries(&mut self, text: &str) -> (Vec<Entry<Pick>>, Option<String>) {
        if text.is_empty() || self.log.is_some() {
            return (Vec::new(), None);
        }
        let key = (
            self.ws.active().id,
            self.editor().document().revision(),
            text.to_owned(),
        );
        if self.goto.hits.as_ref().is_none_or(|(k, _)| *k != key) {
            let doc = self.editor().document().text();
            self.goto.hits = Some((key, goto::find_lines(&doc, text)));
        }
        let hits = self
            .goto
            .hits
            .as_ref()
            .map(|(_, h)| h.as_slice())
            .unwrap_or_default();
        if hits.is_empty() {
            return (Vec::new(), Some(self.t("goto-anything-matches-none", &[])));
        }
        let entries = hits
            .iter()
            .enumerate()
            .map(|(i, h)| Entry {
                action: Pick::Hit(i),
                label: h.preview.trim().to_owned(),
                shortcut: format!("{}", h.line + 1),
                enabled: true,
            })
            .collect();
        (entries, None)
    }

    fn goto_pick(&mut self, pick: Pick, line: Option<(usize, usize)>, other_pane: bool) {
        let id = self.ws.active().id;
        match pick {
            Pick::File(i) => {
                let Some(c) = self.goto.cands.get(i).cloned() else {
                    return;
                };
                // Cmd/Ctrl+Enter: open in the next pane of a split.
                if other_pane && let Some(split) = &mut self.split {
                    split.focus_next();
                    self.activate_focused_pane();
                }
                let tab = match c.source {
                    Source::Tab(tab) => self.ws.position(tab),
                    _ => None,
                };
                if let Some(pos) = tab {
                    self.ws.activate(pos);
                    self.focus_active();
                } else if let Some(p) = &c.path {
                    self.open(p);
                }
                let arrived = match (c.source, &c.path) {
                    (Source::Tab(tab), _) if tab == self.ws.active().id => true,
                    (_, Some(p)) => self.editor().file().is_some_and(|f| {
                        f.path == *p || f.path.canonicalize().ok().as_ref() == Some(p)
                    }),
                    _ => false,
                };
                if arrived && let Some((l, col)) = line {
                    self.editor_mut().go_to(l, col);
                    let id = self.ws.active().id;
                    self.active_view(id).reveal();
                }
            }
            Pick::Line(l, col) => {
                if let Some(log) = &mut self.log {
                    log.jump = Some(l);
                } else {
                    self.editor_mut().go_to(l, col);
                    self.active_view(id).reveal();
                }
            }
            Pick::At(l) => {
                self.editor_mut().go_to(l, 0);
                self.active_view(id).reveal();
            }
            Pick::Hit(i) => {
                let Some(chars) = self
                    .goto
                    .hits
                    .as_ref()
                    .and_then(|(_, h)| h.get(i))
                    .map(|h| h.chars.clone())
                else {
                    return;
                };
                self.editor_mut().select(chars);
                self.active_view(id).reveal();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::menu::MenuAction;
    use text3_app::settings::Settings;
    use text3_app::workspace::Workspace;
    use text3_i18n::Localizer;

    struct Harness {
        shell: Shell,
        ctx: egui::Context,
        time: f64,
    }

    impl Harness {
        fn new() -> Self {
            let shell = Shell::new(
                Workspace::new(None),
                Localizer::new(&["en-US"]).unwrap(),
                Vec::new(),
                0,
                None,
                Settings::default(),
                None,
            );
            let ctx = egui::Context::default();
            ctx.enable_accesskit();
            Self {
                shell,
                ctx,
                time: 0.0,
            }
        }

        /// One frame of the popup; returns the AccessKit update.
        fn frame(&mut self, events: Vec<egui::Event>) -> Option<egui::accesskit::TreeUpdate> {
            self.time += 0.05;
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1000.0, 700.0),
                )),
                time: Some(self.time),
                events,
                ..Default::default()
            };
            let shell = &mut self.shell;
            let mut out = self.ctx.run_ui(input, |ui| {
                let ctx = ui.ctx().clone();
                if shell.palette.open {
                    shell.goto_palette(&ctx);
                }
            });
            out.textures_delta.clear();
            out.platform_output.accesskit_update
        }

        fn open(&mut self) {
            self.shell
                .dispatch(MenuAction::GotoAnything, &self.ctx.clone());
            assert!(self.shell.palette.open);
            assert_eq!(self.shell.palette.mode, Mode::Anything);
            self.frame(Vec::new());
            self.frame(Vec::new());
        }

        fn type_text(&mut self, s: &str) {
            self.frame(vec![egui::Event::Text(s.into())]);
            self.frame(Vec::new());
        }

        fn key(&mut self, key: egui::Key, modifiers: egui::Modifiers) {
            let ev = |pressed| egui::Event::Key {
                key,
                physical_key: None,
                pressed,
                repeat: false,
                modifiers,
            };
            let held = |m| egui::Event::ModifiersChanged(m);
            self.frame(vec![held(modifiers), ev(true)]);
            self.frame(vec![ev(false), held(egui::Modifiers::NONE)]);
        }

        /// Frames until the folder listing is done.
        fn settle(&mut self) {
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(20);
            while self
                .shell
                .goto
                .scan
                .as_ref()
                .is_some_and(|s| s.end.is_none())
                && std::time::Instant::now() < deadline
            {
                self.frame(Vec::new());
                std::thread::sleep(std::time::Duration::from_millis(5));
            }
            self.frame(Vec::new());
            self.frame(Vec::new());
        }

        /// Spoken labels of the list options, and which one is selected.
        fn options(&mut self) -> Vec<(String, bool)> {
            let update = self.frame(Vec::new()).unwrap();
            let mut options: Vec<(f64, String, bool)> = update
                .nodes
                .iter()
                .filter(|(_, n)| n.role() == egui::accesskit::Role::ListBoxOption)
                .map(|(_, n)| {
                    (
                        n.bounds().map_or(0.0, |b| b.y0),
                        n.label().unwrap_or_default().to_owned(),
                        n.is_selected() == Some(true),
                    )
                })
                .collect();
            // Top to bottom, as shown.
            options.sort_by(|a, b| a.0.total_cmp(&b.0));
            options.into_iter().map(|(_, l, s)| (l, s)).collect()
        }

        fn caret_line(&self) -> usize {
            let ed = self.shell.editor();
            let rope = ed.document().rope();
            rope.char_to_line(ed.selections().primary().head.min(rope.len_chars()))
        }
    }

    fn project() -> tempfile::TempDir {
        let d = tempfile::tempdir().unwrap();
        let p = d.path();
        std::fs::create_dir_all(p.join("src")).unwrap();
        std::fs::create_dir_all(p.join("node_modules/pkg")).unwrap();
        std::fs::write(p.join("src/main.rs"), "fn main() {}\n\nfn helper() {}\n").unwrap();
        std::fs::write(p.join("src/lib.rs"), "pub fn lib() {}\n").unwrap();
        std::fs::write(p.join("README.md"), "# Title\n\ntext\n\n## Usage\nmore\n").unwrap();
        std::fs::write(p.join("node_modules/pkg/main.rs"), "").unwrap();
        d
    }

    #[test]
    fn lists_folder_files_and_opens_the_chosen_one_at_a_line() {
        let d = project();
        let mut h = Harness::new();
        h.shell.open(&d.path().join("README.md"));
        h.open();
        h.settle();
        let all = h.options();
        let labels: Vec<&str> = all.iter().map(|(l, _)| l.as_str()).collect();
        // The open tab first, then the folder (node_modules skipped).
        assert_eq!(labels.len(), 3, "{labels:?}");
        assert!(labels[0].starts_with("README.md"), "{labels:?}");
        assert!(labels[0].ends_with("open"), "{labels:?}");
        assert!(all[0].1, "the first option is selected");
        assert!(!labels.iter().any(|l| l.contains("node_modules")));
        h.type_text("main:3");
        let found = h.options();
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found[0].0.starts_with("main.rs"), "{found:?}");
        h.key(egui::Key::Enter, egui::Modifiers::NONE);
        assert!(!h.shell.palette.open, "Enter closes the popup");
        assert_eq!(h.shell.ws.len(), 2);
        let path = h.shell.editor().file().unwrap().path.clone();
        assert_eq!(path.file_name().unwrap(), "main.rs");
        assert_eq!(h.caret_line(), 2);
    }

    #[test]
    fn open_tab_is_activated_not_reopened() {
        let d = project();
        let mut h = Harness::new();
        h.shell.open(&d.path().join("src/lib.rs"));
        h.shell.open(&d.path().join("README.md"));
        assert_eq!(h.shell.ws.len(), 2);
        h.open();
        h.type_text("lib");
        h.key(egui::Key::Enter, egui::Modifiers::NONE);
        assert_eq!(h.shell.ws.len(), 2);
        assert_eq!(h.shell.ws.active_index(), 0);
    }

    #[test]
    fn line_mode_goes_to_the_line() {
        let mut h = Harness::new();
        h.shell.editor_mut().replace_range(0..0, "a\nb\nc\nd\n");
        h.open();
        h.type_text(":3");
        let opts = h.options();
        assert_eq!(opts, vec![("Go to line 3".to_owned(), true)]);
        h.key(egui::Key::Enter, egui::Modifiers::NONE);
        assert_eq!(h.caret_line(), 2);
        assert!(!h.shell.palette.open);
    }

    #[test]
    fn symbol_mode_lists_markdown_headings_and_code_symbols() {
        let d = project();
        let mut h = Harness::new();
        h.shell.open(&d.path().join("README.md"));
        h.open();
        h.type_text("@");
        let opts = h.options();
        let labels: Vec<&str> = opts.iter().map(|(l, _)| l.trim_start()).collect();
        assert_eq!(labels, vec!["Title  1", "Usage  5"]);
        h.type_text("us");
        h.key(egui::Key::Enter, egui::Modifiers::NONE);
        assert_eq!(h.caret_line(), 4);

        h.shell.open(&d.path().join("src/main.rs"));
        h.open();
        h.type_text("@help");
        let opts = h.options();
        assert_eq!(opts, vec![("helper  3".to_owned(), true)]);
        h.key(egui::Key::Enter, egui::Modifiers::NONE);
        assert_eq!(h.caret_line(), 2);
    }

    #[test]
    fn text_mode_jumps_to_the_chosen_match() {
        let mut h = Harness::new();
        h.shell
            .editor_mut()
            .replace_range(0..0, "alpha\nbeta needle\ngamma\nneedle again\n");
        h.open();
        h.type_text("#needle");
        let opts = h.options();
        assert_eq!(opts.len(), 2);
        h.key(egui::Key::ArrowDown, egui::Modifiers::NONE);
        assert!(h.options()[1].1, "the second match is selected");
        h.key(egui::Key::Enter, egui::Modifiers::NONE);
        let sel = h.shell.editor().selections().primary();
        assert_eq!(sel.range(), 24..30);
        h.open();
        h.type_text("#zzz");
        assert!(h.options().is_empty());
    }

    #[test]
    fn escape_closes_and_stops_the_listing() {
        let d = project();
        let mut h = Harness::new();
        h.shell.open(&d.path().join("README.md"));
        h.open();
        assert!(h.shell.goto.scan.is_some());
        h.key(egui::Key::Escape, egui::Modifiers::NONE);
        assert!(!h.shell.palette.open);
        assert!(h.shell.goto.scan.is_none());
        assert!(h.shell.goto.cands.is_empty());
    }

    #[test]
    fn find_in_folder_folder_is_the_project_folder() {
        let d = project();
        let other = tempfile::tempdir().unwrap();
        std::fs::write(other.path().join("elsewhere.txt"), "").unwrap();
        let mut h = Harness::new();
        h.shell.open(&d.path().join("README.md"));
        h.shell.folder.root = Some(other.path().to_path_buf());
        h.open();
        h.settle();
        let labels: Vec<String> = h.options().into_iter().map(|(l, _)| l).collect();
        assert_eq!(labels.len(), 2, "{labels:?}");
        assert!(labels[1].starts_with("elsewhere.txt"), "{labels:?}");
    }

    #[test]
    fn command_enter_opens_in_the_other_pane() {
        use text3_app::panes::{Direction, Panes};
        let d = project();
        let mut h = Harness::new();
        h.shell.open(&d.path().join("README.md"));
        let readme = h.shell.ws.active().id;
        let mut panes = Panes::new(readme);
        assert!(panes.split(Direction::Right, readme));
        h.shell.set_split(Some(panes));
        h.open();
        h.type_text("lib.rs");
        h.key(egui::Key::Enter, egui::Modifiers::COMMAND);
        let split = h.shell.split.as_ref().unwrap();
        assert_eq!(split.focus(), 1, "the second pane took focus");
        let lib = h.shell.ws.active();
        assert_eq!(
            lib.editor.file().unwrap().path.file_name().unwrap(),
            "lib.rs"
        );
        // The first pane still shows the README.
        assert_eq!(split.get(0).unwrap().tab, readme);
    }

    #[test]
    fn field_and_list_are_named_for_screen_readers() {
        let mut h = Harness::new();
        h.open();
        let update = h.frame(Vec::new()).unwrap();
        let hint = h.shell.t("goto-anything-hint", &[]);
        assert!(update.nodes.iter().any(|(_, n)| {
            n.role() == egui::accesskit::Role::ListBox && n.label() == Some(hint.as_str())
        }));
        assert!(
            update
                .nodes
                .iter()
                .any(|(_, n)| n.role() == egui::accesskit::Role::TextInput
                    && n.label() == Some(hint.as_str()))
        );
    }
}
