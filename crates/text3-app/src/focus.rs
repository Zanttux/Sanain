//! Keyboard focus between the window's regions: the tab strip, the find
//! bar, the outline, each pane, the folder search panel and the status
//! bar. F6 / Shift+F6 move from region to region; Tab past the last item
//! of a region (Shift+Tab before the first) moves to the next (previous)
//! one. egui's own Tab order follows the order widgets are created in,
//! which across panels is not the order they appear in, so the shell
//! steers focus at the edges.
//!
//! The order logic is pure and tested here; [`FocusCycle`] holds what the
//! last frame drew (stops, pane entries, region rectangles).

use eframe::egui;

/// How long a Tab press or a region entry counts as the cause of focus
/// landing on an edge stop.
const RECENT_SECS: f64 = 0.5;

/// A part of the window that keyboard focus moves between.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Region {
    Tabs,
    Find,
    Outline,
    /// The n-th pane in pane order (0 without a split).
    Pane(usize),
    Folder,
    Status,
}

/// Which optional regions are shown.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Layout {
    pub find: bool,
    pub outline: bool,
    /// Panes drawn; at least one when a document is shown.
    pub panes: usize,
    pub folder: bool,
}

/// The cycle order: tab strip, find bar, outline, panes in pane order,
/// folder search, status bar. Hidden regions are left out.
pub fn order(l: Layout) -> Vec<Region> {
    let mut out = vec![Region::Tabs];
    if l.find {
        out.push(Region::Find);
    }
    if l.outline {
        out.push(Region::Outline);
    }
    out.extend((0..l.panes).map(Region::Pane));
    if l.folder {
        out.push(Region::Folder);
    }
    out.push(Region::Status);
    out
}

/// The region after (`forward`) or before `from`, wrapping around. When
/// `from` is unknown or no longer shown, forward starts at the first
/// region and backward at the last.
pub fn step(order: &[Region], from: Option<Region>, forward: bool) -> Option<Region> {
    let n = order.len();
    if n == 0 {
        return None;
    }
    let Some(i) = from.and_then(|r| order.iter().position(|o| *o == r)) else {
        return Some(if forward { order[0] } else { order[n - 1] });
    };
    Some(
        order[if forward {
            (i + 1) % n
        } else {
            (i + n - 1) % n
        }],
    )
}

/// Tab inside a region with `len` known stops, at stop `at`: the next
/// stop, or `None` when Tab leaves the region.
pub fn stop_step(len: usize, at: usize, forward: bool) -> Option<usize> {
    if forward {
        (at + 1 < len).then_some(at + 1)
    } else {
        at.checked_sub(1).filter(|_| at < len)
    }
}

/// The invisible focus stops before (`Head`) and after (`Tail`) a region
/// whose own widgets the shell does not list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Edge {
    Head,
    Tail,
}

/// Where to enter a region.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Entry {
    /// The first item (Tab from the region before).
    First,
    /// The last item (Shift+Tab from the region after).
    Last,
    /// The region's main item (F6): the active tab, the pane's view.
    Main,
}

/// Focus landed on a region's edge stop. `tab` is the direction of the
/// Tab press that moved it there (`None`: not a Tab press). Past the tail
/// going forward is the next region; before the head going backward the
/// previous one; anything else enters this region.
pub fn on_edge(order: &[Region], region: Region, edge: Edge, tab: Option<bool>) -> (Region, Entry) {
    let forward = tab.unwrap_or(edge == Edge::Head);
    match (edge, forward) {
        (Edge::Tail, true) => (
            step(order, Some(region), true).unwrap_or(region),
            Entry::First,
        ),
        (Edge::Head, false) => (
            step(order, Some(region), false).unwrap_or(region),
            Entry::Last,
        ),
        (Edge::Head, true) => (region, Entry::First),
        (Edge::Tail, false) => (region, Entry::Last),
    }
}

/// The id of a region's edge stop.
pub fn edge_id(region: Region, edge: Edge) -> egui::Id {
    egui::Id::unique(("text3-focus-edge", region, edge))
}

/// One pane as drawn: its split slot, the widget that takes focus in it
/// (editor, grid, tree or preview) and where it is.
#[derive(Debug, Clone, Copy)]
pub struct PaneStop {
    pub slot: usize,
    pub entry: Option<egui::Id>,
    pub rect: egui::Rect,
}

/// What one frame drew, for steering focus in the next.
#[derive(Debug, Clone, Default)]
pub struct Drawn {
    pub layout: Layout,
    /// Tab strip stops, left to right (each tab, then its close button).
    pub tabs: Vec<egui::Id>,
    /// The active tab's stop.
    pub active_tab: Option<egui::Id>,
    /// Status bar buttons, left to right.
    pub status: Vec<egui::Id>,
    pub panes: Vec<PaneStop>,
    /// Rectangles of the regions without listed stops.
    pub rects: Vec<(Region, egui::Rect)>,
}

