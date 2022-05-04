use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Debug, Component, PartialEq, PartialOrd)]
struct Speed(f32);

#[derive(Debug, Component, PartialEq)]
struct Direction(Vec3);

#[derive(Debug, Component, PartialEq, Eq, PartialOrd, Ord)]
enum CubeGroup {
    Player,
    Enemy,
    Neutral,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: String::from("Simple Bevy Game"),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_startup_entities)
        .run();
}

#[derive(Bundle)]
struct CubeBundle {
    group: CubeGroup,
    speed: Speed,
    direction: Direction,
    #[bundle]
    mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
}

impl CubeBundle {
    fn new(
        group: CubeGroup,
        position: Vec3,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        match group {
            CubeGroup::Player => CubeBundle {
                group,
                speed: Speed(10.),
                direction: Direction(Vec3::X),
                mesh_bundle: MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube::default())).into(),
                    material: materials.add(ColorMaterial::from(Color::BLACK)),
                    transform: Transform::default()
                        .with_scale(Vec3::splat(30.))
                        .with_translation(position),
                    ..default()
                },
            },
            CubeGroup::Enemy => CubeBundle {
                group,
                speed: Speed(10.),
                direction: Direction(Vec3::Y),
                mesh_bundle: MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube::default())).into(),
                    material: materials.add(ColorMaterial::from(Color::RED)),
                    transform: Transform::default()
                        .with_scale(Vec3::splat(30.))
                        .with_translation(position),
                    ..default()
                },
            },
            CubeGroup::Neutral => CubeBundle {
                group,
                speed: Speed(5.),
                direction: Direction(Vec3::X),
                mesh_bundle: MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube::default())).into(),
                    material: materials.add(ColorMaterial::from(Color::YELLOW)),
                    transform: Transform::default()
                        .with_scale(Vec3::splat(20.))
                        .with_translation(position),
                    ..default()
                },
            },
        }
    }
}

fn spawn_startup_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(CubeBundle::new(
        CubeGroup::Player,
        Vec3::new(100., 100., 0.9),
        &mut meshes,
        &mut materials,
    ));
    commands.spawn_bundle(CubeBundle::new(
        CubeGroup::Enemy,
        Vec3::new(100., 500., 0.9),
        &mut meshes,
        &mut materials,
    ));
    commands.spawn_bundle(CubeBundle::new(
        CubeGroup::Neutral,
        Vec3::new(20., 300., 0.1),
        &mut meshes,
        &mut materials,
    ));
}
