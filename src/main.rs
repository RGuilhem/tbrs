use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("p1".to_string())));
    commands.spawn((Person, Name("p2".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("name: {}", name.0);
    }
}

fn hello_world() {
    println!("hello_world");
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people)
            .add_systems(Update, (hello_world, greet_people));
    }
}

fn main() {
    App::new().add_plugins((DefaultPlugins, HelloPlugin)).run();
}
