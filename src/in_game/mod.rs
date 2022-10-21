use bevy::prelude::*;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(say_hi);
    }
}

fn say_hi() {
    println!("Hi from InGame!")
}
