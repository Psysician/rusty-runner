use game_ai::bot::{BotStrategy, RightRunnerBot};
use game_ai::world_state::GameWorldState;

#[test]
fn right_runner_always_moves_right() {
    let mut bot = RightRunnerBot::new();
    let state = GameWorldState {
        player_pos: bevy::math::Vec2::new(100.0, 50.0),
        player_velocity: bevy::math::Vec2::ZERO,
        nearby_enemies: vec![],
        ground_ahead: true,
        gap_ahead: false,
        enemy_ahead: false,
        goal_pos: bevy::math::Vec2::new(1000.0, 50.0),
    };
    let input = bot.decide(&state);
    assert!(input.move_right, "RightRunner should always move right");
}

#[test]
fn right_runner_jumps_over_gaps() {
    let mut bot = RightRunnerBot::new();
    let state = GameWorldState {
        player_pos: bevy::math::Vec2::new(100.0, 50.0),
        player_velocity: bevy::math::Vec2::ZERO,
        nearby_enemies: vec![],
        ground_ahead: false,
        gap_ahead: true,
        enemy_ahead: false,
        goal_pos: bevy::math::Vec2::new(1000.0, 50.0),
    };
    let input = bot.decide(&state);
    assert!(input.move_right);
    assert!(input.jump_pressed, "Should jump over gap");
}
