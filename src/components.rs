use bevy::prelude::*;
use bevy::time::Stopwatch;
use std::collections::HashSet;
use std::time::Duration;

// --------------------------------------------------------------------------------Constants

pub const PLAYER_SPRITE_SIZE: f32 = 50.0;
pub const ENEMY_BOOMER_SPRITE_SIZE: f32 = 75.0;
pub const ENEMY_CRAWLER_SPRITE_SIZE: f32 = 75.0;
pub const ENEMY_ZOOMER_SPRITE_SIZE: f32 = 75.0;
pub const PICKUP_SPRITE_SIZE: f32 = 64.0;
pub const BULLETSPEED: f32 = 1000.0;
pub const BULLET_SPRITE_DIMENSION: Vec2 = Vec2::new(90.0, 54.0);
pub const MAX_NUM_ENEMIES: usize = 10;
pub const TYPES_OF_ENEMIES: usize = 3;
pub const PICK_UP_DURATION: u64 = 5;
pub const TILE_SIZE: f32 = 50_f32;
pub const TILE_TYPES: usize = 3;
pub const VERTS_IN_QUAD: i8 = 4;
pub const ARENA_SIZE: f32 = 1000.0;

//---------------------------------------------------------------------------------Components

/// Used to help identify our main camera
/// 
/// 
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
}
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
pub struct PickUp {
    pub health: bool,
    pub ammo: bool,
}

#[derive(Component, Debug)]
pub struct Ammo {
    pub vec: [i32; 3],
}

#[derive(Component, Clone, Debug)]
pub struct GunType {
    pub gun_type: Guns,
}

impl GunType {
    pub fn gun_type(&self) -> &Guns {
        return &self.gun_type;
    }

    pub fn magasine_size(&self) -> usize {
        match self.gun_type {
            Guns::Pistol => 8,
            Guns::Shotgun => 3,
            Guns::MachineGun => 30,
        }
    }
    pub fn reload_time(&self) -> Duration {
        match self.gun_type {
            Guns::Pistol => Duration::from_secs_f32(1.5),
            Guns::Shotgun => Duration::from_secs_f32(2.5),
            Guns::MachineGun => Duration::from_secs_f32(1.0),
        }
    }

    pub fn gun_damage(&self) -> i32 {
        match self.gun_type {
            Guns::Pistol => 10,
            Guns::Shotgun => 30,
            Guns::MachineGun => 10,
        }
    }
}

#[derive(Component)]
pub struct PickUpDuration {
    pub time: Stopwatch,
}

#[derive(Resource, Debug)]
pub struct ReloadTimer {
    pub time: Stopwatch,
}
// --------------------------------------------------------------------------------ENUMS
#[derive(Debug, Clone)]
pub enum Guns {
    Pistol,
    Shotgun,
    MachineGun,
}

// --------------------------------------------------------------------------------Resources

#[derive(Resource, Debug)]
pub struct WinSize {
    pub width: f32,
    pub height: f32,
}

#[derive(Resource, Debug, Clone)]
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
    pub pickup_ammo: Handle<Image>,
    pub error: Handle<Image>,
}

#[derive(Resource, Debug)]
pub struct EnemiesAlive {
    pub hashset: HashSet<Entity>,
}

#[derive(Resource, Debug)]
pub struct LastDamaged {
    pub time: Timer,
}

#[derive(Resource, Debug)]
pub struct PickUpTimer {
    pub time: Timer,
}

#[derive(Resource, Debug, Clone)]
pub struct EquippedGun {
    pub gun_type: Guns,
    pub bullets_in_magasine: usize,
}
