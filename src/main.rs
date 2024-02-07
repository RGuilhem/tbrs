use crate::game_world::GameMapPlugin;
use crate::player::PlayerPlugin;
use crate::sprites::Sprites;
use crate::ui::UiPlugin;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::render::camera::Viewport;
use bevy_rand::prelude::*;
//use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub mod alive;
pub mod enemies;
pub mod game_world;
pub mod movements;
pub mod player;
pub mod sprites;
pub mod ui;

pub const WIN_HEIGHT: f32 = 1080.0;
pub const WIN_WIDTH: f32 = 1920.0;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "TBRS".into(),
                        resolution: (WIN_WIDTH, WIN_HEIGHT).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(EntropyPlugin::<ChaCha8Rng>::default())
        //.add_plugins(WorldInspectorPlugin::new())
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
            info!("FPS: {:?}", value);
        }
    }
}

pub const GRID_WIDTH: u32 = 17;
pub const GRID_HEIGHT: u32 = 11;
pub const GRID_SIZE: u32 = 64;

#[derive(Component)]
pub struct GameCamera;

fn setup(mut commands: Commands, window: Query<&Window>) {
    let window = window.single();
    let scaling = window.resolution.scale_factor().round() as u32;
    let game_cam = Camera2dBundle {
        camera: Camera {
            viewport: Some(Viewport {
                physical_position: UVec2::new(0, 0),
                physical_size: UVec2::new(
                    GRID_SIZE * GRID_WIDTH * scaling as u32,
                    GRID_SIZE * GRID_HEIGHT * scaling as u32,
                ),
                ..default()
            }),
            ..default()
        },
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
        },
        ..default()
    };

    commands.spawn((game_cam, GameCamera, UiCameraConfig { show_ui: false }));
}
