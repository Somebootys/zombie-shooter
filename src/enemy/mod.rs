use bevy::prelude::*;

//use bevy::sprite::collide_aabb::{collide, Collision};
mod systems;

use systems::*;

use super::systems::despawn_screen;
use crate::components::{AppState, Enemy};
use crate::player::systems::player_movement;
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_horde_of_enemies)
            .add_systems(
                Update,
                (update_enemy.after(player_movement)).run_if(in_state(AppState::InGame)),
            )
            .add_systems(OnExit(AppState::InGame), despawn_screen::<Enemy>);
    }
}

/*
(Startup, spawn_enemy)
.add_systems(Startup, spawn_horde_of_enemies)
.add_systems(Update, update_enemy.after(player_movement))
 //.add_system(check_collision_between_enemies)
;

 */
