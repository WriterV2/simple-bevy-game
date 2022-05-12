use bevy::prelude::*;

pub struct Movement;
impl Plugin for Movement {
    fn build(&self, app: &mut App) {
        app.add_system(movement::<crate::physicalentities::Player>)
            .add_system(movement::<crate::physicalentities::Enemy>)
            .add_system(movement::<crate::physicalentities::Neutral>)
            .add_system(switch_direction::<crate::physicalentities::Player>)
            .add_system(switch_direction::<crate::physicalentities::Enemy>)
            .add_system(switch_direction::<crate::physicalentities::Neutral>);
    }
}

// every entity with a position, speed and direction moves in specified direction with
// specified speed
fn movement<T: crate::physicalentities::Cube>(
    mut query: Query<
        (
            &mut Transform,
            &crate::physicalentities::Speed,
            &crate::physicalentities::Direction,
        ),
        With<T>,
    >,
    window: Res<Windows>,
    time: Res<Time>,
) {
    for (mut transform, speed, direction) in query.iter_mut() {
        // move entity in direction with its speed
        // when entity hits the window edge, it comes out of the opposite side
        if transform.translation.x >= window.primary().width() / 2. {
            transform.translation.x = -window.primary().width() / 2. + 1.;
        } else if transform.translation.x < -(window.primary().width() / 2.) {
            transform.translation.x = window.primary().width() / 2. - 1.;
        } else if transform.translation.y >= window.primary().height() / 2. {
            transform.translation.y = -window.primary().height() / 2. + 1.;
        } else if transform.translation.y < -(window.primary().height() / 2.) {
            transform.translation.y = window.primary().height() / 2. - 1.;
        } else {
            transform.translation += direction.0 * speed.0 * time.delta_seconds();
        }
    }
}

// change direction of moving entity
// player: key input
// enemy: looking for nearest cube
// neutral: randomly
fn switch_direction<T: crate::physicalentities::Cube>(
    mut query: Query<(&mut crate::physicalentities::Direction, &T, &Transform)>,
    neutral_cubes: Query<&Transform, With<crate::physicalentities::Neutral>>,
    keys: Res<Input<KeyCode>>,
) {
    let neutral_cubes_positions: Vec<Vec3> = neutral_cubes.iter().map(|x| x.translation).collect();
    for (mut direction, cube, transform) in query.iter_mut() {
        if let Some(new_direction) = cube.new_direction(&neutral_cubes_positions, &transform, &keys)
        {
            *direction = new_direction;
        }
    }
}
