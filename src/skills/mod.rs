use crate::alive::Hp;
use crate::alive::Target;
use crate::skills::auto_attack::auto_attack_system;
use crate::ui::floating_text::FloatingDamageBundle;
use bevy::prelude::*;

pub mod auto_attack;

pub struct SkillsPlugin;

impl Plugin for SkillsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, auto_attack_system)
            .add_systems(Update, apply_damage_system)
            .add_systems(PreUpdate, check_death_system);
    }
}

#[derive(Component)]
pub struct Damage {
    pub damage: i32,
}

fn check_death_system(
    mut commands: Commands,
    hp_query: Query<(Entity, &Hp), Changed<Hp>>,
    mut target_query: Query<(Entity, &mut Target)>,
) {
    for (id, hp) in hp_query.iter() {
        if hp.current <= 0 {
            info!("Died");
            for (_id_attacker, mut target) in target_query.iter_mut() {
                // can use id_attacker for
                // reward
                if target.entity == Some(id) {
                    target.entity = None;
                    target.pos = None;
                }
            }
            commands.entity(id).despawn_recursive();
        }
    }
}

fn apply_damage_system(mut commands: Commands, mut q: Query<(Entity, &Damage, &mut Hp, &Transform)>) {
    for (id, damage, mut hp, transform) in q.iter_mut() {
        hp.current -= damage.damage;
        commands.entity(id).remove::<Damage>();
        commands.spawn(FloatingDamageBundle::new(damage, transform));
    }
}
