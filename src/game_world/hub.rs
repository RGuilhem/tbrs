use crate::game_world::Collider;
use crate::game_world::MapCellBundle;
use crate::sprites::sprite_index;
use crate::sprites::Sprites;
use crate::GRID_HEIGHT;
use crate::GRID_WIDTH;
use bevy::prelude::*;
use bevy_rand::prelude::ChaCha8Rng;
use bevy_rand::resource::GlobalEntropy;
use rand_core::RngCore;
use std::f32::consts::PI;

pub fn setup_hub(
    mut commands: Commands,
    atlas: Res<Sprites>,
    mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
) {
    // TODO:
    // walls
    // ground
    // decorations
    const SIZE_X: i32 = GRID_WIDTH as i32;
    const SIZE_Y: i32 = GRID_HEIGHT as i32;
    // Setting up walls
    // top
    let s_index = sprite_index(48, 10);
    for x in 1..SIZE_X - 1 {
        // Top and bottom walls
        commands.spawn((
            MapCellBundle::new(x - SIZE_X / 2, SIZE_Y / 2, s_index, &atlas),
            Collider,
        ));
        commands.spawn((
            MapCellBundle::new(x - SIZE_X / 2, -SIZE_Y / 2, s_index, &atlas),
            Collider,
        ));
    }
    for y in 1..SIZE_Y - 1 {
        // Left and right walls
        commands.spawn((
            MapCellBundle::new(SIZE_X / 2, y - SIZE_Y / 2, s_index, &atlas),
            Collider,
        ));
        commands.spawn((
            MapCellBundle::new(-SIZE_X / 2, y - SIZE_Y / 2, s_index, &atlas),
            Collider,
        ));
    }
    //corners
    commands.spawn((
        MapCellBundle::new(-SIZE_X / 2, -SIZE_Y / 2, s_index, &atlas),
        Collider,
    ));
    commands.spawn((
        MapCellBundle::new(SIZE_X / 2, -SIZE_Y / 2, s_index, &atlas),
        Collider,
    ));
    commands.spawn((
        MapCellBundle::new(-SIZE_X / 2, SIZE_Y / 2, s_index, &atlas),
        Collider,
    ));
    commands.spawn((
        MapCellBundle::new(SIZE_X / 2, SIZE_Y / 2, s_index, &atlas),
        Collider,
    ));

    // GROUND looks like shit
    for y in 1..SIZE_Y - 1 {
        for x in 1..SIZE_X - 1 {
            if rng.next_u32() % 3 == 0 {
                continue;
            }
            let s_index = sprite_index(1, 0);
            let mut cell = MapCellBundle::new(x - SIZE_X / 2, y - SIZE_Y / 2, s_index, &atlas);
            cell.sprite.sprite.color = Color::DARK_GRAY;
            cell.sprite
                .transform
                .rotate_z(PI / 10.0 * (rng.next_u32() % 10) as f32);
            commands.spawn(cell);
        }
    }
}
