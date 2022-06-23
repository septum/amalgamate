use bevy::prelude::*;

#[derive(Component)]
pub struct Marker;

pub fn spawn(commands: &mut Commands) {
    let mut camera_bundle = OrthographicCameraBundle::new_2d();

    // let's zoom half-way in
    camera_bundle.orthographic_projection.scale *= 0.50;

    commands.spawn_bundle(camera_bundle).insert(Marker);
}

pub fn follow(camera_transform: &mut Transform, target_transform: &Transform) {
    camera_transform.translation.x = target_transform.translation.x;
    camera_transform.translation.y = target_transform.translation.y;
}
