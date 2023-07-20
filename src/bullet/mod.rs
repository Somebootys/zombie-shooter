use bevy::prelude::*;
mod systems;
use systems::*;
use crate::components::AppState;
pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, 
            
            (update_bullets, bullet_enemy_collision).run_if(in_state(AppState::InGame)))
            ;
    }
}
