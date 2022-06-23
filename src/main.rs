use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;
use bevy_prototype_lyon::prelude::*;

use amalgamate::{
    config,
    core::GameState,
    resources::{self, prelude::*},
    scenes,
};

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugin(config::Plugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(resources::Plugin)
        .add_plugin(scenes::Plugin)
        .insert_resource(ClearColor(Colors::DARK))
        .add_state(GameState::Startup)
        .run();
}
