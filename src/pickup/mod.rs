
use bevy::prelude::*;

mod systems;
use systems::*;

use crate::components::AppState;
pub struct PickUpPlugin;

impl Plugin for PickUpPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_pickup, pickup_health, check_player_health, despawn_pickup).run_if(in_state(AppState::InGame)));
    }
}


/* old code
            .add_systems(
Update, spawn_pickup)
            .add_systems(Update,pickup_health)
            .add_systems(Update,check_player_health)
            .add_systems(Update,despawn_pickup)

            */