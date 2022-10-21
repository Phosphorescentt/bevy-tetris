mod in_game;
// mod main_menu;

use bevy::prelude::*;

enum AppState {
    MainMenu,
    InGame,
    Paused,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugin(main_menu::MainMenuPlugin)
        .add_plugin(in_game::InGamePlugin)
        .run();
}
