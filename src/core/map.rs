use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::resources::prelude::*;

pub fn spawn(commands: &mut Commands, ldtk: &Ldtk) {
    let mut transform = Transform::from_xyz(-1600.0, -1600.0, 1.0);
    transform.scale *= 2.0;
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: ldtk.map.clone(),
        transform,
        ..LdtkWorldBundle::default()
    });
}
