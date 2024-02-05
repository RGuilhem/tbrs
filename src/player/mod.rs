use crate::Sprites;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, check_death);
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
            sprite: SpriteSheetBundle {
                ..default()
            },
        }
    }
}

fn setup_player(mut commands: Commands, atlas: Res<Sprites>) {
    commands.spawn(PlayerBundle {
        sprite: SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(27),
            texture_atlas: atlas.0.clone(),
            ..default()
        },
        ..default()
    });
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
