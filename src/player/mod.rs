use bevy::prelude::*;

pub mod systems;
use systems::*;
//use crate::components::{WinSize};

/// This system prints out all mouse events as they come in

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Startup, spawn_crosshair)
            .add_systems(Update, player_movement)
            .add_systems(Update, update_crosshair)
            .add_systems(Update, rotate_player)
            .add_systems(Update, player_enemy_collision)
            .add_systems(Update, reload_gun);
    }
}
