use bevy::prelude::*;

pub mod input;
pub mod state;

use input::InputPlugin;
use state::StatePlugin;

pub struct GameCorePlugin;

impl Plugin for GameCorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputPlugin);
        app.add_plugins(StatePlugin);
    }
}
