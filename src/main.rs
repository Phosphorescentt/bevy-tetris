mod in_game;
mod main_menu;

// use in_game::InGamePlugin;
// use main_menu::MainMenuPlugin;

use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    MainMenu,
    InGame,
    Paused,
}

fn main() {
    App::new()
        .add_state(AppState::MainMenu)
        // .add_state(AppState::InGame)
        // .add_state(AppState::Paused)
        .add_plugins(DefaultPlugins)
        .add_plugin(main_menu::MainMenuPlugin)
        .add_plugin(in_game::InGamePlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
