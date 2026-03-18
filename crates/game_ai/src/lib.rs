use bevy::prelude::*;
use game_core::input::GameInputMessage;
use game_core::state::AppState;

pub mod bot;
pub mod metrics;
pub mod pathfind;
pub mod replay;
pub mod world_state;

use world_state::{GameWorldStateResource, build_world_state};

#[derive(Resource)]
pub struct ActiveBot(pub Box<dyn bot::BotStrategy>);

pub struct GameAiPlugin;

impl Plugin for GameAiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(metrics::MetricsPlugin)
            .init_resource::<GameWorldStateResource>()
            .add_systems(
                Update,
                (build_world_state, run_bot_decision)
                    .chain()
                    .run_if(in_state(AppState::Playing))
                    .run_if(resource_exists::<ActiveBot>),
            );
    }
}

fn run_bot_decision(
    mut bot: ResMut<ActiveBot>,
    world_state: Res<GameWorldStateResource>,
    mut writer: MessageWriter<GameInputMessage>,
) {
    let input = bot.0.decide(&world_state.0);
    writer.write(input);
}
