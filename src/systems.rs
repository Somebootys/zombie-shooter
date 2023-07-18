use crate::components::{
    DespawnedEnemies, EnemyCount, EquippedGun, GameTextures, Guns, LastDamaged, MainCamera,
    PickUpTimer, Player, ReloadTimer, WinSize,
};

use bevy::prelude::*;
use bevy::time::Stopwatch;
use bevy::window::PrimaryWindow;
//use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use std::collections::HashSet;
pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemyCount(0))
            .add_startup_system(setup.in_base_set(StartupSet::PreStartup))
            .add_startup_system(physics_setup.in_base_set(StartupSet::PreStartup))
            .add_plugins(DefaultPlugins)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugin(RapierDebugRenderPlugin::default())
            //.add_plugin(WorldInspectorPlugin::new())
            .add_system(camera_movement.in_base_set(StartupSet::PostStartup));
    }
}

pub fn physics_setup(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::new(0.0, 0.0);
}

pub fn setup(
    mut commands: Commands,
    query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

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

    //insert player being hit or not

    //insert last damaged time

    commands.insert_resource(LastDamaged {
        time: Timer::from_seconds(0.5, TimerMode::Once),
    });

    //insert pick up timer
    commands.insert_resource(PickUpTimer {
        time: Timer::from_seconds(5.0, TimerMode::Once),
    });

    let gun = EquippedGun {
        gun_type: Guns::Pistol,
        bullets_in_magasine: 8,
    };
    //insert equipped gun
    commands.insert_resource(gun);

    let reload_timer = ReloadTimer {
        time: Stopwatch::new(),
    };

    //insert reload timer resource
    commands.insert_resource(reload_timer);
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
