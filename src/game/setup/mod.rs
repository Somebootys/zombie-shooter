use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub mod systems;
use systems::*;

use crate::components::AppState;
pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup, physics_setup))
            //.add_systems( OnEnter(AppState::InGame), (setup, physics_setup))
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            //.add_plugins(RapierDebugRenderPlugin::default())
            //.add_plugin(WorldInspectorPlugin::new())
            .add_systems(Update, camera_movement.run_if(in_state(AppState::InGame)));
    }
}
