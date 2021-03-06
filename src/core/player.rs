use bevy::prelude::*;

use crate::resources::prelude::Images;

#[derive(Component)]
pub struct Marker;

pub fn spawn(commands: &mut Commands, images: &Images) {
    let transform = Transform::from_xyz(0.0, 0.0, 10.0);
    commands
        .spawn_bundle(SpriteBundle {
            texture: images.pc.clone(),
            transform,
            ..SpriteBundle::default()
        })
        .insert(Marker);
}
