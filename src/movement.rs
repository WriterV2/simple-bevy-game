use bevy::prelude::*;

// every entity with a position, speed and direction moves in specified direction with
// specified speed
pub fn movement(
    mut query: Query<(
        &mut Transform,
        &crate::physicalentities::Speed,
        &crate::physicalentities::Direction,
    )>,
    window: Res<Windows>,
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
            transform.translation += direction.0 * speed.0;
        }
    }
}

// change direction of moving entity
// player: key input
// enemy: looking for nearest cube
// neutral: randomly
pub fn switch_direction(
    mut query: Query<(
        &mut crate::physicalentities::Direction,
        &crate::physicalentities::CubeGroup,
        &Transform,
    )>,
    keys: Res<Input<KeyCode>>,
) {
    // get enemy cube's position
    let enemy_position = if let Some(pos) = query
        .iter()
        .find(|x| x.1 == &crate::physicalentities::CubeGroup::Enemy)
    {
        pos.2.translation
    } else {
        panic!("Enemy not found - fn switch_direction");
    };

    // get all neutral cubes' positions
    let neutral_cubes_positions: Vec<Vec3> = query
        .iter()
        .filter(|x| x.1 == &crate::physicalentities::CubeGroup::Neutral)
        .map(|x| x.2.translation)
        .collect();

    // euclidean distance between enemy and nearest neutral cube
    let mut nearest_distance = f32::MAX;
    // difference of enemy position's x and nearest neutral cube's x
    let mut nearest_distance_x = f32::MAX;
    // difference of enemy position's y and nearest neutral cube's y
    let mut nearest_distance_y = f32::MAX;

    for (mut direction, group, _transform) in query.iter_mut() {
        match group {
            crate::physicalentities::CubeGroup::Player => {
                if let Some(input) = player_input_direction(&keys) {
                    *direction = crate::physicalentities::Direction(input);
                }
            }
            crate::physicalentities::CubeGroup::Enemy => {
                // get the distance between the enemy and the nearest cube
                for neutral_position in &neutral_cubes_positions {
                    if nearest_distance.min(neutral_position.distance(enemy_position))
                        != nearest_distance
                    {
                        nearest_distance = neutral_position.distance(enemy_position);
                        nearest_distance_x = enemy_position.x - neutral_position.x;
                        nearest_distance_y = enemy_position.y - neutral_position.y;
                    }
                }

                // enemy follows the nearest cube on the axis with the longest distance
                *direction = if nearest_distance_x.abs() > nearest_distance_y.abs() {
                    if nearest_distance_x.is_sign_negative() {
                        crate::physicalentities::Direction(Vec3::X)
                    } else {
                        crate::physicalentities::Direction(-Vec3::X)
                    }
                } else {
                    if nearest_distance_y.is_sign_negative() {
                        crate::physicalentities::Direction(Vec3::Y)
                    } else {
                        crate::physicalentities::Direction(-Vec3::Y)
                    }
                };
            }
            crate::physicalentities::CubeGroup::Neutral => {}
        }
    }
}

// change player direction with input: WASD / arrow keys
fn player_input_direction(keys: &Res<Input<KeyCode>>) -> Option<Vec3> {
    if keys.just_pressed(KeyCode::A) || keys.just_pressed(KeyCode::Left) {
        Some(-Vec3::X)
    } else if keys.just_pressed(KeyCode::D) || keys.just_pressed(KeyCode::Right) {
        Some(Vec3::X)
    } else if keys.just_pressed(KeyCode::W) || keys.just_pressed(KeyCode::Up) {
        Some(Vec3::Y)
    } else if keys.just_pressed(KeyCode::S) || keys.just_pressed(KeyCode::Down) {
        Some(-Vec3::Y)
    } else {
        None
    }
}
