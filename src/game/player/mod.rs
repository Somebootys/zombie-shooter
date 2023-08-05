use bevy::prelude::*;

pub mod systems;
use systems::*;
//use crate::components::{WinSize};
use crate::components::AppState;
/// This system prints out all mouse events as they come in

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), (spawn_player, spawn_crosshair))
            .add_systems(
                Update,
                (
                    player_movement,
                    update_crosshair,
                    rotate_player,
                    fire_gun.after(rotate_player),
                    player_enemy_collision,
                    reload_gun,
                    update_timer,
                )
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

/*  old code
    .add_systems(Update,  player_movement )
    .add_systems(Update,  update_crosshair)
    .add_systems(Update,  rotate_player)
    .add_systems(Update,  player_enemy_collision)
    .add_systems(Update,  reload_gun);


*/
