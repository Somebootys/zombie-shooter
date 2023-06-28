use bevy::prelude::*;

//use crate::components::{ };

pub const TILE_SIZE: f32 = 50_f32;
pub const TILE_TYPES: i8 = 3;
pub const VERTS_IN_QUAD: i8 = 4;

#[derive(Component)]
pub struct TileIndices {
    pub first: usize,
    pub last: usize,
}

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("graphic/background_sheet.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(TILE_SIZE, TILE_SIZE),
        1,
        4,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    let tile_indices = TileIndices { first: 0, last: 3 };

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(tile_indices.first),
            transform: Transform {
                translation: Vec3::ZERO,

                rotation: Quat::IDENTITY,

                scale: Vec3::splat(1.0),
            },
            ..default()
        },
        tile_indices,
    ));
}
/*
pub fn setup(mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,){

    //need to build a vertice array of type quad


    commands.spawn(
        SpriteBundle {
            transform: Transform::from_xyz(ENEMY_SPRITE_SIZE, ENEMY_SPRITE_SIZE, 0.0),
            texture: asset_server.load("graphic/background_sheet.png"),
            ..default()
        });


    // Quad 1 spawn top row
    for i in 0..10 {
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(50., 50.)).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform::from_translation(Vec3::new(i as f32 * 50. + -50., 0., 0.)),
            ..default()
        });
    }

//spawning bottom row
for i in 0..10 {
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(50., 50.)).into())
            .into(),
        material: materials.add(ColorMaterial::from(Color::ALICE_BLUE)),
        transform: Transform::from_translation(Vec3::new(i as f32 * 50. + -50., -100., 0.)),
        ..default()
    });
}

/*

    // Quad 2
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(50., 50.)).into())
            .into(),
        material: materials.add(ColorMaterial::from(Color::BLUE)),
        transform: Transform::from_translation(Vec3::new(100., 0., 0.)),
        ..default()
    });
    */



}

*/
