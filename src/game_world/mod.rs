use crate::movements::GridPos;
use crate::sprites::transform_from_grid;
use crate::Sprites;
use bevy::prelude::*;

use self::hub::setup_hub;

pub mod hub;

pub struct GameMapPlugin;

impl Plugin for GameMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_hub);
    }
}

#[derive(Component, Debug)]
pub struct GridCell;

#[derive(Component)]
pub struct Collider;

//#[derive(Component)]
//pub struct GridPosition(Vec2);

#[derive(Bundle)]
pub struct MapCellBundle {
    _grid_cell: GridCell,
    grid_pos: GridPos,
    pub sprite: SpriteSheetBundle,
}

impl MapCellBundle {
    pub fn new(x: i32, y: i32, s_index: usize, atlas: &Res<Sprites>) -> Self {
        let mut sprite = TextureAtlasSprite::new(s_index);
        sprite.color = Color::GRAY;
        MapCellBundle {
            _grid_cell: GridCell,
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
