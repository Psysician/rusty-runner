use avian2d::prelude::*;
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

#[derive(Component)]
pub struct JumperTimer {
    pub timer: Timer,
}

#[derive(Component)]
pub struct FlyerState {
    pub elapsed: f32,
    pub amplitude: f32,
    pub frequency: f32,
    pub base_y: f32,
    pub speed_x: f32,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                walker_patrol,
                jumper_behavior,
                flyer_behavior,
            )
                .run_if(in_state(AppState::Playing)),
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

fn jumper_behavior(
    mut query: Query<(&mut LinearVelocity, &mut JumperTimer), With<Enemy>>,
    time: Res<Time>,
) {
    for (mut velocity, mut jumper) in &mut query {
        jumper.timer.tick(time.delta());
        if jumper.timer.just_finished() {
            velocity.y = 400.0;
        }
    }
}

fn flyer_behavior(
    mut query: Query<(&mut Transform, &mut FlyerState), With<Enemy>>,
    time: Res<Time>,
) {
    for (mut transform, mut flyer) in &mut query {
        flyer.elapsed += time.delta_secs();
        transform.translation.y =
            flyer.base_y + (flyer.elapsed * flyer.frequency).sin() * flyer.amplitude;
        transform.translation.x += flyer.speed_x * time.delta_secs();
    }
}
