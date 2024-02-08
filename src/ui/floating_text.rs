use crate::skills::Damage;
use crate::GRID_SIZE;
use bevy::prelude::*;

#[derive(Component)]
pub struct FloatingDamage {
    pub timer: Timer,
}

#[derive(Bundle)]
pub struct FloatingDamageBundle {
    text: Text2dBundle,
    _floating_damage: FloatingDamage,
}

impl Default for FloatingDamageBundle {
    fn default() -> Self {
        FloatingDamageBundle {
            text: Text2dBundle { ..default() },
            _floating_damage: FloatingDamage {
                timer: Timer::from_seconds(2.0, TimerMode::Once),
            },
        }
    }
}

impl FloatingDamageBundle {
    pub fn new(damage: &Damage, transform: &Transform) -> Self {
        let mut text = Text2dBundle {
            text: Text {
                sections: vec![TextSection::new(damage.damage.to_string(), TextStyle {
                    font_size: 20.0,
                    ..default()
                })],
                ..default()
            },
            ..default()
        };
        text.transform.translation.x = transform.translation.x;
        text.transform.translation.y = transform.translation.y + GRID_SIZE as f32 * 0.75;
        text.transform.translation.z = 2.0;
        FloatingDamageBundle { text, ..default() }
    }
}

pub fn handle_floating_damage_system(
    mut commands: Commands,
    mut q: Query<(Entity, &mut Transform, &mut FloatingDamage)>,
    time: Res<Time>,
) {
    for (id, mut trans, mut floating_damage) in q.iter_mut() {
        if floating_damage.timer.tick(time.delta()).just_finished() {
            commands.entity(id).despawn();
        } else {
            trans.translation.y += time.delta_seconds() * 15.0;
        }
    }
}
