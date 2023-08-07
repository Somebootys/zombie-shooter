use bevy::prelude::*;

use crate::components::{GameScore, AppState, Wave};

use crate::game::components::{
    Bullet, ColliderSquare, DespawnedEnemies, Enemy, Health, Movable, BULLETSPEED,OnGameScreenMarker,  
};

use bevy::sprite::collide_aabb::collide;
use bevy_rapier2d::prelude::*;
use std::collections::HashSet;

const BULLET_RANGE: f32 = 1000.0;

pub fn update_bullets(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Movable, &mut Velocity), With<Bullet>>,
    time: Res<Time>,
) {
    for (entity, mut transform, movable, velocity) in query.iter_mut() {
        //update bullet position

        //update bullet position
        transform.translation.x += velocity.linvel.x * time.delta_seconds() * BULLETSPEED;
        transform.translation.y += velocity.linvel.y * time.delta_seconds() * BULLETSPEED;
        //println!("Bullet position: {:?}", velocity);
        //despawn bullet if it is out of range

        if movable.auto_despawn {
            if transform.translation.x > BULLET_RANGE
                || transform.translation.x < -BULLET_RANGE
                || transform.translation.y > BULLET_RANGE
                || transform.translation.y < -BULLET_RANGE
            {
                println!("Despawn bullet: {:?}", entity);
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

pub fn bullet_enemy_collision(
    query_bullet: Query<(Entity, &ColliderSquare, &Transform), With<Bullet>>,
    query_enemy: Query<(Entity, &ColliderSquare, &Transform), (With<Enemy>, Without<Bullet>)>,
    mut enemy: Query<(Entity, &mut Health), With<Enemy>>,
    mut killed_enemies: ResMut<DespawnedEnemies>,
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    mut score: ResMut<GameScore>,
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

                // reduce the health of the enemy
                for (entity, mut health) in enemy.iter_mut() {
                    if entity == enemy_entity {
                        health.hp -= 10;
                        println!("Enemy health: {:?}", health.hp);
                        if health.hp <= 0 {

                            //10 points for killing an enemy
                            score.0 += 10;


                            //spawn a blood splat instead of the enemy
                            cmd.spawn((SpriteBundle {
                                texture: asset_server.load("graphic/blood.png"),
                                transform: Transform::from_translation(Vec3::new(
                                    enemy_transform.translation.x,
                                    enemy_transform.translation.y,
                                    5.0,
                                )),
                                ..Default::default()
                            },
                            OnGameScreenMarker,));

                            //remove the enemy
                            cmd.entity(enemy_entity).despawn();
                            despawned_entities.insert(enemy_entity);
                            killed_enemies.entities.insert(enemy_entity);

                            
                        }
                    }
                }

                break;
            }
        }
    }
}

pub fn next_level(
    mut killed_enemies: ResMut<DespawnedEnemies>,
    mut state: ResMut<NextState<AppState>>,
    mut wave: ResMut<Wave>,
    
) {
    if killed_enemies.entities.len() == 10 {
        wave.0 += 1;
        state.set(AppState::LevelUp);
        //reset the despawned enemies
        killed_enemies.entities.clear();
    }
}
