use bevy::prelude::*;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
        .add_plugins(DefaultPlugins);
    }
}


/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;

pub fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}