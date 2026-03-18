use bevy::prelude::*;
use game_core::GameCorePlugin;
use game_ai::GameAiPlugin;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(GameCorePlugin)
        .add_plugins(GameAiPlugin)
        .run();
}
