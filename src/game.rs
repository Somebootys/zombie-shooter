use bevy::prelude::*;

use crate::arena::ArenaPlugin;
use crate::bullet::BulletPlugin;
use crate::components::{AppState, MainCamera, OnGameScreenMarker};
use crate::enemy::EnemyPlugin;
use crate::hud::HudPlugin;
use crate::pickup::PickUpPlugin;
use crate::player::PlayerPlugin;
use crate::systems::{despawn_screen, SetupPlugin};

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
            .add_systems(
                OnExit(AppState::InGame),
                despawn_screen::<OnGameScreenMarker>,
            );
    }
}
fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera, OnGameScreenMarker));
}
