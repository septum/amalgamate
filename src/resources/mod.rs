mod colors;
mod fonts;
mod images;
mod ldtk;
mod sounds;

pub mod prelude;

use crate::core::GameState;
use bevy::prelude::{Plugin as BevyPlugin, *};
use prelude::*;

pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Startup).with_system(startup))
            .add_system_set(SystemSet::on_update(GameState::Loading).with_system(check_loading));
    }
}

fn startup(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    asset_server: Res<AssetServer>,
) {
    let fonts = Fonts::load(&asset_server);
    let images = Images::load(&asset_server);
    let ldkt = Ldtk::load(&asset_server);
    let sounds = Sounds::load(&asset_server);

    commands.insert_resource(fonts);
    commands.insert_resource(images);
    commands.insert_resource(ldkt);
    commands.insert_resource(sounds);

    state.set(GameState::Loading).unwrap();
}

fn check_loading(
    mut state: ResMut<State<GameState>>,
    asset_server: Res<AssetServer>,
    fonts: Res<Fonts>,
    images: Res<Images>,
    ldtk: Res<Ldtk>,
    sounds: Res<Sounds>,
) {
    let all_loaded = fonts.all_loaded(&asset_server)
        && images.all_loaded(&asset_server)
        && ldtk.all_loaded(&asset_server)
        && sounds.all_loaded(&asset_server);

    if all_loaded {
        state.set(GameState::Title).unwrap();
    }
}
