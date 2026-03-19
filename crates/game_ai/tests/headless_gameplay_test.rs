use avian2d::prelude::*;
use bevy::ecs::error::{BevyError, DefaultErrorHandler, ErrorContext};
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use game_ai::bot::RightRunnerBot;
use game_ai::metrics::RunMetrics;
use game_ai::{ActiveBot, GameAiPlugin};
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

fn diagnostic_handler(error: BevyError, ctx: ErrorContext) {
    panic!(
        "ECS error in {} `{}`: {}",
        ctx.kind(),
        ctx.name(),
        error
    );
}

// 2000 wide — give RightRunnerBot enough ground to run across.
fn spawn_ground(mut commands: Commands) {
    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(2000.0, 50.0),
        Transform::from_xyz(0.0, -50.0, 0.0),
    ));
}

/// Builds a headless app with all gameplay plugins + GameAiPlugin + RightRunnerBot.
/// LevelPlugin excluded (bevy_ecs_tilemap requires RenderApp in headless mode).
/// Duplicates build_full_test_app from game_core tests because Rust doesn't
/// expose another crate's tests/ directory.
fn build_gameplay_test_app() -> App {
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
    app.add_plugins(GameAiPlugin);
    app.insert_resource(ActiveBot(Box::new(RightRunnerBot::new())));
    app.world_mut()
        .insert_resource(DefaultErrorHandler(diagnostic_handler));
    app.add_systems(OnEnter(AppState::Playing), spawn_ground);
    app.finish();
    app.cleanup();
    app
}

fn transition_to(app: &mut App, state: AppState) {
    app.world_mut()
        .resource_mut::<NextState<AppState>>()
        .set(state);
    app.update();
    app.update();
}

#[test]
fn headless_rightrunner_completes_test_level() {
    let mut app = build_gameplay_test_app();
    app.update();

    transition_to(&mut app, AppState::Playing);

    for _ in 0..500 {
        app.update();
    }

    let metrics = app.world().resource::<RunMetrics>();
    assert!(
        !metrics.path.is_empty(),
        "RunMetrics.path is empty — bot did not run or player was never spawned"
    );
    if metrics.path.len() >= 2 {
        let start_x = metrics.path[0][0];
        let end_x = metrics.path[metrics.path.len() - 1][0];
        assert!(
            end_x > start_x,
            "RightRunnerBot should have moved right: start_x={start_x}, end_x={end_x}"
        );
    }
}
