use bevy::prelude::*;


use crate::arena::ArenaPlugin;
use crate::bullet::BulletPlugin;
use crate::enemy::EnemyPlugin;
use crate::pickup::PickUpPlugin;
use crate::player::PlayerPlugin;
use crate::systems::SetupPlugin;


pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ArenaPlugin)
            .add_plugin(BulletPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(PickUpPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(SetupPlugin);
    }
}