//! Default apps window (#164): offered once on first start and from
//! Settings › General. The platform work is in [`text3_app::defaults`].

use text3_app::defaults::{self, Applied, Group, Support};

use super::{Shell, egui};

pub(super) struct DefaultApps {
    support: Support,
    /// Ticked groups, in [`Group::ALL`] order.
    chosen: [bool; 5],
    /// Whether Text³ already opens each group's main type (`None`: the
    /// platform does not say).
    current: [Option<bool>; 5],
    /// The result of the last attempt: (message, is error).
    outcome: Option<(String, bool)>,
}

impl Shell {
    /// First start: offer once, unless this copy cannot change defaults.
    pub fn offer_default_apps(&mut self) {
        if self.settings.default_apps_asked {
            return;
        }
        let Ok(exe) = defaults::current_exe() else {
            return;
        };
        if defaults::support(&exe) == Support::Unavailable {
            return;
        }
        self.open_default_apps();
    }

    pub(super) fn open_default_apps(&mut self) {
        let exe = defaults::current_exe().ok();
        let support = exe
            .as_deref()
            .map_or(Support::Unavailable, defaults::support);
        let current = Group::ALL.map(|g| exe.as_deref().and_then(|e| defaults::is_default(e, g)));
        self.default_apps = Some(DefaultApps {
            support,
            // Already ours stays ticked; otherwise the preselection.
            chosen: std::array::from_fn(|i| {
                current[i].unwrap_or(false) || Group::ALL[i].preselected()
            }),
            current,
            outcome: None,
        });
    }

    /// Answering in any way (even closing) counts as asked.
    fn mark_default_apps_asked(&mut self) {
        if !self.settings.default_apps_asked {
            self.settings.default_apps_asked = true;
            self.save_settings();
        }
    }

