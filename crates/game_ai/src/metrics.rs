use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use game_core::player::Player;
use game_core::state::{AppState, GameData};

#[derive(Resource, Default, Serialize, Deserialize)]
pub struct RunMetrics {
    pub level: String,
    pub bot: String,
    pub completed: bool,
    pub time_seconds: f64,
    pub deaths: u32,
    pub coins_collected: u32,
    pub coins_total: u32,
    pub path: Vec<[f32; 2]>,
    pub anomalies: Vec<String>,
}

#[derive(Resource)]
pub struct MetricsConfig {
    pub timeout_seconds: f64,
    pub stuck_threshold_seconds: f64,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            timeout_seconds: 300.0,
            stuck_threshold_seconds: 10.0,
        }
    }
}

#[derive(Resource)]
pub struct MetricsOutputPath(pub String);

#[derive(Resource, Default)]
struct RunTimer(f64);

#[derive(Resource)]
struct StuckDetector {
    last_pos: Vec2,
    stuck_time: f64,
}

impl Default for StuckDetector {
    fn default() -> Self {
        Self {
            last_pos: Vec2::ZERO,
            stuck_time: 0.0,
        }
    }
}

#[derive(Resource)]
struct InitialLives(u32);

pub struct MetricsPlugin;

impl Plugin for MetricsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RunMetrics>()
            .init_resource::<MetricsConfig>()
            .init_resource::<RunTimer>()
            .init_resource::<StuckDetector>()
            .add_systems(OnEnter(AppState::Playing), init_metrics)
            .add_systems(
                Update,
                (track_metrics, check_timeout, check_stuck)
                    .chain()
                    .run_if(in_state(AppState::Playing)),
            )
            .add_systems(OnEnter(AppState::LevelComplete), (mark_completed, output_metrics).chain())
            .add_systems(OnEnter(AppState::GameOver), (mark_failed, output_metrics).chain());
    }
}

fn init_metrics(
    mut metrics: ResMut<RunMetrics>,
    mut timer: ResMut<RunTimer>,
    mut stuck: ResMut<StuckDetector>,
    game_data: Res<GameData>,
    mut commands: Commands,
) {
    metrics.level = game_data.current_level.clone();
    metrics.completed = false;
    metrics.time_seconds = 0.0;
    metrics.deaths = 0;
    metrics.coins_collected = 0;
    metrics.path.clear();
    metrics.anomalies.clear();
    timer.0 = 0.0;
    *stuck = StuckDetector::default();
    commands.insert_resource(InitialLives(game_data.lives));
}

fn track_metrics(
    time: Res<Time>,
    mut timer: ResMut<RunTimer>,
    mut metrics: ResMut<RunMetrics>,
    game_data: Res<GameData>,
    initial_lives: Option<Res<InitialLives>>,
    player_query: Query<&Transform, With<Player>>,
) {
    timer.0 += time.delta_secs_f64();
    metrics.time_seconds = timer.0;
    metrics.coins_collected = game_data.coins;

    if let Some(init) = initial_lives {
        let lives_lost = init.0.saturating_sub(game_data.lives);
        metrics.deaths = lives_lost;
    }

    if let Ok(transform) = player_query.single() {
        let pos = transform.translation;
        metrics.path.push([pos.x, pos.y]);
    }
}

fn check_timeout(
    timer: Res<RunTimer>,
    config: Res<MetricsConfig>,
    mut metrics: ResMut<RunMetrics>,
    mut exit_writer: MessageWriter<AppExit>,
) {
    if timer.0 >= config.timeout_seconds {
        metrics.completed = false;
        metrics.anomalies.push(format!(
            "Timeout after {:.1}s (limit: {:.1}s)",
            timer.0, config.timeout_seconds
        ));
        output_metrics_to_stdout(&metrics);
        exit_writer.write(AppExit::Success);
    }
}

fn check_stuck(
    time: Res<Time>,
    config: Res<MetricsConfig>,
    mut stuck: ResMut<StuckDetector>,
    mut metrics: ResMut<RunMetrics>,
    player_query: Query<&Transform, With<Player>>,
    mut exit_writer: MessageWriter<AppExit>,
) {
    let Ok(transform) = player_query.single() else {
        return;
    };

    let pos = Vec2::new(transform.translation.x, transform.translation.y);
    let distance = pos.distance(stuck.last_pos);

    if distance < 1.0 {
        stuck.stuck_time += time.delta_secs_f64();
    } else {
        stuck.stuck_time = 0.0;
        stuck.last_pos = pos;
    }

    if stuck.stuck_time >= config.stuck_threshold_seconds {
        metrics.completed = false;
        metrics.anomalies.push(format!(
            "Bot stuck for {:.1}s at ({:.0}, {:.0})",
            stuck.stuck_time, pos.x, pos.y
        ));
        output_metrics_to_stdout(&metrics);
        exit_writer.write(AppExit::Success);
    }
}

fn mark_completed(mut metrics: ResMut<RunMetrics>) {
    metrics.completed = true;
}

fn mark_failed(mut metrics: ResMut<RunMetrics>) {
    metrics.completed = false;
}

fn output_metrics(
    metrics: Res<RunMetrics>,
    output_path: Option<Res<MetricsOutputPath>>,
) {
    output_metrics_to_stdout(&metrics);

    if let Some(path) = output_path {
        let json = serde_json::to_string_pretty(&*metrics).expect("Failed to serialize metrics");
        std::fs::write(&path.0, json).expect("Failed to write metrics file");
    }
}

fn output_metrics_to_stdout(metrics: &RunMetrics) {
    let json = serde_json::to_string_pretty(metrics).expect("Failed to serialize metrics");
    println!("--- METRICS ---");
    println!("{json}");
    println!("--- END METRICS ---");
}
