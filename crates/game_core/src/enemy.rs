use bevy::prelude::*;

use crate::state::AppState;

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Debug, Clone, PartialEq)]
pub enum EnemyType {
    Walker,
    Jumper,
    Flyer,
    Spiker,
}

#[derive(Component)]
pub struct PatrolDirection {
    pub right: bool,
}

#[derive(Component)]
pub struct PatrolConfig {
    pub speed: f32,
    pub distance: f32,
    pub origin_x: f32,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            walker_patrol.run_if(in_state(AppState::Playing)),
        );
    }
}

fn walker_patrol(
    mut query: Query<
        (&mut Transform, &mut PatrolDirection, &PatrolConfig),
        (With<Enemy>, With<EnemyType>),
    >,
    time: Res<Time>,
) {
    for (mut transform, mut direction, config) in &mut query {
        let speed = if direction.right {
            config.speed
        } else {
            -config.speed
        };

        transform.translation.x += speed * time.delta_secs();

        if transform.translation.x > config.origin_x + config.distance {
            direction.right = false;
        } else if transform.translation.x < config.origin_x - config.distance {
            direction.right = true;
        }
    }
}
