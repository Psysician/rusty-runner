use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Loading,
    Playing,
    Paused,
    LevelComplete,
    GameOver,
    Victory,
}

#[derive(SubStates, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[source(AppState = AppState::Playing)]
pub enum GamePhase {
    #[default]
    Active,
    Dying,
}

#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct GameData {
    pub current_level: String,
    pub lives: u32,
    pub score: u32,
    pub coins: u32,
}

impl Default for GameData {
    fn default() -> Self {
        Self {
            current_level: "1-1".into(),
            lives: 3,
            score: 0,
            coins: 0,
        }
    }
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_sub_state::<GamePhase>()
            .init_resource::<GameData>();
    }
}
