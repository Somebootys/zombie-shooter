use crate::components::{
    ColliderSquare, GameTextures, Health, PickUp, PickUpDuration, PickUpTimer, Player, WinSize,
    PICK_UP_DURATION,
};
use bevy::prelude::*;
use bevy::time::Stopwatch;
use rand::Rng;

pub struct PickUpPlugin;

impl Plugin for PickUpPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_pickup)
            .add_system(pickup_health)
            .add_system(check_player_health)
            .add_system(despawn_pickup);
    }
}

pub fn spawn_pickup(
    mut cmd: Commands,
    game_textures: ResMut<GameTextures>,
    time: Res<Time>,
    mut puckup_timer: ResMut<PickUpTimer>,
    win_size: Res<WinSize>,
) {
    let width_positive_bounds = win_size.width / 2.0;
    let width_negative_bounds = -win_size.width / 2.0;
    let height_positive_bounds = win_size.height / 2.0;
    let height_negative_bounds = -win_size.height / 2.0;

    if puckup_timer.time.tick(time.delta()).just_finished() {
        let mut rng = rand::thread_rng();

        // possible spawn locations are the entire screen
        let x = rng.gen_range(width_negative_bounds..width_positive_bounds);
        let y = rng.gen_range(height_negative_bounds..height_positive_bounds);

        cmd.spawn((
            SpriteBundle {
                //spawn in top left corner
                transform: Transform::from_xyz(x, y, 0.0),
                texture: game_textures.pickup_health.clone(),
                ..default()
            },
            PickUp {},
            ColliderSquare {
                dimension: Vec2::new(32.0, 32.0),
            },
            PickUpDuration {
                time: Stopwatch::new(),
            },
        ));

        println!("spawned pickup");

        puckup_timer.time.reset();
    }
}

pub fn despawn_pickup(
    mut cmd: Commands,
    mut query: Query<(Entity, &mut PickUpDuration), With<PickUp>>,
    time: Res<Time>,
) {
    // despawn pickups after a certain amount of time
    for (entity, mut pickup_duration) in query.iter_mut() {
        pickup_duration.time.tick(time.delta());

        if pickup_duration.time.elapsed().as_secs() > PICK_UP_DURATION {
            cmd.entity(entity).despawn();
        }
    }
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
