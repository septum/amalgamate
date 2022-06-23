use bevy::{asset::LoadState, prelude::*};

pub struct Images {
    pub pc: Handle<Image>,
    pub npc: Handle<Image>,
}

impl Images {
    #[must_use]
    pub fn load(asset_server: &Res<AssetServer>) -> Images {
        Images {
            pc: asset_server.load("images/pc.png"),
            npc: asset_server.load("images/npc.png"),
        }
    }

    #[must_use]
    pub fn all_loaded(&self, asset_server: &Res<AssetServer>) -> bool {
        asset_server.get_load_state(self.pc.clone()) == LoadState::Loaded
            && asset_server.get_load_state(self.npc.clone()) == LoadState::Loaded
    }
}
