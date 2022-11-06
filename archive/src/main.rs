mod in_game;
mod main_menu;

use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, InspectorPlugin, WorldInspectorPlugin};

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
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(main_menu::MainMenuPlugin)
        .add_plugin(in_game::InGamePlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
