use bevy::prelude::*;

//---------------------------------------------------------------------------------Components

/// Used to help identify our main camera

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
    LevelUp,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct TileIndices {
    pub first: usize,
    pub last: usize,
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
pub struct GameScore(pub i32);

#[derive(Resource)]
pub struct EnemyCount(pub usize);

#[derive(Resource, Debug, Clone)]
pub struct Wave (pub usize);
