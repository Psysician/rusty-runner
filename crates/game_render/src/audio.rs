use bevy::prelude::*;

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, _app: &mut App) {
        // Audio systems will play sounds on game events
        // Actual .ogg files go in assets/audio/
    }
}