#[derive(Debug, Default)]
pub struct FocusCycle {
    /// The previous frame.
    pub last: Drawn,
    /// The frame being drawn.
    pub now: Drawn,
    /// Direction of the latest Tab press and when (input time, seconds).
    tab: Option<(bool, f64)>,
    /// Enter a region through its edge stop at the start of the next
    /// frame (regions whose items are not listed).
    pub pending: Option<(Region, Entry)>,
    /// Focus at the end of the previous frame (Escape clears it before
    /// the shell sees the key).
    pub prev_focus: Option<egui::Id>,
    pub popup_was_open: bool,
    /// Move focus to the widget before this one at the start of the
    /// next frame (Shift+Tab out of a widget that keeps Tab from egui).
    pub back_from: Option<egui::Id>,
    /// Where focus should go, noted by an edge stop during the frame.
    pub go: Option<(Region, Entry)>,
    /// The region last entered through an edge stop, and when.
    entering: Option<(Region, Entry, f64)>,
}

impl FocusCycle {
    pub fn order(&self) -> Vec<Region> {
        order(self.last.layout)
    }

    /// Note a Tab press of this pass. Returns its direction.
    pub fn note_tab(&mut self, ctx: &egui::Context) -> Option<bool> {
        let dir = ctx.input(|i| {
            i.events.iter().find_map(|e| match e {
                egui::Event::Key {
                    key: egui::Key::Tab,
                    pressed: true,
                    modifiers,
                    ..
                } if !modifiers.command && !modifiers.ctrl && !modifiers.alt => {
                    Some(!modifiers.shift)
                }
                _ => None,
            })
        });
        if let Some(d) = dir {
            self.tab = Some((d, ctx.input(|i| i.time)));
        }
        dir
    }

    /// The direction of a Tab press just now. Focus can move a few
    /// passes after the key (Shift+Tab a frame late, a grid letting go).
    pub fn recent_tab(&self, ctx: &egui::Context) -> Option<bool> {
        let now = ctx.input(|i| i.time);
        self.tab
            .filter(|(_, at)| now - *at <= RECENT_SECS)
            .map(|(d, _)| d)
    }

    /// An invisible focus stop at a region edge. When focus has just
    /// landed on it, notes where it should go instead (see [`Self::go`]).
    pub fn edge(&mut self, ui: &mut egui::Ui, region: Region, edge: Edge) {
        let id = edge_id(region, edge);
        let at = if edge == Edge::Head {
            ui.max_rect().min
        } else {
            ui.min_rect().max
        };
        let r = ui.interact(
            egui::Rect::from_min_size(at, egui::Vec2::ZERO),
            id,
            egui::Sense::focusable_noninteractive(),
        );
        ui.ctx().accesskit_node_builder(id, |n| n.set_hidden());
        if !r.has_focus() || self.go.is_some() {
            return;
        }
        let order = self.order();
        let now = ui.ctx().input(|i| i.time);
        // Entering a region that has nothing to focus: on to the next.
        if let Some((er, entry, at)) = self.entering
            && er == region
            && now - at <= RECENT_SECS
        {
            let forward = entry != Entry::Last;
            if (edge == Edge::Tail) == forward {
                let next = step(&order, Some(region), forward).unwrap_or(region);
                self.go = Some((next, if forward { Entry::First } else { Entry::Last }));
                return;
            }
        }
        self.go = Some(on_edge(&order, region, edge, self.recent_tab(ui.ctx())));
    }

    /// Enter a region through its edge stop: give focus to the first
    /// widget after the head (or the last before the tail).
    pub fn enter(&mut self, ctx: &egui::Context, region: Region, entry: Entry) {
        let (edge, dir) = if entry == Entry::Last {
            (Edge::Tail, egui::FocusDirection::Previous)
        } else {
            (Edge::Head, egui::FocusDirection::Next)
        };
        self.entering = Some((region, entry, ctx.input(|i| i.time)));
        ctx.memory_mut(|m| {
            m.request_focus(edge_id(region, edge));
            m.move_focus(dir);
        });
        ctx.request_repaint();
    }

    /// Which region holds widget `id`, judged by the last frame.
    pub fn region_of(&self, ctx: &egui::Context, id: egui::Id) -> Option<Region> {
        let d = &self.last;
        if d.tabs.contains(&id) {
            return Some(Region::Tabs);
        }
        if d.status.contains(&id) {
            return Some(Region::Status);
        }
        if let Some(n) = d.panes.iter().position(|p| p.entry == Some(id)) {
            return Some(Region::Pane(n));
        }
        if let Some(region) = order(d.layout).into_iter().find(|r| {
            [Edge::Head, Edge::Tail]
                .iter()
                .any(|e| edge_id(*r, *e) == id)
        }) {
            return Some(region);
        }
        let r = ctx.read_response(id)?;
        if r.layer_id.order != egui::Order::Background {
            return None;
        }
        let c = r.interact_rect.center();
        d.panes
            .iter()
            .enumerate()
            .find(|(_, p)| p.rect.contains(c))
            .map(|(n, _)| Region::Pane(n))
            .or_else(|| {
                d.rects
                    .iter()
                    .find(|(_, rect)| rect.contains(c))
                    .map(|(r, _)| *r)
            })
    }

