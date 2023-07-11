use crate::components::{ColliderSquare, GameTextures, Health, PickUp, Player};
use bevy::prelude::*;

pub struct PickUpPlugin;

impl Plugin for PickUpPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_pickup)
            .add_system(pickup_health)
            .add_system(check_player_health);
    }
}

pub fn spawn_pickup(mut cmd: Commands, game_textures: ResMut<GameTextures>) {
    cmd.spawn((
        SpriteBundle {
            //spawn in top left corner
            transform: Transform::from_xyz(100.0, 100.0, 0.0),
            texture: game_textures.pickup_health.clone(),
            ..default()
        },
        PickUp {},
        ColliderSquare {
            dimension: Vec2::new(32.0, 32.0),
        },
    ));

    println!("spawned pickup");
}

pub fn pickup_health(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform), With<PickUp>>,
    mut player_query: Query<(&mut Health, &Transform), With<Player>>,
) {
    for (entity, transform_pickup) in query.iter_mut() {
        for (mut player, transform_player) in player_query.iter_mut() {
            let relative_position = transform_pickup
                .translation
                .distance(transform_player.translation);

            if relative_position < 32.0 {
                player.hp += 10;
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn check_player_health(query: Query<&Health, With<Player>>, kb_input: Res<Input<KeyCode>>) {
    if kb_input.just_pressed(KeyCode::O) {
        for player in query.iter() {
            println!("player health: {:?}", &player);
        }
    }
}
