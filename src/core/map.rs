use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::resources::prelude::*;

pub fn spawn(commands: &mut Commands, ldtk: &Ldtk) {
    let mut transform = Transform::from_xyz(-4_096.0, -4_096.0, 0.5);
    // this increases x and y
    transform.scale *= 2.0;
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: ldtk.map.clone(),
        transform,
        ..LdtkWorldBundle::default()
    });
}
