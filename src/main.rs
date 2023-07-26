use bevy::prelude::*;


pub mod game;

pub mod game_menu;
pub mod arena;
pub mod bullet;
pub mod components;
pub mod enemy;
pub mod pickup;
pub mod player;
pub mod systems;
pub mod hud;

use crate::game::GamePlugin;
use crate::game_menu::GameMenuPlugin;
use crate::components::{AppState, EnemyCount};
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(EnemyCount(0))
        .add_state::<AppState>()
        //.add_systems(Startup, setup)
        // This system runs when we enter `AppState::Menu`, during the `StateTransition` schedule.
        // All systems from the exit schedule of the state we're leaving are run first,
        // and then all systems from the enter schedule of the state we're entering are run second.
        .add_plugins(GameMenuPlugin)
        .add_plugins(GamePlugin)
        
        .run();
}






