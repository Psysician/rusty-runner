use bevy::prelude::*;
use crate::state::AppState;

#[derive(Component)]
pub struct MovingPlatform {
    pub points: Vec<Vec2>,
    pub speed: f32,
    pub current_target: usize,
}

#[derive(Component)]
pub struct OneWayPlatform;

pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            move_platforms.run_if(in_state(AppState::Playing)),
        );
    }
}

fn move_platforms(
    mut query: Query<(&mut Transform, &mut MovingPlatform)>,
    time: Res<Time>,
) {
    for (mut transform, mut platform) in &mut query {
        let target = platform.points[platform.current_target];
        let current = transform.translation.truncate();
        let direction = (target - current).normalize_or_zero();
        let distance = current.distance(target);

        if distance < 2.0 {
            platform.current_target = (platform.current_target + 1) % platform.points.len();
        } else {
            let movement = direction * platform.speed * time.delta_secs();
            transform.translation.x += movement.x;
            transform.translation.y += movement.y;
        }
    }
}
