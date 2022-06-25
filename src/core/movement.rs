#![allow(clippy::collapsible_else_if)]

use bevy::prelude::*;

pub const ROTATION_ACCELERATION_RATE: f32 = 0.005;
pub const THRUST_ACCELERATION_RATE: f32 = 0.1;

pub struct Movement {
    pub rotation: f32,
    pub thrust: f32,
}

pub fn handle_input(keyboard_input: &Input<KeyCode>, movement: &mut Movement) {
    if keyboard_input.pressed(KeyCode::Left) {
        movement.rotation += ROTATION_ACCELERATION_RATE;
    } else if keyboard_input.pressed(KeyCode::Right) {
        movement.rotation -= ROTATION_ACCELERATION_RATE;
    } else {
        if movement.rotation < 0.0 {
            movement.rotation += ROTATION_ACCELERATION_RATE;
        } else if movement.rotation > 0.0 {
            movement.rotation -= ROTATION_ACCELERATION_RATE;
        }
    }

    if keyboard_input.pressed(KeyCode::Up) {
        movement.thrust += THRUST_ACCELERATION_RATE;
    } else {
        if movement.thrust > 0.0 {
            movement.thrust -= THRUST_ACCELERATION_RATE;
        }
    }
}

pub fn process(transform: &mut Transform, movement: &Movement, delta: f32) {
    transform.rotate(Quat::from_rotation_z(movement.rotation * delta));

    // starting direction is upwards
    let direction = transform.rotation * Vec3::Y;
    transform.translation.x += direction.x * movement.thrust * delta;
    transform.translation.y += direction.y * movement.thrust * delta;
}