    /// End of frame: what was drawn becomes the reference.
    pub fn end_frame(&mut self, ctx: &egui::Context) {
        self.now.layout.panes = self.now.panes.len();
        self.last = std::mem::take(&mut self.now);
        self.prev_focus = ctx.memory(|m| m.focused());
        self.popup_was_open = egui::Popup::is_any_open(ctx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Region::*;

    fn all() -> Layout {
        Layout {
            find: true,
            outline: true,
            panes: 2,
            folder: true,
        }
    }

    #[test]
    fn regions_follow_the_window_from_top_to_bottom() {
        assert_eq!(
            order(all()),
            [Tabs, Find, Outline, Pane(0), Pane(1), Folder, Status]
        );
        let single = Layout {
            panes: 1,
            ..Layout::default()
        };
        assert_eq!(order(single), [Tabs, Pane(0), Status]);
    }

    #[test]
    fn hidden_regions_and_panes_are_skipped() {
        let l = Layout {
            find: false,
            outline: true,
            panes: 3,
            folder: false,
        };
        let o = order(l);
        assert_eq!(o, [Tabs, Outline, Pane(0), Pane(1), Pane(2), Status]);
        assert_eq!(step(&o, Some(Tabs), true), Some(Outline));
        assert_eq!(step(&o, Some(Pane(2)), true), Some(Status));
        assert_eq!(step(&o, Some(Outline), false), Some(Tabs));
        // A closed find bar or pane no longer in the cycle: start over.
        assert_eq!(step(&o, Some(Find), true), Some(Tabs));
        assert_eq!(step(&o, Some(Pane(5)), false), Some(Status));
        assert_eq!(step(&o, None, true), Some(Tabs));
        assert_eq!(step(&o, None, false), Some(Status));
    }

    #[test]
    fn the_cycle_wraps_both_ways() {
        let o = order(all());
        assert_eq!(step(&o, Some(Status), true), Some(Tabs));
        assert_eq!(step(&o, Some(Tabs), false), Some(Status));
        // Forward through everything returns to the start.
        let mut at = Tabs;
        let mut seen = vec![at];
        for _ in 0..o.len() {
            at = step(&o, Some(at), true).unwrap();
            seen.push(at);
        }
        assert_eq!(seen.first(), seen.last());
        assert_eq!(&seen[..o.len()], &o[..]);
        // Backward visits the same regions in reverse.
        let mut back = vec![Tabs];
        for _ in 0..o.len() {
            back.push(step(&o, back.last().copied(), false).unwrap());
        }
        back.reverse();
        assert_eq!(back, seen);
        assert_eq!(step(&[], None, true), None);
    }

    #[test]
    fn tab_moves_through_stops_then_leaves() {
        assert_eq!(stop_step(4, 0, true), Some(1));
        assert_eq!(stop_step(4, 3, true), None);
        assert_eq!(stop_step(4, 3, false), Some(2));
        assert_eq!(stop_step(4, 0, false), None);
        assert_eq!(stop_step(1, 0, true), None);
        assert_eq!(stop_step(0, 0, false), None);
    }

    #[test]
    fn edges_lead_to_the_neighbouring_region() {
        let o = order(Layout {
            panes: 2,
            ..Layout::default()
        });
        // Tab past pane 1's last widget: pane 2; past pane 2's: status.
        assert_eq!(
            on_edge(&o, Pane(0), Edge::Tail, Some(true)),
            (Pane(1), Entry::First)
        );
        assert_eq!(
            on_edge(&o, Pane(1), Edge::Tail, Some(true)),
            (Status, Entry::First)
        );
        // Shift+Tab before pane 1's first widget: the tab strip's last.
        assert_eq!(
            on_edge(&o, Pane(0), Edge::Head, Some(false)),
            (Tabs, Entry::Last)
        );
        // Arriving from outside (the menu bar, nothing focused) enters it.
        assert_eq!(
            on_edge(&o, Pane(1), Edge::Head, Some(true)),
            (Pane(1), Entry::First)
        );
        assert_eq!(
            on_edge(&o, Pane(1), Edge::Tail, Some(false)),
            (Pane(1), Entry::Last)
        );
        assert_eq!(
            on_edge(&o, Pane(0), Edge::Head, None),
            (Pane(0), Entry::First)
        );
        assert_eq!(
            on_edge(&o, Pane(0), Edge::Tail, None),
            (Pane(0), Entry::Last)
        );
    }
}
