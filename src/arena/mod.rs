use bevy::prelude::*;

mod systems;
use systems::setup;

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
