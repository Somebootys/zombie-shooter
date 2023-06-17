use bevy::prelude::*;
mod systems;
use systems::*;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
       app.add_system(shoot_bullet)
       .add_system(player_fire_system);
    }
}