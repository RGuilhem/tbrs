pub mod ai;
pub mod spawner;

use crate::alive::AliveBundle;
use crate::enemies::ai::ai_system;
use crate::enemies::spawner::Spawner;
use crate::movements::GridPos;
use crate::skills::auto_attack::AttackStyle;
use crate::sprites::sprite_index;
use crate::sprites::transform_from_grid;
use crate::Sprites;
use bevy::prelude::*;

use self::ai::Ai;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_enemies)
            .add_systems(Update, enemies_spawn_system)
            .add_systems(Update, ai_system);
    }
}

fn setup_enemies(commands: Commands) {
    let spawner = Spawner::new(GridPos(IVec2::splat(5)));
    spawner.spawn_all_of(commands, SpawnEnemy::goblin);
}

#[derive(Component)]
pub struct Enemy;

#[derive(Bundle)]
pub struct EnemyBundle {
    pub alive_bundle: AliveBundle,
    pub _enemy: Enemy,
    pub _ai: Ai,
}

impl Default for EnemyBundle {
    fn default() -> Self {
        EnemyBundle {
            alive_bundle: AliveBundle::default(),
            _enemy: Enemy,
            _ai: Ai,
        }
    }
}

#[derive(Component)]
pub struct SpawnEnemy {
    pub name: String,
    pub sprite: TextureAtlasSprite,
    pub transform: Transform,
    pub attack_style: AttackStyle,
}

impl SpawnEnemy {
    fn goblin(x: i32, y: i32) -> Self {
        let mut sprite = TextureAtlasSprite::new(sprite_index(25, 2));
        sprite.color = Color::DARK_GREEN;
        SpawnEnemy {
            name: "Goblin".to_string(),
            sprite,
            transform: transform_from_grid(x, y, 1),
            attack_style: AttackStyle::Melee,
        }
    }
}

fn enemies_spawn_system(
    mut commands: Commands,
    q: Query<(Entity, &SpawnEnemy)>,
    atlas: Res<Sprites>,
) {
    for (id, spawn) in q.iter() {
        let mut alive_bundle =
            AliveBundle::with_sprite(spawn.sprite.clone(), &atlas, spawn.transform);
        alive_bundle.name.0 = spawn.name.clone();
        alive_bundle.auto_attack.auto_attack.style = spawn.attack_style;
        commands.spawn(EnemyBundle {
            alive_bundle,
            ..default()
        });
        commands.entity(id).despawn();
    }
}