    pub(super) fn default_apps_window(&mut self, ctx: &egui::Context) {
        let Some(mut state) = self.default_apps.take() else {
            return;
        };
        let l = |id: &str| self.t(id, &[]);
        let title = l("default-apps-title");
        let body = match state.support {
            Support::Direct => l("default-apps-body"),
            Support::SystemSettings => l("default-apps-body-windows"),
            Support::Unavailable => l("default-apps-unavailable"),
        };
        let labels = Group::ALL.map(|g| {
            let examples: Vec<String> = g
                .extensions()
                .iter()
                .take(4)
                .map(|e| format!(".{e}"))
                .collect();
            (l(g.label_id()), examples.join(" "))
        });
        let already = l("default-apps-is-default");
        let apply_label = match state.support {
            Support::SystemSettings => l("default-apps-open-settings"),
            _ => l("default-apps-apply"),
        };
        let not_now = l("default-apps-not-now");
        let close_label = l("default-apps-close");
        let mut open = true;
        let mut apply = false;
        let mut dismiss = false;
        egui::Window::new(title)
            .id(egui::Id::unique("text3-default-apps"))
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.set_max_width(460.0);
                ui.label(body);
                ui.add_space(8.0);
                if state.support == Support::Direct {
                    for (i, (label, examples)) in labels.iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut state.chosen[i], label);
                            ui.weak(examples);
                            if state.current[i] == Some(true) {
                                ui.weak(format!("· {already}"));
                            }
                        });
                    }
                    ui.add_space(8.0);
                }
                if let Some((msg, err)) = &state.outcome {
                    if *err {
                        ui.colored_label(ui.visuals().error_fg_color, msg);
                    } else {
                        ui.label(msg);
                    }
                    ui.add_space(4.0);
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if state.support != Support::Unavailable {
                        let any = state.support != Support::Direct || state.chosen.contains(&true);
                        if ui
                            .add_enabled(any, egui::Button::new(&apply_label))
                            .clicked()
                        {
                            apply = true;
                        }
                    }
                    let done = state.outcome.as_ref().is_some_and(|(_, err)| !err);
                    let label = if done || state.support == Support::Unavailable {
                        &close_label
                    } else {
                        &not_now
                    };
                    if ui.button(label).clicked() {
                        dismiss = true;
                    }
                });
            });
        if apply {
            let groups: Vec<Group> = Group::ALL
                .into_iter()
                .zip(state.chosen)
                .filter_map(|(g, on)| on.then_some(g))
                .collect();
            let result = defaults::current_exe().and_then(|exe| {
                let r = defaults::apply(&exe, &groups);
                if r.is_ok() {
                    state.current = Group::ALL.map(|g| defaults::is_default(&exe, g));
                }
                r
            });
            state.outcome = Some(match result {
                Ok(Applied::Set) => (self.t("default-apps-done", &[]), false),
                Ok(Applied::OpenedSettings) => (self.t("default-apps-settings-opened", &[]), false),
                Err(e) => (
                    self.t("default-apps-failed", &[("reason", e.to_string().into())]),
                    true,
                ),
            });
            self.mark_default_apps_asked();
        }
        if dismiss || !open {
            self.mark_default_apps_asked();
        } else {
            self.default_apps = Some(state);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
                shell.default_apps_window(&ctx);
            });
            out.textures_delta.clear();
            out.platform_output.accesskit_update
        }

        /// The AccessKit node labelled `label` (buttons, check boxes) or
        /// with that value (text labels), if shown.
        fn find(&mut self, label: &str) -> Option<egui::accesskit::NodeId> {
            self.frame(Vec::new());
            let update = self.frame(Vec::new())?;
            update.nodes.into_iter().find_map(|(id, n)| {
                (n.label() == Some(label) || n.value() == Some(label)).then_some(id)
            })
        }

        /// Activate it as a screen reader would. (Pointer positions from
        /// the tree go stale once the window has settled.)
        fn click(&mut self, label: &str) {
            let target_node = self.find(label).unwrap_or_else(|| panic!("no {label:?}"));
            self.frame(vec![egui::Event::AccessKitActionRequest(
                egui::accesskit::ActionRequest {
                    action: egui::accesskit::Action::Click,
                    target_tree: egui::accesskit::TreeId::ROOT,
                    target_node,
                    data: None,
                },
            )]);
            self.frame(Vec::new());
        }
    }

    /// The window as each platform shows it (the test binary itself is
    /// not Text³.app, so the kind of support is set by hand).
    fn opened(support: Support) -> Harness {
        let mut h = Harness::new();
        h.shell.open_default_apps();
        h.shell.default_apps.as_mut().unwrap().support = support;
        h
    }

    #[test]
    fn not_now_is_remembered_and_settings_reopen_it() {
        for (support, button) in [
            (Support::Direct, "default-apps-not-now"),
            (Support::SystemSettings, "default-apps-not-now"),
            (Support::Unavailable, "default-apps-close"),
        ] {
            let mut h = opened(support);
            if support == Support::Unavailable {
                let note = h.shell.t("default-apps-unavailable", &[]);
                assert!(h.find(&note).is_some());
            }
            let label = h.shell.t(button, &[]);
            h.click(&label);
            assert!(h.shell.default_apps.is_none(), "{support:?}: closed");
            assert!(h.shell.settings.default_apps_asked, "{support:?}");
            // An answered offer is not made again.
            h.shell.offer_default_apps();
            assert!(h.shell.default_apps.is_none());
            // Settings › General still opens it.
            h.shell.open_default_apps();
            assert!(h.shell.default_apps.is_some());
        }
    }

    #[test]
    fn closing_the_window_counts_as_answered() {
        let mut h = opened(Support::Direct);
        // Ticking works from the keyboard and screen readers too.
        let md = h.shell.t(Group::Markdown.label_id(), &[]);
        h.click(&md);
        assert!(
            !h.shell.default_apps.as_ref().unwrap().chosen[1],
            "unticked"
        );
        assert!(!h.shell.settings.default_apps_asked, "not answered yet");
        h.click("Close window");
        assert!(h.shell.default_apps.is_none());
        assert!(h.shell.settings.default_apps_asked);
    }

    #[test]
    fn groups_shown_only_where_they_can_be_set() {
        let label = |h: &Harness| h.shell.t(Group::Markdown.label_id(), &[]);
        let mut h = opened(Support::Direct);
        let l = label(&h);
        assert!(h.find(&l).is_some(), "checkboxes on macOS and Linux");
        let mut h = opened(Support::SystemSettings);
        let l = label(&h);
        assert!(h.find(&l).is_none(), "Windows asks in its own Settings");
        let open = h.shell.t("default-apps-open-settings", &[]);
        assert!(h.find(&open).is_some());
    }

    #[test]
    fn web_pages_are_not_preselected() {
        let mut h = Harness::new();
        h.shell.open_default_apps();
        let s = h.shell.default_apps.as_ref().unwrap();
        for (i, g) in Group::ALL.into_iter().enumerate() {
            if s.current[i] != Some(true) {
                assert_eq!(s.chosen[i], g.preselected(), "{g:?}");
            }
        }
    }
}
