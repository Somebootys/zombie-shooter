use bevy::prelude::*;
use bevy::ecs::system::SystemParam;
pub mod components;
pub mod game_menu;
pub mod levelup;
pub mod systems;

//use the game plugin to add all the other plugins
//pub mod game;
pub mod game;
use crate::components::{AppState, EnemyCount};
use crate::game::GamePlugin;
use crate::game_menu::GameMenuPlugin;
use crate::levelup::LevelUpPlugin;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(EnemyCount(0))
        .add_state::<AppState>()
        // This system runs when we enter `AppState::Menu`, during the `StateTransition` schedule.
        // All systems from the exit schedule of the state we're leaving are run first,
        // and then all systems from the enter schedule of the state we're entering are run second.
        .add_plugins(GameMenuPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(LevelUpPlugin)
        .run();
}
