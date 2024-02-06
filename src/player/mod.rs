use crate::GameCamera;
use crate::Sprites;
use crate::GRID_SIZE;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, check_death)
            .add_systems(Update, camera_follow)
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
        }
    }
}

fn setup_player(mut commands: Commands, atlas: Res<Sprites>) {
    commands.spawn(PlayerBundle {
        sprite: SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(27),
            texture_atlas: atlas.0.clone(),
            transform: Transform::from_xyz(GRID_SIZE as f32 * 0.0, 0.0, 0.0),
            ..default()
        },
        ..default()
    });
}

fn player_movement(keys: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Player>>) {
    let mut dir = Vec2::new(0.0, 0.0);
    if keys.pressed(KeyCode::W) {
        dir[1] = 1.0;
    }
    if keys.pressed(KeyCode::S) {
        dir[1] = -1.0;
    }
    if keys.pressed(KeyCode::D) {
        dir[0] = 1.0;
    }
    if keys.pressed(KeyCode::A) {
        dir[0] = -1.0;
    }
    let mut trans = query.single_mut();
    trans.translation[0] += dir[0];
    trans.translation[1] += dir[1];
}

fn camera_follow(
    mut set: ParamSet<(
        Query<&Transform, With<Player>>,
        Query<&mut Transform, With<GameCamera>>,
    )>,
) {
    let mut trans = Transform::default();
    for p_transform in set.p0().iter() {
        //println!("p => {:#?}", p_transform);
        trans.translation = p_transform.translation;
    }

    for mut c_transform in set.p1().iter_mut() {
        //println!("c => {:#?}", c_transform);
        c_transform.translation = trans.translation;
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
