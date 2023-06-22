use bevy::prelude::*;

pub mod systems;
pub mod player;
pub mod bullet;
pub mod components;
use crate::systems::{SetupPlugin};
use crate::player::PlayerPlugin;
use crate::bullet::BulletPlugin;

fn main() {
    App::new()
        .add_plugin(SetupPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(BulletPlugin)
        .run();
}



