use crate::player::PlayerPlugin;
use crate::ui::UiPlugin;
use bevy::prelude::*;

pub mod player;
pub mod ui;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TbrsPlugin))
        .add_systems(Startup,  setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

pub struct TbrsPlugin;

impl Plugin for TbrsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerPlugin).add_plugins(UiPlugin);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

}
