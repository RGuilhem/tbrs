use crate::game_world::Collider;
use crate::game_world::GridCell;
use crate::player::Player;
use crate::GRID_SIZE;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Movement {
    pub speed: f32,
    pub directions: Vec2,
}

const SPEED_REDUCTION: f32 = 35.0;

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

#[derive(Component)]
pub struct GridPos(Vec2);

#[derive(Bundle)]
pub struct MovementBundle {
    movement: Movement,
    sub_pos: SubPos,
    grid_pos: GridPos,
}

impl Default for MovementBundle {
    fn default() -> Self {
        MovementBundle {
            movement: Movement::default(),
            sub_pos: SubPos(Vec2::new(0.0, 0.0)),
            grid_pos: GridPos(Vec2::new(0.0, 0.0)),
        }
    }
}

pub fn player_movement(keys: Res<Input<KeyCode>>, mut query: Query<&mut Movement, With<Player>>) {
    let mut dir = (0.0, 0.0);
    if keys.pressed(KeyCode::W) {
        dir.1 += 1.0;
    }
    if keys.pressed(KeyCode::S) {
        dir.1 += -1.0;
    }
    if keys.pressed(KeyCode::D) {
        dir.0 += 1.0;
    }
    if keys.pressed(KeyCode::A) {
        dir.0 += -1.0;
    }
    if keys.pressed(KeyCode::Q) {
        dir.0 = -1.0;
        dir.1 = 1.0;
    }
    if keys.pressed(KeyCode::E) {
        dir.0 = 1.0;
        dir.1 = 1.0;
    }
    if keys.pressed(KeyCode::C) {
        dir.0 = 1.0;
        dir.1 = -1.0;
    }
    if keys.pressed(KeyCode::Z) {
        dir.0 = -1.0;
        dir.1 = -1.0;
    }
    let mut mov = query.single_mut();
    mov.directions.x = dir.0;
    mov.directions.y = dir.1;
}

pub fn apply_movements(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Movement, &mut SubPos, &mut GridPos)>,
    q: Query<&GridCell, With<Collider>>,
) {
    for (mut trans, mov, mut sub_pos, mut grid_pos) in query.iter_mut() {
        // initiate a move
        if (mov.directions.x != 0.0 || mov.directions.y != 0.0)
            && sub_pos.0.x == 0.0
            && sub_pos.0.y == 0.0
        {
            let mut can_move = true;
            for cell in q.iter() {
                if cell.x == (mov.directions.x + grid_pos.0.x).round() as i32
                    && cell.y == (mov.directions.y + grid_pos.0.y).round() as i32
                {
                    can_move = false;
                    break;
                }
            }
            if can_move {
                sub_pos.0.x += time.delta_seconds() * mov.directions.x;
                sub_pos.0.y += time.delta_seconds() * mov.directions.y;
            }
        }
        let mag = Vec2::new(sub_pos.0.x, sub_pos.0.y);
        let mag = mag.normalize_or_zero();
        sub_pos.0.x += time.delta_seconds() * mag.x * mov.speed;
        sub_pos.0.y += time.delta_seconds() * mag.y * mov.speed;

        if sub_pos.0.x.abs() > 1.0 || sub_pos.0.y.abs() > 1.0 {
            sub_pos.0.x = 0.0;
            sub_pos.0.y = 0.0;
            grid_pos.0.x += mag[0].round();
            grid_pos.0.y += mag[1].round();
        }
        trans.translation[0] = (grid_pos.0.x + sub_pos.0.x) * GRID_SIZE as f32;
        trans.translation[1] = (grid_pos.0.y + sub_pos.0.y) * GRID_SIZE as f32;
    }
}
