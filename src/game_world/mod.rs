use crate::Sprites;
use crate::GRID_HEIGHT;
use crate::GRID_SIZE;
use crate::GRID_WIDTH;
use bevy::prelude::*;
use bevy_rand::prelude::*;
use rand_core::RngCore;

pub struct GameMapPlugin;

impl Plugin for GameMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_game_map);
    }
}

#[derive(Component)]
pub struct GridCell;

//#[derive(Component)]
//pub struct GridPosition(Vec2);

#[derive(Bundle)]
pub struct MapCellBundle {
    _grid_cell: GridCell,
    //grid_position: GridPosition,
    sprite: SpriteSheetBundle,
}

fn setup_game_map(
    mut commands: Commands,
    atlas: Res<Sprites>,
    mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
) {
    println!("Setting up game map");
    for row in 0..GRID_HEIGHT {
        for col in 0..GRID_WIDTH {
            let res = rng.next_u32() % 3;
            let mut sprite = TextureAtlasSprite::new(5 + res as usize);
            if rng.next_u32() % 11 != 0 {
                sprite.color = Color::NONE;
            } else {
                sprite.color = Color::rgb(0.1, 0.75, 0.1);
            }
            commands.spawn(MapCellBundle {
                _grid_cell: GridCell,
                //grid_position: GridPosition(Vec2::new(col as f32, row as f32)),
                sprite: SpriteSheetBundle {
                    sprite,
                    texture_atlas: atlas.0.clone(),
                    transform: Transform::from_xyz(
                        GRID_SIZE as f32 * (col as f32 - (GRID_WIDTH / 2) as f32),
                        GRID_SIZE as f32 * (row as f32 - (GRID_HEIGHT / 2) as f32),
                        0.0,
                    ),
                    ..default()
                },
            });
        }
    }
}
