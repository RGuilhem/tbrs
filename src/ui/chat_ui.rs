use crate::GRID_HEIGHT;
use crate::GRID_SIZE;
use crate::GRID_WIDTH;
use crate::WIN_HEIGHT;
use bevy::prelude::*;

#[derive(Component)]
pub struct ChatUi;

#[derive(Bundle)]
pub struct ChatUiBundle {
    node: NodeBundle,
    _chat_ui: ChatUi,
}

const BG_CHAT: Color = Color::rgb(0.76, 0.75, 0.65);

impl Default for ChatUiBundle {
    fn default() -> Self {
        ChatUiBundle {
            node: NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Px((GRID_SIZE * GRID_WIDTH) as f32),
                    height: Val::Px(WIN_HEIGHT - (GRID_SIZE * GRID_HEIGHT) as f32),
                    top: Val::Px((GRID_SIZE * GRID_HEIGHT) as f32),
                    left: Val::Px(0.0),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                background_color: BG_CHAT.into(),
                border_color: Color::MAROON.into(),
                ..default()
            },
            _chat_ui: ChatUi,
        }
    }
}
