use crate::game_world::GameMapPlugin;
use crate::player::PlayerPlugin;
use crate::ui::UiPlugin;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::render::camera::Viewport;

pub mod game_world;
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
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .init_resource::<Sprites>()
        .add_systems(Startup, setup)
        .add_plugins(TbrsPlugin)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

pub struct TbrsPlugin;

#[derive(Resource)]
pub struct DebugTimer(Timer);

impl Plugin for TbrsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GameMapPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(UiPlugin)
            .insert_resource(DebugTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Update, fps_info);
    }
}

fn fps_info(diagnostics: Res<DiagnosticsStore>, mut timer: ResMut<DebugTimer>, time: Res<Time>) {
    if timer.0.tick(time.delta()).just_finished() {
        if let Some(value) = diagnostics
            .get(FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            println!("{:?}", value);
        }
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
        let texture_handle = asset_server.load("tilesheet.png");

        let atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 49, 22, None, None);
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
            //transform: Transform::from_scale(Vec3::new(0.25, 0.25, 1.0)),
            ..default()
        },
        GameCamera,
        UiCameraConfig { show_ui: false },
    ));
}
