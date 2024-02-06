use crate::player::Player;
use bevy::prelude::*;

#[derive(Component)]
pub struct Movement {
    speed: f32,
    directions: (f32, f32),
}

impl Default for Movement {
    fn default() -> Self {
        Movement {
            speed: 1.25,
            directions: (0.0, 0.0),
        }
    }
}

pub fn player_movement(keys: Res<Input<KeyCode>>, mut query: Query<&mut Movement, With<Player>>) {
    let mut dir = (0.0, 0.0);
    if keys.pressed(KeyCode::W) {
        dir.1 = 1.0;
    }
    if keys.pressed(KeyCode::S) {
        dir.1 = -1.0;
    }
    if keys.pressed(KeyCode::D) {
        dir.0 = 1.0;
    }
    if keys.pressed(KeyCode::A) {
        dir.0 = -1.0;
    }
    let mut mov = query.single_mut();
    mov.directions.0 = dir.0;
    mov.directions.1 = dir.1;
}

pub fn apply_movements(mut query: Query<(&mut Transform, &Movement)>) {
    for (mut trans, mov) in query.iter_mut() {
        trans.translation[0] += mov.directions.0;
        trans.translation[1] += mov.directions.1;
    }
}
