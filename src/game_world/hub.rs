use crate::sprites::sprite_index;
use crate::sprites::transform_from_grid;
use crate::sprites::Sprites;
use bevy::prelude::*;

pub fn setup_hub(mut commands: Commands, atlas: Res<Sprites>) {
    let mut sprite = TextureAtlasSprite::new(sprite_index(0, 1));
    sprite.color = Color::rgb(0.1, 0.75, 0.1);
    commands.spawn(SpriteSheetBundle {
        sprite,
        texture_atlas: atlas.0.clone(),
        transform: transform_from_grid(1, 1, 0),
        ..default()
    });
}
