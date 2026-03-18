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
        if !storage.layers().next().is_none() {
            next_state.set(AppState::Playing);
        }
    }
}

fn process_tiled_objects(
    mut commands: Commands,
    objects: Query<(Entity, &TiledName), (With<TiledObject>, Without<PlayerSpawn>, Without<LevelGoal>, Without<Coin>, Without<crate::enemy::Enemy>)>,
) {
    for (entity, name) in objects.iter() {
        match name.0.as_str() {
            "player_spawn" => {
                commands.entity(entity).insert(PlayerSpawn);
            }
            "goal" => {
                commands.entity(entity).insert(LevelGoal);
            }
            n if n.starts_with("coin") => {
                commands.entity(entity).insert(Coin);
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
                ));
            }
            _ => {}
        }
    }
}
