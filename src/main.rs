mod movement;
mod physicalentities;

use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: String::from("Simple Bevy Game"),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_startup_entities)
        .add_system(movement::movement)
        .add_system(movement::switch_direction)
        .run();
}

fn spawn_startup_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Res<Windows>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(physicalentities::CubeBundle::new(
        physicalentities::CubeGroup::Player,
        &mut meshes,
        &mut materials,
        window.primary(),
    ));
    commands.spawn_bundle(physicalentities::CubeBundle::new(
        physicalentities::CubeGroup::Enemy,
        &mut meshes,
        &mut materials,
        window.primary(),
    ));
    commands.spawn_bundle(physicalentities::CubeBundle::new(
        physicalentities::CubeGroup::Neutral,
        &mut meshes,
        &mut materials,
        window.primary(),
    ));
    commands.spawn_bundle(physicalentities::BallBundle::new(
        physicalentities::BallGroup::SizeBoost(1.1),
        &mut meshes,
        &mut materials,
        window.primary(),
    ));
    commands.spawn_bundle(physicalentities::BallBundle::new(
        physicalentities::BallGroup::SpeedBoost(1.1),
        &mut meshes,
        &mut materials,
        window.primary(),
    ));
    commands.spawn_bundle(physicalentities::BallBundle::new(
        physicalentities::BallGroup::SizeDecrease(1.1),
        &mut meshes,
        &mut materials,
        window.primary(),
    ));
    commands.spawn_bundle(physicalentities::BallBundle::new(
        physicalentities::BallGroup::SpeedDecrease(1.1),
        &mut meshes,
        &mut materials,
        window.primary(),
    ));
}
