use bevy::prelude::*;
mod systems;
use crate::components::AppState;
use systems::*;
pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_bullets, bullet_enemy_collision, next_level).run_if(in_state(AppState::InGame)),
        );
    }
}
