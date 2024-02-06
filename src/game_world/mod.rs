use crate::Sprites;
use crate::GRID_HEIGHT;
use crate::GRID_SIZE;
use crate::GRID_WIDTH;
use bevy::prelude::*;

pub struct GameMapPlugin;

impl Plugin for GameMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_game_map);
    }
}

#[derive(Component)]
pub struct GridCell;

#[derive(Component)]
pub struct GridPosition(Vec2);

#[derive(Bundle)]
pub struct MapCellBundle {
    _grid_cell: GridCell,
    grid_position: GridPosition,
    sprite: SpriteSheetBundle,
}

fn setup_game_map(mut commands: Commands, atlas: Res<Sprites>) {
    println!("Setting up game map");
    for row in -(GRID_HEIGHT as i32 + 1) / 2..(GRID_HEIGHT as i32 + 1) / 2 {
        for col in -(GRID_WIDTH as i32) / 2..GRID_WIDTH as i32 / 2 + 1 {
            let mut sprite = TextureAtlasSprite::new(3);
            if row == 0 && col == 0 {
                sprite.color = Color::rgb(0.0, 0.8, 0.0);
            } else {
                sprite.color = Color::rgb(0.8, 0.0, 0.0);
            }
            commands.spawn(MapCellBundle {
                _grid_cell: GridCell,
                grid_position: GridPosition(Vec2::new(col as f32, row as f32)),
                sprite: SpriteSheetBundle {
                    sprite,
                    texture_atlas: atlas.0.clone(),
                    transform: Transform::from_xyz(
                        GRID_SIZE as f32 * col as f32,
                        GRID_SIZE as f32 * row as f32,
                        0.0,
                    ),
                    ..default()
                },
            });
        }
    }
}
