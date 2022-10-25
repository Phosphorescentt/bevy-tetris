use bevy::prelude::*;

use crate::AppState;

pub struct InGamePlugin;

struct UIEntity {
    text_entity: Entity,
}

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup_ui))
            .add_system_set(SystemSet::on_update(AppState::InGame).with_system(run_ui))
            .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(cleanup_ui));
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

fn run_ui(keys: Res<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if keys.just_pressed(KeyCode::P) {
        let _ = app_state.set(AppState::MainMenu);
    }
}

fn cleanup_ui(mut commands: Commands, ui_entity: Res<UIEntity>) {
    commands.entity(ui_entity.text_entity).despawn_recursive();
}
