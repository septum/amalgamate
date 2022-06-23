use bevy::prelude::*;

use crate::resources::prelude::Images;

#[derive(Component)]
pub struct Marker;

pub fn spawn(commands: &mut Commands, transform: Transform, images: &Images) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: images.npc.clone(),
            transform,
            ..SpriteBundle::default()
        })
        .insert(Marker);
}
