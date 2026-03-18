use game_core::input::GameInputMessage;
use crate::world_state::GameWorldState;

pub trait BotStrategy: Send + Sync {
    fn decide(&mut self, state: &GameWorldState) -> GameInputMessage;
}

pub struct RightRunnerBot;

impl RightRunnerBot {
    pub fn new() -> Self { Self }
}

impl BotStrategy for RightRunnerBot {
    fn decide(&mut self, state: &GameWorldState) -> GameInputMessage {
        let should_jump = state.gap_ahead || state.enemy_ahead;
        GameInputMessage {
            move_left: false,
            move_right: true,
            jump_pressed: should_jump,
            jump_held: should_jump,
            dash: false,
            pause: false,
        }
    }
}

pub struct RandomBot {
    rng_state: u64,
}

impl RandomBot {
    pub fn new(seed: u64) -> Self { Self { rng_state: seed } }
    fn next_bool(&mut self) -> bool {
        self.rng_state ^= self.rng_state << 13;
        self.rng_state ^= self.rng_state >> 7;
        self.rng_state ^= self.rng_state << 17;
        self.rng_state % 2 == 0
    }
}

impl BotStrategy for RandomBot {
    fn decide(&mut self, _state: &GameWorldState) -> GameInputMessage {
        GameInputMessage {
            move_left: self.next_bool(),
            move_right: self.next_bool(),
            jump_pressed: self.next_bool(),
            jump_held: self.next_bool(),
            dash: self.next_bool(),
            pause: false,
        }
    }
}
