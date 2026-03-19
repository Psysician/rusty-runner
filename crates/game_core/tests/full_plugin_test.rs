mod common;

use common::{build_full_test_app, set_state};
use game_core::state::AppState;

/// Smoke test: all gameplay plugins boot without ECS panics.
#[test]
fn full_game_core_plugin_boots_without_panic() {
    let mut app = build_full_test_app();
    for _ in 0..10 {
        app.update();
    }
}

/// Verifies no query conflicts when Playing state activates all gameplay systems.
#[test]
fn full_game_core_plugin_survives_playing_state() {
    let mut app = build_full_test_app();
    app.update();
    set_state(&mut app, AppState::Playing);
    for _ in 0..20 {
        app.update();
    }
}

/// Cycles through every AppState variant to catch state-transition panics.
#[test]
fn full_game_core_plugin_all_states() {
    let mut app = build_full_test_app();
    app.update();

    let states = [
        AppState::MainMenu,
        AppState::Loading,
        AppState::Playing,
        AppState::Paused,
        AppState::Playing,
        AppState::LevelComplete,
        AppState::GameOver,
        AppState::Victory,
    ];

    for state in states {
        set_state(&mut app, state);
        for _ in 0..5 {
            app.update();
        }
    }
}
