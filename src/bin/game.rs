use bevy::prelude::*;
use game_core::GameCorePlugin;
use game_render::GameRenderPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rusty Runner".into(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GameCorePlugin)
        .add_plugins(GameRenderPlugin)
        .run();
}
