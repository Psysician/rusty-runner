use bevy::prelude::*;
use game_core::GameCorePlugin;
use game_core::level::CurrentLevel;
use game_core::state::AppState;
use game_ai::{GameAiPlugin, ActiveBot};
use game_ai::bot::RightRunnerBot;

fn run_level(level_path: &str) -> bool {
    let bot = Box::new(RightRunnerBot::new());
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(AssetPlugin::default());
    app.add_plugins(bevy::state::app::StatesPlugin);
    app.add_plugins(GameCorePlugin);
    app.add_plugins(GameAiPlugin);
    app.insert_resource(ActiveBot(bot));
    app.insert_resource(CurrentLevel {
        path: format!("levels/{level_path}.tmx"),
    });

    // Transition to Loading
    app.update();
    app.world_mut()
        .resource_mut::<NextState<AppState>>()
        .set(AppState::Loading);

    for _ in 0..10000 {
        app.update();
        let state = app.world().resource::<State<AppState>>();
        match state.get() {
            AppState::LevelComplete | AppState::Victory => return true,
            AppState::GameOver => return false,
            _ => {}
        }
    }
    false
}

fn main() {
    let levels = vec!["test-level"];
    let mut failures = vec![];

    for level in &levels {
        eprint!("Testing level {level}... ");
        if run_level(level) {
            eprintln!("PASS");
        } else {
            eprintln!("FAIL");
            failures.push(level.to_string());
        }
    }

    eprintln!("\n--- Results ---");
    eprintln!("{}/{} levels passed", levels.len() - failures.len(), levels.len());

    if !failures.is_empty() {
        eprintln!("Failed levels: {:?}", failures);
        std::process::exit(1);
    }
}
