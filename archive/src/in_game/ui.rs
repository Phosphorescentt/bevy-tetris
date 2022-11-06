use bevy::prelude::*;

use crate::AppState;

pub struct UIEntity {
    text_entity: Entity,
}

pub struct PauseUIEntity {
    text_entity: Entity,
}

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_entity = commands
        .spawn_bundle(TextBundle::from_section(
            "InGame",
            TextStyle {
                font: asset_server.load("fonts/Silkscreen-Regular.ttf"),
                font_size: 60.0,
                color: Color::WHITE,
            },
        ))
        .id();

    commands.insert_resource(UIEntity { text_entity });
}

pub fn run_ui(mut keys: ResMut<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if keys.just_pressed(KeyCode::Escape) {
        let _ = app_state.set(AppState::MainMenu);
        keys.clear();
    }

    if keys.just_pressed(KeyCode::P) {
        let _ = app_state.push(AppState::Paused).unwrap();
        keys.clear();
    }
}

pub fn cleanup_ui(mut commands: Commands, ui_entity: Res<UIEntity>) {
    commands.entity(ui_entity.text_entity).despawn_recursive();
}

pub fn setup_pause_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ui_text_entity = commands
        .spawn_bundle(TextBundle::from_section(
            "Paused",
            TextStyle {
                font: asset_server.load("fonts/Silkscreen-Regular.ttf"),
                font_size: 60.0,
                color: Color::WHITE,
            },
        ))
        .id();

    commands.insert_resource(PauseUIEntity {
        text_entity: ui_text_entity,
    });
}

pub fn run_pause_ui(mut keys: ResMut<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if keys.just_pressed(KeyCode::Escape) {
        let _ = app_state.pop().unwrap();
        keys.clear();
    }
}

pub fn cleanup_pause_ui(mut commands: Commands, pause_ui_entity: Res<PauseUIEntity>) {
    commands
        .entity(pause_ui_entity.text_entity)
        .despawn_recursive();
}
