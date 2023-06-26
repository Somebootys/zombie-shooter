use bevy::prelude::*;

/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Velocity {
    pub velocity: Vec2,
}

#[derive(Component)]
pub struct Bullet {
    pub angle: f32,
}

#[derive(Component)]
pub struct Movable {
    pub auto_despawn: bool,
}

#[derive(Component)]
pub struct Enemy;

pub const PLAYER_SPRITE_SIZE: f32 = 50.0;
pub const ENEMY_SPRITE_SIZE: f32 = 75.0;
pub const BULLETSPEED: f32 = 1000.0;
