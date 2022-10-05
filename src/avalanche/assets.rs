use autodefault::autodefault;
use std::collections::HashMap;

use bevy::prelude::*;
use bevy::render::texture::DEFAULT_IMAGE_HANDLE;
use strum::IntoEnumIterator;

use super::shapes::Shape;

/// Material of a `Sprite` with a texture and color
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Clone)]
pub struct SpriteMaterial {
    pub color: Color,
    pub texture: Handle<Image>,
}
impl SpriteMaterial {
    #[autodefault::autodefault]
    pub fn _sprite(&self, custom_size: Vec2, transform: Transform) -> SpriteBundle {
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(custom_size),
                color: self.color,
            },
            texture: self.texture.clone(),
            transform,
        }
    }
    #[autodefault::autodefault]
    pub fn node(&self, style: Style) -> NodeBundle {
        NodeBundle {
            style,
            color: self.color.into(),
            image: self.texture.clone().into(),
        }
    }
    pub fn _button(&self, style: Style) -> ButtonBundle {
        ButtonBundle {
            style,
            color: self.color.into(),
            image: self.texture.clone().into(),
            ..Default::default()
        }
    }
}
impl Default for SpriteMaterial {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            texture: DEFAULT_IMAGE_HANDLE.typed(),
        }
    }
}
/// Assets for the board. Must be used as a resource.
///
/// Use the loader for partial setup
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Clone)]
pub struct BoardAssets {
    pub board: SpriteMaterial,
    pub tray: SpriteMaterial,
    pub bg: SpriteMaterial,
    pub sq: SpriteMaterial,
    pub brick: HashMap<Shape, SpriteMaterial>,
    pub dot: Vec<SpriteMaterial>,
    pub font: Handle<Font>,
}
impl FromWorld for BoardAssets {
    #[autodefault::autodefault(except(BoardAssets))]
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        BoardAssets {
            bg: SpriteMaterial { color: Color::NONE },
            board: SpriteMaterial {
                color: Color::rgb_u8(112, 112, 255),
            },
            tray: SpriteMaterial { color: Color::PINK },
            sq: SpriteMaterial {
                texture: asset_server.load("sprites/red.png"),
                color: Color::NONE,
            },
            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
            dot: [
                Color::LIME_GREEN,
                Color::TEAL,
                Color::AQUAMARINE,
                Color::TOMATO,
                Color::MAROON,
                Color::PURPLE,
            ]
            .iter()
            .map(|&color| SpriteMaterial {
                color,
                texture: asset_server.load("sprites/red.png"),
            })
            .collect(),
            brick: Shape::iter()
                .zip(
                    [
                        Color::LIME_GREEN,
                        Color::TEAL,
                        Color::AQUAMARINE,
                        Color::TOMATO,
                        Color::MAROON,
                        Color::PURPLE,
                    ]
                    .iter()
                    .map(|&color| SpriteMaterial {
                        color,
                        texture: asset_server.load("sprites/red.png"),
                    }),
                )
                .collect(),
        }
    }
}
impl BoardAssets {
    #[autodefault(except(TextStyle, TextAlignment))]
    pub fn write_text<S: Into<String>>(&self, label: S) -> TextBundle {
        TextBundle {
            style: Style {
                margin: UiRect {
                    right: Val::Px(1.),
                    left: Val::Px(1.),
                },
                flex_basis: Val::Px(0.),
            },
            text: Text::from_section(
                label.into(),
                TextStyle {
                    font: self.font.clone(),
                    font_size: 18.,
                    color: Color::RED,
                },
            )
            .with_alignment(TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            }),
        }
    }
}
