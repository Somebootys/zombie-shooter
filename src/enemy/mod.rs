use bevy::prelude::*;

use crate::components::{Enemy, ENEMY_SPRITE_SIZE};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_enemy);
    }
}

pub fn spawn_enemy(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(ENEMY_SPRITE_SIZE, ENEMY_SPRITE_SIZE, 0.0),
            texture: asset_server.load("graphic/boomer.png"),
            ..default()
        },
        Enemy {},
    ));
}
