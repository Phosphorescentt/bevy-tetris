use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(say_hi);
    }
}

fn say_hi() {
    println!("Hi from MainMenu!")
}
