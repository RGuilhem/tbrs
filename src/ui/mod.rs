use crate::alive::Hp;
use crate::player::Player;
use crate::ui::chat_ui::ChatUiBundle;
use crate::ui::right_panel_ui::BottomRightPanelBundle;
use crate::ui::right_panel_ui::RighPanelBundle;
use crate::ui::right_panel_ui::TopRightPanelBundle;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;

pub mod chat_ui;
pub mod right_panel_ui;

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
                //border: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Right panel
            parent
                .spawn(RighPanelBundle::default())
                .with_children(|parent| {
                    parent.spawn(TopRightPanelBundle::default());
                    parent.spawn(BottomRightPanelBundle::default());
                });
            // Bottom panel
            parent.spawn(ChatUiBundle::default());
        });
}

fn hp_ui_system(hp: Query<&Hp, With<Player>>) {
    if let Ok(_hp) = hp.get_single() {
        //println!("hp: {:#?}", hp);
    }
}
