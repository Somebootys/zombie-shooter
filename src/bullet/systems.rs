use bevy::prelude::*;

use crate::components::{
    Bullet, ColliderSquare, Enemy, Movable, Velocity, BULLET_SPRITE_DIMENSION,
};
use bevy::sprite::collide_aabb::collide;
use std::collections::HashSet;
//use bevy_rapier2d::prelude::*;

const BULLET_RANGE: f32 = 1000.0;

pub fn update_bullets(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Movable, &Velocity), With<Bullet>>,
    time: Res<Time>,
) {
    for (entity, mut transform, movable, velocity) in query.iter_mut() {
        //update bullet position

        //add collider to spawned bullet
        commands.entity(entity).insert(ColliderSquare {
            dimension: BULLET_SPRITE_DIMENSION,
        });

        //update bullet position
        transform.translation.x += velocity.velocity.x * time.delta_seconds();
        transform.translation.y += velocity.velocity.y * time.delta_seconds();

        //despawn bullet if it is out of range

        if movable.auto_despawn {
            if transform.translation.x > BULLET_RANGE
                || transform.translation.x < -BULLET_RANGE
                || transform.translation.y > BULLET_RANGE
                || transform.translation.y < -BULLET_RANGE
            {
                println!("Despawn bullet: {:?}", entity);
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn bullet_enemy_collision(
    query_bullet: Query<(Entity, &ColliderSquare, &Transform), With<Bullet>>,
    query_enemy: Query<(Entity, &ColliderSquare, &Transform), With<Enemy>>,

    mut cmd: Commands,
) {
    let mut despawned_entities: HashSet<Entity> = HashSet::new();
    for (bullet_entity, bullet_collider, bullet_transform) in query_bullet.iter() {
        if despawned_entities.contains(&bullet_entity) {
            continue;
        }
        for (enemy_entity, enemy_collider, enemy_transform) in query_enemy.iter() {
            if despawned_entities.contains(&enemy_entity)
                || despawned_entities.contains(&bullet_entity)
            {
                continue;
            }

            let collision = collide(
                bullet_transform.translation,
                bullet_collider.dimension,
                enemy_transform.translation,
                enemy_collider.dimension,
            );

            // perform the collision
            if collision.is_some() {
                // remove the enemy

                // remove the laser
                cmd.entity(bullet_entity).despawn();
                despawned_entities.insert(bullet_entity);

                break;
            }
        }
    }
}
