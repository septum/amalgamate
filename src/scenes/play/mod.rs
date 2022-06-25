use crate::{
    core::{
        camera_follow,
        exploration::{
            self, BeamMarker, Exploration, ResonanceMarker, SourceResonanceMarker,
            TargetResonanceMarker,
        },
        interaction::{self, Interaction},
        movement::{self, Movement},
        physics, spawn_npc, spawn_player, CameraMarker, GameState, NpcMarker, PlayerMarker,
    },
    resources::prelude::*,
};
use bevy::{
    app::AppExit,
    prelude::{Plugin as BevyPlugin, *},
};
use bevy_kira_audio::Audio;

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

        app.insert_resource(Interaction { orbit: false });

        app.add_system_set(SystemSet::on_enter(GameState::Play).with_system(setup))
            .add_system_set(
                SystemSet::on_update(GameState::Play)
                    .with_system(handle_input)
                    .with_system(process_movement)
                    .with_system(gravitational_attraction)
                    .with_system(entity_collision)
                    .with_system(beam_reflection)
                    .with_system(resonance_proximity)
                    .with_system(update_source_resonance)
                    .with_system(dissipate_resonance)
                    .with_system(orbit_criteria)
                    .with_system(orbit_resonance)
                    .with_system(orbit_absorption),
            );
    }
}

fn setup(mut commands: Commands, images: Res<Images>) {
    let npc_positions: [Vec3; 5] = [
        Vec3::new(0.0, 280.0, 5.0),
        Vec3::new(320.0, -560.0, 5.0),
        Vec3::new(600.0, -200.0, 5.0),
        Vec3::new(-2200.0, -200.0, 5.0),
        Vec3::new(-2400.0, 400.0, 5.0),
    ];
    spawn_player(&mut commands, &images);
    for npc_position in npc_positions {
        spawn_npc(
            &mut commands,
            Transform::from_translation(npc_position),
            &images,
        );
    }
}

fn handle_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut interaction: ResMut<Interaction>,
    mut movement: ResMut<Movement>,
    mut exploration: ResMut<Exploration>,
) {
    movement::handle_input(&keyboard_input, &mut movement);
    exploration::handle_input(&keyboard_input, &mut exploration);
    interaction::handle_input(&keyboard_input, &mut interaction);
}

fn process_movement(
    time: Res<Time>,
    movement: Res<Movement>,
    mut player_query: Query<&mut Transform, With<PlayerMarker>>,
    mut camera_query: Query<&mut Transform, (With<CameraMarker>, Without<PlayerMarker>)>,
) {
    let mut player_transform = player_query.single_mut();
    movement::process(&mut player_transform, &movement, time.delta_seconds());

    let mut camera_transform = camera_query.single_mut();
    camera_follow(&mut camera_transform, &player_transform);
}

fn gravitational_attraction(
    time: Res<Time>,
    interaction: Res<Interaction>,
    mut player_query: Query<&mut Transform, With<PlayerMarker>>,
    npcs_query: Query<&Transform, (With<NpcMarker>, Without<PlayerMarker>)>,
) {
    if !interaction.orbit {
        let mut player_transform = player_query.single_mut();
        for npc_transform in npcs_query.iter() {
            let source = player_transform.translation.truncate();
            let target = npc_transform.translation.truncate();
            let size = Vec2::splat(192.0);

            if physics::collision(source, size, target, size) {
                physics::deviate_trajectory(
                    &mut player_transform.translation,
                    &npc_transform.translation,
                    time.delta_seconds(),
                );
            }
        }
    }
}

fn entity_collision(
    mut game_state: ResMut<State<GameState>>,
    player_query: Query<&Transform, With<PlayerMarker>>,
    npcs_query: Query<&Transform, (With<NpcMarker>, Without<PlayerMarker>)>,
) {
    let player_transform = player_query.single();
    for npc_transform in npcs_query.iter() {
        let source = player_transform.translation.truncate();
        let target = npc_transform.translation.truncate();
        let size = Vec2::splat(48.0);

        if physics::collision(source, size, target, size) {
            game_state.set(GameState::Title).unwrap();
        }
    }
}

