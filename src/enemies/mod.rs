use crate::alive::AliveBundle;
use crate::sprites::sprite_index;
use crate::sprites::transform_from_grid;
use crate::Sprites;
use bevy::prelude::*;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_enemies);
    }
}

fn setup_enemies(mut commands: Commands, atlas: Res<Sprites>) {
    let mut sprite = TextureAtlasSprite::new(sprite_index(25, 2));
    sprite.color = Color::DARK_GREEN;
    let trans = transform_from_grid(2, 1, 1);
    let mut alive_bundle = AliveBundle::with_sprite(sprite, &atlas, trans);
    alive_bundle.name.0 = "Skeleton".to_string();
    commands.spawn(EnemyBundle {
        alive_bundle,
        ..default()
    });
    let mut sprite = TextureAtlasSprite::new(sprite_index(25, 2));
    sprite.color = Color::DARK_GREEN;
    let trans = transform_from_grid(2, 2, 1);
    let mut alive_bundle = AliveBundle::with_sprite(sprite, &atlas, trans);
    alive_bundle.name.0 = "Skeleton".to_string();
    commands.spawn(EnemyBundle {
        alive_bundle,
        ..default()
    });
}

#[derive(Component)]
pub struct Enemy;

#[derive(Bundle)]
pub struct EnemyBundle {
    pub alive_bundle: AliveBundle,
    pub _enemy: Enemy,
}

impl Default for EnemyBundle {
    fn default() -> Self {
        EnemyBundle {
            alive_bundle: AliveBundle::default(),
            _enemy: Enemy,
        }
    }
}
