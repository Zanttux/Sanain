//! Layout of the split editor: up to [`MAX_PANES`] panes, each showing a
//! tab, independent of the GUI toolkit.
//!
//! The first split sets the outer direction. Further splits in that
//! direction add a group next to the focused one; a split in the other
//! direction stacks a pane inside the focused group. That allows one level
//! of nesting (for example two panes side by side, the right one split top
//! and bottom) and no deeper. Panes share the space evenly.

/// Most panes the editor shows at once.
pub const MAX_PANES: usize = 4;

/// Where a new pane goes relative to the focused one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    /// Side by side (Split Right).
    Right,
    /// Top and bottom (Split Down).
    Down,
}

impl Direction {
    fn cross(self) -> Self {
        match self {
            Self::Right => Self::Down,
            Self::Down => Self::Right,
        }
    }
}

/// One pane: the tab it shows and a stable slot for its per-pane view
/// state (scroll, preview sync). Slots are `0..MAX_PANES`, unique among
/// the panes, and kept while the pane lives.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pane<T> {
    pub tab: T,
    pub slot: usize,
}

/// A pane's share of the split area, as fractions `0.0..=1.0` of its
/// width and height.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Frac {
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
}

/// The panes of the editor area, at least one, in reading order.
#[derive(Clone, Debug, PartialEq)]
pub struct Panes<T> {
    /// How the groups are placed; panes inside a group go the other way.
    dir: Direction,
    groups: Vec<Vec<Pane<T>>>,
    /// Index of the focused pane in reading order.
    focus: usize,
}

