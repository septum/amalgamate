use bevy::prelude::*;

const ATTRACTION_MAX_DISTANCE: f32 = 192.0;
const DISTANCE_REDUCTION_MULTIPLIER: f32 = 0.42;

#[must_use]
pub fn collision(
    source_position: Vec2,
    source_size: Vec2,
    target_position: Vec2,
    target_size: Vec2,
) -> bool {
    let source_min = source_position - source_size / 2.0;
    let source_max = source_position + source_size / 2.0;
    let target_min = target_position - target_size / 2.0;
    let target_max = target_position + target_size / 2.0;

    source_min.x < target_max.x
        && source_max.x > target_min.x
        && source_min.y < target_max.y
        && source_max.y > target_min.y
}

pub fn deviate_trajectory(source: &mut Vec3, target: &Vec3, delta: f32) {
    let source_position = source.truncate();
    let target_position = target.truncate();
    let direction = Vec2::normalize(source_position - target_position);
    let distance = source_position.distance(target_position);
    let distance_modifier = (ATTRACTION_MAX_DISTANCE - distance) * DISTANCE_REDUCTION_MULTIPLIER;

    source.x -= (direction.x * distance_modifier) * delta;
    source.y -= (direction.y * distance_modifier) * delta;
}
