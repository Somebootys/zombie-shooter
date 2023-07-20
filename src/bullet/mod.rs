use bevy::prelude::*;
mod systems;
use systems::*;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_bullets)
            .add_systems(Update, bullet_enemy_collision);
    }
}
