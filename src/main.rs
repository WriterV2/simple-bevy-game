mod movement;
mod physicalentities;

use bevy::prelude::*;
use physicalentities::Cube;
use rand::Rng;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: String::from("Simple Bevy Game"),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_startup_entities)
        .add_plugin(movement::Movement)
        //.add_system(movement::switch_direction)
        .add_system(physicalentities::spawn_balls)
        .run();
}

fn spawn_startup_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Res<Windows>,
) {
    let player = physicalentities::Player;
    let enemy = physicalentities::Enemy;

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    player.spawn(&mut commands, &mut meshes, &mut materials, window.primary());
    enemy.spawn(&mut commands, &mut meshes, &mut materials, window.primary());
    physicalentities::Neutral::spawn_neutral_cubes(
        &mut commands,
        &mut meshes,
        &mut materials,
        window.primary(),
        rand::thread_rng().gen_range(10..20),
    );
}
