use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::{thread_rng, Rng};

// speed of physical object
#[derive(Debug, Component, PartialEq, PartialOrd)]
pub struct Speed(pub f32);

// direction physical object moves
#[derive(Debug, Component, PartialEq)]
pub struct Direction(pub Vec3);

// type of cube - player and enemy must conquer neutral cubes to win
#[derive(Debug, Component, PartialEq, Eq, PartialOrd, Ord)]
pub enum CubeGroup {
    Player,
    Enemy,
    Neutral,
}

// type of ball player and enemy can consume
#[derive(Debug, Component, PartialEq, PartialOrd)]
pub enum BallGroup {
    SpeedBoost(f32),
    SizeBoost(f32),
    SpeedDecrease(f32),
    SizeDecrease(f32),
}

// every in-game object that has a position
#[derive(Debug, PartialEq, PartialOrd)]
enum PhysicalEntity<'a> {
    Cube(&'a CubeGroup),
    Ball(&'a BallGroup),
}

// helper to get starting position based on physical object's group
fn get_starting_position(group: PhysicalEntity, window: &Window) -> Vec3 {
    let mut rng = thread_rng();
    match group {
        PhysicalEntity::Cube(CubeGroup::Player) => Vec3::new(0., -window.height() / 3., 100.0),
        PhysicalEntity::Cube(CubeGroup::Enemy) => Vec3::new(0., window.height() / 3., 99.0),
        _ => Vec3::new(
            rng.gen_range((-window.width() / 2.)..window.width() / 2.),
            rng.gen_range((-window.height() / 2.)..window.height() / 2.),
            98.0,
        ),
    }
}

// helper to get starting size based on physical object's group
fn get_starting_size(group: PhysicalEntity, window: &Window) -> f32 {
    let longer_side = window.width().max(window.height());
    match group {
        PhysicalEntity::Cube(CubeGroup::Player) => longer_side / 20.,
        PhysicalEntity::Cube(CubeGroup::Enemy) => longer_side / 15.,
        PhysicalEntity::Cube(CubeGroup::Neutral) => longer_side / 50.,
        PhysicalEntity::Ball(_any) => longer_side / 70.,
    }
}

// helper to get starting speed based on physical object's group
fn get_starting_speed(group: PhysicalEntity) -> f32 {
    match group {
        PhysicalEntity::Cube(CubeGroup::Player) => 5.,
        PhysicalEntity::Cube(CubeGroup::Enemy) => 4.,
        PhysicalEntity::Cube(CubeGroup::Neutral) => 4.,
        PhysicalEntity::Ball(_any) => 0.,
    }
}

// helper to get starting speed based on physical object's group
fn get_starting_color(group: PhysicalEntity) -> Color {
    match group {
        PhysicalEntity::Cube(CubeGroup::Player) => Color::PURPLE,
        PhysicalEntity::Cube(CubeGroup::Enemy) => Color::GOLD,
        PhysicalEntity::Cube(CubeGroup::Neutral) => Color::DARK_GRAY,
        PhysicalEntity::Ball(BallGroup::SizeBoost(_) | BallGroup::SpeedBoost(_)) => {
            Color::LIME_GREEN
        }
        PhysicalEntity::Ball(BallGroup::SizeDecrease(_) | BallGroup::SpeedDecrease(_)) => {
            Color::RED
        }
    }
}

// bundle for cube entity
#[derive(Bundle)]
pub struct CubeBundle {
    group: CubeGroup,
    speed: Speed,
    direction: Direction,
    #[bundle]
    mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
}

// bundle for ball entity
#[derive(Bundle)]
pub struct BallBundle {
    group: BallGroup,
    #[bundle]
    mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
}

impl CubeBundle {
    // constructor for a cube
    pub fn new(
        group: CubeGroup,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        window: &Window,
    ) -> Self {
        CubeBundle {
            speed: Speed(get_starting_speed(PhysicalEntity::Cube(&group))),
            direction: Direction(Vec3::Y),
            mesh_bundle: MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Cube::default())).into(),
                material: materials.add(ColorMaterial::from(get_starting_color(
                    PhysicalEntity::Cube(&group),
                ))),
                transform: Transform::default()
                    .with_scale(Vec3::splat(get_starting_size(
                        PhysicalEntity::Cube(&group),
                        window,
                    )))
                    .with_translation(get_starting_position(PhysicalEntity::Cube(&group), window)),
                ..default()
            },
            group,
        }
    }
}

impl BallBundle {
    // constructor for a ball
    pub fn new(
        group: BallGroup,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        window: &Window,
    ) -> Self {
        BallBundle {
            mesh_bundle: MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere::default())).into(),
                material: materials.add(ColorMaterial::from(get_starting_color(
                    PhysicalEntity::Ball(&group),
                ))),
                transform: Transform::default()
                    .with_scale(Vec3::splat(get_starting_size(
                        PhysicalEntity::Ball(&group),
                        window,
                    )))
                    .with_translation(get_starting_position(PhysicalEntity::Ball(&group), window)),
                ..default()
            },
            group,
        }
    }
}
