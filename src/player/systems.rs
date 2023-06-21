use bevy::prelude::*;
use bevy::window::Window;
use bevy::window::PrimaryWindow;
use libm;

use crate::systems::MainCamera;



/// This system prints out all mouse events as they come in
#[derive(Component)]
pub struct Player {}

const PLAYER_SPEED: f32 = 500.0;




pub fn spawn_player(    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
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
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    
     // need to get window dimensions
     mut windows: Query<&mut Window>,
     // query to get camera transform
     camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,


    
) {



    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = camera_q.single();

    // get the window that the camera is displaying to (or the primary window)
    let window = windows.single_mut();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        // face the player towards the cursor
        if let Ok(mut transform) = player_query.get_single_mut() {

            // get the angle between the player and the cursor in radians. atan2f is the inverse tangent function y/x, and returns radians.
            let angle = libm::atan2f(world_position.y - transform.translation.y, world_position.x - transform.translation.x);
            

            //Rotate the sprite around the z axis by the angle. transform.rotation takes a Quat, also needs to be in radians, so no need to convert the angle.
           
            transform.rotation = Quat::from_rotation_z(angle);


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