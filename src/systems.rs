use crate::components::{ColliderSquare, Enemy, MainCamera, Player};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_plugins(DefaultPlugins)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(WorldInspectorPlugin::new())
            .add_system(handle_player_enemy_collision)
            //.add_system(handle_bullet_enemy_collision)
            ;
    }
}

pub fn setup(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

    //turn of gravity force, since this is a top down game
    rapier_config.gravity = Vec2::new(0.0, 0.0_f32);
}

pub fn handle_player_enemy_collision(
    query_player: Query<(Entity, &ColliderSquare, &Transform), With<Player>>,
    query_enemy: Query<(Entity, &ColliderSquare, &Transform), With<Enemy>>,
) {
    for (_player_entity, player_collider, player_transform) in query_player.iter() {
        for (_enemy_entity, enemy_collider, enemy_transform) in query_enemy.iter() {
            let collision = collide(
                player_transform.translation,
                player_collider.dimension,
                enemy_transform.translation,
                enemy_collider.dimension,
            );
            if let Some(collision) = collision {
                println!("Collision detected: {:?}", collision);
            }
        }
    }
}
