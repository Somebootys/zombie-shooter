use bevy::prelude::*;


use crate::arena::ArenaPlugin;
use crate::bullet::BulletPlugin;
use crate::enemy::EnemyPlugin;
use crate::pickup::PickUpPlugin;
use crate::player::PlayerPlugin;
use crate::systems::SetupPlugin;
use crate::hud::HudPlugin;


pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ArenaPlugin)
            .add_plugins(BulletPlugin)
            .add_plugins(EnemyPlugin)
            .add_plugins(PickUpPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(SetupPlugin)
            .add_plugins(HudPlugin);
    }
}