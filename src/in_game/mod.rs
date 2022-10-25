use bevy::prelude::*;

use crate::AppState;

pub struct InGamePlugin;

struct UIEntity {
    text_entity: Entity,
}

struct PauseUIEntity {
    text_entity: Entity,
}

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup_ui))
            .add_system_set(SystemSet::on_update(AppState::InGame).with_system(run_ui))
            .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(cleanup_ui))
            .add_system_set(SystemSet::on_enter(AppState::Paused).with_system(setup_pause_ui))
            .add_system_set(SystemSet::on_update(AppState::Paused).with_system(run_pause_ui))
            .add_system_set(SystemSet::on_exit(AppState::Paused).with_system(cleanup_pause_ui));
    }
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
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

fn run_ui(mut keys: ResMut<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if keys.just_pressed(KeyCode::Escape) {
        let _ = app_state.set(AppState::MainMenu);
        keys.clear();
    }

    if keys.just_pressed(KeyCode::P) {
        let _ = app_state.push(AppState::Paused).unwrap();
        keys.clear();
    }
}

fn cleanup_ui(mut commands: Commands, ui_entity: Res<UIEntity>) {
    commands.entity(ui_entity.text_entity).despawn_recursive();
}

fn setup_pause_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
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

fn run_pause_ui(mut keys: ResMut<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if keys.just_pressed(KeyCode::Escape) {
        let _ = app_state.pop().unwrap();
        keys.clear();
    }
}

fn cleanup_pause_ui(mut commands: Commands, pause_ui_entity: Res<PauseUIEntity>) {
    commands
        .entity(pause_ui_entity.text_entity)
        .despawn_recursive();
}
