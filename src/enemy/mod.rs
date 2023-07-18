use bevy::prelude::*;



//use bevy::sprite::collide_aabb::{collide, Collision};
mod systems;
use systems::*;

use crate::player::systems::player_movement;


pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_enemy)
            .add_startup_system(spawn_horde_of_enemies)
            .add_system(update_enemy.after(player_movement))
             //.add_system(check_collision_between_enemies)
            ;
    }
}

