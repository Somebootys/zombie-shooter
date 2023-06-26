use bevy::prelude::*;

pub mod bullet;
pub mod components;
pub mod enemy;
pub mod player;
pub mod systems;
use crate::bullet::BulletPlugin;
use crate::enemy::EnemyPlugin;
use crate::player::PlayerPlugin;
use crate::systems::SetupPlugin;

fn main() {
    App::new()
        .add_plugin(SetupPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(BulletPlugin)
        .run();
}
