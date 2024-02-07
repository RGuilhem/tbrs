use bevy::prelude::*;

#[derive(Resource)]
pub struct Sprites(pub Handle<TextureAtlas>);

impl FromWorld for Sprites {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        let texture_handle = asset_server.load("tilesheet.png");

        let atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 49, 22, None, None);
        let mut texture_atlases = world.get_resource_mut::<Assets<TextureAtlas>>().unwrap();
        let atlas_handle = texture_atlases.add(atlas);

        Self(atlas_handle)
    }
}
