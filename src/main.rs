use crate::player::PlayerPlugin;
use crate::ui::UiPlugin;
use bevy::prelude::*;

pub mod player;
pub mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Sprites>()
        .add_systems(Startup, setup)
        .add_systems(Startup, test)
        .add_plugins(TbrsPlugin)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

pub struct TbrsPlugin;

impl Plugin for TbrsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerPlugin).add_plugins(UiPlugin);
    }
}

#[derive(Resource)]
pub struct Sprites(Handle<TextureAtlas>);

impl FromWorld for Sprites {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        let texture_handle = asset_server.load("monochrome-transparent_packed.png");

        let atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(16.0, 16.0),
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

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn test(mut commands: Commands, atlas: Res<Sprites>) {
    commands.spawn(SpriteSheetBundle {
        sprite: TextureAtlasSprite::new(27),
        texture_atlas: atlas.0.clone(),
        ..default()
    });
}
