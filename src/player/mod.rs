use crate::player::inventory::Inventory;
use crate::alive::spawn_names_system;
use crate::alive::AliveBundle;
use crate::movements::apply_movements;
use crate::movements::initiate_movements;
use crate::player::input::change_target_system;
use crate::player::input::click_target_system;
use crate::player::input::player_movement;
use crate::player::input::setup_target_system;
use crate::player::input::target_border_system;
use crate::player::input::update_target_system;
use crate::skills::auto_attack::AttackStyle;
use crate::GameCamera;
use crate::Sprites;
use bevy::prelude::*;

pub mod input;
pub mod inventory;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, camera_follow)
            .add_systems(Update, initiate_movements)
            .add_systems(Update, apply_movements)
            .add_systems(Update, player_movement)
            .add_systems(Update, spawn_names_system)
            .add_systems(Update, change_target_system)
            .add_systems(Startup, setup_target_system)
            .add_systems(Update, update_target_system)
            .add_systems(Update, target_border_system)
            .add_systems(Update, click_target_system);
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    alive_bundle: AliveBundle,
    inventory: Inventory,
    _player: Player,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        PlayerBundle {
            alive_bundle: AliveBundle::default(),
            inventory: Inventory::new(),
            _player: Player,
        }
    }
}

fn setup_player(mut commands: Commands, atlas: Res<Sprites>) {
    let mut sprite = TextureAtlasSprite::new(27);
    sprite.color = Color::RED;
    let trans = Transform::from_xyz(0.0, 0.0, 1.0);
    let mut alive_bundle = AliveBundle::with_sprite(sprite, &atlas, trans);
    alive_bundle.name.0 = "John".to_string();
    alive_bundle.auto_attack.auto_attack.style = AttackStyle::Range;

    commands.spawn(PlayerBundle {
        alive_bundle,
        ..default()
    });
}

fn camera_follow(
    mut set: ParamSet<(
        Query<&Transform, With<Player>>,
        Query<&mut Transform, With<GameCamera>>,
    )>,
) {
    let trans = set.p0().get_single().unwrap().translation;
    for mut c_transform in set.p1().iter_mut() {
        c_transform.translation.x = trans.x;
        c_transform.translation.y = trans.y;
    }
}
