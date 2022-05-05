use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

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

#[derive(Debug, Component, PartialEq, PartialOrd)]
enum BallGroup {
    SpeedBoost(f32),
    SizeBoost(f32),
    SpeedDecrease(f32),
    SizeDecrease(f32),
}

#[derive(Debug, PartialEq, PartialOrd)]
enum PhysicalObject<'a> {
    Cube(&'a CubeGroup),
    Ball(&'a BallGroup),
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

fn get_starting_position(group: PhysicalObject, window: &Window) -> Vec3 {
    let mut rng = rand::thread_rng();
    match group {
        PhysicalObject::Cube(CubeGroup::Player) => Vec3::new(0., -window.height() / 3., 100.0),
        PhysicalObject::Cube(CubeGroup::Enemy) => Vec3::new(0., window.height() / 3., 99.0),
        _ => Vec3::new(
            rng.gen_range((-window.width() / 2.)..window.width() / 2.),
            rng.gen_range((-window.height() / 2.)..window.height() / 2.),
            98.0,
        ),
    }
}

fn get_starting_size(group: PhysicalObject, window: &Window) -> f32 {
    let longer_side = window.width().max(window.height());
    match group {
        PhysicalObject::Cube(CubeGroup::Player) => longer_side / 20.,
        PhysicalObject::Cube(CubeGroup::Enemy) => longer_side / 15.,
        PhysicalObject::Cube(CubeGroup::Neutral) => longer_side / 50.,
        PhysicalObject::Ball(_any) => longer_side / 70.,
    }
}

fn get_starting_speed(group: PhysicalObject) -> f32 {
    match group {
        PhysicalObject::Cube(CubeGroup::Player) => 10.,
        PhysicalObject::Cube(CubeGroup::Enemy) => 8.,
        PhysicalObject::Cube(CubeGroup::Neutral) => 8.,
        PhysicalObject::Ball(_any) => 0.,
    }
}

#[derive(Bundle)]
struct CubeBundle {
    group: CubeGroup,
    speed: Speed,
    direction: Direction,
    #[bundle]
    mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
}

#[derive(Bundle)]
struct BallBundle {
    group: BallGroup,
    #[bundle]
    mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
}

impl CubeBundle {
    fn new(
        group: CubeGroup,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        window: &Window,
    ) -> Self {
        let color = match group {
            CubeGroup::Player => Color::BLACK,
            CubeGroup::Enemy => Color::RED,
            CubeGroup::Neutral => Color::ORANGE,
        };

        CubeBundle {
            speed: Speed(get_starting_speed(PhysicalObject::Cube(&group))),
            direction: Direction(Vec3::Y),
            mesh_bundle: MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Cube::default())).into(),
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform::default()
                    .with_scale(Vec3::splat(get_starting_size(
                        PhysicalObject::Cube(&group),
                        window,
                    )))
                    .with_translation(get_starting_position(PhysicalObject::Cube(&group), window)),
                ..default()
            },
            group,
        }
    }
}

impl BallBundle {
    fn new(
        group: BallGroup,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        window: &Window,
    ) -> Self {
        BallBundle {
            mesh_bundle: MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere::default())).into(),
                material: materials.add(ColorMaterial::from(Color::YELLOW_GREEN)),
                transform: Transform::default()
                    .with_scale(Vec3::splat(get_starting_size(
                        PhysicalObject::Ball(&group),
                        window,
                    )))
                    .with_translation(get_starting_position(PhysicalObject::Ball(&group), window)),
                ..default()
            },
            group,
        }
    }
}

fn spawn_startup_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Res<Windows>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(CubeBundle::new(
        CubeGroup::Player,
        &mut meshes,
        &mut materials,
        window.primary(),
    ));
    commands.spawn_bundle(CubeBundle::new(
        CubeGroup::Enemy,
        &mut meshes,
        &mut materials,
        window.primary(),
    ));
    commands.spawn_bundle(CubeBundle::new(
        CubeGroup::Neutral,
        &mut meshes,
        &mut materials,
        window.primary(),
    ));
    commands.spawn_bundle(BallBundle::new(
        BallGroup::SizeBoost(1.1),
        &mut meshes,
        &mut materials,
        window.primary(),
    ));
    commands.spawn_bundle(BallBundle::new(
        BallGroup::SpeedBoost(1.1),
        &mut meshes,
        &mut materials,
        window.primary(),
    ));
    commands.spawn_bundle(BallBundle::new(
        BallGroup::SizeDecrease(1.1),
        &mut meshes,
        &mut materials,
        window.primary(),
    ));
    commands.spawn_bundle(BallBundle::new(
        BallGroup::SpeedDecrease(1.1),
        &mut meshes,
        &mut materials,
        window.primary(),
    ));
}
