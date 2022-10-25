use bevy::prelude::*;

use crate::AppState;

pub struct MainMenuPlugin;

struct MenuEntity {
    text_entity: Entity,
}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(setup_menu))
            .add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(run_menu))
            .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(cleanup_menu));
    }
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_entity = commands
        .spawn_bundle(TextBundle::from_section(
            "MainMenu",
            TextStyle {
                font: asset_server.load("fonts/Silkscreen-Regular.ttf"),
                font_size: 60.0,
                color: Color::WHITE,
            },
        ))
        .id();

    commands.insert_resource(MenuEntity { text_entity });
}

fn run_menu(keys: Res<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if keys.just_pressed(KeyCode::O) {
        let _ = app_state.set(AppState::InGame);
    }
}

fn cleanup_menu(mut commands: Commands, menu_entity: Res<MenuEntity>) {
    commands.entity(menu_entity.text_entity).despawn_recursive();
}
