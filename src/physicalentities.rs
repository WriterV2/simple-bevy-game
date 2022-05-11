use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

// speed of physical object
#[derive(Debug, Component, PartialEq, PartialOrd)]
pub struct Speed(pub f32);

// direction physical object moves
#[derive(Debug, Component, PartialEq)]
pub struct Direction(pub Vec3);

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
                mesh: meshes.add(Mesh::from(shape::Icosphere::default())).into(),
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
