mod game;
mod ui;

use bevy::prelude::*;
use bevy::time::FixedTimestep;

use crate::AppState;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .with_system(ui::setup_ui)
                .with_system(game::make_square)
                .with_system(game::setup_game),
        )
        .add_system_set(SystemSet::on_update(AppState::InGame).with_system(ui::run_ui))
        .add_system_set(
            SystemSet::on_exit(AppState::InGame)
                .with_system(ui::cleanup_ui)
                .with_system(game::remove_square)
                .with_system(game::cleanup_game),
        )
        .add_system_set(SystemSet::on_enter(AppState::Paused).with_system(ui::setup_pause_ui))
        .add_system_set(SystemSet::on_update(AppState::Paused).with_system(ui::run_pause_ui))
        .add_system_set(SystemSet::on_exit(AppState::Paused).with_system(ui::cleanup_pause_ui))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.5))
                .with_system(game::move_square_on_tick),
        );
    }
}
