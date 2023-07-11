use bevy::prelude::*;

pub mod systems;
use systems::*;
//use crate::components::{WinSize};

/// This system prints out all mouse events as they come in

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_startup_system(spawn_crosshair)
            .add_system(player_movement)
            .add_system(update_crosshair)
            .add_system(rotate_player)
            .add_system(player_enemy_collision);
        
    }
}
