use std::fs;
use std::path::PathBuf;
use bevy::prelude::*;
use crate::state::{AppState, GameData};

const SAVE_FILE: &str = "save.json";

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_save)
            .add_systems(OnEnter(AppState::LevelComplete), save_game);
    }
}

fn save_path() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("rusty-runner")
        .join(SAVE_FILE)
}

fn load_save(mut game_data: ResMut<GameData>) {
    let path = save_path();
    if path.exists() {
        if let Ok(data) = fs::read_to_string(&path) {
            if let Ok(saved) = serde_json::from_str::<GameData>(&data) {
                *game_data = saved;
            }
        }
    }
}

fn save_game(game_data: Res<GameData>) {
    let path = save_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(json) = serde_json::to_string_pretty(&*game_data) {
        let _ = fs::write(&path, json);
    }
}
