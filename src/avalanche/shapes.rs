use {bevy::prelude::*, strum_macros::EnumIter};

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Default, Debug, EnumIter, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Shape {
    #[default]
    L,
    S,
    I,
    O,
    T,
    Z,
}
use Shape::*;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Clone, Component, Copy, Default, EnumIter, Hash, PartialEq, Eq)]
pub enum Dir {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    pub const fn opp(&self) -> Self {
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }

    pub const fn turn(&self) -> Self {
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    pub const fn is_top_right(&self) -> bool {
        match self {
            Right | Up => true,
            Left | Down => false,
        }
    }
    pub fn if_tr<T>(&self, y: T, n: T) -> T {
        if self.is_top_right() {
            y
        } else {
            n
        }
    }
    pub fn if_h<T>(&self, y: T, n: T) -> T {
        if self.is_horizontal() {
            y
        } else {
            n
        }
    }
    pub const fn is_horizontal(self) -> bool {
        match self {
            Right | Left => true,
            Up | Down => false,
        }
    }
}
use strum::IntoEnumIterator;
use Dir::*;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Clone, Copy, Default)]
pub struct Dot(pub u8, pub u8);
impl Dot {
    pub const fn to_idx(self, orig: u8, width: u8) -> usize {
        (self.0 + self.1 * width + orig) as usize
    }
    pub fn group_connected(ids: &[Self]) -> Vec<Vec<Self>> {
        let mut res = Vec::new();
        let mut ids = ids.to_vec();
        while !ids.is_empty() {
            let mut cs = vec![ids.pop().unwrap()];

            let mut d = true;
            while d{
                d = false;
                let mut i = 0;
                while i < ids.len() {
                    if cs.iter().any(|&x| x.0.abs_diff(ids[i].0) + x.1.abs_diff(ids[i].1) == 1) {
                        cs.push(ids[i]);
                        ids.remove(i);
                        d = true;
                    } else {i+=1;}
                }
            }
            res.push(cs);
        }
        if res.is_empty() {
            vec![vec![]]
        } else {res}
    }
}

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Clone, Default)]
pub struct Brick(pub Vec<Dot>, pub u8, pub Shape);

impl Brick {
    pub fn from(shape: Shape, dir: Dir) -> Self {
        Self(
            match (shape, dir) {
                (O, _)            => vec![Dot(0, 0), Dot(1, 0), Dot(1, 1), Dot(0, 1)],
                (L, Up)           => vec![Dot(1, 0), Dot(0, 0), Dot(0, 1), Dot(0, 2)],
                (L, Right)        => vec![Dot(0, 0), Dot(0, 1), Dot(1, 1), Dot(2, 1)],
                (L, Down)         => vec![Dot(0, 2), Dot(1, 2), Dot(1, 1), Dot(1, 0)],
                (L, Left)         => vec![Dot(0, 0), Dot(1, 0), Dot(2, 0), Dot(2, 1)],
                (T, Up)           => vec![Dot(0, 1), Dot(1, 1), Dot(2, 1), Dot(1, 0)],
                (T, Right)        => vec![Dot(0, 1), Dot(1, 1), Dot(1, 0), Dot(1, 2)],
                (T, Down)         => vec![Dot(0, 0), Dot(0, 1), Dot(0, 2), Dot(1, 1)],
                (T, Left)         => vec![Dot(0, 0), Dot(0, 1), Dot(0, 2), Dot(1, 1)],
                (S, Up | Down)    => vec![Dot(0, 0), Dot(1, 0), Dot(1, 1), Dot(2, 1)],
                (S, Right | Left) => vec![Dot(0, 2), Dot(0, 1), Dot(1, 1), Dot(1, 0)],
                (Z, Up | Down)    => vec![Dot(0, 1), Dot(1, 1), Dot(1, 0), Dot(2, 0)],
                (Z, Right | Left) => vec![Dot(0, 0), Dot(0, 1), Dot(1, 1), Dot(1, 2)],
                (I, Up | Down)    => vec![Dot(0, 0), Dot(0, 1), Dot(0, 2), Dot(0, 3)],
                (I, Right | Left) => vec![Dot(0, 0), Dot(1, 0), Dot(2, 0), Dot(3, 0)],
            },
            0,
            shape,
        )
    }

    pub fn iterator() -> impl Iterator<Item = Self> {
        [
            (O, Up),
            (L, Up),
            (L, Right),
            (L, Down),
            (L, Left),
            (T, Up),
            (T, Right),
            (T, Down),
            (T, Left),
            (S, Up),
            (S, Right),
            (Z, Up),
            (Z, Right),
            (I, Up),
            (I, Right),
        ]
        .iter()
        .map(|&(shape, dir)| Self::from(shape, dir))
    }
    pub fn orig(&self) -> u8 {self.1}
    pub fn iter_for_width(&self, width: u8) -> impl Iterator<Item = usize> + '_ {
        self.0.iter().map(move |x| x.to_idx(self.1, width))
    }

    pub fn contains(&self, id: usize, width: u8) -> bool {
        self.iter_for_width(width).any(|x| x == id)
    }
    pub fn contains_any(&self, ids: &[usize], width: u8) -> bool {
        self.iter_for_width(width)
            .any(|x| ids.iter().any(|&y| x == y))
    }
    pub fn height(&self) -> u8 {
        self.0.iter().fold(0, |a, x| if x.1 > a { x.1 } else { a })
    }
    pub fn width(&self) -> u8 {
        self.0.iter().fold(0, |a, x| if x.0 > a { x.0 } else { a })
    }
    pub fn dim_in(&self, dir: Dir) -> u8 {
        dir.if_h(self.width(), self.height())
    }
    pub fn reshift_orig(&mut self,width: u8) {
        let shift = self.0.iter().fold((3u8, 3u8), |a, &d| {
            (
                if d.0 < a.0 { d.0 } else { a.0 },
                if d.1 < a.1 { d.1 } else { a.1 },
            )
        });
        self.1 += shift.0 + width * shift.1;
        self.0.iter_mut().for_each(|d| {
            d.0 -= shift.0;
            d.1 -= shift.1
        });
    }
    pub fn cut_at(&mut self, ids: &[usize], width: u8) -> Vec<Self> {
        let orig = self.1;
        self.0.retain(|d| !ids.contains(&d.to_idx(self.1, width)));
        let mut dot_groups = Dot::group_connected(&self.0);
        trace!("After: {self:?} dg: {dot_groups:?}");
        self.0 = dot_groups.pop().unwrap();
        self.reshift_orig(width);
        dot_groups.iter().map(|g| {
            let mut b = Self(g.to_vec(),orig,self.2);
            b.reshift_orig(width);
            b
        }).collect()
    }
}

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Clone, Default)]
pub struct Grid<T: Default> {
    grid: Vec<T>,
    bricks: Vec<Brick>,
    total: u8,
    width: u8,
}
