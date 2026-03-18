use bevy::prelude::*;

pub mod enemy;
pub mod input;
pub mod level;
pub mod physics;
pub mod platforms;
pub mod player;
pub mod state;

use enemy::EnemyPlugin;
use input::InputPlugin;
use level::LevelPlugin;
use physics::PhysicsPlugin;
use platforms::PlatformPlugin;
use player::PlayerPlugin;
use state::StatePlugin;

pub struct GameCorePlugin;

impl Plugin for GameCorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputPlugin);
        app.add_plugins(StatePlugin);
        app.add_plugins(PhysicsPlugin);
        app.add_plugins(LevelPlugin);
        app.add_plugins(PlayerPlugin);
        app.add_plugins(EnemyPlugin);
        app.add_plugins(PlatformPlugin);
    }
}
