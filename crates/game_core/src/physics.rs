use avian2d::prelude::*;
use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            avian2d::PhysicsPlugins::default()
                .with_length_unit(100.0),
        )
        .insert_resource(Gravity(Vec2::NEG_Y * 1800.0));
    }
}
