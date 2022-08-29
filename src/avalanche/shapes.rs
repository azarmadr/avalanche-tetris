use {bevy::log::info, bevy::prelude::*, strum_macros::EnumIter};

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
use Dir::*;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Clone, Copy, Default)]
pub struct Dot(pub u8, pub u8);
impl Dot {
    pub const fn to_idx(self, orig: u8, width: u8) -> usize {
        (self.0 + self.1 * width + orig) as usize
    }
}

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Clone, Copy, Default)]
pub struct Brick(pub [Dot; 4], pub u8, pub Shape);

impl Brick {
    pub const fn from(shape: Shape, dir: Dir) -> Self {
        Self(
            match (shape, dir) {
                (O, _) => [Dot(0, 0), Dot(1, 0), Dot(0, 1), Dot(1, 1)],
                (L, Up) => [Dot(0, 0), Dot(0, 1), Dot(0, 2), Dot(1, 0)],
                (L, Right) => [Dot(0, 0), Dot(0, 1), Dot(1, 1), Dot(2, 1)],
                (L, Down) => [Dot(0, 2), Dot(1, 2), Dot(1, 1), Dot(1, 0)],
                (L, Left) => [Dot(0, 0), Dot(1, 0), Dot(2, 0), Dot(2, 1)],
                (T, Up) => [Dot(0, 1), Dot(1, 1), Dot(2, 1), Dot(1, 0)],
                (T, Right) => [Dot(0, 1), Dot(1, 1), Dot(1, 0), Dot(1, 2)],
                (T, Down) => [Dot(0, 0), Dot(0, 1), Dot(0, 2), Dot(1, 1)],
                (T, Left) => [Dot(0, 0), Dot(0, 1), Dot(0, 2), Dot(1, 1)],
                (S, Up | Down) => [Dot(0, 0), Dot(0, 1), Dot(1, 1), Dot(1, 2)],
                (S, Right | Left) => [Dot(0, 1), Dot(0, 2), Dot(1, 1), Dot(1, 0)],
                (I, Up | Down) => [Dot(0, 0), Dot(0, 1), Dot(0, 2), Dot(0, 3)],
                (I, Right | Left) => [Dot(0, 0), Dot(1, 0), Dot(2, 0), Dot(3, 0)],
                (Z, Up | Down) => [Dot(0, 1), Dot(1, 1), Dot(1, 0), Dot(2, 0)],
                (Z, Right | Left) => [Dot(0, 0), Dot(0, 1), Dot(1, 1), Dot(1, 2)],
            },
            0,
            shape,
        )
    }

    pub fn contains(&self, id: usize, width: u8) -> bool {
        self.0.iter().any(|x| x.to_idx(self.1, width) == id)
    }
    pub fn contains_any(&self, ids: &Vec<usize>, width: u8) -> bool {
        self.0
            .iter()
            .any(|x| ids.iter().any(|&y| x.to_idx(self.1, width) == y))
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
    // pub fn can_move(&self, dir:Dir, width: u8) -> bool{
    //     if dir.if_tr()
    //     self.1 + sefl.dim_in(dir) < width - 1
    // }
}
