use super::assets::BoardAssets;
use super::components::Idx;
use super::grid::GridsnBricks;
use super::shapes::Dir;
use super::ScoreBoard;

use bevy::prelude::*;

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
    mut local: Local<bool>,
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
        *local = false;
        grid.play(&dir);
        grid.gen_tray_brick();
    }
    if !*local {
        *local = true;
        let anim = |c: Color, n| {
            Tween::new(
                EaseFunction::QuadraticIn,
                TweeningType::Once,
                ANIM_TIME * n,
                BeTween::with_lerp(move |e: &mut UiColor, s: &UiColor, r| {
                    let start: Vec4 = s.0.into();
                    *e = UiColor(start.lerp(c.into(), r).into());
                }),
            )
        };
        for (e, &id, d) in color.iter() {
            cmd.entity(e).insert(Animator::new(anim(
                {
                    let val = grid.get_dot_val(id.0, d) as usize;
                    if val > 0 {
                        assets.dot[val].color
                    } else {
                        assets.sq.color
                    }
                },
                1,
            )));
        }
        for dot in grid.clear_lines() {
            for (e, &id, d) in color.iter() {
                if d.is_none() && id.0 == dot {
                    cmd.entity(e).insert(Animator::new(
                        anim(Color::ORANGE_RED, 7).then(anim(Color::NONE, 3)),
                    ));
                }
            }
        }
        info!("New Turn\n");
        grid.bricks.iter().for_each(|b| trace!("{b:?}"));
        score.single_mut().sections[0].value = format!("Score: {}", grid.score());
    }
}
