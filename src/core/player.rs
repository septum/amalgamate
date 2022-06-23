use bevy::prelude::*;

use crate::resources::prelude::Images;

#[derive(Component)]
pub struct Marker;

pub fn spawn(commands: &mut Commands, images: &Images) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: images.player.clone(),
            ..SpriteBundle::default()
        })
        .insert(Marker);
}
