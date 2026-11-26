//! Layout of the split editor: up to [`MAX_PANES`] panes, each showing a
//! tab, independent of the GUI toolkit.
//!
//! The first split sets the outer direction. Further splits in that
//! direction add a group next to the focused one; a split in the other
//! direction stacks a pane inside the focused group. That allows one level
//! of nesting (for example two panes side by side, the right one split top
//! and bottom) and no deeper.
//!
//! Panes start out sharing the space evenly. Each level (the groups, and
//! the panes inside each group) keeps its own shares, which the dividers
//! between them move; adding or closing a pane renormalises the shares so
//! they always sum to one.

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

/// Smallest share a restored layout may give a group or pane.
const MIN_RESTORED_SHARE: f32 = 0.05;

/// Which divider: between groups `i` and `i + 1`, or between panes `k`
/// and `k + 1` inside group `g`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DividerId {
    Outer(usize),
    Inner { group: usize, index: usize },
}

/// A divider line, in fractions of the split area. A `vertical` divider
/// separates left from right: it sits at x = `at` and runs from y =
/// `from` to y = `to`; a horizontal one the other way round.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Divider {
    pub id: DividerId,
    pub vertical: bool,
    pub at: f32,
    pub from: f32,
    pub to: f32,
}

/// A layout as plain data, for saving: the direction, each group's share
/// and its panes (what each shows and its share), and the focused pane in
/// reading order.
#[derive(Clone, Debug, PartialEq)]
pub struct Layout<U> {
    pub dir: Direction,
    pub groups: Vec<GroupLayout<U>>,
    pub focus: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GroupLayout<U> {
    pub share: f32,
    pub panes: Vec<(U, f32)>,
}

impl<U> Layout<U> {
    /// Map what each pane shows; panes mapped to `None` are dropped with
    /// their shares, as are groups left empty. Focus moves to the pane
    /// before a dropped focused pane.
    pub fn filter_map<V>(self, mut f: impl FnMut(U) -> Option<V>) -> Layout<V> {
        let mut focus = 0;
        let mut seen = 0;
        let mut kept = 0;
        let mut groups = Vec::new();
        for g in self.groups {
            let mut panes = Vec::new();
            for (u, share) in g.panes {
                let v = f(u);
                if seen <= self.focus && v.is_some() {
                    focus = kept;
                }
                seen += 1;
                if let Some(v) = v {
                    panes.push((v, share));
                    kept += 1;
                }
            }
            if !panes.is_empty() {
                groups.push(GroupLayout {
                    share: g.share,
                    panes,
                });
            }
        }
        Layout {
            dir: self.dir,
            groups,
            focus,
        }
    }

