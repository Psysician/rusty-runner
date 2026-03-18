use bevy::prelude::*;

/// Abstracted game input -- all game systems read this instead of raw keyboard/gamepad.
/// AI bots can inject these directly, bypassing keyboard input.
#[derive(Message, Clone, Debug)]
pub struct GameInputMessage {
    pub move_left: bool,
    pub move_right: bool,
    pub jump_pressed: bool,
    pub jump_held: bool,
    pub dash: bool,
    pub pause: bool,
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<GameInputMessage>();
        app.add_systems(
            PreUpdate,
            gather_keyboard_input.run_if(resource_exists::<ButtonInput<KeyCode>>),
        );
    }
}

fn gather_keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut writer: MessageWriter<GameInputMessage>,
) {
    writer.write(GameInputMessage {
        move_left: keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft),
        move_right: keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight),
        jump_pressed: keys.just_pressed(KeyCode::Space)
            || keys.just_pressed(KeyCode::KeyW)
            || keys.just_pressed(KeyCode::ArrowUp),
        jump_held: keys.pressed(KeyCode::Space)
            || keys.pressed(KeyCode::KeyW)
            || keys.pressed(KeyCode::ArrowUp),
        dash: keys.just_pressed(KeyCode::ShiftLeft)
            || keys.just_pressed(KeyCode::ShiftRight),
        pause: keys.just_pressed(KeyCode::Escape),
    });
}
