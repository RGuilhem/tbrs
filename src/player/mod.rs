use crate::alive::AliveBundle;
use crate::movements::apply_movements;
use crate::movements::initiate_movements;
use crate::player::input::player_movement;
use crate::GameCamera;
use crate::Sprites;
use crate::GRID_SIZE;
use bevy::prelude::*;

pub mod input;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, camera_follow)
            .add_systems(Update, initiate_movements)
            .add_systems(Update, apply_movements)
            .add_systems(Update, player_movement);
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    alive_bundle: AliveBundle,
    _player: Player,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        PlayerBundle {
            alive_bundle: AliveBundle::default(),
            _player: Player,
        }
    }
}

fn setup_player(mut commands: Commands, atlas: Res<Sprites>) {
    let mut sprite = TextureAtlasSprite::new(27);
    sprite.color = Color::RED;
    let trans = Transform::from_xyz(0.0, 0.0, 1.0);
    let mut alive_bundle = AliveBundle::with_sprite(sprite, &atlas, trans);
    alive_bundle.name.0 = "Player".to_string();

    let style = TextStyle {
        font_size: 11.0,
        ..default()
    };
    commands
        .spawn(PlayerBundle {
            alive_bundle,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(Text2dBundle {
                text: Text {
                    sections: vec![TextSection::new("Player", style)],
                    ..default()
                },
                transform: Transform::from_xyz(0.0, GRID_SIZE as f32 / 2.0 + 5.0, 0.0),
                ..default()
            });
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
