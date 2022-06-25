use bevy::{asset::LoadState, prelude::*};
use bevy_kira_audio::{AudioChannel, AudioSource};

pub struct Channels {
    pub sfx: AudioChannel,
    pub music: AudioChannel,
}

pub struct Sfx {
    pub beam: Handle<AudioSource>,
    pub resonance: Handle<AudioSource>,
    pub absorption: Handle<AudioSource>,
}

pub struct Music {
    pub ambiment: Handle<AudioSource>,
}

pub struct Sounds {
    pub channels: Channels,
    pub sfx: Sfx,
    pub music: Music,
}

impl Sounds {
    pub fn load(asset_server: &Res<AssetServer>) -> Sounds {
        let channels = Channels {
            sfx: AudioChannel::new("sfx".to_string()),
            music: AudioChannel::new("music".to_string()),
        };
        let sfx = Sfx {
            beam: asset_server.load("sounds/fx/beam.wav"),
            resonance: asset_server.load("sounds/fx/resonance.wav"),
            absorption: asset_server.load("sounds/fx/absorption.wav"),
        };
        let music = Music {
            ambiment: asset_server.load("sounds/music/ambiment.mp3"),
        };

        Sounds {
            channels,
            sfx,
            music,
        }
    }

    pub fn all_loaded(&self, asset_server: &Res<AssetServer>) -> bool {
        let sounds_untyped = vec![
            self.sfx.beam.clone_untyped(),
            self.sfx.resonance.clone_untyped(),
            self.sfx.absorption.clone_untyped(),
            self.music.ambiment.clone_untyped(),
        ];

        for sound_handle in sounds_untyped {
            if asset_server.get_load_state(sound_handle) != LoadState::Loaded {
                return false;
            }
        }

        true
    }
}
