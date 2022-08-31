use autodefault::autodefault;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use strum::IntoEnumIterator;

use bevy::prelude::*;

use super::assets::BoardAssets;
use super::components::Idx;
use super::shapes::{Brick, Dir};

pub type Sq = bool;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Clone)]
pub struct GridsnBricks {
    grid: Vec<Sq>,
    tray: HashMap<Dir, Vec<Sq>>,
    pub bricks: Vec<Brick>,
    pub tray_bricks: HashMap<Dir, Vec<Brick>>,
    width: u8,
    height: u8,
    score: u32,
}

impl Default for GridsnBricks {
    fn default() -> Self {
        Self {
            grid: vec![false; 7 * 7],
            bricks: vec![],
            tray: Dir::iter().map(|dir| (dir, vec![false; 4 * 7])).collect(),
            tray_bricks: Dir::iter().map(|dir| (dir, vec![])).collect(),
            width: 7,
            height: 7,
            score: 0,
        }
    }
}

impl GridsnBricks {
    pub fn init() -> Self {
        // let grid = [false; GRID_SIZE];
        // for &square in self.grid.iter_mut(){ square = false }
        Self::default()
    }
    pub const fn size(&self) -> u8 {
        self.height * self.width
    }
    pub const fn width(&self) -> u8 {
        self.width
    }
    pub const fn height(&self) -> u8 {
        self.height
    }
    fn tray_to_grid(&mut self, dir: &Dir) {
        let mut bricks = self.tray_bricks.remove(dir).unwrap();
        bricks.retain_mut(|brick| {
            let height = brick.height();
            let width = brick.width();
            trace!("{height} {width}");
            let orig = brick.1;
            let occupied = self.occupy_grid(
                brick,
                match dir {
                    Dir::Left => brick.1 / 4 * self.width,
                    Dir::Down => brick.1,
                    Dir::Right => brick.1 / 4 * self.width + self.width - width - 1,
                    Dir::Up => brick.1 + self.width * (self.height - height - 1),
                },
            );
            if occupied {
                let width = dir.if_h(4, self.width);
                for d in brick.0.iter() {
                    self.tray.get_mut(dir).unwrap()[d.to_idx(orig, width)] = false
                }
            }
            !occupied
        });
        self.tray_bricks.insert(*dir, bricks);
    }
    fn occupy_grid(&mut self, brick: &mut Brick, orig: u8) -> bool {
        let can_occupy = !brick
            .0
            .iter()
            .any(|&d| self.grid[d.to_idx(orig, self.width)]);
        if can_occupy {
            for dot in &brick.0 {
                self.grid[dot.to_idx(orig, self.width)] = true;
            }
            brick.1 = orig;
            self.bricks.push(brick.clone());
        }
        can_occupy
    }

    pub fn occupy_tray(&mut self, dir: Dir, brick: Brick) {
        let grid = self.tray.get_mut(&dir).unwrap();
        let width = dir.if_h(4, self.width);
        if !brick.iter_for_width(width).any(|d| grid[d]) {
            trace!("dir: {dir:?} brick: {brick:?}");
            brick.iter_for_width(width).for_each(|d| grid[d] = true);
            self.tray_bricks.get_mut(&dir).unwrap().push(brick);
        }
    }

    /// 1. try to move bricks on the grid
    /// 2. then bring from the tray
    pub fn play(&mut self, dir: &Dir) {
        let mut dirty = true;
        let mut ids = Vec::new();
        while dirty {
            dirty = false;
        for (i,b) in self.bricks.iter_mut().enumerate() {
            if ids.contains(&i) {continue;}
            let orig_p = dir.if_h(b.1 % self.width, b.1 / self.width);
            if dir.if_tr(orig_p + b.dim_in(*dir) < self.width - 1, orig_p > 0) {
                let delta = |x| match dir {
                    Dir::Up    => x+ self.width as usize,
                    Dir::Down  => x- self.width as usize,
                    Dir::Left  => x- 1,
                    Dir::Right => x+ 1,
                };
                b.iter_for_width(self.width).for_each(|d| self.grid[d]=false);
                if !b.iter_for_width(self.width).any(|d| self.grid[delta(d)]) {
                    b.1 = delta(b.1 as usize) as u8;
                    dirty = true;
                    ids.push(i)
                }
                b.iter_for_width(self.width).for_each(|d| self.grid[d]=true);
            } else { ids.push(i)}
        }}
        self.tray_to_grid(&dir.opp());
    }