fn beam_reflection(
    mut commands: Commands,
    exploration: Res<Exploration>,
    player_query: Query<&Transform, With<PlayerMarker>>,
    npcs_query: Query<&Transform, With<NpcMarker>>,
    beams_query: Query<Entity, With<BeamMarker>>,
    audio: Res<Audio>,
    sounds: Res<Sounds>,
) {
    if exploration.beam {
        let player_transform = player_query.single();
        for npc_transform in npcs_query.iter() {
            let source = player_transform.translation.truncate();
            let target = npc_transform.translation.truncate();
            if exploration::beam_reflected(source, target) {
                exploration::beam(&mut commands, source, target);
                let audio_source = sounds.sfx.beam.clone();
                let channel_id = &sounds.channels.sfx;
                audio.play_in_channel(audio_source, channel_id);
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
    interaction: Res<Interaction>,
    mut exploration: ResMut<Exploration>,
    player_query: Query<&Transform, With<PlayerMarker>>,
    npcs_query: Query<&Transform, With<NpcMarker>>,
    audio: Res<Audio>,
    sounds: Res<Sounds>,
) {
    if !interaction.orbit {
        if !exploration.resonance {
            let player_transform = player_query.single();
            for npc_transform in npcs_query.iter() {
                let source = player_transform.translation.truncate();
                let target = npc_transform.translation.truncate();
                exploration.resonance = exploration::in_resonance(source, target);
                if exploration.resonance {
                    exploration::resonance(&mut commands, source, target);
                    let audio_source = sounds.sfx.resonance.clone();
                    let channel_id = &sounds.channels.sfx;
                    audio.play_in_channel(audio_source, channel_id);
                    break;
                }
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
    interaction: Res<Interaction>,
    player_query: Query<&Transform, With<PlayerMarker>>,
    target_resonance_query: Query<&Transform, With<TargetResonanceMarker>>,
    resonance_query: Query<Entity, With<ResonanceMarker>>,
) {
    if !interaction.orbit {
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
}

fn orbit_criteria(
    mut interaction: ResMut<Interaction>,
    movement: Res<Movement>,
    exploration: Res<Exploration>,
    source_resonance_query: Query<&Transform, With<SourceResonanceMarker>>,
    target_resonance_query: Query<&Transform, With<TargetResonanceMarker>>,
    audio: Res<Audio>,
    sounds: Res<Sounds>,
) {
    if exploration.resonance && exploration.beam && movement.thrust <= 40.0 {
        if let Ok(source_transform) = source_resonance_query.get_single() {
            if let Ok(target_transform) = target_resonance_query.get_single() {
                let source = source_transform.translation.truncate();
                let target = target_transform.translation.truncate();
                let distance = source.distance(target);
                if distance <= 160.0 && distance >= 128.0 {
                    interaction.orbit = true;
                    let audio_source = sounds.sfx.absorption.clone();
                    let channel_id = &sounds.channels.sfx;
                    audio.play_in_channel(audio_source, channel_id);
                }
            }
        }
    }
}

fn orbit_resonance(
    mut movement: ResMut<Movement>,
    interaction: Res<Interaction>,
    mut player_query: Query<&mut Transform, With<PlayerMarker>>,
    target_resonance_query: Query<&Transform, (With<TargetResonanceMarker>, Without<PlayerMarker>)>,
) {
    if interaction.orbit {
        let mut player_transform = player_query.single_mut();
        if let Ok(target_transform) = target_resonance_query.get_single() {
            let mut source: Vec2 = player_transform.translation.truncate();
            let target: Vec2 = target_transform.translation.truncate();
            source -= target;

            // half radian
            let angle: f32 = 0.0085;
            let cos = angle.cos();
            let sin = angle.sin();
            let new_x = (source.x * cos - source.y * sin) + target.x;
            let new_y = (source.x * sin + source.y * cos) + target.y;
            player_transform.translation.x = new_x;
            player_transform.translation.y = new_y;
            movement.rotation = 0.51;
        }
    }
}

fn orbit_absorption(
    mut commands: Commands,
    time: Res<Time>,
    mut movement: ResMut<Movement>,
    mut interaction: ResMut<Interaction>,
    mut exploration: ResMut<Exploration>,
    mut npcs_query: Query<(&mut Transform, Entity), With<NpcMarker>>,
    target_resonance_query: Query<
        (&Transform, Entity),
        (With<TargetResonanceMarker>, Without<NpcMarker>),
    >,
) {
    if interaction.orbit {
        if let Ok((target_transform, target_entity)) = target_resonance_query.get_single() {
            for (mut npc_transform, npc_entity) in npcs_query.iter_mut() {
                if target_transform.translation.x.floor() == npc_transform.translation.x.floor()
                    && target_transform.translation.y.floor() == npc_transform.translation.y.floor()
                {
                    npc_transform.scale -= 0.05 * time.delta_seconds();
                    if npc_transform.scale.x < 0.0 || npc_transform.scale.y < 0.0 {
                        commands.entity(npc_entity).despawn_recursive();
                        commands.entity(target_entity).despawn_recursive();
                        exploration.resonance = false;
                    }
                }
            }
        } else if !exploration.resonance {
            interaction.orbit = false;
            movement.thrust = 40.0;
        }
    }
}
