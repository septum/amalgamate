use bevy::{asset::LoadState, prelude::*};
use bevy_ecs_ldtk::prelude::*;

pub struct Ldtk {
    pub map: Handle<LdtkAsset>,
}

impl Ldtk {
    #[must_use]
    pub fn load(asset_server: &Res<AssetServer>) -> Ldtk {
        Ldtk {
            map: asset_server.load("ldtk/map.ldtk"),
        }
    }

    #[must_use]
    pub fn all_loaded(&self, asset_server: &Res<AssetServer>) -> bool {
        asset_server.get_load_state(self.map.clone()) == LoadState::Loaded
    }
}
