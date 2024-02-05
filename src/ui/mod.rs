use crate::player::Hp;
use crate::player::Player;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
            .add_systems(Update, hp_ui_system);
    }
}

const UI_LAYER: RenderLayers = RenderLayers::layer(10);

#[derive(Component)]
pub struct UiCamera;

fn setup_ui(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: 1,
                ..default()
            },
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::None,
            },
            ..default()
        },
        UiCamera,
        UI_LAYER
    ));
}

fn hp_ui_system(hp: Query<&Hp, With<Player>>) {
    if let Ok(_hp) = hp.get_single() {
        //println!("hp: {:#?}", hp);
    }
}
