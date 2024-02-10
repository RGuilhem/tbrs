use std::f32::consts::SQRT_2;

use crate::movements::GridPos;
use crate::alive::Target;
use crate::enemies::Enemy;
use crate::player::Player;
use bevy::prelude::*;

pub fn ai_in_target_range(distance: i32) -> bool {
    distance < (7.0 * SQRT_2) as i32 * (7.0 * SQRT_2) as i32
}

#[derive(Component)]
pub struct Ai;

pub fn ai_system(
    q_p: Query<(Entity, &GridPos, &Transform), With<Player>>,
    mut q_e: Query<(&mut Target, &GridPos), (With<Enemy>, Without<Player>)>,
) {
    let (id, player_pos, t) = q_p.single();
    for (mut target, enemy_pos) in q_e.iter_mut() {
        let distance = player_pos.0.distance_squared(enemy_pos.0);
        if ai_in_target_range(distance) {
            target.entity = Some(id);
            target.pos = Some(Vec2::new(t.translation.x, t.translation.y));
        } else {
            target.entity = None;
            target.pos = None;
        }
    }
}
