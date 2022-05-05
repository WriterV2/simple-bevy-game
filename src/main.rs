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

#[derive(Debug, Component, PartialEq, Eq, PartialOrd, Ord)]
enum BoosterGroup {
    Speed,
    Size,
}

#[derive(Debug, Component, PartialEq, Eq, PartialOrd, Ord)]
enum DecreaseGroup {
    Speed,
    Size,
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

fn get_starting_position(group: &CubeGroup, window: &Window) -> Vec3 {
    let mut rng = rand::thread_rng();
    match group {
        CubeGroup::Player => Vec3::new(0., -window.height() / 3., 100.0),
        CubeGroup::Enemy => Vec3::new(0., window.height() / 3., 99.0),
        CubeGroup::Neutral => Vec3::new(
            rng.gen_range((-window.width() / 2.)..window.width() / 2.),
            rng.gen_range((-window.height() / 2.)..window.height() / 2.),
            98.0,
        ),
    }
}

fn get_starting_size(group: &CubeGroup, window: &Window) -> f32 {
    let longer_side = window.width().max(window.height());
    match group {
        CubeGroup::Player => longer_side / 20.,
        CubeGroup::Enemy => longer_side / 15.,
        CubeGroup::Neutral => longer_side / 50.,
    }
}

fn get_starting_speed(group: &CubeGroup, window: &Window) -> f32 {
    match group {
        CubeGroup::Player => 10.,
        CubeGroup::Enemy => 8.,
        CubeGroup::Neutral => 8.,
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

impl CubeBundle {
    fn new(
        group: CubeGroup,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        window: &Window,
    ) -> Self {
        match group {
            CubeGroup::Player => CubeBundle {
                mesh_bundle: MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube::default())).into(),
                    material: materials.add(ColorMaterial::from(Color::BLACK)),
                    transform: Transform::default()
                        .with_scale(Vec3::splat(get_starting_size(&group, window)))
                        .with_translation(get_starting_position(&group, window)),
                    ..default()
                },

                speed: Speed(get_starting_speed(&group, window)),
                direction: Direction(Vec3::X),
                group,
            },
            CubeGroup::Enemy => CubeBundle {
                mesh_bundle: MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube::default())).into(),
                    material: materials.add(ColorMaterial::from(Color::RED)),
                    transform: Transform::default()
                        .with_scale(Vec3::splat(get_starting_size(&group, window)))
                        .with_translation(get_starting_position(&group, window)),
                    ..default()
                },

                speed: Speed(get_starting_speed(&group, window)),
                direction: Direction(Vec3::Y),
                group,
            },
            CubeGroup::Neutral => CubeBundle {
                mesh_bundle: MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube::default())).into(),
                    material: materials.add(ColorMaterial::from(Color::YELLOW)),
                    transform: Transform::default()
                        .with_scale(Vec3::splat(get_starting_size(&group, window)))
                        .with_translation(get_starting_position(&group, window)),
                    ..default()
                },

                speed: Speed(get_starting_speed(&group, window)),
                direction: Direction(Vec3::X),
                group,
            },
        }
    }
}

#[derive(Bundle)]
struct BoosterCubeBundle {
    #[bundle]
    cube: CubeBundle,
    booster: BoosterGroup,
}

impl BoosterCubeBundle {
    fn new(
        group: BoosterGroup,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        window: &Window,
    ) -> Self {
        BoosterCubeBundle {
            cube: CubeBundle::new(CubeGroup::Neutral, meshes, materials, window),
            booster: group,
        }
    }
}

#[derive(Bundle)]
struct DecreaseCubeBundle {
    #[bundle]
    cube: CubeBundle,
    decrease: DecreaseGroup,
}

impl DecreaseCubeBundle {
    fn new(
        group: DecreaseGroup,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        window: &Window,
    ) -> Self {
        DecreaseCubeBundle {
            cube: CubeBundle::new(CubeGroup::Neutral, meshes, materials, window),
            decrease: group,
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
    commands.spawn_bundle(BoosterCubeBundle::new(
        BoosterGroup::Speed,
        &mut meshes,
        &mut materials,
        window.primary(),
    ));

    commands.spawn_bundle(BoosterCubeBundle::new(
        BoosterGroup::Size,
        &mut meshes,
        &mut materials,
        window.primary(),
    ));

    commands.spawn_bundle(DecreaseCubeBundle::new(
        DecreaseGroup::Size,
        &mut meshes,
        &mut materials,
        window.primary(),
    ));

    commands.spawn_bundle(DecreaseCubeBundle::new(
        DecreaseGroup::Speed,
        &mut meshes,
        &mut materials,
        window.primary(),
    ));
}
