use bevy::prelude::*;


pub mod game;


pub mod arena;
pub mod bullet;
pub mod components;
pub mod enemy;
pub mod pickup;
pub mod player;
pub mod systems;

use crate::game::GamePlugin;

fn main() {
    App::new()
        .add_plugin(GamePlugin)
        .run();
}
