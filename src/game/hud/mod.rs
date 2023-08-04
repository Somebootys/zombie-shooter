use crate::components::AppState;

use bevy::prelude::*;
mod systems;
use systems::*;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), hud_setup)
            .add_systems(
                Update,
                (
                    hud_score_update_system,
                    hud_ammo_update_system,
                    hud_health_update_system,
                )
                    .run_if(in_state(AppState::InGame)),
            );
    }
}
