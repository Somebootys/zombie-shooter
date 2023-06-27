use bevy::prelude::*;
//use bevy_rapier2d::prelude::*;
use crate::components::{ColliderSquare, Enemy, Health, ENEMY_SPRITE_SIZE};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_enemy);
    }
}

pub fn spawn_enemy(mut commands: Commands, asset_server: Res<AssetServer>) {
    let off_set = 30.0;
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(ENEMY_SPRITE_SIZE, ENEMY_SPRITE_SIZE, 0.0),
            texture: asset_server.load("graphic/boomer.png"),
            ..default()
        },
        Enemy {},
        ColliderSquare {
            dimension: Vec2::new(
                ENEMY_SPRITE_SIZE / 2.0 - off_set,
                ENEMY_SPRITE_SIZE / 2.0 - off_set,
            ),
        },
        Health { health: 100 },
    ));
}
