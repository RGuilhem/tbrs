use crate::game_world::Collider;
use crate::movements::GridPos;
use crate::movements::MovementBundle;
use crate::Sprites;
use crate::GRID_SIZE;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Hp {
    pub base: u32,
    pub current: u32,
    pub max: u32,
}

#[derive(Component)]
pub struct Alive;

#[derive(Bundle)]
pub struct AliveBundle {
    pub hp: Hp,
    pub movement: MovementBundle,
    pub sprite: SpriteSheetBundle,
    pub _collider: Collider,
    pub _alive: Alive,
}

impl Default for AliveBundle {
    fn default() -> Self {
        AliveBundle {
            hp: Hp {
                base: 100,
                current: 100,
                max: 100,
            },
            sprite: SpriteSheetBundle { ..default() },
            movement: MovementBundle::default(),
            _collider: Collider,
            _alive: Alive,
        }
    }
}

impl AliveBundle {
    pub fn with_sprite(
        sprite: TextureAtlasSprite,
        atlas: &Res<Sprites>,
        transform: Transform,
    ) -> Self {
        let t = transform.translation;
        AliveBundle {
            sprite: SpriteSheetBundle {
                sprite,
                texture_atlas: atlas.0.clone(),
                transform,
                ..default()
            },
            movement: MovementBundle {
                grid_pos: GridPos(IVec2::new(
                    t.x as i32 / GRID_SIZE as i32,
                    t.y as i32 / GRID_SIZE as i32,
                )),
                ..default()
            },
            ..default()
        }
    }
}
