use crate::game_world::MapCellBundle;
use crate::sprites::sprite_index;
use crate::sprites::Sprites;
use bevy::prelude::*;

pub fn setup_hub(mut commands: Commands, atlas: Res<Sprites>) {
    commands.spawn(MapCellBundle::new(1, 1, sprite_index(0, 1), atlas));
}
