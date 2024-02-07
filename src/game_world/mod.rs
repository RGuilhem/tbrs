use crate::player::movement::GridPos;
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

#[derive(Component, Debug)]
pub struct GridCell {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Collider;

//#[derive(Component)]
//pub struct GridPosition(Vec2);

#[derive(Bundle)]
pub struct MapCellBundle {
    _grid_cell: GridCell,
    //grid_pos: GridPos,
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
            if rng.next_u32() % 11 == 0 {
                sprite.color = Color::rgb(0.1, 0.75, 0.1);
                commands.spawn((
                    MapCellBundle {
                        _grid_cell: GridCell {
                            x: col as i32 - (GRID_WIDTH / 2) as i32,
                            y: row as i32 - (GRID_HEIGHT / 2) as i32,
                        },
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
                    },
                    Collider,
                ));
            }
        }
    }
    // spawning dummy colliding tile
    let sprite = TextureAtlasSprite::new(55);
    commands.spawn((
        MapCellBundle {
            _grid_cell: GridCell { x: 1, y: 1 },
            //grid_position: GridPosition(Vec2::new(col as f32, row as f32)),
            sprite: SpriteSheetBundle {
                sprite,
                texture_atlas: atlas.0.clone(),
                transform: Transform::from_xyz(1.0 * GRID_SIZE as f32, 1.0 * GRID_SIZE as f32, 0.0),
                ..default()
            },
        },
        Collider,
    ));
}
