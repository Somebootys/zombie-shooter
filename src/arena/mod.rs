use bevy::prelude::*;

use crate::components::{ArenaSize, TileIndices};
use rand::Rng;
pub const TILE_SIZE: f32 = 50_f32;
pub const TILE_TYPES: usize = 3;
pub const VERTS_IN_QUAD: i8 = 4;

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
    // width and height of the arena
    let arena_size = ArenaSize {
        width: 500.0,
        height: 500.0,
    };

    let world_width = (arena_size.width as i32) / (TILE_SIZE as i32);
    let world_height = (arena_size.width as i32) / (TILE_SIZE as i32);
    commands.insert_resource(arena_size);

    for w in 0..world_width {
        for h in 0..(world_height) {
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
            let tile_indices = TileIndices {
                first: 0,
                last: TILE_TYPES,
            };
            let x = -250 as f32 + w as f32 * TILE_SIZE + TILE_SIZE;
            let y = -250 as f32 + h as f32 * TILE_SIZE + TILE_SIZE;
            let z = 0.0;

            // if outer edge then spawn wall
            if h == 0 || h == world_height - 1 || w == 0 || w == world_width - 1 {
                // Use the wall texture
                commands.spawn((
                    SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle,
                        sprite: TextureAtlasSprite::new(tile_indices.last),
                        transform: Transform {
                            translation: Vec3::new(x, y, z),

                            rotation: Quat::IDENTITY,

                            scale: Vec3::splat(1.0),
                        },
                        ..default()
                    },
                    tile_indices,
                ));
            }
            // spawn the normal floor tiles
            else {
                let mut rng = rand::thread_rng();
                let random_number: usize = rng.gen_range(0..=2); // generates a usize between 0 and 2
                                                                 // Use the floor texture
                commands.spawn((
                    SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle.clone(),
                        sprite: TextureAtlasSprite::new(tile_indices.first + random_number),
                        transform: Transform {
                            translation: Vec3::new(x, y, z),

                            rotation: Quat::IDENTITY,

                            scale: Vec3::splat(1.0),
                        },
                        ..default()
                    },
                    tile_indices,
                ));
            }
        }
    }
}

/*
 // Load the texture atlas containing our sprites. Texture atlases are a
    // collection of smaller images, combined into a single larger image.
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
    // Use only the subset of sprites in the sheet that make up the tile types.
    let tile_indices = TileIndices { first: 0, last: TILE_TYPES };

commands.spawn((
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    sprite: TextureAtlasSprite::new(tile_indices.first +1),
                    transform: Transform {
                        translation: Vec3::ZERO,

                        rotation: Quat::IDENTITY,

                        scale: Vec3::splat(1.0),
                    },
                    ..default()
                },
                tile_indices,
            )); */

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
