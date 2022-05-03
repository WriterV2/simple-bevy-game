use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Debug, Component, PartialEq, PartialOrd)]
struct Speed(f32);

#[derive(Debug, Component, PartialEq)]
struct Direction(Vec3);

#[derive(Debug, Component, PartialEq, Eq, PartialOrd, Ord)]
enum CubeGroup {
    Player,
    Enemy,
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
        speed: f32,
        direction: Vec3,
        size: f32,
        position: Vec3,
        color: Color,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        CubeBundle {
            group,
            speed: Speed(speed),
            direction: Direction(direction),
            mesh_bundle: MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Cube::default())).into(),
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform::default()
                    .with_scale(Vec3::splat(size))
                    .with_translation(position),
                ..default()
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
        10.0,
        Vec3::ONE,
        30.,
        Vec3::new(100.0, 100.0, 0.1),
        Color::BLACK,
        &mut meshes,
        &mut materials,
    ));
    commands.spawn_bundle(CubeBundle::new(
        CubeGroup::Enemy,
        10.0,
        Vec3::ONE,
        30.,
        Vec3::new(100.0, 500.0, 0.1),
        Color::RED,
        &mut meshes,
        &mut materials,
    ));
}
