use bevy::prelude::*;

use crate::components::{ Movable, Bullet, Velocity,};
//use bevy_rapier2d::prelude::*;



const BULLET_RANGE: f32 = 1000.0;




pub fn update_bullets(
    mut commands: Commands,
	mut query: Query<(Entity, &mut Transform, &Movable, &Velocity), With<Bullet>>,
    time : Res<Time>,   


){
   for (entity, mut transform, movable, velocity) in query.iter_mut() {
        
         
         //update bullet position


        
        transform.translation.x += velocity.velocity.x*time.delta_seconds();
        transform.translation.y += velocity.velocity.y*time.delta_seconds();

        if movable.auto_despawn {
            if transform.translation.x > BULLET_RANGE || transform.translation.x < -BULLET_RANGE || transform.translation.y > BULLET_RANGE || transform.translation.y < -BULLET_RANGE {
                println!("Despawn bullet: {:?}", entity);
                commands.entity(entity).despawn();
            }
        }
    }

}




pub fn bullet_enemy_collision(
    keyboard_input: Res<Input<KeyCode>>,

) {



    if keyboard_input.just_pressed(KeyCode::Left) {


        println!("Shoot bullet");
    }

}