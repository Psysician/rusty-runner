use bevy::prelude::*;
use game_core::input::{GameInputMessage, InputPlugin};

#[derive(Resource, Default)]
struct TestCapture(Vec<GameInputMessage>);

fn capture_inputs(mut reader: MessageReader<GameInputMessage>, mut capture: ResMut<TestCapture>) {
    for msg in reader.read() {
        capture.0.push(msg.clone());
    }
}

#[test]
fn keyboard_input_produces_game_input_messages() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(InputPlugin);
    app.init_resource::<TestCapture>();
    app.add_systems(Update, capture_inputs);

    let mut input = ButtonInput::<KeyCode>::default();
    input.press(KeyCode::KeyD);
    app.insert_resource(input);

    app.update();

    let capture = app.world().resource::<TestCapture>();
    assert!(!capture.0.is_empty(), "Expected at least one input message");
    assert!(capture.0[0].move_right, "Expected move_right to be true");
}
