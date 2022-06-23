use crate::{
    core::{
        camera_follow,
        exploration::{
            self, BeamMarker, Exploration, ResonanceMarker, SourceResonanceMarker,
            TargetResonanceMarker,
        },
        movement::{self, Movement},
        spawn_camera, spawn_npc, spawn_player, CameraMarker, GameState, NpcMarker, PlayerMarker,
    },
    resources::prelude::*,
};
use bevy::prelude::{Plugin as BevyPlugin, *};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Movement {
            rotation: 0.0,
            thrust: 0.0,
        });

        app.insert_resource(Exploration {
            beam: false,
            resonance: false,
        });

        app.add_system_set(SystemSet::on_enter(GameState::Play).with_system(setup))
            .add_system_set(
                SystemSet::on_update(GameState::Play)
                    .with_system(handle_input)
                    .with_system(follow_player)
                    .with_system(process_movement)
                    .with_system(beam_reflection)
                    .with_system(resonance_proximity)
                    .with_system(update_source_resonance)
                    .with_system(dissipate_resonance),
            );
    }
}

fn setup(mut commands: Commands, images: Res<Images>) {
    let npc_positions: [Vec3; 5] = [
        Vec3::new(0.0, 128.0, 5.0),
        Vec3::new(320.0, 0.0, 5.0),
        Vec3::new(600.0, -200.0, 5.0),
        Vec3::new(-2200.0, -200.0, 5.0),
        Vec3::new(-2400.0, 400.0, 5.0),
    ];
    spawn_camera(&mut commands);
    spawn_player(&mut commands, &images);
    for npc_position in npc_positions.into_iter() {
        spawn_npc(
            &mut commands,
            Transform::from_translation(npc_position),
            &images,
        );
    }
}

fn handle_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut movement: ResMut<Movement>,
    mut exploration: ResMut<Exploration>,
) {
    movement::handle_input(&keyboard_input, &mut movement);
    exploration::handle_input(&keyboard_input, &mut exploration);
}

fn follow_player(
    player_query: Query<&Transform, With<PlayerMarker>>,
    mut camera_query: Query<&mut Transform, (With<CameraMarker>, Without<PlayerMarker>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();
    camera_follow(&mut camera_transform, &player_transform);
}

fn process_movement(
    time: Res<Time>,
    movement: Res<Movement>,
    mut player_query: Query<&mut Transform, With<PlayerMarker>>,
) {
    let mut transform = player_query.single_mut();
    movement::process(&mut transform, &movement, time.delta_seconds());
}

fn beam_reflection(
    mut commands: Commands,
    exploration: Res<Exploration>,
    player_query: Query<&Transform, With<PlayerMarker>>,
    npcs_query: Query<&Transform, With<NpcMarker>>,
    beams_query: Query<Entity, With<BeamMarker>>,
) {
    if exploration.beam {
        let player_transform = player_query.single();
        for npc_transform in npcs_query.iter() {
            let source = player_transform.translation.truncate();
            let target = npc_transform.translation.truncate();
            if exploration::beam_reflected(source, target) {
                exploration::beam(&mut commands, source, target);
            }
        }
    } else {
        for beam_entity in beams_query.iter() {
            commands.entity(beam_entity).despawn_recursive();
        }
    }
}

fn resonance_proximity(
    mut commands: Commands,
    mut exploration: ResMut<Exploration>,
    player_query: Query<&Transform, With<PlayerMarker>>,
    npcs_query: Query<&Transform, With<NpcMarker>>,
) {
    if !exploration.resonance {
        let player_transform = player_query.single();
        for npc_transform in npcs_query.iter() {
            let source = player_transform.translation.truncate();
            let target = npc_transform.translation.truncate();
            exploration.resonance = exploration::in_resonance(source, target);
            if exploration.resonance {
                exploration::resonance(&mut commands, source, target);
                break;
            }
        }
    }
}

fn update_source_resonance(
    player_query: Query<&Transform, With<PlayerMarker>>,
    mut source_resonance_query: Query<
        &mut Transform,
        (With<SourceResonanceMarker>, Without<PlayerMarker>),
    >,
) {
    if let Ok(mut source_transform) = source_resonance_query.get_single_mut() {
        let player_transform = player_query.single();
        source_transform.translation.x = player_transform.translation.x;
        source_transform.translation.y = player_transform.translation.y;
    }
}

fn dissipate_resonance(
    mut commands: Commands,
    mut exploration: ResMut<Exploration>,
    player_query: Query<&Transform, With<PlayerMarker>>,
    target_resonance_query: Query<&Transform, With<TargetResonanceMarker>>,
    resonance_query: Query<Entity, With<ResonanceMarker>>,
) {
    if exploration.resonance {
        let player_transform = player_query.single();
        if let Ok(target_transform) = target_resonance_query.get_single() {
            let source = player_transform.translation.truncate();
            let target = target_transform.translation.truncate();
            exploration.resonance = exploration::in_resonance(source, target);
        }
    } else {
        for resonance_entity in resonance_query.iter() {
            commands.entity(resonance_entity).despawn_recursive();
        }
    }
}
