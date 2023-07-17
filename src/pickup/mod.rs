use crate::components::{
    Ammo, ColliderSquare, EquippedGun, GameTextures, GunType, Health, PickUp, PickUpDuration,
    PickUpTimer, Player, ARENA_SIZE, PICKUP_SPRITE_SIZE, PICK_UP_DURATION, TILE_SIZE,
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
    mut pickup_timer: ResMut<PickUpTimer>,
) {
    let width = ARENA_SIZE / 2.0;
    let height = ARENA_SIZE / 2.0;
    if pickup_timer.time.tick(time.delta()).just_finished() {
        let mut rng = rand::thread_rng();

        // possible spawn locations are the entire screen
        let x = rng.gen_range(-width + TILE_SIZE..width - TILE_SIZE);
        let y = rng.gen_range(-height + TILE_SIZE..height - TILE_SIZE);
        let pickup_type = rng.gen_range(0..2);

        let tex = match pickup_type {
            0 => game_textures.pickup_health.clone(),
            1 => game_textures.pickup_ammo.clone(),
            _ => game_textures.pickup_health.clone(),
        };

        cmd.spawn((
            SpriteBundle {
                //spawn in top left corner
                transform: Transform::from_xyz(x, y, 0.0),
                texture: tex,
                ..default()
            },
            PickUp {
                ammo: pickup_type == 1,
                health: pickup_type == 0,
            },
            ColliderSquare {
                dimension: Vec2::new(32.0, 32.0),
            },
            PickUpDuration {
                time: Stopwatch::new(),
            },
        ));

        println!("spawned pickup");

        pickup_timer.time.reset();
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
    mut player_query: Query<(&mut Health, &Transform, &mut Ammo), With<Player>>,
    pickup_query: Query<&PickUp>,
    gun: ResMut<EquippedGun>,
) {
    let gun_data = GunType {
        gun_type: gun.gun_type.clone(),
    };

    let x = gun_data.gun_type.clone();

    for (entity, transform_pickup) in query.iter_mut() {
        for (mut player, transform_player, mut ammo) in player_query.iter_mut() {
            let pickup = pickup_query.get(entity).unwrap();
            let relative_position = transform_pickup
                .translation
                .distance(transform_player.translation);

            match pickup.health {
                true => {
                    if relative_position < PICKUP_SPRITE_SIZE / 2.0_f32 {
                        player.hp += 10;
                        commands.entity(entity).despawn();
                    }
                }
                false => {
                    if relative_position < PICKUP_SPRITE_SIZE / 2.0_f32 {
                        ammo.vec[x.clone() as usize] += gun_data.magasine_size().clone() as i32;
                        commands.entity(entity).despawn();
                    }
                }
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
