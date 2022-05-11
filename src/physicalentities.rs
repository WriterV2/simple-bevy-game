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

// player cube component
#[derive(Debug, Component)]
pub struct Player;

// enemy cube component
#[derive(Debug, Component)]
pub struct Enemy;

// neutral cube component
#[derive(Debug, Component, Clone)]
pub struct Neutral;

// speed boost ball component
#[derive(Debug, Component, Clone)]
pub struct SpeedBoost(pub f32);

// size boost ball component
#[derive(Debug, Component, Clone)]
pub struct SizeBoost(pub f32);

// speed decrease ball component
#[derive(Debug, Component, Clone)]
pub struct SpeedDecrease(pub f32);

// size decrease ball component
#[derive(Debug, Component, Clone)]
pub struct SizeDecrease(pub f32);

// moving entitiy with cube form
pub trait Cube: GameEntity {
    fn spawn(
        self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        window: &Window,
    ) {
        commands
            .spawn()
            .insert(self)
            .insert(Direction(Vec3::Y))
            .insert(Speed(Self::set_starting_speed()))
            .insert_bundle(MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Cube::default())).into(),
                material: materials.add(ColorMaterial::from(Self::set_starting_color())),
                transform: Transform::default()
                    .with_scale(Vec3::splat(Self::set_starting_size(window)))
                    .with_translation(Self::set_starting_position(window)),
                ..default()
            });
    }

    fn set_starting_speed() -> f32;
}

// boost/decrease entity with a ball form
pub trait Ball: GameEntity {
    fn spawn(
        self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        window: &Window,
    ) {
        commands
            .spawn()
            .insert(self)
            .insert_bundle(MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Cube::default())).into(),
                material: materials.add(ColorMaterial::from(Self::set_starting_color())),
                transform: Transform::default()
                    .with_scale(Vec3::splat(Self::set_starting_size(window)))
                    .with_translation(Self::set_starting_position(window)),
                ..default()
            });
    }
}

// every in-game entity
pub trait GameEntity: Component + Sized {
    fn set_starting_position(window: &Window) -> Vec3;

    fn set_starting_size(window: &Window) -> f32;

    fn set_starting_color() -> Color;
}

impl GameEntity for Player {
    fn set_starting_position(window: &Window) -> Vec3 {
        Vec3::new(0., -window.height() / 3., 100.0)
    }

    fn set_starting_size(window: &Window) -> f32 {
        window.width().max(window.height()) / 20.
    }

    fn set_starting_color() -> Color {
        Color::PURPLE
    }
}

impl GameEntity for Enemy {
    fn set_starting_position(window: &Window) -> Vec3 {
        Vec3::new(0., window.height() / 3., 99.0)
    }

    fn set_starting_size(window: &Window) -> f32 {
        window.width().max(window.height()) / 15.
    }

    fn set_starting_color() -> Color {
        Color::GOLD
    }
}

impl GameEntity for Neutral {
    fn set_starting_position(window: &Window) -> Vec3 {
        Vec3::new(
            rand::thread_rng().gen_range((-window.width() / 2.)..window.width() / 2.),
            rand::thread_rng().gen_range((-window.height() / 2.)..window.height() / 2.),
            98.0,
        )
    }

    fn set_starting_size(window: &Window) -> f32 {
        window.width().max(window.height()) / 50.
    }

    fn set_starting_color() -> Color {
        Color::DARK_GRAY
    }
}

impl GameEntity for SpeedBoost {
    fn set_starting_position(window: &Window) -> Vec3 {
        Vec3::new(
            rand::thread_rng().gen_range((-window.width() / 2.)..window.width() / 2.),
            rand::thread_rng().gen_range((-window.height() / 2.)..window.height() / 2.),
            98.0,
        )
    }

    fn set_starting_size(window: &Window) -> f32 {
        window.width().max(window.height()) / 70.
    }

    fn set_starting_color() -> Color {
        Color::LIME_GREEN
    }
}

