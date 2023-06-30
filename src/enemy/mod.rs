use bevy::prelude::*;
//use bevy_rapier2d::prelude::*;
use crate::{
    components::{
        ColliderSquare, Enemy, EnemyCount, Health, Player, ENEMY_SPRITE_SIZE, MAX_NUM_ENEMIES,
    },
    player::systems::player_movement,
};
use rand::Rng;
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_enemy)
            .add_system(update_enemy.after(player_movement));
    }
}

pub fn spawn_enemy(mut commands: Commands, asset_server: Res<AssetServer>) {
    let off_set = 30.0;

    let mut rng = rand::thread_rng();
    let x = rng.gen_range(-640.0..640.0);
    let y = rng.gen_range(-360.0..360.0);

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(x, y, 10.0),
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

pub fn update_enemy(
    mut enemy_query: Query<(Entity, &mut Transform), (With<Enemy>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    // chase the player
    for (enemy_entity, mut enemy_transform) in enemy_query.iter_mut() {
        for player_transform in player_query.iter() {
            let mut direction = player_transform.translation - enemy_transform.translation;
            direction = direction.normalize();
            enemy_transform.translation += direction * 2.0;

            //let enemy face the player
            let angle = direction.y.atan2(direction.x);
            enemy_transform.rotation = Quat::from_rotation_z(angle);
        }
    }
}
