use crate::components::{
    Ammo, Bullet, ColliderSquare, CrossHair, Enemy, EquippedGun, GunType, Health, LastDamaged,
    MainCamera, Movable, Player, PlayerDamagedTimer, ReloadTimer, PLAYER_SPRITE_SIZE, BULLET_SPRITE_DIMENSION
};
use bevy::prelude::*;
use bevy::window::Window;
use bevy_rapier2d::prelude::*;
//use bevy_rapier2d::prelude::*;
use bevy::sprite::collide_aabb::collide;
use libm;
use std::f32::consts::PI;
use std::time::Duration;
//use bevy::utils::Duration;

const PLAYER_SPEED: f32 = 500.0;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            ColliderSquare {
                dimension: Vec2::new(PLAYER_SPRITE_SIZE + 5.0, PLAYER_SPRITE_SIZE + 5.0),
            },
            SpriteBundle {
                transform: Transform::from_xyz(0.0, 0.0, 10.0),
                texture: asset_server.load("graphic/player.png"),
                ..default()
            },
            Player(PLAYER_SPEED),
            PlayerDamagedTimer(Timer::from_seconds(10.0, TimerMode::Once)),
            Health { hp: 50 },
            Ammo { vec: [8, 0, 0] },
            RigidBody::Dynamic,
            Velocity {
                linvel: Vec2::new(0.0, 0.0),
                angvel: 0.0,
            },
            Collider::cuboid(PLAYER_SPRITE_SIZE / 2.0, PLAYER_SPRITE_SIZE / 2.0),
        ))
        //.insert(LockedAxes::TRANSLATION_LOCKED)
        .insert(ActiveCollisionTypes::default() | ActiveCollisionTypes::DYNAMIC_STATIC)
        .insert(Damping {
            linear_damping: 15.0,
            angular_damping: 10.0,
        });
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,

    mut player_info: Query<(&Player, &mut Velocity)>,
) {
    for (player, mut rb_vels) in &mut player_info {
        let up = keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]);
        let down = keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]);
        let left = keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]);
        let right = keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]);

        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        let mut move_delta = Vec2::new(x_axis as f32, y_axis as f32);
        if move_delta != Vec2::ZERO {
            move_delta /= move_delta.length();
        }

        // Update the velocity on the rigid_body_component,
        // the bevy_rapier plugin will update the Sprite transform.
        rb_vels.linvel = move_delta * player.0;
    }
}

pub fn spawn_crosshair(mut cmd: Commands, asset_server: Res<AssetServer>) {
    cmd.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 15.0),
            texture: asset_server.load("graphic/crosshair.png"),
            ..default()
        },
        CrossHair,
    ));
}
pub fn update_crosshair(
    // need to get window dimensions
    mut windows: Query<&mut Window>,
    // query to get camera transform
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut crosshair_query: Query<&mut Transform, With<CrossHair>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = camera_q.single();
    // get the window that the camera is displaying to (or the primary window)
    let window = windows.single_mut();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        for mut transform in crosshair_query.iter_mut() {
            transform.translation = Vec3::new(world_position.x, world_position.y, 15.0);
        }
    }
}

