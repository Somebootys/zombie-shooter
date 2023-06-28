use crate::components::{MainCamera, WinSize};
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

pub fn setup(mut commands: Commands, query: Query<&Window, With<PrimaryWindow>>) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
    let Ok(primary) = query.get_single() else {
        return;
    };
    let (win_w, win_h) = (primary.width(), primary.height());

    let win_size = WinSize {
        width: win_w,
        height: win_h,
    };

    commands.insert_resource(win_size);
}
