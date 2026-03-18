use bevy::prelude::*;

mod camera;
mod sprites;

use camera::CameraPlugin;

pub struct GameRenderPlugin;

impl Plugin for GameRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CameraPlugin);
        app.add_plugins(sprites::SpritesPlugin);
    }
}
