
use bevy::prelude::*;

mod systems;
use systems::*;


pub struct PickUpPlugin;

impl Plugin for PickUpPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_pickup)
            .add_systems(Update,pickup_health)
            .add_systems(Update,check_player_health)
            .add_systems(Update,despawn_pickup);
    }
}

