use rand::seq::IteratorRandom;
use rand::Rng;
use strum::IntoEnumIterator;

use crate::avalanche::shapes::Brick;

use super::assets::BoardAssets;
use super::components::Idx;
use super::grid::GridsnBricks;
use super::shapes::Dir;
use super::ScoreBoard;

use {super::Shape, bevy::prelude::*};

use bevy_tweening::{lens::*, *};

pub(crate) const ANIM_TIME: std::time::Duration = std::time::Duration::from_millis(81);

pub type Lerp<T> = dyn Fn(&mut T, &T, f32) + Send + Sync + 'static;
pub struct BeTween<T> {
    lerp: Box<Lerp<T>>,
    start: Option<T>,
}
impl<T> BeTween<T> {
    /// Construct a lens from a pair of getter functions
    pub fn with_lerp<U>(lerp: U) -> Self
    where
        U: Fn(&mut T, &T, f32) + Send + Sync + 'static,
    {
        Self {
            lerp: Box::new(lerp),
            start: None,
        }
    }
}
impl<T: Clone> Lens<T> for BeTween<T> {
    fn lerp(&mut self, target: &mut T, ratio: f32) {
        if self.start.is_none() {
            self.start = Some(target.clone());
        }
        if let Some(start) = &self.start {
            (self.lerp)(target, start, ratio);
        }
    }
}

pub fn spawn_shape(
    // mut cmd: Commands,
    keys: Res<Input<KeyCode>>,
    mut grid: ResMut<GridsnBricks>,
    // mut local: Local<u8>,
    assets: Res<BoardAssets>,
    color: Query<(Entity, &Idx, Option<&Dir>)>,
    mut score: Query<&mut Text, With<ScoreBoard>>,
    mut cmd: Commands,
) {
    if let Some(dir) = if keys.just_pressed(KeyCode::Left) {
        Some(Dir::Left)
    } else if keys.just_pressed(KeyCode::Right) {
        Some(Dir::Right)
    } else if keys.just_pressed(KeyCode::Up) {
        Some(Dir::Up)
    } else if keys.just_pressed(KeyCode::Down) {
        Some(Dir::Down)
    } else {
        None
    } {
        // if *local == 0 
        {
            let mut rng = rand::thread_rng();
            let dir = Dir::iter().choose(&mut rng).unwrap();
            let mut brick = Brick::iterator().choose(&mut rng).unwrap();
            let max = grid.width() - brick.dim_in(dir.turn());
            brick.1 = rng.gen_range(0..max) * dir.if_h(4, 1);

            grid.occupy_tray(dir, brick);
        }
        // *local = (*local + 1) % 3;
        grid.play(&dir);
        let anim = |c: Color, n| Tween::new(
                EaseFunction::QuadraticIn,
                TweeningType::Once,
                ANIM_TIME*n,
                BeTween::with_lerp(move |e: &mut UiColor, s: &UiColor, r| {
                    let start: Vec4 = s.0.into();
                    *e = UiColor(start.lerp(c.into(), r).into());
                }),
            );
        for (e, _, _) in color.iter() {
            cmd.entity(e).insert(Animator::new(anim(assets.sq.color, 1)));
        }
        let w = grid.width();
        for dir in Dir::iter() {
            for brick in grid.tray_bricks.get(&dir).unwrap() {
                for (e, &id, d) in color.iter() {
                    if d.map_or(false, |&x| dir == x) && brick.contains(id.0, dir.if_h(4, w)) {
                        cmd.entity(e)
                            .insert(Animator::new(anim(assets.brick.get(&brick.2).unwrap().color,1)));
                    }
                }
            }
        }
        for brick in grid.bricks.iter() {
            for (e, &id, d) in color.iter() {
                if d.is_none() && brick.contains(id.0, w) {
                    cmd.entity(e)
                        .insert(Animator::new(anim(assets.brick.get(&brick.2).unwrap().color,1)));
                }
            }
        }
        for dot in grid.clear_lines() {
            for (e, &id, d) in color.iter() {
                if d.is_none() && id.0 == dot {
                    cmd.entity(e).insert(Animator::new(anim(Color::ORANGE_RED,7).then(anim(Color::NONE,3))));
                }
            }
        }
        info!("New Turn\n");
        grid.bricks.iter().for_each(|b|trace!("{b:?}"));
        score.single_mut().sections[0].value = format!("Score: {}", grid.score());
    }
}
