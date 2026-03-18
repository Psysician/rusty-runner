use avian2d::prelude::*;
use bevy::prelude::*;

use crate::enemy::Enemy;
use crate::state::GamePhase;

const BOSS_BASE_SPEED: f32 = 250.0;
const BOSS_SPEED_SCALE_PER_DAMAGE: f32 = 50.0;
const BOSS_PAUSE_DURATION: f32 = 1.5;
const BOSS_JUMP_VELOCITY: f32 = 600.0;
const WALKER_SPAWN_INTERVAL: f32 = 5.0;
const SHOCKWAVE_LIFETIME: f32 = 0.5;
const SHOCKWAVE_WIDTH: f32 = 200.0;
const SHOCKWAVE_HEIGHT: f32 = 16.0;

#[derive(Component)]
pub struct Boss {
    pub hp: u32,
    pub max_hp: u32,
}

#[derive(Component)]
pub enum BossPhase {
    Charging { direction: f32 },
    PausedAtWall { timer: Timer },
    Jumping,
}

#[derive(Component)]
pub struct BossArena {
    pub left: f32,
    pub right: f32,
    pub ground_y: f32,
}

#[derive(Component)]
pub struct Shockwave {
    pub timer: Timer,
}

pub struct BossPlugin;

impl Plugin for BossPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (boss_movement, boss_phase_transitions, boss_spawn_walkers, boss_shockwave)
                .chain()
                .run_if(in_state(GamePhase::Active)),
        );
    }
}

fn boss_movement(
    mut query: Query<(&Boss, &BossPhase, &mut LinearVelocity)>,
) {
    for (boss, phase, mut velocity) in query.iter_mut() {
        let damage_taken = boss.max_hp.saturating_sub(boss.hp);
        let speed = BOSS_BASE_SPEED + damage_taken as f32 * BOSS_SPEED_SCALE_PER_DAMAGE;

        match phase {
            BossPhase::Charging { direction } => {
                velocity.x = direction * speed;
            }
            BossPhase::PausedAtWall { .. } => {
                velocity.x = 0.0;
            }
            BossPhase::Jumping => {
                // Horizontal velocity maintained from before jump, vertical handled by physics
            }
        }
    }
}

fn boss_phase_transitions(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &Boss, &mut BossPhase, &BossArena, &Transform, &LinearVelocity)>,
) {
    for (entity, boss, mut phase, arena, transform, velocity) in query.iter_mut() {
        let x = transform.translation.x;
        let y = transform.translation.y;

        match phase.as_mut() {
            BossPhase::Charging { direction: _ } => {
                let at_left = x <= arena.left;
                let at_right = x >= arena.right;

                if at_left || at_right {
                    *phase = BossPhase::PausedAtWall {
                        timer: Timer::from_seconds(BOSS_PAUSE_DURATION, TimerMode::Once),
                    };
                }
            }
            BossPhase::PausedAtWall { timer } => {
                timer.tick(time.delta());
                if timer.just_finished() {
                    let at_left = x <= (arena.left + arena.right) / 2.0;
                    let new_direction = if at_left { 1.0 } else { -1.0 };

                    if boss.hp == 1 {
                        let nanos = time.elapsed().as_nanos();
                        if nanos % 3 == 0 {
                            *phase = BossPhase::Jumping;
                            commands.entity(entity).insert(
                                LinearVelocity(Vec2::new(new_direction * (BOSS_BASE_SPEED + (boss.max_hp.saturating_sub(boss.hp)) as f32 * BOSS_SPEED_SCALE_PER_DAMAGE), BOSS_JUMP_VELOCITY)),
                            );
                            continue;
                        }
                    }

                    *phase = BossPhase::Charging { direction: new_direction };
                }
            }
            BossPhase::Jumping => {
                let near_ground = y <= arena.ground_y + 5.0;
                let falling = velocity.y <= 0.0;
                if near_ground && falling {
                    let landing_x = x;
                    commands.spawn((
                        Shockwave {
                            timer: Timer::from_seconds(SHOCKWAVE_LIFETIME, TimerMode::Once),
                        },
                        Transform::from_xyz(landing_x, arena.ground_y, 0.0),
                        Collider::rectangle(SHOCKWAVE_WIDTH, SHOCKWAVE_HEIGHT),
                        Sensor,
                        CollisionEventsEnabled,
                    ));

                    let at_left = x <= (arena.left + arena.right) / 2.0;
                    let new_direction = if at_left { 1.0 } else { -1.0 };
                    *phase = BossPhase::Charging { direction: new_direction };
                }
            }
        }
    }
}

fn boss_spawn_walkers(
    mut commands: Commands,
    time: Res<Time>,
    query: Query<(&Boss, &BossArena)>,
    mut spawn_timer: Local<Option<Timer>>,
) {
    for (boss, arena) in query.iter() {
        if boss.hp > boss.max_hp.saturating_sub(1) {
            continue;
        }

        let timer = spawn_timer.get_or_insert_with(|| {
            Timer::from_seconds(WALKER_SPAWN_INTERVAL, TimerMode::Repeating)
        });
        timer.tick(time.delta());

        if timer.just_finished() {
            let spawn_x = if time.elapsed().as_nanos() % 2 == 0 {
                arena.left
            } else {
                arena.right
            };

            commands.spawn((
                Enemy,
                crate::enemy::EnemyType::Walker,
                crate::enemy::PatrolDirection { right: spawn_x < 0.0 },
                crate::enemy::PatrolConfig {
                    speed: 80.0,
                    distance: (arena.right - arena.left) / 2.0,
                    origin_x: (arena.left + arena.right) / 2.0,
                },
                Transform::from_xyz(spawn_x, arena.ground_y + 20.0, 0.0),
                RigidBody::Kinematic,
                Collider::rectangle(24.0, 24.0),
                CollisionEventsEnabled,
            ));
        }
    }
}

fn boss_shockwave(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Shockwave)>,
) {
    for (entity, mut shockwave) in query.iter_mut() {
        shockwave.timer.tick(time.delta());
        if shockwave.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
