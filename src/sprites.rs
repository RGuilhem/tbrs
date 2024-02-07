use crate::GRID_SIZE;
use bevy::prelude::*;

#[derive(Resource)]
pub struct Sprites(pub Handle<TextureAtlas>);

impl FromWorld for Sprites {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        let texture_handle = asset_server.load("tilesheet.png");

        let atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(GRID_SIZE as f32, GRID_SIZE as f32),
            49,
            22,
            None,
            None,
        );
        let mut texture_atlases = world.get_resource_mut::<Assets<TextureAtlas>>().unwrap();
        let atlas_handle = texture_atlases.add(atlas);

        Self(atlas_handle)
    }
}

pub fn transform_from_grid(x: i32, y: i32, z: i32) -> Transform {
    Transform::from_xyz(GRID_SIZE as f32 * x as f32, GRID_SIZE as f32 * y as f32, z as f32)
}

pub fn sprite_index(col: u32, row: u32) -> usize {
    (row * 49 + col) as usize
}