pub fn rotate_player(
    mut player_query: Query<&mut Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    buttons: Res<Input<MouseButton>>,
    mut cmd: Commands,

    // need to get window dimensions
    mut windows: Query<&mut Window>,
    // query to get camera transform
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,

    mut eq_gun: ResMut<EquippedGun>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = camera_q.single();

    // get the window that the camera is displaying to (or the primary window)
    let window = windows.single_mut();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        // face the player towards the cursor
        if let Ok(mut transform) = player_query.get_single_mut() {
            let (x, y) = (transform.translation.x, transform.translation.y);

            // get the angle between the player and the cursor in radians. atan2f is the inverse tangent function y/x, and returns radians.
            let angle = libm::atan2f(world_position.y - y, world_position.x - x);
            let diff = world_position - Vec2::new(x, y);

            //Rotate the sprite around the z axis by the angle. transform.rotation takes a Quat, also needs to be in radians, so no need to convert the angle.
            transform.rotation = Quat::from_rotation_z(angle);

            //TODO: review if this is the best way to handle the offset

            if buttons.just_released(MouseButton::Left) && eq_gun.bullets_in_magasine > 0 {
                let c = PLAYER_SPRITE_SIZE.clone();
                let a = libm::cosf(angle) * c;
                let b: f32 = libm::sinf(angle) * c;

                let offset_laser_x = a;
                let offset_laser_y = b;
                let mut spawn_transform = Transform::from_scale(Vec3::splat(1.0));
                spawn_transform.translation =
                    transform.translation + Vec3::new(offset_laser_x, offset_laser_y, 5.0);
                spawn_transform.rotation =
                    Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle - PI / 2.0_f32);

                cmd.spawn((
                    SpriteBundle {
                        transform: spawn_transform,
                        texture: asset_server.load("graphic/laser_a_01.png"),
                        ..default()
                    },
                    Bullet { angle: angle },
                    Movable { auto_despawn: true },
                    Velocity {
                        linvel: diff.normalize(),
                        angvel: angle,
                    },
                ))
                 //add collider to spawned bullet
                .insert(ColliderSquare {
                     dimension: BULLET_SPRITE_DIMENSION,
        });
                if eq_gun.bullets_in_magasine == 0 {
                    println!("Out of bullets!!");
                } else {
                    eq_gun.bullets_in_magasine -= 1;
                }
            }
        }
    }
}

pub fn player_enemy_collision(
    mut player: Query<(Entity, &ColliderSquare, &Transform, &mut Health), With<Player>>,
    query_enemy: Query<(&ColliderSquare, &Transform), With<Enemy>>,
    mut cmd: Commands,
    time: Res<Time>,
    mut player_hit: ResMut<LastDamaged>,
) {
    for (player_entity, player_collider, player_transform, mut player) in &mut player.iter_mut() {
        for (enemy_collider, enemy_transform) in &mut query_enemy.iter() {
            let collision = collide(
                player_transform.translation,
                player_collider.dimension,
                enemy_transform.translation,
                enemy_collider.dimension,
            );

            if let Some(_collision) = collision {
                if player_hit.time.tick(time.delta()).just_finished() {
                    // damage the player
                    println!("player hit ");
                    player.hp -= 10;

                    if player.hp <= 0 {
                        cmd.entity(player_entity).despawn_recursive();
                    }

                    player_hit.time.reset();
                }
            }
        }
    }
}

pub fn reload_gun(
    mut player_query: Query<&mut Ammo, With<Player>>,
    mut eq_gun: ResMut<EquippedGun>,
    kb_input: Res<Input<KeyCode>>,
    mut reload_timer: ResMut<ReloadTimer>,
) {
    let gun_data = GunType {
        gun_type: eq_gun.gun_type.clone(),
    };

    let gun_magasine_capacity = gun_data.magasine_size();
    let gun_type = gun_data.gun_type.clone() as usize;
    let gun_reload_time = gun_data.reload_time();

    //handle crash if ammo vector does not exist cannot use unwrap
    if let Ok(mut player_ammo) = player_query.get_single_mut() {
        if kb_input.just_released(KeyCode::R) {
            reload_timer.time.tick(Duration::from_secs_f32(1.5));

            if player_ammo.vec[gun_type] <= 0 {
                println!("Out of bullets!!");
                reload_timer.time.reset();
            } else if eq_gun.bullets_in_magasine < gun_magasine_capacity {
                println!("Reloading...");
                // start timer, reloading take x amount of time

                if reload_timer.time.elapsed().as_secs_f32() >= gun_reload_time.as_secs_f32() {
                    // subtact bullets in the magasin from total capacity of magezine, then add that to the magasine
                    player_ammo.vec[gun_type] -=
                        (gun_magasine_capacity - eq_gun.bullets_in_magasine) as i32;
                    eq_gun.bullets_in_magasine = gun_magasine_capacity;
                    reload_timer.time.reset();
                } else {
                    println!("Reloading...");
                }
            }
        }
    }

    //TODO fix the zero in 233 also need to add reload time
}
