use crate::alive::Alive;
use crate::alive::Target;
use crate::movements::GridPos;
use crate::skills::Damage;
use crate::GRID_SIZE;
use bevy::prelude::*;
use std::f32::consts::SQRT_2;

#[derive(Component)]
pub enum AttackStyle {
    Melee,
    Range,
    Magic,
}

#[derive(Component)]
pub struct AutoAttack {
    pub timer: Timer,
    pub damage: i32,
    pub style: AttackStyle,
}

#[derive(Bundle)]
pub struct AutoAttackBundle {
    pub auto_attack: AutoAttack,
}

impl Default for AutoAttackBundle {
    fn default() -> Self {
        AutoAttackBundle {
            auto_attack: AutoAttack {
                timer: Timer::from_seconds(2.0, TimerMode::Repeating),
                damage: 20,
                style: AttackStyle::Melee,
            },
        }
    }
}

fn check_range(style: &AttackStyle, a_pos: &GridPos, d_pos: Option<Vec2>) -> bool {
    if let Some(d_pos) = d_pos {
        let d_pos = d_pos / GRID_SIZE as f32;
        let a_pos = Vec2::new(a_pos.0.x as f32, a_pos.0.y as f32);
        let distance = a_pos.distance(d_pos);
        match style {
            AttackStyle::Melee => return distance <= SQRT_2 * 1.0,
            AttackStyle::Range => return distance <= SQRT_2 * 7.0,
            AttackStyle::Magic => return distance <= SQRT_2 * 5.0,
        }
    }
    false
}

pub fn auto_attack_system(
    mut commands: Commands,
    mut q_attacker: Query<(&mut AutoAttack, &Target, &GridPos)>,
    q_defender: Query<Entity, With<Alive>>,
    time: Res<Time>,
) {
    for (mut attack, target, grid_pos) in q_attacker.iter_mut() {
        if attack.timer.tick(time.delta()).just_finished() {
            if let Some(target_entity) = target.entity {
                for id in q_defender.iter() {
                    if id == target_entity && check_range(&attack.style, grid_pos, target.pos) {
                        info!("Can attack");
                        commands.entity(id).insert(Damage {
                            damage: attack.damage,
                        });
                    }
                }
            }
        }
    }
}
