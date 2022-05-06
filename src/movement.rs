use bevy::prelude::*;

// every entity with a position, speed and direction moves in specified direction with
// specified speed
pub fn movement(
    mut query: Query<(
        &mut Transform,
        &crate::physicalentities::Speed,
        &crate::physicalentities::Direction,
    )>,
) {
    for (mut transform, speed, direction) in query.iter_mut() {
        transform.translation += direction.0 * speed.0;
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
    )>,
    keys: Res<Input<KeyCode>>,
) {
    for (mut direction, group) in query.iter_mut() {
        match group {
            crate::physicalentities::CubeGroup::Player => {
                if let Some(input) = player_input_direction(&keys) {
                    *direction = crate::physicalentities::Direction(input);
                }
            }
            crate::physicalentities::CubeGroup::Enemy => {}
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
