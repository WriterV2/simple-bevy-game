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
