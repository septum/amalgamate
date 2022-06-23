use bevy::{asset::LoadState, prelude::*};

pub struct Images {
    pub player: Handle<Image>,
}

impl Images {
    #[must_use]
    pub fn load(asset_server: &Res<AssetServer>) -> Images {
        Images {
            player: asset_server.load("images/player.png"),
        }
    }

    #[must_use]
    pub fn all_loaded(&self, asset_server: &Res<AssetServer>) -> bool {
        asset_server.get_load_state(self.player.clone()) == LoadState::Loaded
    }
}
