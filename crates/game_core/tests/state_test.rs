use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use game_core::state::{AppState, StatePlugin};

#[test]
fn initial_state_is_main_menu() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(StatesPlugin);
    app.add_plugins(StatePlugin);
    app.update();

    let state = app.world().resource::<State<AppState>>();
    assert_eq!(*state.get(), AppState::MainMenu);
}

#[test]
fn transition_to_playing_activates_game_phase() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(StatesPlugin);
    app.add_plugins(StatePlugin);
    app.update();

    app.world_mut()
        .resource_mut::<NextState<AppState>>()
        .set(AppState::Playing);
    app.update();
    app.update();

    let state = app.world().resource::<State<AppState>>();
    assert_eq!(*state.get(), AppState::Playing);
}
