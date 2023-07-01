use crate::components::{EnemyCount, GameTextures, MainCamera, WinSize};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_plugins(DefaultPlugins)
            .add_plugin(WorldInspectorPlugin::new());
    }
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
    };

    commands.insert_resource(game_textures);
    //enemy count resource init
    commands.insert_resource(EnemyCount(0));
}
