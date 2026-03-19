use avian2d::prelude::*;
use bevy::asset::AssetPlugin;
use bevy::ecs::error::{BevyError, DefaultErrorHandler, ErrorContext};
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use game_core::input::{GameInputMessage, InputPlugin};
use game_core::physics::PhysicsPlugin;
use game_core::player::{Player, PlayerPlugin};
use game_core::state::{AppState, StatePlugin};

fn diagnostic_handler(error: BevyError, ctx: ErrorContext) {
    panic!(
        "ECS error in {} `{}`: {}",
        ctx.kind(),
        ctx.name(),
        error
    );
}

fn spawn_ground(mut commands: Commands) {
    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(1000.0, 50.0),
        Transform::from_xyz(0.0, -50.0, 0.0),
    ));
}

fn build_test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(AssetPlugin::default());
    app.add_plugins(bevy::image::ImagePlugin::default());
    app.add_plugins(TransformPlugin);
    app.add_plugins(bevy::scene::ScenePlugin);
    app.add_plugins(StatesPlugin);
    app.add_plugins(StatePlugin);
    app.add_plugins(InputPlugin);
    app.add_plugins(PhysicsPlugin);
    app.add_plugins(PlayerPlugin);
    app.world_mut()
        .insert_resource(DefaultErrorHandler(diagnostic_handler));
    app.add_systems(OnEnter(AppState::Playing), spawn_ground);
    app.finish();
    app.cleanup();
    app
}

fn transition_to_playing(app: &mut App) {
    app.update();
    app.world_mut()
        .resource_mut::<NextState<AppState>>()
        .set(AppState::Playing);
    app.update();
    app.update();
}

fn run_physics_frames(app: &mut App, frames: usize) {
    for _ in 0..frames {
        app.update();
    }
}

#[test]
fn player_spawns_in_playing_state() {
    let mut app = build_test_app();
    transition_to_playing(&mut app);

    let player_count = app
        .world_mut()
        .query_filtered::<Entity, With<Player>>()
        .iter(app.world())
        .count();
    assert_eq!(player_count, 1, "Expected exactly one player entity");
}

#[test]
fn player_moves_right_on_input() {
    let mut app = build_test_app();
    transition_to_playing(&mut app);

    run_physics_frames(&mut app, 200);

    let initial_x = app
        .world_mut()
        .query_filtered::<&Transform, With<Player>>()
        .single(app.world())
        .unwrap()
        .translation
        .x;

    fn send_move_right(mut writer: MessageWriter<GameInputMessage>) {
        writer.write(GameInputMessage {
            move_left: false,
            move_right: true,
            jump_pressed: false,
            jump_held: false,
            dash: false,
            pause: false,
        });
    }

    app.add_systems(PreUpdate, send_move_right);

    run_physics_frames(&mut app, 200);

    let final_x = app
        .world_mut()
        .query_filtered::<&Transform, With<Player>>()
        .single(app.world())
        .unwrap()
        .translation
        .x;

    assert!(
        final_x > initial_x,
        "Player should have moved right: initial_x={initial_x}, final_x={final_x}"
    );
}
