use crate::{
    core::{
        camera_follow,
        movement::{self, Movement},
        spawn_camera, spawn_player, CameraMarker, GameState, PlayerMarker,
    },
    resources::prelude::*,
};
use bevy::prelude::{Plugin as BevyPlugin, *};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(movement::Movement {
            rotation: 0.0,
            thrust: 0.0,
        });

        app.add_system_set(SystemSet::on_enter(GameState::Play).with_system(setup))
            .add_system_set(
                SystemSet::on_update(GameState::Play)
                    .with_system(handle_input)
                    .with_system(process_movement),
            );
    }
}

fn setup(mut commands: Commands, images: Res<Images>) {
    spawn_camera(&mut commands);
    spawn_player(&mut commands, &images);

    commands.spawn_bundle(SpriteBundle {
        texture: images.player.clone(),
        ..SpriteBundle::default()
    });
}

fn handle_input(keyboard_input: Res<Input<KeyCode>>, mut movement: ResMut<Movement>) {
    movement::handle_input(&keyboard_input, &mut movement);
}

fn process_movement(
    time: Res<Time>,
    movement: Res<Movement>,
    mut camera_query: Query<&mut Transform, (With<CameraMarker>, Without<PlayerMarker>)>,
    mut query: Query<&mut Transform, With<PlayerMarker>>,
) {
    let mut transform = query.single_mut();
    movement::process(&mut transform, &movement, time.delta_seconds());

    let mut camera_transform = camera_query.single_mut();
    camera_follow(&mut camera_transform, &transform);
}
