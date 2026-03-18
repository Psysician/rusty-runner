use bevy::prelude::*;
use clap::Parser;
use game_core::GameCorePlugin;
use game_ai::{GameAiPlugin, ActiveBot};
use game_ai::bot::{RightRunnerBot, RandomBot};

#[derive(Parser)]
struct Args {
    #[arg(long)]
    level: Option<String>,
    #[arg(long, default_value = "rightrunner")]
    bot: String,
    #[arg(long)]
    all_levels: bool,
    #[arg(long)]
    output: Option<String>,
    #[arg(long, default_value_t = 1)]
    runs: u32,
}

fn main() {
    let args = Args::parse();
    let bot: Box<dyn game_ai::bot::BotStrategy> = match args.bot.as_str() {
        "rightrunner" => Box::new(RightRunnerBot::new()),
        "random" => Box::new(RandomBot::new(42)),
        "pathfind" => Box::new(game_ai::pathfind::PathfindBot::new()),
        _ => {
            eprintln!("Unknown bot: {}. Available: rightrunner, random, pathfind", args.bot);
            std::process::exit(1);
        }
    };

    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(AssetPlugin::default())
        .add_plugins(bevy::state::app::StatesPlugin)
        .add_plugins(GameCorePlugin)
        .add_plugins(GameAiPlugin)
        .insert_resource(ActiveBot(bot))
        .run();
}
