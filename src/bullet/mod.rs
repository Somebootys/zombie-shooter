use bevy::prelude::*;
mod systems;
use systems::*;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_bullets)
            .add_system(bullet_enemy_collision);
    }
}
