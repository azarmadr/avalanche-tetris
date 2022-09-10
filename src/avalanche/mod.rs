use {
    assets::*,
    autodefault::autodefault,
    bevy::{ecs::schedule::StateData, prelude::*},
    grid::*,
    // menu::MenuPlugin,
    menu_plugin::MenuMaterials,
    // rand::seq::SliceRandom,
    // std::time::Duration,
    shapes::*,
};
mod assets;
mod components;
mod grid;
mod shapes;
mod systems;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Board;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct ScoreBoard;

#[cfg(feature = "debug")]
use bevy_inspector_egui::InspectorPlugin;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    InGame,
    Splash,
    _Menu,
}
use AppState::*;
#[derive(Deref)]
pub struct AvalancheGamePlugin<T>(pub T);
impl<T: StateData + Copy> Plugin for AvalancheGamePlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_state(AppState::Splash)
            .add_plugin(bevy_tweening::TweeningPlugin)
            .add_system_set(SystemSet::on_enter(InGame).with_system(create_grid))
            .init_resource::<BoardAssets>()
            .add_system_set(
                SystemSet::on_update(InGame)
                    // .with_system(systems::deck_complete.exclusive_system().at_end())
                    .with_system(systems::spawn_shape)
                    // .with_system(systems::move_bricks),
            )
            // .add_system_set(
            //     SystemSet::on_in_stack_update(InGame)
            //         .with_system(systems::uncover)
            //         .with_system(systems::card_flip),
            // )
            // .add_system_set(SystemSet::on_pause(InGame).with_system(hide_board))
            // .add_system_set(SystemSet::on_resume(InGame).with_system(show_board))
            // .add_system_set(SystemSet::on_exit(InGame).with_system(despawn::<Board>))
            // .add_system_set(SystemSet::on_exit(InGame).with_system(despawn::<ScoreBoard>))
            // .add_system(component_animator_system::<Visibility>)
            .add_system(bevy_tweening::component_animator_system::<UiColor>)
            // .add_plugin(MenuPlugin {
            //     game: InGame,
            //     menu: Menu,
            // })
            .add_system_set(SystemSet::on_enter(**self).with_system(splash_off))
            // .add_system_set(SystemSet::on_in_stack_update(**self).with_system(on_completion))
            // .add_system_set(SystemSet::on_exit(**self).with_system(splash_on))
            ;
        #[cfg(feature = "debug")]
        {
            app
                // .add_plugin(InspectorPlugin::<GridsnBricks>::new())
                .add_plugin(InspectorPlugin::<BoardAssets>::new())
                ;
        }
    }
}

#[autodefault]
pub fn create_grid(
    mut cmd: Commands,
    menu: Res<MenuMaterials>,
    // opts: Res<MemoryGOpts>,
    assets: Res<BoardAssets>,
) {
    // let mut rng = rand::thread_rng();
    let grid = GridsnBricks::init(7,7);
    let size = menu.size / (grid.height() + 2 * 4) as f32;
    cmd.spawn_bundle(assets.bg.node(Style {
        position_type: PositionType::Absolute,
        size: Size::new(Val::Percent(100.), Val::Percent(100.)),
        flex_direction: FlexDirection::ColumnReverse,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        align_self: AlignSelf::Center,
    }))
    .insert(Name::new("BG"))
    .insert(Board)
    .with_children(|p| {
        grid.spawn(p, size, &assets);
    });
    let score_board = TextBundle {
        style: Style {
            position: UiRect {
                left: Val::Percent(77.),
                bottom: Val::Percent(77.),
            },
        },
        ..assets.write_text("Score")
    };
    cmd.spawn_bundle(score_board)
        .insert(ScoreBoard)
        .insert(Name::new("ScoreBoard"));
    cmd.insert_resource(grid);
}
pub fn splash_off(mut state: ResMut<State<AppState>>) {
    if state.inactives().is_empty() {
        state.replace(AppState::InGame).unwrap();
    } else {
        state.pop().unwrap();
    }
}
