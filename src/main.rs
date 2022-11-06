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
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_system(say_hi)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn say_hi() {
    println!("Hey")
}
