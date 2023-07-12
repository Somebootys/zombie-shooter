use bevy::prelude::*;
use std::collections::HashSet;

//------------------------------------------------------------------------Components


/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Player(pub f32);

#[derive(Component)]
pub struct PlayerDamagedTimer(pub Timer);

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
    pub hp: i32,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct TileIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component)]
pub struct Alive(pub bool);

#[derive(Component)]
pub struct CrossHair;

#[derive(Component)]
pub struct PickUp;


// --------------------------------------------------------------------------------Resources


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

#[derive(Resource, Debug)]
pub struct PlayerAngle(pub f32);



#[derive(Resource)]
pub struct EnemyCount(pub usize);

#[derive(Resource)]
pub struct DespawnedEnemies {
    pub entities: HashSet<Entity>,
}

#[derive(Resource)]
pub struct GameTextures {
    pub player: Handle<Image>,
    pub projectile: Handle<Image>,
    pub enemy_boomer: Handle<Image>,
    pub enemy_crawler: Handle<Image>,
    pub enemy_zoomer: Handle<Image>,
    pub pickup_health: Handle<Image>,
}

#[derive(Resource, Debug)]
pub struct EnemiesAlive {
    pub hashset: HashSet<Entity>,
}

#[derive(Resource, Debug)]
pub struct LastDamaged {
    pub time: Timer,
}

// --------------------------------------------------------------------------------Constants

pub const PLAYER_SPRITE_SIZE: f32 = 50.0;
pub const ENEMY_BOOMER_SPRITE_SIZE: f32 = 75.0;
pub const ENEMY_CRAWLER_SPRITE_SIZE: f32 = 75.0;
pub const ENEMY_ZOOMER_SPRITE_SIZE: f32 = 75.0;
pub const BULLETSPEED: f32 = 1000.0;
pub const BULLET_SPRITE_DIMENSION: Vec2 = Vec2::new(90.0, 54.0);
pub const MAX_NUM_ENEMIES: usize = 10;
pub const TYPES_OF_ENEMIES: usize = 3;
