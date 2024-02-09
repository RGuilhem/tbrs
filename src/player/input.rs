use crate::alive::Target;
use crate::enemies::Enemy;
use crate::movements::GridPos;
use crate::movements::Movement;
use crate::player::Player;
use crate::WorldCoords;
use crate::GRID_SIZE;
use bevy::prelude::*;
use std::cmp::Ordering;
use std::f32::consts::PI;

pub fn player_movement(keys: Res<Input<KeyCode>>, mut query: Query<&mut Movement, With<Player>>) {
    let mut dir = (0.0, 0.0);
    if keys.pressed(KeyCode::W) || keys.pressed(KeyCode::E) || keys.pressed(KeyCode::Q) {
        dir.1 += 1.0;
    }
    if keys.pressed(KeyCode::S) || keys.pressed(KeyCode::C) || keys.pressed(KeyCode::Z) {
        dir.1 += -1.0;
    }
    if keys.pressed(KeyCode::D) || keys.pressed(KeyCode::E) || keys.pressed(KeyCode::C) {
        dir.0 += 1.0;
    }
    if keys.pressed(KeyCode::A) || keys.pressed(KeyCode::Q) || keys.pressed(KeyCode::Z) {
        dir.0 += -1.0;
    }
    let mut mov = query.single_mut();
    mov.directions.x = dir.0;
    mov.directions.y = dir.1;
}

#[derive(Default, Debug)]
pub struct TragetIndex(usize);

pub fn change_target_system(
    mut index: Local<TragetIndex>,
    keys: Res<Input<KeyCode>>,
    mut p: Query<&mut Target, With<Player>>,
    e: Query<(Entity, &Transform), (With<Enemy>, Without<Player>)>,
) {
    if keys.just_pressed(KeyCode::Tab) {
        let targets_number = e.iter().size_hint().0;
        let mut enemies: Vec<(Entity, &Transform)> = e.iter().collect();
        enemies.sort_by(|a, b| {
            if a.1.translation.x > b.1.translation.x || a.1.translation.x > b.1.translation.y {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });
        let mut target = p.single_mut();
        for (i, (enemy, trans)) in enemies.iter().enumerate() {
            if i == index.0 {
                target.entity = Some(*enemy);
                target.pos = Some(Vec2::new(trans.translation.x, trans.translation.y));
            }
        }
        if targets_number != 0 {
            index.0 = (index.0 + 1) % targets_number;
        } else {
            index.0 = 0;
        }
    }
}

pub fn update_target_system(
    mut targets: Query<&mut Target>,
    targets_pos: Query<(Entity, &Transform), Changed<Transform>>,
) {
    for mut target in targets.iter_mut() {
        if let Some(entity) = target.entity {
            for (checking, trans) in targets_pos.iter() {
                if checking == entity {
                    target.pos = Some(Vec2::new(trans.translation.x, trans.translation.y));
                    return;
                }
            }
        }
    }
}

pub fn click_target_system(
    mouse: Res<Input<MouseButton>>,
    mouse_pos: Res<WorldCoords>,
    mut p: Query<&mut Target, With<Player>>,
    e: Query<(Entity, &GridPos, &Transform), (With<Enemy>, Without<Player>)>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let mut target = p.single_mut();
        for (enemy, pos, trans) in e.iter() {
            if pos.0.x == mouse_pos.0.x && pos.0.y == mouse_pos.0.y {
                target.entity = Some(enemy);
                target.pos = Some(Vec2::new(trans.translation.x, trans.translation.y));
                break;
            }
        }
    }
}

#[derive(Component)]
pub struct TargetBorder;

pub fn setup_target_system(
    mut commands: Commands,
) {
    let sprite = Sprite {
        color: Color::rgba(1.0, 1.0, 0.1, 0.3),
        custom_size: Some(Vec2::splat(GRID_SIZE as f32 / 2.0)),
        ..default()
    };
    commands.spawn((
        SpriteBundle {
            sprite,
            visibility: Visibility::Hidden,
            ..default()
        },
        TargetBorder,
    ));
}

// TODO: fix flicker on quick target change
pub fn target_border_system(
    target_query: Query<&Target, With<Player>>,
    mut trans: Query<(&mut Transform, &mut Visibility), With<TargetBorder>>,
    time: Res<Time>,
) {
    let target = target_query.single();
    for (mut transform, mut vis) in trans.iter_mut() {
        if let Some(_) = target.entity {
            *vis = Visibility::Visible;
            if let Some(pos) = &target.pos {
                transform.translation.x = pos.x;
                transform.translation.y = pos.y;
                transform.translation.z = 0.1;
                transform.rotate_z(time.delta_seconds() * PI / 3.0);
            }
        } else {
            *vis = Visibility::Hidden;
        }
    }
}
