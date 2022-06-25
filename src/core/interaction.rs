use bevy::prelude::*;

pub struct Interaction {
    pub orbit: bool,
}

pub fn handle_input(keyboard_input: &Input<KeyCode>, interaction: &mut Interaction) {
    let movement_keys_pressed = keyboard_input.just_pressed(KeyCode::Up)
        || keyboard_input.just_pressed(KeyCode::Left)
        || keyboard_input.just_pressed(KeyCode::Right);

    if movement_keys_pressed {
        interaction.orbit = false;
    } else if keyboard_input.just_pressed(KeyCode::Space) {
        if interaction.orbit {}
    }
}