impl<T: Copy + PartialEq> Panes<T> {
    /// A single pane showing `tab`, in slot 0.
    pub fn new(tab: T) -> Self {
        Self {
            dir: Direction::Right,
            groups: vec![vec![Pane { tab, slot: 0 }]],
            focus: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.groups.iter().map(Vec::len).sum()
    }

    /// Always false: there is at least one pane.
    pub fn is_empty(&self) -> bool {
        false
    }

    /// More than one pane is shown.
    pub fn is_split(&self) -> bool {
        self.len() > 1
    }

    /// The direction groups are placed in (side by side or stacked).
    pub fn direction(&self) -> Direction {
        self.dir
    }

    /// The panes in reading order.
    pub fn panes(&self) -> impl Iterator<Item = &Pane<T>> {
        self.groups.iter().flatten()
    }

    /// Index of the focused pane in reading order.
    pub fn focus(&self) -> usize {
        self.focus
    }

    pub fn focused(&self) -> Pane<T> {
        self.get(self.focus).copied().unwrap_or(self.groups[0][0])
    }

    pub fn get(&self, i: usize) -> Option<&Pane<T>> {
        self.panes().nth(i)
    }

    fn get_mut(&mut self, i: usize) -> Option<&mut Pane<T>> {
        self.groups.iter_mut().flatten().nth(i)
    }

    /// Group index and index within it of pane `i`.
    fn locate(&self, i: usize) -> (usize, usize) {
        let mut rest = i;
        for (g, group) in self.groups.iter().enumerate() {
            if rest < group.len() {
                return (g, rest);
            }
            rest -= group.len();
        }
        (0, 0)
    }

    /// Show `tab` in pane `i`.
    pub fn set_tab(&mut self, i: usize, tab: T) {
        if let Some(p) = self.get_mut(i) {
            p.tab = tab;
        }
    }

    /// Focus pane `i`; returns whether it exists.
    pub fn set_focus(&mut self, i: usize) -> bool {
        if i < self.len() {
            self.focus = i;
            true
        } else {
            false
        }
    }

    /// Index of the pane using `slot`.
    pub fn index_of_slot(&self, slot: usize) -> Option<usize> {
        self.panes().position(|p| p.slot == slot)
    }

    /// Add a pane showing `tab` next to the focused one, in `dir`. Focus
    /// stays where it was. Returns false at [`MAX_PANES`].
    pub fn split(&mut self, dir: Direction, tab: T) -> bool {
        if self.len() >= MAX_PANES {
            return false;
        }
        let Some(slot) = (0..MAX_PANES).find(|s| self.panes().all(|p| p.slot != *s)) else {
            return false;
        };
        let pane = Pane { tab, slot };
        if !self.is_split() {
            self.dir = dir;
        }
        let (g, k) = self.locate(self.focus);
        if dir == self.dir {
            self.groups.insert(g + 1, vec![pane]);
        } else {
            self.groups[g].insert(k + 1, pane);
        }
        true
    }

    /// Close the focused pane; the pane before it (or the new first one)
    /// takes focus. Returns the closed pane, or `None` when it is the
    /// only one.
    pub fn close_focused(&mut self) -> Option<Pane<T>> {
        if !self.is_split() {
            return None;
        }
        let (g, k) = self.locate(self.focus);
        let closed = self.groups[g].remove(k);
        if self.groups[g].is_empty() {
            self.groups.remove(g);
        }
        // A single group left is a plain row or column the other way.
        if self.groups.len() == 1 && self.groups[0].len() > 1 {
            let group = self.groups.remove(0);
            self.dir = self.dir.cross();
            self.groups = group.into_iter().map(|p| vec![p]).collect();
        }
        self.focus = self.focus.saturating_sub(1).min(self.len() - 1);
        Some(closed)
    }

    /// Focus the next pane in reading order, wrapping around.
    pub fn focus_next(&mut self) {
        self.focus = (self.focus + 1) % self.len();
    }

    /// Focus the previous pane in reading order, wrapping around.
    pub fn focus_prev(&mut self) {
        let n = self.len();
        self.focus = (self.focus + n - 1) % n;
    }

    /// Each pane's share of the area, in reading order.
    pub fn fractions(&self) -> Vec<Frac> {
        let n = self.groups.len() as f32;
        let mut out = Vec::with_capacity(self.len());
        for (g, group) in self.groups.iter().enumerate() {
            let (a0, a1) = (g as f32 / n, (g + 1) as f32 / n);
            let m = group.len() as f32;
            for k in 0..group.len() {
                let (b0, b1) = (k as f32 / m, (k + 1) as f32 / m);
                out.push(match self.dir {
                    Direction::Right => Frac {
                        x0: a0,
                        x1: a1,
                        y0: b0,
                        y1: b1,
                    },
                    Direction::Down => Frac {
                        x0: b0,
                        x1: b1,
                        y0: a0,
                        y1: a1,
                    },
                });
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tabs(p: &Panes<u32>) -> Vec<u32> {
        p.panes().map(|p| p.tab).collect()
    }

    #[test]
    fn starts_with_one_pane_that_cannot_close() {
        let mut p = Panes::new(1);
        assert_eq!(p.len(), 1);
        assert!(!p.is_split());
        assert_eq!(p.close_focused(), None);
        assert_eq!(p.focused(), Pane { tab: 1, slot: 0 });
    }

    #[test]
    fn first_split_sets_direction_and_keeps_focus() {
        let mut p = Panes::new(1);
        assert!(p.split(Direction::Down, 2));
        assert_eq!(p.direction(), Direction::Down);
        assert_eq!(tabs(&p), [1, 2]);
        assert_eq!(p.focus(), 0);
        let f = p.fractions();
        assert_eq!((f[0].y0, f[0].y1, f[1].y0, f[1].y1), (0.0, 0.5, 0.5, 1.0));
        assert_eq!((f[0].x0, f[0].x1), (0.0, 1.0));
    }

    #[test]
    fn same_direction_splits_add_panes_after_the_focused_one() {
        let mut p = Panes::new(1);
        assert!(p.split(Direction::Right, 2));
        assert!(p.split(Direction::Right, 3));
        assert_eq!(tabs(&p), [1, 3, 2]);
        let f = p.fractions();
        assert!((f[1].x0 - 1.0 / 3.0).abs() < 1e-6);
        assert!(f.iter().all(|f| f.y0 == 0.0 && f.y1 == 1.0));
    }

    #[test]
    fn stops_at_the_limit() {
        let mut p = Panes::new(1);
        for t in 2..=MAX_PANES as u32 {
            assert!(p.split(Direction::Right, t));
        }
        assert!(!p.split(Direction::Right, 9));
        assert!(!p.split(Direction::Down, 9));
        assert_eq!(p.len(), MAX_PANES);
        let mut slots: Vec<_> = p.panes().map(|p| p.slot).collect();
        slots.sort_unstable();
        assert_eq!(slots, [0, 1, 2, 3]);
    }

    #[test]
    fn other_direction_nests_one_level_in_the_focused_group() {
        let mut p = Panes::new(1);
        p.split(Direction::Right, 2);
        assert!(p.set_focus(1));
        p.split(Direction::Down, 3);
        assert_eq!(tabs(&p), [1, 2, 3]);
        let f = p.fractions();
        assert_eq!((f[0].x0, f[0].x1, f[0].y0, f[0].y1), (0.0, 0.5, 0.0, 1.0));
        assert_eq!((f[1].x0, f[1].y0, f[1].y1), (0.5, 0.0, 0.5));
        assert_eq!((f[2].x0, f[2].y0, f[2].y1), (0.5, 0.5, 1.0));
        // Splitting right from a stacked pane adds a new column.
        assert!(p.set_focus(2));
        p.split(Direction::Right, 4);
        assert_eq!(tabs(&p), [1, 2, 3, 4]);
        let f = p.fractions();
        assert!((f[3].x0 - 2.0 / 3.0).abs() < 1e-6);
        assert_eq!((f[3].y0, f[3].y1), (0.0, 1.0));
    }

    #[test]
    fn closing_moves_focus_back_and_frees_the_slot() {
        let mut p = Panes::new(1);
        p.split(Direction::Right, 2);
        p.split(Direction::Right, 3);
        p.set_focus(1);
        let closed = p.close_focused().unwrap();
        assert_eq!(closed.tab, 3);
        assert_eq!(tabs(&p), [1, 2]);
        assert_eq!(p.focus(), 0);
        p.split(Direction::Right, 4);
        assert_eq!(p.get(1).unwrap().slot, closed.slot);
    }

    #[test]
    fn closing_to_one_group_flattens_the_other_way() {
        let mut p = Panes::new(1);
        p.split(Direction::Right, 2);
        p.set_focus(1);
        p.split(Direction::Down, 3);
        p.set_focus(0);
        p.close_focused();
        assert_eq!(tabs(&p), [2, 3]);
        assert_eq!(p.direction(), Direction::Down);
        // A further Split Down adds a third row, not a nested group.
        p.split(Direction::Down, 4);
        let f = p.fractions();
        assert!(f.iter().all(|f| f.x0 == 0.0 && f.x1 == 1.0));
        p.close_focused();
        p.close_focused();
        assert_eq!(p.len(), 1);
        assert_eq!(p.close_focused(), None);
    }

    #[test]
    fn focus_cycles_both_ways() {
        let mut p = Panes::new(1);
        p.split(Direction::Right, 2);
        p.split(Direction::Down, 3);
        assert_eq!(p.focus(), 0);
        p.focus_next();
        assert_eq!(p.focus(), 1);
        p.focus_next();
        p.focus_next();
        assert_eq!(p.focus(), 0);
        p.focus_prev();
        assert_eq!(p.focus(), 2);
        assert!(!p.set_focus(3));
        assert_eq!(p.index_of_slot(p.get(2).unwrap().slot), Some(2));
    }
}
