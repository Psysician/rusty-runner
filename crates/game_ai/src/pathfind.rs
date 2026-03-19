use bevy::prelude::*;
use crate::bot::BotStrategy;
use crate::world_state::GameWorldState;
use game_core::input::GameInputMessage;

#[derive(Default)]
pub struct PathfindBot {
    last_pos: Option<Vec2>,
}

impl PathfindBot {
    pub fn new() -> Self {
        Self::default()
    }
}

impl BotStrategy for PathfindBot {
    fn decide(&mut self, state: &GameWorldState) -> GameInputMessage {
        let diff = state.goal_pos - state.player_pos;
        let move_right = diff.x > 5.0;
        let move_left = diff.x < -5.0;
        let should_jump = diff.y > 10.0 || state.gap_ahead || state.enemy_ahead;

        let stuck = self.last_pos.is_some_and(|lp| lp.distance(state.player_pos) < 1.0);
        self.last_pos = Some(state.player_pos);

        GameInputMessage {
            move_left,
            move_right,
            jump_pressed: should_jump || stuck,
            jump_held: should_jump || stuck,
            dash: false,
            pause: false,
        }
    }
}
