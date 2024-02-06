use crate::player::Hp;
use crate::player::Player;
use crate::GRID_HEIGHT;
use crate::GRID_SIZE;
use crate::GRID_WIDTH;
use crate::WIN_HEIGHT;
use crate::WIN_WIDTH;
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
        UI_LAYER,
    ));

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Px(WIN_WIDTH - (GRID_SIZE * GRID_WIDTH) as f32),
                    height: Val::Percent(100.0),
                    left: Val::Px((GRID_SIZE * GRID_WIDTH) as f32),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                background_color: Color::rgb(0.6, 0.6, 0.6).into(),
                border_color: Color::rgb(0.2, 1.0, 0.2).into(),
                ..default()
            });
            parent.spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Px((GRID_SIZE * GRID_WIDTH) as f32),
                    height: Val::Px(WIN_HEIGHT - (GRID_SIZE * GRID_HEIGHT) as f32),
                    top: Val::Px((GRID_SIZE * GRID_HEIGHT) as f32),
                    left: Val::Px(0.0),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                background_color: Color::rgb(0.6, 0.6, 0.6).into(),
                border_color: Color::rgb(0.2, 1.0, 0.2).into(),
                ..default()
            });
        });
}

fn hp_ui_system(hp: Query<&Hp, With<Player>>) {
    if let Ok(_hp) = hp.get_single() {
        //println!("hp: {:#?}", hp);
    }
}
