use crate::alive::Target;
use crate::enemies::Enemy;
use crate::movements::GridPos;
use crate::movements::Movement;
use crate::player::Player;
use crate::sprites::transform_from_grid;
use crate::WorldCoords;
use crate::GRID_SIZE;
use bevy::prelude::*;

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

pub fn change_target_system(
    keys: Res<Input<KeyCode>>,
    mut p: Query<&mut Target, With<Player>>,
    e: Query<Entity, (With<Enemy>, Without<Player>)>,
) {
    if keys.just_pressed(KeyCode::Tab) {
        let mut target = p.single_mut();
        for enemy in e.iter() {
            target.entity = Some(enemy);
        }
    }
}

pub fn click_target_system(
    mouse: Res<Input<MouseButton>>,
    mouse_pos: Res<WorldCoords>,
    mut p: Query<&mut Target, With<Player>>,
    e: Query<(Entity, &GridPos), (With<Enemy>, Without<Player>)>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let mut target = p.single_mut();
        for (enemy, pos) in e.iter() {
            if pos.0.x == mouse_pos.0.x && pos.0.y == mouse_pos.0.y {
                target.entity = Some(enemy);
                target.pos = Some(pos.clone());
                break;
            }
        }
    }
}

#[derive(Component)]
pub struct TargetBorder;

pub fn setup_target_system(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::YELLOW,
                custom_size: Some(Vec2::new(GRID_SIZE as f32, GRID_SIZE as f32)),
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        },
        TargetBorder,
    ));
}

pub fn target_border_system(
    target_query: Query<&Target, With<Player>>,
    mut trans: Query<(&mut Transform, &mut Visibility), With<TargetBorder>>,
) {
    let target = target_query.single();
    for (mut transform, mut vis) in trans.iter_mut() {
        if let Some(_) = target.entity {
            *vis = Visibility::Visible;
            if let Some(pos) = &target.pos {
                transform.translation = transform_from_grid(pos.0.x, pos.0.y, 0).translation;
            }
        } else {
            *vis = Visibility::Hidden;
        }
    }
}
