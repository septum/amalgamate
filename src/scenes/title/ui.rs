use bevy::prelude::*;

use crate::{
    resources::prelude::*,
    ui::{Action, ActionMarker, EmbossedText, Housing, Overlay, SimpleText},
};

#[derive(Component)]
pub struct ScopedMarker;

fn spawn_camera(commands: &mut Commands) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(ScopedMarker);
}

pub fn spawn(commands: &mut Commands, fonts: &Fonts) {
    let font = &fonts.dogica;
    let button_size = Size::new(Val::Px(400.0), Val::Px(56.0));

    let overlay = Overlay::new();
    let top = Housing::percent(100.0, 36.0);
    let bottom = Housing::percent(100.0, 36.0);
    let mut actions = Housing::percent(100.0, 70.0);
    let footer = Housing::percent(100.0, 30.0);

    let title = EmbossedText::big("amalgamate", font);
    let notice = SimpleText::small("https://septum.io", font);
    let play = Action::new("play", font, button_size);
    let quit = Action::new("quit", font, button_size);

    actions
        .justify_content(JustifyContent::SpaceEvenly)
        .align_items(AlignItems::Center);

    overlay.spawn(
        commands,
        |parent| {
            top.spawn(parent, |parent| {
                title.spawn(parent);
            });
            bottom.spawn(parent, |parent| {
                actions.spawn(parent, |parent| {
                    play.spawn(parent, ActionMarker::play());
                    quit.spawn(parent, ActionMarker::quit());
                });
                footer.spawn(parent, |parent| {
                    notice.spawn(parent);
                });
            });
        },
        ScopedMarker,
    );

    spawn_camera(commands);
}
