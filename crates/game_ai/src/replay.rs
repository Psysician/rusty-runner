use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use game_core::input::GameInputMessage;
use game_core::state::AppState;

#[derive(Resource, Default, Serialize, Deserialize)]
pub struct ReplayData {
    pub frames: Vec<RecordedFrame>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RecordedFrame {
    pub tick: u64,
    pub input: SerializableInput,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SerializableInput {
    pub move_left: bool,
    pub move_right: bool,
    pub jump_pressed: bool,
    pub jump_held: bool,
    pub dash: bool,
}

#[derive(Resource, Default)]
struct FrameCounter(u64);

pub struct ReplayRecordPlugin;

impl Plugin for ReplayRecordPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ReplayData>()
            .init_resource::<FrameCounter>()
            .add_systems(
                Update,
                record_frame.run_if(in_state(AppState::Playing)),
            );
    }
}

fn record_frame(
    mut reader: MessageReader<GameInputMessage>,
    mut replay: ResMut<ReplayData>,
    mut counter: ResMut<FrameCounter>,
) {
    for input in reader.read() {
        replay.frames.push(RecordedFrame {
            tick: counter.0,
            input: SerializableInput {
                move_left: input.move_left,
                move_right: input.move_right,
                jump_pressed: input.jump_pressed,
                jump_held: input.jump_held,
                dash: input.dash,
            },
        });
    }
    counter.0 += 1;
}
