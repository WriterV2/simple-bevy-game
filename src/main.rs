use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: String::from("Simple Bevy Game"),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .run();
}
