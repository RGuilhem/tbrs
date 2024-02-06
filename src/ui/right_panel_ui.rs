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
                    //justify_content: JustifyContent::Stretch,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: Color::DARK_GRAY.into(),
                border_color: Color::MAROON.into(),
                ..default()
            },
            _chat_ui: RightPanel,
        }
    }
}

#[derive(Component)]
pub struct TopRightPanel;

#[derive(Bundle)]
pub struct TopRightPanelBundle {
    node: NodeBundle,
    _top_right_panel: TopRightPanel,
}

impl Default for TopRightPanelBundle {
    fn default() -> Self {
        TopRightPanelBundle {
            node: NodeBundle {
                style: Style {
                    height: Val::Percent(40.0),
                    width: Val::Percent(100.0),
                    ..default()
                },
                background_color: Color::NAVY.into(),
                ..default()
            },
            _top_right_panel: TopRightPanel,
        }
    }
}

#[derive(Component)]
pub struct BottomRightPanel;

#[derive(Bundle)]
pub struct BottomRightPanelBundle {
    node: NodeBundle,
    _bottom_right_panel: BottomRightPanel,
}

impl Default for BottomRightPanelBundle {
    fn default() -> Self {
        BottomRightPanelBundle {
            node: NodeBundle {
                style: Style {
                    height: Val::Percent(60.0),
                    width: Val::Percent(100.0),
                    ..default()
                },
                background_color: Color::DARK_GREEN.into(),
                ..default()
            },
            _bottom_right_panel: BottomRightPanel,
        }
    }
}
