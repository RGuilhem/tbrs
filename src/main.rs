use crate::player::PlayerPlugin;
use crate::ui::UiPlugin;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::render::camera::Viewport;

pub mod player;
pub mod ui;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "TBRS".into(),
                        resolution: (1440.0, 1080.0).into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .init_resource::<Sprites>()
        .add_systems(Startup, setup)
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

pub const GRID_WIDTH: u32 = 17;
pub const GRID_HEIGHT: u32 = 11;
pub const GRID_SIZE: u32 = 64;

#[derive(Resource)]
pub struct Sprites(Handle<TextureAtlas>);

impl FromWorld for Sprites {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        let texture_handle = asset_server.load("monochrome-transparent_packed.png");

        let atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 49, 22, None, None);
        let mut texture_atlases = world.get_resource_mut::<Assets<TextureAtlas>>().unwrap();
        let atlas_handle = texture_atlases.add(atlas);

        Self(atlas_handle)
    }
}

#[derive(Component)]
pub struct GameCamera;

fn setup(mut commands: Commands, window: Query<&Window>) {
    let window = window.single();
    let scaling = window.resolution.scale_factor().round() as u32;
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                viewport: Some(Viewport {
                    physical_position: UVec2::new(0, 0),
                    physical_size: UVec2::new(
                        GRID_SIZE * GRID_WIDTH * scaling,
                        GRID_SIZE * GRID_HEIGHT * scaling,
                    ),
                    ..default()
                }),
                ..default()
            },
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::rgb(0.2, 0.2, 0.2)),
            },
            transform: Transform::from_scale(Vec3::new(0.25, 0.25, 1.0)),
            ..default()
        },
        GameCamera,
        UiCameraConfig { show_ui: false },
    ));
}
