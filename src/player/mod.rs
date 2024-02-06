use crate::player::movement::MovementBundle;
use crate::player::movement::{apply_movements, player_movement};
use crate::GameCamera;
use crate::Sprites;
use bevy::prelude::*;

pub mod movement;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, check_death)
            .add_systems(Update, camera_follow)
            .add_systems(Update, apply_movements)
            .add_systems(Update, player_movement);
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component, Debug)]
pub struct Hp {
    base: u32,
    current: u32,
    max: u32,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    hp: Hp,
    _player: Player,
    movement: MovementBundle,
    sprite: SpriteSheetBundle,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        PlayerBundle {
            _player: Player,
            hp: Hp {
                base: 100,
                current: 100,
                max: 100,
            },
            sprite: SpriteSheetBundle { ..default() },
            movement: MovementBundle::default(),
        }
    }
}

fn setup_player(mut commands: Commands, atlas: Res<Sprites>) {
    commands.spawn(PlayerBundle {
        sprite: SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(27),
            texture_atlas: atlas.0.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        },
        ..default()
    });
}

fn camera_follow(
    mut set: ParamSet<(
        Query<&Transform, With<Player>>,
        Query<&mut Transform, With<GameCamera>>,
    )>,
) {
    // TODO: do not use ParamSet: https://johanhelsing.studio/posts/extreme-bevy-2
    let trans = set.p0().get_single().unwrap().translation;
    for mut c_transform in set.p1().iter_mut() {
        c_transform.translation.x = trans.x;
        c_transform.translation.y = trans.y;
    }
}

fn check_death(mut commands: Commands, q: Query<(Entity, &Hp, Option<&Player>)>) {
    for (entity, hp, player) in q.iter() {
        if hp.current <= 0 {
            // HACK: remove
            let _ = hp.base;
            let _ = hp.max;
            println!("{:?}: died", entity);

            // The player is the one who died
            if let Some(_player) = player {
                println!("player died");
            }
            commands.entity(entity).despawn();
        }
    }
}
