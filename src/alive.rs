use crate::game_world::Collider;
use crate::movements::GridPos;
use crate::movements::MovementBundle;
use crate::Sprites;
use crate::GRID_SIZE;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Hp {
    pub base: u32,
    pub current: u32,
    pub max: u32,
}

#[derive(Component)]
pub struct Alive;

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct FloatingName;

#[derive(Component, Debug)]
pub struct Target {
    pub entity: Option<Entity>,
    pub pos: Option<GridPos>,
}

#[derive(Bundle)]
pub struct AliveBundle {
    pub hp: Hp,
    pub name: Name,
    pub movement: MovementBundle,
    pub sprite: SpriteSheetBundle,
    pub target: Target,
    pub _collider: Collider,
    pub _alive: Alive,
}

impl Default for AliveBundle {
    fn default() -> Self {
        AliveBundle {
            hp: Hp {
                base: 100,
                current: 100,
                max: 100,
            },
            name: Name("NO_NAME".to_string()),
            sprite: SpriteSheetBundle { ..default() },
            movement: MovementBundle::default(),
            target: Target {
                entity: None,
                pos: None,
            },
            _collider: Collider,
            _alive: Alive,
        }
    }
}

impl AliveBundle {
    pub fn with_sprite(
        sprite: TextureAtlasSprite,
        atlas: &Res<Sprites>,
        transform: Transform,
    ) -> Self {
        let t = transform.translation;
        AliveBundle {
            sprite: SpriteSheetBundle {
                sprite,
                texture_atlas: atlas.0.clone(),
                transform,
                ..default()
            },
            movement: MovementBundle {
                grid_pos: GridPos(IVec2::new(
                    t.x as i32 / GRID_SIZE as i32,
                    t.y as i32 / GRID_SIZE as i32,
                )),
                ..default()
            },
            ..default()
        }
    }
}

pub fn spawn_names_system(
    mut commands: Commands,
    q: Query<(Entity, &Name), Without<FloatingName>>,
) {
    for (entity, name) in q.iter() {
        let style = TextStyle {
            font_size: 11.0,
            ..default()
        };
        let text = &name.0;
        info!("floating name: {:?}", text);
        let id = commands
            .spawn((
                Text2dBundle {
                    text: Text {
                        sections: vec![TextSection::new(text, style)],
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, GRID_SIZE as f32 / 2.0 + 5.0, 0.0),
                    ..default()
                },
                FloatingName,
            ))
            .id();
        commands.entity(entity).insert(FloatingName);
        commands.entity(entity).add_child(id);
    }
}
