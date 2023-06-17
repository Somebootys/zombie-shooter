use bevy::prelude::*;
use crate::player::systems::{Player};





const SPRITE_SCALE: f32 = 0.5;



pub fn player_fire_system(mut cmd: Commands,
    kb: Res<Input<KeyCode>>,
    player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,

){
   
   if let Ok(plaer_tf) = player_query.get_single() {
    if kb.just_pressed(KeyCode::Space) {
        let (x, y) = (plaer_tf.translation.x, plaer_tf.translation.y);
        cmd.spawn(SpriteBundle {
            texture: asset_server.load("graphic/laser_a_01.png"),
            transform: Transform::from_xyz(x, y, 0.0),
            
            ..Default::default()
         });
        
    }

}
}



pub fn shoot_bullet(
    keyboard_input: Res<Input<KeyCode>>,

) {



    if keyboard_input.just_pressed(KeyCode::Left) {


        println!("Shoot bullet");
    }

}