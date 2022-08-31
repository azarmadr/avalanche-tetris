#![forbid(unsafe_code)]
#![warn(
    clippy::all,
    clippy::nursery,
    // clippy::pedantic,
    nonstandard_style,
    rustdoc::broken_intra_doc_links
)]
#![allow(
    clippy::default_trait_access,
    clippy::module_name_repetitions,
    clippy::redundant_pub_crate
)]
use bevy::log::LogSettings;
use {avalanche::*, bevy::prelude::*, menu_plugin::MenuMaterials, std::time::Duration};

mod avalanche;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Game {
    Avalnche,
    Menu,
}

#[bevy_main]
pub fn main() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: "avalanche-tetris".to_string(),
        width: 600.,
        height: 600.,
        ..default()
    })
    .insert_resource(LogSettings { ..default() });
    app.add_plugins(DefaultPlugins)
        .init_resource::<MenuMaterials>();

    app.add_plugin(AvalancheGamePlugin(Game::Avalnche))
        .add_state(Game::Menu)
        .add_startup_system(startup)
        .add_system(game_timer);

    #[cfg(target_arch = "wasm32")]
    app.add_system(handle_browser_resize);

    // Debug hierarchy inspector
    #[cfg(feature = "debug")]
    app.add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new());

    app.run();
}

/// Pre launch setup of assets and options
fn startup(mut commands: Commands, mut menu: ResMut<MenuMaterials>, mut windows: ResMut<Windows>) {
    commands.spawn_bundle(Camera3dBundle::default());
    let window = windows.primary_mut();
    // window.set_maximized(true);
    info!("{window:?}");
    menu.size = window
        .requested_width()
        .min(0.8 * window.requested_height());
    // menu.size = window.physical_width().min(window.physical_height()) as f32;
}
fn game_timer(mut state: ResMut<State<Game>>, time: Res<Time>, mut timer: Local<Timer>) {
    if timer.duration() == Duration::ZERO {
        timer.set_duration(Duration::from_millis(9));
    }
    if timer.tick(time.delta()).just_finished() {
        state.replace(Game::Avalnche).unwrap();
    }
}
#[cfg(target_arch = "wasm32")]
fn handle_browser_resize(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    let wasm_window = web_sys::window().unwrap();
    let (target_width, target_height) = (
        wasm_window.inner_width().unwrap().as_f64().unwrap() as f32,
        wasm_window.inner_height().unwrap().as_f64().unwrap() as f32,
    );
    if window.width() != target_width || window.height() != target_height {
        window.set_resolution(target_width, target_height);
    }
}
