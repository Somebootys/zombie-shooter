use crate::components::AppState;

use bevy::prelude::*;
mod systems;
use systems::*;

//does this work?
use super::systems::despawn_screen;

pub struct LevelUpPlugin;

impl Plugin for LevelUpPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::LevelUp), setup)
            .add_systems(
                Update,
                (
                    button_system,
                    menu_action,
                    update_stats,
                    continue_to_next_level,
                )
                    .run_if(in_state(AppState::LevelUp)),
            )
            .add_systems(
                OnExit(AppState::LevelUp),
                despawn_screen::<OnLevelUpScreenMarker>,
            ); //make despawn screen and then go to game state
    }
}
