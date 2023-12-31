use crate::components::AppState;

use crate::game::components::{
    AmmoCount, Bullet, ColliderSquare, CrossHair, Enemy, EquippedGun, Health, LastDamaged,
    MainCamera, Movable, OnGameScreenMarker, Player, PlayerDamagedTimer,
    BULLET_SPRITE_DIMENSION, PLAYER_SPRITE_SIZE, ReloadTimer, PlayerOrientation
};

use bevy::audio::PlaybackMode;
use bevy::prelude::*;
use bevy::window::Window;
use bevy_rapier2d::prelude::*;

use bevy::sprite::collide_aabb::collide;
use libm;
use std::f32::consts::PI;


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
            RigidBody::Dynamic,
            Velocity {
                linvel: Vec2::new(0.0, 0.0),
                angvel: 0.0,
            },
            OnGameScreenMarker,
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
    mut state: ResMut<NextState<AppState>>,
) {
    for (player, mut rb_vels) in &mut player_info {
        let up = keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]);
        let down = keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]);
        let left = keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]);
        let right = keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]);

        let debug: bool = keyboard_input.any_pressed([KeyCode::Y]);

        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        let mut move_delta = Vec2::new(x_axis as f32, y_axis as f32);
        if move_delta != Vec2::ZERO {
            move_delta /= move_delta.length();
        }

        if debug {
            state.set(AppState::LevelUp);
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
        OnGameScreenMarker,
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

    if let Ok((camera, camera_transform)) = camera_q.get_single() {
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
}

pub fn fire_gun(mut player_query: Query<&mut Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    buttons: Res<Input<MouseButton>>,
    mut cmd: Commands,
    mut eq_gun: ResMut<EquippedGun>,
     player_orientation: ResMut<PlayerOrientation>,
     reloading: Res<ReloadTimer>,
    
){
    //TODO: review if this is the best way to handle the offset
        
        
    if let Ok(transform) = player_query.get_single_mut() {
        

        let angle = player_orientation.angle.clone();
       

    if !reloading.reloading { 
    if buttons.just_released(MouseButton::Left) && eq_gun.0.magazine.current <= 0  {

        println!("Gun is empty");  

        cmd.spawn((
           AudioBundle {
               source: asset_server.load("audio/gun/9mm-Pistol-Dry-Fire.ogg"),
               settings : PlaybackSettings {
                   mode: PlaybackMode::Despawn,
                   ..default()
               },
               ..default()
           },
        )); 
       } else if buttons.just_released(MouseButton::Left) && eq_gun.0.magazine.current  > 0  {

           eq_gun.0.magazine.current -= 1;
           
       
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
               
               Bullet { angle: player_orientation.angle },
               Movable { auto_despawn: true },
               Velocity {
                   linvel: player_orientation.linvel,
                   angvel: player_orientation.angle,
               },
           ))

           
           //add collider to spawned bullet
           .insert(ColliderSquare {
               dimension: BULLET_SPRITE_DIMENSION,
           });
           //spawn bullet sound separately so it does not get despawned with the bullet. Hoping this will fix laggy sound
               cmd.spawn(AudioBundle {
                source: asset_server.load("audio/gun/9mm-Single.ogg"),
               settings:  PlaybackSettings {
                 mode: PlaybackMode::Despawn,
                 ..default()
             },

                ..default()
            },);

              
           }
        }}
       
           
}

pub fn rotate_player(
    mut player_query: Query<&mut Transform, With<Player>>,
   

    // need to get window dimensions
    mut windows: Query<&mut Window>,
    // query to get camera transform
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut player_orientation: ResMut<PlayerOrientation>,

    
   
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK. However, adding more states, I need get_single, and I need to handle potential 'no camera'-cases
    if let Ok((camera, camera_transform)) = camera_q.get_single() {
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

                player_orientation.angle = angle;
                player_orientation.linvel = diff.normalize();


                //Rotate the sprite around the z axis by the angle. transform.rotation takes a Quat, also needs to be in radians, so no need to convert the angle.
                transform.rotation = Quat::from_rotation_z(angle);

                
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
    asset_server: Res<AssetServer>,
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
                        
                        cmd.spawn((SpriteBundle {
                            texture: asset_server.load("graphic/blood.png"),
                            transform: Transform::from_translation(Vec3::new(
                                player_transform.translation.x,
                                player_transform.translation.y,
                                5.0,
                            )),
                            ..Default::default()
                
                        },
                            AudioBundle {
                                source: asset_server.load("audio/other/death_voice.ogg"),
                                settings: PlaybackSettings {
                                    // I want my splat entity to remain after the sound has finished playing, but I don't need to sound entity/component itself to remain
                                    mode: PlaybackMode::Remove,
                                    ..default()
                                },
                                ..default()
                            },
                            OnGameScreenMarker,
                        ));
                        cmd.entity(player_entity).despawn_recursive();
                    }

                    player_hit.time.reset();
                }
            }
        }
    }
}

pub fn update_timer(
    time: Res<Time>,
    mut reload: ResMut<ReloadTimer>,
    
    
) {
    // assume we have exactly one player that jumps with Spacebar
  
    if reload.reloading {
        // Check if the timer has finished
        if reload.timer.tick(time.delta()).just_finished() {
            reload.reloading = false; // Allow pressing "R" again after the delay
        }
    }
   
}

pub fn reload_gun(
    mut ammo_inventory: ResMut<AmmoCount>,
    mut eq_gun: ResMut<EquippedGun>,
    kb_input: Res<Input<KeyCode>>,
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    mut reloader: ResMut<ReloadTimer>,
    time : Res<Time>,
    
    
) {

    //println!("timer: {:?}", reload_timer.timer);
   
       
     
        //check if the gun is already full
        let diff = (eq_gun.0.magazine.capacity - eq_gun.0.magazine.current) as i32;
       //handle crash if ammo vector does not exist cannot use unwrap
   //get the r key input
    if kb_input.just_pressed(KeyCode::R)  {  
        
      
        
       

        

        if diff == 0 {
            println!("Gun is already full");
        }
        else if ammo_inventory.pistol == 0 {
            println!("No ammo left");
        }
        else if reloader.reloading == false && ammo_inventory.pistol > 0 && diff > 0  {
            
            reloader.timer.tick(time.delta());

            //make sure the replay sound has finished before you can shoot or reload

            cmd.spawn((
                AudioBundle {
                    source: asset_server.load("audio/gun/9mm-Pistol-Reload.ogg"),
                    ..default()
                },
            ));



            let x = (ammo_inventory.pistol as i32 - diff ) as i32;

                if x < 0 {
                    eq_gun.0.magazine.current += ammo_inventory.pistol;
                    ammo_inventory.pistol = 0;
                }
                else {
                    eq_gun.0.magazine.current = eq_gun.0.magazine.capacity;
                    ammo_inventory.pistol -= diff as usize;
                }
                
                reloader.reloading = true;
                reloader.timer.reset();
            
            }

           

        }
       
}
    

    //TODO fix the zero in 233 also need to add reload time

