use crate::movements::GridPos;
use bevy::prelude::*;
use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};
use noise::Perlin;
use rand::prelude::*;

use super::GridCellType;

pub struct AreaCell {
    cell_type: GridCellType,
    grid_pos: GridPos,
}

pub struct Area {
    size: UVec2,
    cells: Vec<Vec<AreaCell>>,
}

impl std::fmt::Debug for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

fn perlin_noise(size: UVec2, spread_x: Vec2, spread_y: Vec2) -> Vec<Vec<f64>> {
    let perlin = Perlin::new(50);
    let map = PlaneMapBuilder::<_, 2>::new(&perlin)
        .set_size(size.x as usize, size.y as usize)
        .set_x_bounds(spread_x.x as f64, spread_x.y as f64)
        .set_y_bounds(spread_y.x as f64, spread_y.y as f64)
        .build();
    let mut noise: Vec<Vec<f64>> = vec![];
    for row in 0..size.y {
        let mut temp: Vec<f64> = vec![];
        for col in 0..size.x {
            temp.push(map.get_value(col as usize, row as usize));
        }
        noise.push(temp);
    }
    noise
}

fn perlin_noise_simple(size: UVec2, spread: Vec2) -> Vec<Vec<f64>> {
    perlin_noise(size, spread, spread)
}

fn gen_random_area() -> Area {
    let size = UVec2::splat(40);
    let noise = perlin_noise_simple(size, Vec2::new(-5.0, 5.0));
    let mut cells: Vec<Vec<AreaCell>> = vec![];
    for (i, row) in noise.iter().enumerate() {
        let mut temp: Vec<AreaCell> = vec![];
        for (j, col) in row.iter().enumerate() {
            let kind;
            if *col > 0.5 {
                kind = GridCellType::Wall;
            } else {
                kind = GridCellType::Ground;
            }
            temp.push(AreaCell {
                cell_type: kind,
                grid_pos: GridPos(IVec2::new(j as i32, i as i32)),
            });
        }
        cells.push(temp);
    }
    Area {
        size,
        cells,
    }
}

pub fn create_random_map(mut _commands: Commands) {
    let area = gen_random_area();
}
