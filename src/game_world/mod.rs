use crate::game_world::areas::create_random_map;
use crate::movements::GridPos;
use crate::sprites::transform_from_grid;
use crate::Sprites;
use bevy::prelude::*;

pub mod areas;
pub mod hub;

pub struct GameMapPlugin;

impl Plugin for GameMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_random_map);
    }
}

#[derive(Component, Debug)]
pub enum GridCellType {
    Wall,
    Ground,
    Empty,
}

#[derive(Component)]
pub struct Collider;

#[derive(Bundle)]
pub struct MapCellBundle {
    cell_type: GridCellType,
    grid_pos: GridPos,
    pub sprite: SpriteSheetBundle,
}

impl MapCellBundle {
    pub fn new(x: i32, y: i32, s_index: usize, atlas: &Res<Sprites>) -> Self {
        let mut sprite = TextureAtlasSprite::new(s_index);
        sprite.color = Color::GRAY;
        MapCellBundle {
            cell_type: GridCellType::Empty,
            grid_pos: GridPos(IVec2::new(x, y)),
            sprite: SpriteSheetBundle {
                sprite,
                texture_atlas: atlas.0.clone(),
                transform: transform_from_grid(x, y, 0),
                ..default()
            },
        }
    }
}
