use bevy::prelude::*;

pub mod arena;
pub mod bullet;
pub mod components;
pub mod enemy;
pub mod hud;
pub mod pickup;
pub mod player;
pub mod setup;
use components::{MainCamera, OnGameScreenMarker};

//import all the game plugins
use crate::game::{
    arena::ArenaPlugin, bullet::BulletPlugin, enemy::EnemyPlugin, hud::HudPlugin,
    pickup::PickUpPlugin, player::PlayerPlugin, setup::SetupPlugin,
};

use crate::components::AppState;
use crate::systems::despawn_screen;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ArenaPlugin)
            .add_plugins(BulletPlugin)
            .add_plugins(EnemyPlugin)
            .add_plugins(PickUpPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(SetupPlugin)
            .add_plugins(HudPlugin)
            .add_systems(OnEnter(AppState::InGame), setup)
            //when I exit the game screen, despawn all the entities otherwise they will be rendered on top of the menu screen or whatever screen I go to next
            .add_systems(
                OnExit(AppState::InGame),
                despawn_screen::<OnGameScreenMarker>,
            );
    }
}
fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera, OnGameScreenMarker));
}