    pub fn len(&self) -> usize {
        self.groups.iter().map(|g| g.panes.len()).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Make `v` a valid set of shares: positive, summing to one, none tiny.
/// Anything unusable (non-finite, zero or negative) falls back to even.
fn normalise(v: &mut [f32]) {
    if v.is_empty() {
        return;
    }
    let even = 1.0 / v.len() as f32;
    let sum: f32 = v.iter().sum();
    if v.iter().any(|x| !x.is_finite() || *x <= 0.0) || !sum.is_finite() || sum <= 0.0 {
        v.fill(even);
        return;
    }
    let min = MIN_RESTORED_SHARE.min(even);
    for x in v.iter_mut() {
        *x = (*x / sum).max(min);
    }
    let sum: f32 = v.iter().sum();
    for x in v.iter_mut() {
        *x /= sum;
    }
}

/// Add a share for a new item at `at`: it gets `1/n` of the new total and
/// the others shrink in proportion, so even shares stay even.
fn insert_share(v: &mut Vec<f32>, at: usize) {
    let n = (v.len() + 1) as f32;
    for x in v.iter_mut() {
        *x *= (n - 1.0) / n;
    }
    v.insert(at.min(v.len()), 1.0 / n);
    normalise(v);
}

/// Remove the share at `at`; the rest grow in proportion.
fn remove_share(v: &mut Vec<f32>, at: usize) {
    if at < v.len() {
        v.remove(at);
    }
    normalise(v);
}

/// Move the boundary between `v[i]` and `v[i + 1]` to `t` (measured from
/// the start of `v[0]`), keeping both at least `min` (or half their
/// combined share when that is smaller).
fn move_boundary(v: &mut [f32], i: usize, t: f32, min: f32) {
    if i + 1 >= v.len() || !t.is_finite() {
        return;
    }
    let start: f32 = v[..i].iter().sum();
    let total = v[i] + v[i + 1];
    let min = min.max(0.0).min(total / 2.0);
    let a = (t - start).clamp(min, total - min);
    v[i] = a;
    v[i + 1] = total - a;
}

/// The panes of the editor area, at least one, in reading order.
#[derive(Clone, Debug, PartialEq)]
pub struct Panes<T> {
    /// How the groups are placed; panes inside a group go the other way.
    dir: Direction,
    groups: Vec<Vec<Pane<T>>>,
    /// Share of each group along `dir`; sums to one.
    outer: Vec<f32>,
    /// Share of each pane inside its group, across `dir`; each sums to one.
    inner: Vec<Vec<f32>>,
    /// Index of the focused pane in reading order.
    focus: usize,
}

impl<T: Copy + PartialEq> Panes<T> {
    /// A single pane showing `tab`, in slot 0.
    pub fn new(tab: T) -> Self {
        Self {
            dir: Direction::Right,
            groups: vec![vec![Pane { tab, slot: 0 }]],
            outer: vec![1.0],
            inner: vec![vec![1.0]],
            focus: 0,
        }
    }

    /// The layout as plain data.
    pub fn layout(&self) -> Layout<T> {
        Layout {
            dir: self.dir,
            groups: self
                .groups
                .iter()
                .zip(&self.outer)
                .zip(&self.inner)
                .map(|((g, share), inner)| GroupLayout {
                    share: *share,
                    panes: g.iter().map(|p| p.tab).zip(inner.iter().copied()).collect(),
                })
                .collect(),
            focus: self.focus,
        }
    }

    /// Rebuild panes from a saved layout. Shares are made valid, panes
    /// past [`MAX_PANES`] are dropped, a single group becomes a row or
    /// column the other way, and slots are given out in reading order.
    /// Returns `None` unless at least two panes remain.
    pub fn from_layout(layout: Layout<T>) -> Option<Self> {
        let mut dir = layout.dir;
        let mut groups = Vec::new();
        let mut outer = Vec::new();
        let mut inner = Vec::new();
        let mut slot = 0;
        for g in layout.groups {
            let mut panes = Vec::new();
            let mut shares = Vec::new();
            for (tab, share) in g.panes {
                if slot < MAX_PANES {
                    panes.push(Pane { tab, slot });
                    shares.push(share);
                    slot += 1;
                }
            }
            if !panes.is_empty() {
                groups.push(panes);
                outer.push(g.share);
                inner.push(shares);
            }
        }
        if slot < 2 {
            return None;
        }
        if groups.len() == 1 {
            dir = dir.cross();
            groups = groups.remove(0).into_iter().map(|p| vec![p]).collect();
            outer = inner.remove(0);
            inner = vec![vec![1.0]; groups.len()];
        }
        normalise(&mut outer);
        for v in &mut inner {
            normalise(v);
        }
        Some(Self {
            dir,
            groups,
            outer,
            inner,
            focus: layout.focus.min(slot - 1),
        })
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
            self.inner.insert(g + 1, vec![1.0]);
            insert_share(&mut self.outer, g + 1);
        } else {
            self.groups[g].insert(k + 1, pane);
            insert_share(&mut self.inner[g], k + 1);
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
        remove_share(&mut self.inner[g], k);
        if self.groups[g].is_empty() {
            self.groups.remove(g);
            self.inner.remove(g);
            remove_share(&mut self.outer, g);
        }
        // A single group left is a plain row or column the other way,
        // keeping the sizes its panes had.
        if self.groups.len() == 1 && self.groups[0].len() > 1 {
            let group = self.groups.remove(0);
            self.dir = self.dir.cross();
            self.groups = group.into_iter().map(|p| vec![p]).collect();
            self.outer = self.inner.remove(0);
            self.inner = vec![vec![1.0]; self.groups.len()];
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
        let mut out = Vec::with_capacity(self.len());
        let mut a0 = 0.0;
        for (g, group) in self.groups.iter().enumerate() {
            let a1 = if g + 1 == self.groups.len() {
                1.0
            } else {
                a0 + self.outer[g]
            };
            let mut b0 = 0.0;
            for k in 0..group.len() {
                let b1 = if k + 1 == group.len() {
                    1.0
                } else {
                    b0 + self.inner[g][k]
                };
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
                b0 = b1;
            }
            a0 = a1;
        }
        out
    }

    /// The dividers between groups and between panes inside a group.
    pub fn dividers(&self) -> Vec<Divider> {
        let vertical = self.dir == Direction::Right;
        let mut out = Vec::new();
        let mut a0 = 0.0;
        for (g, group) in self.groups.iter().enumerate() {
            let a1 = a0 + self.outer[g];
            let mut b = 0.0;
            for k in 0..group.len().saturating_sub(1) {
                b += self.inner[g][k];
                out.push(Divider {
                    id: DividerId::Inner { group: g, index: k },
                    vertical: !vertical,
                    at: b,
                    from: a0,
                    to: if g + 1 == self.groups.len() { 1.0 } else { a1 },
                });
            }
            if g + 1 < self.groups.len() {
                out.push(Divider {
                    id: DividerId::Outer(g),
                    vertical,
                    at: a1,
                    from: 0.0,
                    to: 1.0,
                });
            }
            a0 = a1;
        }
        out
    }

    /// Move a divider to `t` (a fraction of the area across the divider),
    /// keeping the panes on both sides at least `min` (also a fraction).
    pub fn move_divider(&mut self, id: DividerId, t: f32, min: f32) {
        match id {
            DividerId::Outer(i) => move_boundary(&mut self.outer, i, t, min),
            DividerId::Inner { group, index } => {
                if let Some(v) = self.inner.get_mut(group) {
                    move_boundary(v, index, t, min);
                }
            }
        }
    }

    /// Share the space evenly again on the divider's level: all groups,
    /// or all panes of its group.
    pub fn reset_divider(&mut self, id: DividerId) {
        let v = match id {
            DividerId::Outer(_) => Some(&mut self.outer),
            DividerId::Inner { group, .. } => self.inner.get_mut(group),
        };
        if let Some(v) = v {
            let even = 1.0 / v.len() as f32;
            v.fill(even);
        }
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

    fn close(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-5
    }

    fn sums_to_one(p: &Panes<u32>) {
        let l = p.layout();
        assert!(close(l.groups.iter().map(|g| g.share).sum(), 1.0));
        for g in &l.groups {
            assert!(close(g.panes.iter().map(|p| p.1).sum(), 1.0));
        }
    }

    #[test]
    fn dragging_a_divider_resizes_its_neighbours() {
        let mut p = Panes::new(1);
        p.split(Direction::Right, 2);
        p.split(Direction::Right, 3);
        let d = p.dividers();
        assert_eq!(d.len(), 2);
        assert!(d.iter().all(|d| d.vertical && d.from == 0.0 && d.to == 1.0));
        p.move_divider(DividerId::Outer(0), 0.5, 0.1);
        let f = p.fractions();
        assert!(close(f[0].x1, 0.5) && close(f[1].x0, 0.5));
        // The third column keeps its size.
        assert!(close(f[2].x0, 2.0 / 3.0) && f[2].x1 == 1.0);
        sums_to_one(&p);
    }

    #[test]
    fn dragging_clamps_to_the_minimum() {
        let mut p = Panes::new(1);
        p.split(Direction::Down, 2);
        p.move_divider(DividerId::Outer(0), -5.0, 0.1);
        assert!(close(p.fractions()[0].y1, 0.1));
        p.move_divider(DividerId::Outer(0), 0.99, 0.1);
        assert!(close(p.fractions()[0].y1, 0.9));
        // A minimum too large for both sides stops halfway.
        p.move_divider(DividerId::Outer(0), 0.0, 0.8);
        assert!(close(p.fractions()[0].y1, 0.5));
        // Nonsense input changes nothing.
        p.move_divider(DividerId::Outer(0), f32::NAN, 0.1);
        p.move_divider(DividerId::Outer(5), 0.3, 0.1);
        p.move_divider(DividerId::Inner { group: 9, index: 0 }, 0.3, 0.1);
        assert!(close(p.fractions()[0].y1, 0.5));
        sums_to_one(&p);
    }

    #[test]
    fn inner_dividers_move_within_their_group_and_reset_evenly() {
        let mut p = Panes::new(1);
        p.split(Direction::Right, 2);
        p.set_focus(1);
        p.split(Direction::Down, 3);
        p.move_divider(DividerId::Outer(0), 0.3, 0.1);
        let inner = DividerId::Inner { group: 1, index: 0 };
        let d = p.dividers();
        let h = d.iter().find(|d| d.id == inner).unwrap();
        assert!(!h.vertical && close(h.at, 0.5) && close(h.from, 0.3) && h.to == 1.0);
        p.move_divider(inner, 0.25, 0.1);
        let f = p.fractions();
        assert!(close(f[1].y1, 0.25) && close(f[2].y0, 0.25));
        assert!(close(f[1].x0, 0.3));
        p.reset_divider(inner);
        assert!(close(p.fractions()[1].y1, 0.5));
        assert!(close(p.fractions()[1].x0, 0.3));
        p.reset_divider(DividerId::Outer(0));
        assert!(close(p.fractions()[1].x0, 0.5));
    }

    #[test]
    fn adding_and_closing_panes_renormalises() {
        let mut p = Panes::new(1);
        p.split(Direction::Right, 2);
        p.move_divider(DividerId::Outer(0), 0.8, 0.1);
        // The new column takes a third; the others keep their 4:1 ratio.
        p.split(Direction::Right, 3);
        let s: Vec<f32> = p.layout().groups.iter().map(|g| g.share).collect();
        assert!(close(s[1], 1.0 / 3.0));
        assert!(close(s[0] / s[2], 4.0));
        sums_to_one(&p);
        p.set_focus(1);
        p.close_focused();
        let s: Vec<f32> = p.layout().groups.iter().map(|g| g.share).collect();
        assert!(close(s[0], 0.8) && close(s[1], 0.2));
        sums_to_one(&p);
    }

    #[test]
    fn flattening_keeps_the_inner_sizes() {
        let mut p = Panes::new(1);
        p.split(Direction::Right, 2);
        p.set_focus(1);
        p.split(Direction::Down, 3);
        p.move_divider(DividerId::Inner { group: 1, index: 0 }, 0.7, 0.1);
        p.set_focus(0);
        p.close_focused();
        assert_eq!(p.direction(), Direction::Down);
        assert!(close(p.fractions()[0].y1, 0.7));
        assert_eq!(p.dividers().len(), 1);
        sums_to_one(&p);
    }

    #[test]
    fn layout_round_trips() {
        let mut p = Panes::new(1);
        p.split(Direction::Right, 2);
        p.set_focus(1);
        p.split(Direction::Down, 3);
        p.move_divider(DividerId::Outer(0), 0.4, 0.1);
        p.set_focus(2);
        let q = Panes::from_layout(p.layout()).unwrap();
        assert_eq!(q.layout(), p.layout());
        assert_eq!(tabs(&q), [1, 2, 3]);
        assert_eq!(q.focus(), 2);
        let slots: Vec<_> = q.panes().map(|p| p.slot).collect();
        assert_eq!(slots, [0, 1, 2]);
    }

    #[test]
    fn filtering_a_layout_drops_panes_and_moves_focus() {
        let mut p = Panes::new(1);
        p.split(Direction::Right, 2);
        p.set_focus(1);
        p.split(Direction::Down, 3);
        p.set_focus(0);
        p.split(Direction::Right, 4);
        // Reading order 1, 4, 2, 3; focus on 2 (index 2).
        p.set_focus(2);
        let l = p.layout().filter_map(|t| (t != 2).then_some(t));
        assert_eq!(l.len(), 3);
        assert_eq!(l.focus, 1);
        let q = Panes::from_layout(l).unwrap();
        assert_eq!(tabs(&q), [1, 4, 3]);
        sums_to_one(&q);
        // Dropping a whole group, and leaving one group of two, flattens.
        let l = p.layout().filter_map(|t| (t == 2 || t == 3).then_some(t));
        assert_eq!(l.focus, 0);
        let q = Panes::from_layout(l).unwrap();
        assert_eq!(q.direction(), Direction::Down);
        assert_eq!(tabs(&q), [2, 3]);
        // One pane left is no split.
        assert_eq!(
            Panes::from_layout(p.layout().filter_map(|t| (t == 1).then_some(t))),
            None
        );
    }

    #[test]
    fn from_layout_repairs_bad_shares_and_limits_panes() {
        let l = Layout {
            dir: Direction::Right,
            groups: vec![
                GroupLayout {
                    share: f32::NAN,
                    panes: vec![(1, 0.0), (2, 1.0)],
                },
                GroupLayout {
                    share: 3.0,
                    panes: vec![(3, 2.0), (4, 2.0), (5, 1.0)],
                },
                GroupLayout {
                    share: 1.0,
                    panes: vec![],
                },
            ],
            focus: 99,
        };
        let q = Panes::from_layout(l).unwrap();
        assert_eq!(tabs(&q), [1, 2, 3, 4]);
        assert_eq!(q.focus(), 3);
        sums_to_one(&q);
        let s = q.layout();
        assert!(close(s.groups[0].share, 0.5));
        assert!(close(s.groups[0].panes[0].1, 0.5));
        // Tiny shares are raised so no pane vanishes.
        let l = Layout {
            dir: Direction::Down,
            groups: vec![
                GroupLayout {
                    share: 1e-9,
                    panes: vec![(1, 1.0)],
                },
                GroupLayout {
                    share: 1.0,
                    panes: vec![(2, 1.0)],
                },
            ],
            focus: 0,
        };
        let q = Panes::from_layout(l).unwrap();
        assert!(q.fractions()[0].y1 > 0.04);
    }
}
