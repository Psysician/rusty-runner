use bevy::prelude::*;
use game_core::GameCorePlugin;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(GameCorePlugin)
        .run();
}
