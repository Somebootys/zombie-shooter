use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::components::{MainCamera};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
 
;
    }
}




pub fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

