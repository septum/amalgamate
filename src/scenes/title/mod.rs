mod ui;

use bevy::{
    app::AppExit,
    prelude::{Input, Plugin as BevyPlugin, *},
};
use bevy_ecs_ldtk::prelude::*;
use bevy_kira_audio::Audio;

use crate::{
    core::{map, spawn_camera, GameState},
    resources::prelude::*,
    ui::{ActionKind, ActionMarker},
};

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelSelection::Index(0));

        app.add_system_set(SystemSet::on_enter(GameState::Title).with_system(setup))
            .add_system_set(
                SystemSet::on_update(GameState::Title).with_system(buttons_interactions),
            )
            .add_system_set(SystemSet::on_exit(GameState::Title).with_system(cleanup));
    }
}

fn setup(
    mut commands: Commands,
    fonts: Res<Fonts>,
    ldtk: Res<Ldtk>,
    audio: Res<Audio>,
    sounds: Res<Sounds>,
) {
    let audio_source = sounds.music.ambiment.clone();
    let channel_id = &sounds.channels.music;
    audio.play_looped_in_channel(audio_source, channel_id);
    audio.set_volume_in_channel(0.5, channel_id);

    spawn_camera(&mut commands);
    map::spawn(&mut commands, &ldtk);
    ui::spawn(&mut commands, &fonts);
}

fn buttons_interactions(
    mut game_state: ResMut<State<GameState>>,
    mut exit_event: EventWriter<AppExit>,
    mut mouse_button_input: ResMut<Input<MouseButton>>,
    mut query: Query<
        (&ActionMarker, &Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (action, interaction, mut color) in query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                // workaround for input persistence between states
                // see: https://github.com/bevyengine/bevy/issues/1700#issuecomment-886999222
                mouse_button_input.reset(MouseButton::Left);

                match action.kind() {
                    ActionKind::Play => {
                        game_state.set(GameState::Play).unwrap();
                    }
                    ActionKind::Quit => {
                        exit_event.send(AppExit);
                    }
                };

                *color = Colors::PRIMARY.into();
            }
            Interaction::Hovered => {
                *color = Colors::LIGHT.into();
            }
            Interaction::None => {
                *color = Colors::DARK.into();
            }
        }
    }
}

fn cleanup(mut commands: Commands, entities: Query<Entity, With<ui::ScopedMarker>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
