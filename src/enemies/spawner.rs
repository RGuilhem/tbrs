use crate::enemies::SpawnEnemy;
use crate::movements::GridPos;
use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct Spawner {
    pub grid_pos: GridPos,
    pub density: f32,
    pub amount: usize,
}

impl Default for Spawner {
    fn default() -> Self {
        Spawner {
            grid_pos: GridPos(IVec2::ZERO),
            density: 1.0,
            amount: 5,
        }
    }
}

impl Spawner {
    pub fn new(grid_pos: GridPos) -> Self {
        Spawner {
            grid_pos,
            ..default()
        }
    }

    pub fn spawn_all_of(&self, mut commands: Commands, creator: fn(i32, i32) -> SpawnEnemy) {
        let mut rng = rand::thread_rng();
        for _ in 0..self.amount {
            let x = rng.gen_range(self.grid_pos.0.x - 10..self.grid_pos.0.x + 10);
            let y = rng.gen_range(self.grid_pos.0.y - 10..self.grid_pos.0.y + 10);
            commands.spawn(creator(x, y));
        }
    }
}
