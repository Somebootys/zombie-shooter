
use bevy::prelude::*;

pub mod systems;
pub mod player;
pub mod bullet;
pub mod components;
pub mod enemy;
use crate::systems::{SetupPlugin};
use crate::player::PlayerPlugin;
use crate::bullet::BulletPlugin;
use crate::enemy::EnemyPlugin;

fn main() {
    App::new()
        .add_plugin(SetupPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(BulletPlugin)
        .run();
}



