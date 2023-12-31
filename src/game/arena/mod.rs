use bevy::prelude::*;

pub mod systems;
use systems::*;

use crate::components::AppState;

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup);
    }
}
