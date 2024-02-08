use crate::game_world::Collider;
use crate::GRID_SIZE;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Movement {
    pub speed: f32,
    pub directions: Vec2,
}

const SPEED_REDUCTION: f32 = 30.0;

impl Default for Movement {
    fn default() -> Self {
        Movement {
            speed: 50.0 / SPEED_REDUCTION,
            directions: Vec2::new(0.0, 0.0),
        }
    }
}

#[derive(Component)]
pub struct SubPos(Vec2);

#[derive(Component, Clone)]
pub struct GridPos(pub IVec2);

#[derive(Bundle)]
pub struct MovementBundle {
    pub movement: Movement,
    pub sub_pos: SubPos,
    pub grid_pos: GridPos,
}

impl Default for MovementBundle {
    fn default() -> Self {
        MovementBundle {
            movement: Movement::default(),
            sub_pos: SubPos(Vec2::ZERO),
            grid_pos: GridPos(IVec2::ZERO),
        }
    }
}

pub fn initiate_movements(
    mut query: Query<(&Movement, &mut SubPos, &GridPos)>,
    q: Query<&GridPos, With<Collider>>,
) {
    for (mov, mut sub_pos, grid_pos) in query.iter_mut() {
        // initiate a move
        if (mov.directions.x != 0.0 || mov.directions.y != 0.0)
            && sub_pos.0.x == 0.0
            && sub_pos.0.y == 0.0
        {
            let mut can_move = true;
            for cell in q.iter() {
                if cell.0.x == (mov.directions.x + grid_pos.0.x as f32).round() as i32
                    && cell.0.y == (mov.directions.y + grid_pos.0.y as f32).round() as i32
                {
                    can_move = false;
                    break;
                }
            }
            if can_move {
                sub_pos.0.x += 0.0166 * mov.directions.x;
                sub_pos.0.y += 0.0166 * mov.directions.y;
            }
        }
    }
}

pub fn apply_movements(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Movement, &mut SubPos, &mut GridPos)>,
) {
    for (mut trans, mov, mut sub_pos, mut grid_pos) in query.iter_mut() {
        let mag = Vec2::new(sub_pos.0.x, sub_pos.0.y);
        let mag = mag.normalize_or_zero();
        sub_pos.0.x += time.delta_seconds() * mag.x * mov.speed;
        sub_pos.0.y += time.delta_seconds() * mag.y * mov.speed;

        if sub_pos.0.x.abs() > 1.0 || sub_pos.0.y.abs() > 1.0 {
            sub_pos.0.x = 0.0;
            sub_pos.0.y = 0.0;
            grid_pos.0.x += mag[0].round() as i32;
            grid_pos.0.y += mag[1].round() as i32;
        }
        trans.translation.x = (grid_pos.0.x as f32 + sub_pos.0.x) * GRID_SIZE as f32;
        trans.translation.y = (grid_pos.0.y as f32 + sub_pos.0.y) * GRID_SIZE as f32;
    }
}
