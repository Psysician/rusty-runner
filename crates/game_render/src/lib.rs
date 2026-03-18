use bevy::prelude::*;

mod audio;
mod camera;
mod hud;
mod menus;
mod sprites;

use camera::CameraPlugin;

pub struct GameRenderPlugin;

impl Plugin for GameRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(audio::GameAudioPlugin);
        app.add_plugins(CameraPlugin);
        app.add_plugins(hud::HudPlugin);
        app.add_plugins(menus::MenusPlugin);
        app.add_plugins(sprites::SpritesPlugin);
    }
}
