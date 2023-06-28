use bevy::prelude::*;

pub mod systems;
use systems::*;
//use crate::components::{WinSize};

/// This system prints out all mouse events as they come in

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(print_mouse_events_system);
    }
}