impl GameEntity for SizeBoost {
    fn set_starting_position(window: &Window) -> Vec3 {
        Vec3::new(
            rand::thread_rng().gen_range((-window.width() / 2.)..window.width() / 2.),
            rand::thread_rng().gen_range((-window.height() / 2.)..window.height() / 2.),
            98.0,
        )
    }

    fn set_starting_size(window: &Window) -> f32 {
        window.width().max(window.height()) / 70.
    }

    fn set_starting_color() -> Color {
        Color::LIME_GREEN
    }
}

impl GameEntity for SpeedDecrease {
    fn set_starting_position(window: &Window) -> Vec3 {
        Vec3::new(
            rand::thread_rng().gen_range((-window.width() / 2.)..window.width() / 2.),
            rand::thread_rng().gen_range((-window.height() / 2.)..window.height() / 2.),
            98.0,
        )
    }

    fn set_starting_size(window: &Window) -> f32 {
        window.width().max(window.height()) / 70.
    }

    fn set_starting_color() -> Color {
        Color::RED
    }
}

impl GameEntity for SizeDecrease {
    fn set_starting_position(window: &Window) -> Vec3 {
        Vec3::new(
            rand::thread_rng().gen_range((-window.width() / 2.)..window.width() / 2.),
            rand::thread_rng().gen_range((-window.height() / 2.)..window.height() / 2.),
            98.0,
        )
    }

    fn set_starting_size(window: &Window) -> f32 {
        window.width().max(window.height()) / 70.
    }

    fn set_starting_color() -> Color {
        Color::RED
    }
}

impl Cube for Player {
    fn set_starting_speed() -> f32 {
        100.
    }
}

impl Cube for Enemy {
    fn set_starting_speed() -> f32 {
        90.
    }
}

impl Cube for Neutral {
    fn set_starting_speed() -> f32 {
        90.
    }
}

impl Ball for SpeedBoost {}

impl Ball for SizeBoost {}

impl Ball for SpeedDecrease {}

impl Ball for SizeDecrease {}

impl Neutral {
    pub fn spawn_neutral_cubes(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        window: &Window,
        number_of_cubes: i32,
    ) {
        let neutral_cubes = vec![Self; number_of_cubes as usize];
        for cube in neutral_cubes {
            cube.spawn(commands, meshes, materials, window)
        }
    }
}

impl Default for SpeedBoost {
    fn default() -> Self {
        Self(rand::thread_rng().gen_range(1.1..=2.))
    }
}

impl Default for SizeBoost {
    fn default() -> Self {
        Self(rand::thread_rng().gen_range(1.1..=2.))
    }
}

impl Default for SpeedDecrease {
    fn default() -> Self {
        Self(rand::thread_rng().gen_range(1.1..=2.))
    }
}

impl Default for SizeDecrease {
    fn default() -> Self {
        Self(rand::thread_rng().gen_range(1.1..=2.))
    }
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
        PhysicalEntity::Cube(CubeGroup::Player) => 100.,
        PhysicalEntity::Cube(CubeGroup::Enemy) => 90.,
        PhysicalEntity::Cube(CubeGroup::Neutral) => 90.,
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

    pub fn spawn_neutral_cubes(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        window: &Window,
    ) {
        for _ in 0..rand::thread_rng().gen_range(10..=20) {
            commands.spawn_bundle(Self::new(CubeGroup::Neutral, meshes, materials, window));
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

pub fn spawn_balls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Res<Windows>,
) {
    if rand::thread_rng().gen_ratio(1, 100) {
        match rand::thread_rng().gen_range(0..=3) {
            0 => SpeedBoost::default().spawn(
                &mut commands,
                &mut meshes,
                &mut materials,
                window.primary(),
            ),
            1 => SizeBoost::default().spawn(
                &mut commands,
                &mut meshes,
                &mut materials,
                window.primary(),
            ),
            2 => SpeedDecrease::default().spawn(
                &mut commands,
                &mut meshes,
                &mut materials,
                window.primary(),
            ),
            3 => SizeDecrease::default().spawn(
                &mut commands,
                &mut meshes,
                &mut materials,
                window.primary(),
            ),
            _ => unreachable!(),
        };
    }
}
