//this is the component file in src/components.rs
use crate::components::{GameScore, WinSize, Wave};

//this is the component file in src/game/components.rs
use crate::game::components::{
    DespawnedEnemies, EquippedGun, GameTextures, Gun, LastDamaged, MainCamera, PickUpTimer,
    Player, GunType, AmmoCount, PlayerOrientation, ReloadTimer
};


use bevy::{prelude::*,  window::PrimaryWindow};

//use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use std::collections::HashSet;

pub fn physics_setup(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::new(0.0, 0.0);
}

pub fn setup(
    mut commands: Commands,
    query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
   
) {
    //win resource init
    let Ok(primary) = query.get_single() else {
        return;
    };
    let (win_w, win_h) = (primary.width(), primary.height());

    let win_size = WinSize {
        width: win_w,
        height: win_h,
    };

    commands.insert_resource(win_size);

    //insert player angle
    commands.insert_resource(PlayerOrientation{
        angle: 0.0,
        linvel: Vec2::new(0.0, 0.0),
    });

    //insert score resource
    let score = GameScore(0);

    commands.insert_resource(score);

    //texture atlas resource init
    let game_textures = GameTextures {
        player: asset_server.load("graphic/player.png"),
        projectile: asset_server.load("graphic/laser_a_01.png"),
        enemy_boomer: asset_server.load("graphic/boomer.png"),
        enemy_crawler: asset_server.load("graphic/crawler.png"),
        enemy_zoomer: asset_server.load("graphic/zoomer.png"),
        pickup_health: asset_server.load("graphic/health_pickup.png"),
        pickup_ammo: asset_server.load("graphic/ammo_pickup.png"),
        error: asset_server.load("graphic/icon.png"),
    };

    commands.insert_resource(game_textures);

    //despawned enemies resource init
    commands.insert_resource(DespawnedEnemies {
        entities: HashSet::new(),
    });

    //insert Ammo inventory 
    commands.insert_resource(AmmoCount {
        pistol: 0,
        shotgun: 0,
        machine_gun: 0,
    });

    //insert last damaged time

    commands.insert_resource(LastDamaged {
        time: Timer::from_seconds(0.5, TimerMode::Once),
    });

    //insert pick up timer
    commands.insert_resource(PickUpTimer {
        time: Timer::from_seconds(5.0, TimerMode::Once),
    });

    let gun = EquippedGun(Gun::from_gun_type(GunType::Pistol));
    //insert equipped gun
    commands.insert_resource(gun);

    //insert reload timer
    commands.insert_resource(ReloadTimer {
        timer: Timer::from_seconds(1.2, TimerMode::Once),
        reloading: false,
    });

    //insert wave resource
    commands.insert_resource(Wave(1));

   

   
}

pub fn camera_movement(
    player: Query<&mut Transform, (With<Player>, Without<MainCamera>)>,
    mut camera: Query<&mut Transform, With<MainCamera>>,
    //mut windows: Query<&mut Window>,
) {
    //move camera with the player

    for player_transform in player.iter() {
        for mut camera_transform in camera.iter_mut() {
            camera_transform.translation.x = player_transform.translation.x;
            camera_transform.translation.y = player_transform.translation.y;
        }
    }
}
