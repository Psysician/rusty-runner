use bevy::ecs::error::{BevyError, DefaultErrorHandler, ErrorContext};
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use game_core::boss::BossPlugin;
use game_core::collision::CollisionPlugin;
use game_core::enemy::EnemyPlugin;
use game_core::input::InputPlugin;
use game_core::items::ItemPlugin;
use game_core::physics::PhysicsPlugin;
use game_core::platforms::PlatformPlugin;
use game_core::player::PlayerPlugin;
use game_core::save::SavePlugin;
use game_core::state::{AppState, StatePlugin};
use game_core::wind::WindPlugin;

pub fn diagnostic_handler(error: BevyError, ctx: ErrorContext) {
    panic!(
        "ECS error in {} `{}`: {}",
        ctx.kind(),
        ctx.name(),
        error
    );
}

/// Builds a headless app with all GameCorePlugin sub-plugins except LevelPlugin.
/// LevelPlugin is excluded because bevy_ecs_tilemap requires a RenderApp sub-app
/// that doesn't exist in headless mode. The gameplay plugins (Player, Enemy,
/// Collision, etc.) are where query conflicts like B0001 actually occur.
pub fn build_full_test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(AssetPlugin::default());
    app.add_plugins(bevy::image::ImagePlugin::default());
    app.add_plugins(TransformPlugin);
    app.add_plugins(bevy::scene::ScenePlugin);
    app.add_plugins(StatesPlugin);
    app.add_plugins(InputPlugin);
    app.add_plugins(StatePlugin);
    app.add_plugins(BossPlugin);
    app.add_plugins(PhysicsPlugin);
    app.add_plugins(PlayerPlugin);
    app.add_plugins(EnemyPlugin);
    app.add_plugins(CollisionPlugin);
    app.add_plugins(ItemPlugin);
    app.add_plugins(PlatformPlugin);
    app.add_plugins(WindPlugin);
    app.add_plugins(SavePlugin);
    app.world_mut()
        .insert_resource(DefaultErrorHandler(diagnostic_handler));
    app.finish();
    app.cleanup();
    app
}

pub fn set_state(app: &mut App, state: AppState) {
    app.world_mut()
        .resource_mut::<NextState<AppState>>()
        .set(state);
    app.update();
    app.update();
}
