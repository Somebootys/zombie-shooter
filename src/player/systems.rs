use bevy::prelude::*;
use bevy::window::Window;

use crate::components::{
    Bullet, ColliderSquare, MainCamera, Movable, Player, Velocity, BULLETSPEED, PLAYER_SPRITE_SIZE,
};
//use bevy_rapier2d::prelude::*;
use libm;
use std::f32::consts::PI;

const PLAYER_SPEED: f32 = 500.0;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        ColliderSquare {
            dimension: Vec2::new(PLAYER_SPRITE_SIZE, PLAYER_SPRITE_SIZE),
        },
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            texture: asset_server.load("graphic/player.png"),
            ..default()
        },
        Player {},
    ));
}

/// This system prints out all mouse events as they come in
pub fn print_mouse_events_system(
    mut cmd: Commands,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,

    // need to get window dimensions
    mut windows: Query<&mut Window>,
    // query to get camera transform
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    asset_server: Res<AssetServer>,
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

            if buttons.just_released(MouseButton::Left) {
                let c = PLAYER_SPRITE_SIZE.clone();
                let a = libm::cosf(angle) * c;
                let b: f32 = libm::sinf(angle) * c;

                let offset_laser_x = a;
                let offset_laser_y = b;
                let mut spawn_transform = Transform::from_scale(Vec3::splat(1.0));
                spawn_transform.translation =
                    transform.translation + Vec3::new(offset_laser_x, offset_laser_y, 0.0);
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
                        velocity: diff.normalize() * BULLETSPEED,
                    },
                ));
            }

            let mut direction = Vec3::ZERO;
            if keyboard_input.pressed(KeyCode::A) {
                direction += Vec3::new(-1.0, 0.0, 0.0);
            }
            if keyboard_input.pressed(KeyCode::D) {
                direction += Vec3::new(1.0, 0.0, 0.0);
            }
            if keyboard_input.pressed(KeyCode::W) {
                direction += Vec3::new(0.0, 1.0, 0.0);
            }
            if keyboard_input.pressed(KeyCode::S) {
                direction += Vec3::new(0.0, -1.0, 0.0);
            }

            if direction.length() > 0.0 {
                direction = direction.normalize();
            }

            //let camera_transform = camera_query.single().unwrap();

            transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
        }
    }
}
