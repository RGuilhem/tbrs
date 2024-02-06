use crate::GRID_SIZE;
use crate::GRID_WIDTH;
use crate::WIN_WIDTH;
use bevy::prelude::*;

#[derive(Component)]
pub struct RightPanel;

#[derive(Bundle)]
pub struct RighPanelBundle {
    node: NodeBundle,
    _chat_ui: RightPanel,
}

impl Default for RighPanelBundle {
    fn default() -> Self {
        RighPanelBundle {
            node: NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Px(WIN_WIDTH - (GRID_SIZE * GRID_WIDTH) as f32),
                    height: Val::Percent(100.0),
                    left: Val::Px((GRID_SIZE * GRID_WIDTH) as f32),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                background_color: Color::rgb(0.6, 0.6, 0.6).into(),
                border_color: Color::MAROON.into(),
                ..default()
            },
            _chat_ui: RightPanel,
        }
    }
}
