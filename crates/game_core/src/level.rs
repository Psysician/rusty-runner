use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

use crate::state::AppState;

#[derive(Component)]
pub struct PlayerSpawn;

#[derive(Component)]
pub struct LevelGoal;

#[derive(Component)]
pub struct Coin;

#[derive(Component)]
pub struct LevelMap;

#[derive(Resource)]
pub struct CurrentLevel {
    pub path: String,
}

impl Default for CurrentLevel {
    fn default() -> Self {
        Self {
            path: "levels/test-level.tmx".into(),
        }
    }
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentLevel>();
        app.add_plugins(TiledPlugin(TiledPluginConfig {
            tiled_types_export_file: None,
            ..default()
        }));
        app.add_plugins(TiledPhysicsPlugin::<TiledPhysicsAvianBackend>::default());
        app.add_systems(OnEnter(AppState::Loading), load_level);
        app.add_systems(
            Update,
            check_level_loaded.run_if(in_state(AppState::Loading)),
        );
        app.add_systems(
            Update,
            process_tiled_objects.run_if(in_state(AppState::Loading)),
        );
    }
}

fn load_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level: Res<CurrentLevel>,
) {
    commands.spawn((
        LevelMap,
        TiledMap(asset_server.load(&level.path)),
        TiledPhysicsSettings::<TiledPhysicsAvianBackend> {
            objects_layer_filter: TiledFilter::from(vec!["collision"]),
            ..default()
        },
    ));
}

fn check_level_loaded(
    map_query: Query<&TiledMapStorage, With<LevelMap>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for storage in map_query.iter() {
        if storage.layers().next().is_some() {
            next_state.set(AppState::Playing);
        }
    }
}

fn process_tiled_objects(
    mut commands: Commands,
    objects: Query<(Entity, &TiledName), (With<TiledObject>, Without<PlayerSpawn>, Without<LevelGoal>, Without<Coin>, Without<crate::enemy::Enemy>, Without<crate::platforms::MovingPlatform>, Without<crate::wind::WindZone>, Without<crate::items::ItemType>, Without<crate::boss::Boss>)>,
) {
    for (entity, name) in objects.iter() {
        match name.0.as_str() {
            "player_spawn" => {
                commands.entity(entity).insert(PlayerSpawn);
            }
            "goal" => {
                commands.entity(entity).insert((
                    LevelGoal,
                    Collider::rectangle(32.0, 64.0),
                    Sensor,
                    CollisionEventsEnabled,
                ));
            }
            n if n.starts_with("coin") => {
                commands.entity(entity).insert((
                    Coin,
                    Collider::rectangle(16.0, 16.0),
                    Sensor,
                    CollisionEventsEnabled,
                ));
            }
            n if n.starts_with("enemy_walker") => {
                commands.entity(entity).insert((
                    crate::enemy::Enemy,
                    crate::enemy::EnemyType::Walker,
                    crate::enemy::PatrolDirection { right: true },
                    crate::enemy::PatrolConfig {
                        speed: 80.0,
                        distance: 100.0,
                        origin_x: 0.0,
                    },
                    avian2d::prelude::RigidBody::Kinematic,
                    avian2d::prelude::Collider::rectangle(24.0, 24.0),
                    CollisionEventsEnabled,
                ));
            }
            n if n.starts_with("enemy_jumper") => {
                commands.entity(entity).insert((
                    crate::enemy::Enemy,
                    crate::enemy::EnemyType::Jumper,
                    crate::enemy::JumperTimer {
                        timer: bevy::time::Timer::from_seconds(
                            2.0,
                            bevy::time::TimerMode::Repeating,
                        ),
                    },
                    RigidBody::Dynamic,
                    Collider::rectangle(24.0, 24.0),
                    LockedAxes::ROTATION_LOCKED,
                    CollisionEventsEnabled,
                ));
            }
            n if n.starts_with("enemy_flyer") => {
                commands.entity(entity).insert((
                    crate::enemy::Enemy,
                    crate::enemy::EnemyType::Flyer,
                    crate::enemy::FlyerState {
                        elapsed: 0.0,
                        amplitude: 40.0,
                        frequency: 2.0,
                        base_y: 0.0,
                        speed_x: 50.0,
                    },
                    RigidBody::Kinematic,
                    Collider::rectangle(24.0, 24.0),
                    CollisionEventsEnabled,
                ));
            }
            n if n.starts_with("enemy_spiker") => {
                commands.entity(entity).insert((
                    crate::enemy::Enemy,
                    crate::enemy::EnemyType::Spiker,
                    RigidBody::Static,
                    Collider::rectangle(24.0, 24.0),
                    CollisionEventsEnabled,
                ));
            }
            n if n.starts_with("moving_platform") => {
                commands.entity(entity).insert((
                    crate::platforms::MovingPlatform {
                        points: vec![
                            Vec2::new(0.0, 0.0),
                            Vec2::new(200.0, 0.0),
                        ],
                        speed: 60.0,
                        current_target: 0,
                    },
                    RigidBody::Kinematic,
                    Collider::rectangle(128.0, 16.0),
                ));
            }
            n if n.starts_with("powerup_growth") => {
                commands.entity(entity).insert((
                    crate::items::ItemType::Growth,
                    Collider::rectangle(20.0, 20.0),
                    Sensor,
                    CollisionEventsEnabled,
                ));
            }
            n if n.starts_with("powerup_dash") => {
                commands.entity(entity).insert((
                    crate::items::ItemType::Special,
                    Collider::rectangle(20.0, 20.0),
                    Sensor,
                    CollisionEventsEnabled,
                ));
            }
            n if n.starts_with("powerup_invincible") => {
                commands.entity(entity).insert((
                    crate::items::ItemType::Invincible,
                    Collider::rectangle(20.0, 20.0),
                    Sensor,
                    CollisionEventsEnabled,
                ));
            }
            n if n.starts_with("wind") => {
                commands.entity(entity).insert((
                    crate::wind::WindZone {
                        force: Vec2::new(0.0, 500.0),
                    },
                    Collider::rectangle(100.0, 200.0),
                    Sensor,
                    CollisionEventsEnabled,
                ));
            }
            "boss_spawn" => {
                commands.entity(entity).insert((
                    crate::boss::Boss { hp: 3, max_hp: 3 },
                    crate::boss::BossPhase::Charging { direction: 1.0 },
                    crate::boss::BossArena { left: -300.0, right: 300.0, ground_y: -50.0 },
                    crate::enemy::Enemy,
                    RigidBody::Dynamic,
                    Collider::rectangle(48.0, 48.0),
                    LockedAxes::ROTATION_LOCKED,
                    CollisionEventsEnabled,
                ));
            }
            _ => {}
        }
    }
}
