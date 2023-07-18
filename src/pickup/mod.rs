
use bevy::prelude::*;

mod systems;
use systems::*;


pub struct PickUpPlugin;

impl Plugin for PickUpPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_pickup)
            .add_system(pickup_health)
            .add_system(check_player_health)
            .add_system(despawn_pickup);
    }
}

