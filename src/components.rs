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

#[derive(Component)]
pub struct ColliderSquare {
    pub dimension: Vec2,
}

#[derive(Component, Debug)]
pub struct Health {
    pub health: i32,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct TileIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Resource, Debug)]
pub struct WinSize {
    pub width: f32,
    pub height: f32,
}

#[derive(Resource, Debug)]
pub struct ArenaSize {
    pub width: f32,
    pub height: f32,
}

pub const PLAYER_SPRITE_SIZE: f32 = 50.0;
pub const ENEMY_SPRITE_SIZE: f32 = 75.0;
pub const BULLETSPEED: f32 = 1000.0;
pub const BULLET_SPRITE_DIMENSION: Vec2 = Vec2::new(90.0, 54.0);
