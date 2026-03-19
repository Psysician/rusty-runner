mod common;

use bevy::prelude::*;
use common::{build_full_test_app, set_state};
use game_core::state::AppState;

fn assert_current_state(app: &App, expected: AppState) {
    assert_eq!(
        *app.world().resource::<State<AppState>>().get(),
        expected,
        "Expected state {expected:?}"
    );
}

/// Walks through a realistic state sequence, asserting correct state after each transition.
#[test]
fn state_machine_fuzz_all_transitions() {
    let mut app = build_full_test_app();
    app.update();

    let transitions = [
        AppState::MainMenu,
        AppState::Loading,
        AppState::Playing,
        AppState::Paused,
        AppState::Playing,
        AppState::LevelComplete,
        AppState::MainMenu,
        AppState::Loading,
        AppState::Playing,
        AppState::GameOver,
        AppState::MainMenu,
        AppState::Loading,
        AppState::Playing,
        AppState::Victory,
    ];

    for state in transitions {
        set_state(&mut app, state.clone());
        for _ in 0..3 {
            app.update();
        }
        assert_current_state(&app, state);
    }
}

/// Tests that rapid state changes (faster than one full transition cycle) don't panic.
/// Intentionally uses a single update() per transition so OnEnter systems are not
/// expected to complete — this is a stress test for the state machine itself.
#[test]
fn rapid_state_cycling_no_panic() {
    let mut app = build_full_test_app();
    app.update();

    let cycle = [
        AppState::MainMenu,
        AppState::Loading,
        AppState::Playing,
        AppState::Paused,
        AppState::Playing,
        AppState::LevelComplete,
        AppState::MainMenu,
        AppState::Loading,
        AppState::Playing,
        AppState::GameOver,
        AppState::MainMenu,
        AppState::Loading,
        AppState::Playing,
        AppState::Victory,
    ];

    for _ in 0..3 {
        for state in &cycle {
            app.world_mut()
                .resource_mut::<NextState<AppState>>()
                .set(state.clone());
            app.update(); // deliberate: only one frame per transition
        }
    }
}
