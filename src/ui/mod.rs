use crate::player::Hp;
use crate::player::Player;
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, hp_ui_system);
    }
}

#[derive(Component)]
pub struct UiCamera;

fn hp_ui_system(hp: Query<&Hp, With<Player>>) {
    if let Ok(_hp) = hp.get_single() {
        //println!("hp: {:#?}", hp);
    }
}