    pub fn clear_lines(&mut self) -> Vec<usize> {
        let w = self.width as usize;
        let mut cleared: Vec<usize> = vec![];
        for idx in 0..w {
            let h = self
                .grid
                .iter()
                .enumerate()
                .all(|(i, &x)| i / w != idx || x);
            let v = self
                .grid
                .iter()
                .enumerate()
                .all(|(i, &x)| i % w != idx || x);
            if h {
                self.score += 1
            }
            if v {
                self.score += 1
            }

            for i in 0..w {
                if h {
                    cleared.push(idx * w + i)
                }
                if v {
                    cleared.push(idx + w * i)
                }
            }
        }
        if !cleared.is_empty(){
        let mut cleared_bricks: Vec<Brick> = vec![];
        self.bricks.iter_mut().for_each(|b| {
            if b.contains_any(&cleared, w as u8) {
                cleared_bricks.append(&mut b.cut_at(&cleared, w as u8));
            }
        });
        self.bricks.retain(|b| !b.0.is_empty());
        self.bricks.append(&mut cleared_bricks);
        for &ele in cleared.iter() {
            self.grid[ele] = false;
        }}
        cleared
    }

    // pub fn get_mut_tray(&mut self, dir:&Dir) -> GridType { *self.tray.get(dir).unwrap() }
    #[autodefault]
    pub fn spawn(&self, parent: &mut ChildBuilder, size: f32, assets: &BoardAssets) {
        let grid_styles = |width: u8, height: u8| Style {
            size: Size::new(
                Val::Px((size + 2.2) * height as f32),
                Val::Px((size + 2.2) * width as f32),
            ),
            flex_wrap: FlexWrap::Wrap,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_content: AlignContent::Center,
        };
        let ssq = |p: &mut ChildBuilder, (i, _), d| {
            p.spawn_bundle(assets.sq.node(Style {
                size: Size::new(Val::Px(size), Val::Px(size)),
                margin: UiRect::all(Val::Px(1.0)),
            }))
            .insert(Name::new(format!("Sq ({i})")))
            .insert(d)
            .insert(Idx(i))
            .with_children(|_p| {
                #[cfg(feature = "debug")]
                _p.spawn_bundle(assets.write_text(format!("{i}")));
            });
        };
        parent
            .spawn_bundle(assets.tray.node(grid_styles(4, self.height)))
            .with_children(|p| {
                self.tray
                    .get(&Dir::Up)
                    .unwrap()
                    .iter()
                    .enumerate()
                    .for_each(|x| {
                        ssq(p, x, Dir::Up);
                    });
            });
        parent
            .spawn_bundle(assets.bg.node(Style {
                // size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
            }))
            .with_children(|p| {
                p.spawn_bundle(assets.tray.node(grid_styles(self.height, 4)))
                    .with_children(|p| {
                        self.tray
                            .get(&Dir::Left)
                            .unwrap()
                            .iter()
                            .enumerate()
                            .for_each(|x| {
                                ssq(p, x, Dir::Left);
                            });
                    });
                p.spawn_bundle(assets.board.node(grid_styles(self.height, self.width)))
                    .with_children(|p| {
                        self.iter().enumerate().for_each(|(i, _)| {
                            p.spawn_bundle(assets.sq.node(Style {
                                size: Size::new(Val::Px(size), Val::Px(size)),
                                margin: UiRect::all(Val::Px(1.0)),
                            }))
                            .insert(Name::new(format!("Sq ({i})")))
                            .with_children(|_p| {
                                #[cfg(feature = "debug")]
                                _p.spawn_bundle(assets.write_text(format!("{i}")));
                            })
                            .insert(Idx(i));
                        });
                    });
                p.spawn_bundle(assets.tray.node(grid_styles(self.height, 4)))
                    .with_children(|p| {
                        self.tray
                            .get(&Dir::Right)
                            .unwrap()
                            .iter()
                            .enumerate()
                            .for_each(|x| {
                                ssq(p, x, Dir::Right);
                            });
                    });
            });
        parent
            .spawn_bundle(assets.tray.node(grid_styles(4, self.height)))
            .with_children(|p| {
                self.tray
                    .get(&Dir::Down)
                    .unwrap()
                    .iter()
                    .enumerate()
                    .for_each(|x| {
                        ssq(p, x, Dir::Down);
                    });
            });
    }
    pub const fn score(&self) -> u32 {
        self.score
    }
}
impl Deref for GridsnBricks {
    type Target = Vec<Sq>;

    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}
impl DerefMut for GridsnBricks {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.grid
    }
}
