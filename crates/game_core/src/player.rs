use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua::builtins::{TnuaBuiltinJumpConfig, TnuaBuiltinWalkConfig};
use bevy_tnua_avian2d::prelude::*;

use crate::input::GameInputMessage;
use crate::state::{AppState, GameData, GamePhase};

#[derive(Component, Debug, Clone, PartialEq, Default)]
pub enum PowerUpState {
    #[default]
    Small,
    Big,
    Dash,
    Invincible,
}

#[derive(Component)]
pub struct DashCooldown {
    pub timer: Timer,
    pub available: bool,
}

#[derive(Component)]
pub struct InvincibilityTimer {
    pub timer: Timer,
}

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
pub struct PlayerConfig {
    pub move_speed: f32,
    pub jump_height: f32,
    pub float_height: f32,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self {
            move_speed: 300.0,
            jump_height: 200.0,
            float_height: 20.0,
        }
    }
}

#[derive(TnuaScheme)]
#[scheme(basis = TnuaBuiltinWalk)]
pub enum PlayerScheme {
    Jump(TnuaBuiltinJump),
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerConfig>();
        app.add_plugins(TnuaControllerPlugin::<PlayerScheme>::new(PhysicsSchedule));
        app.add_plugins(TnuaAvian2dPlugin::new(PhysicsSchedule));
        app.add_systems(OnEnter(AppState::Playing), spawn_player);
        app.add_systems(
            PhysicsSchedule,
            player_movement.in_set(TnuaUserControlsSystems),
        );
        app.add_systems(
            Update,
            check_fall_death.run_if(in_state(GamePhase::Active)),
        );
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<PlayerConfig>,
    mut configs: ResMut<Assets<PlayerSchemeConfig>>,
    spawn_query: Query<&Transform, With<crate::level::PlayerSpawn>>,
) {
    let spawn_pos = spawn_query
        .iter()
        .next()
        .map(|t| t.translation)
        .unwrap_or(Vec3::new(0.0, 100.0, 0.0));

    let config_handle = configs.add(PlayerSchemeConfig {
        basis: TnuaBuiltinWalkConfig {
            float_height: config.float_height,
            speed: config.move_speed,
            ..default()
        },
        jump: TnuaBuiltinJumpConfig {
            height: config.jump_height,
            ..default()
        },
    });

    commands.spawn((
        Player,
        PowerUpState::default(),
        RigidBody::Dynamic,
        Collider::rectangle(16.0, 24.0),
        LockedAxes::ROTATION_LOCKED,
        TnuaController::<PlayerScheme>::default(),
        TnuaConfig::<PlayerScheme>(config_handle),
        Transform::from_xyz(spawn_pos.x, spawn_pos.y, spawn_pos.z),
        Sprite {
            image: asset_server.load("sprites/player.png"),
            custom_size: Some(Vec2::new(16.0, 24.0)),
            ..default()
        },
        DespawnOnExit(AppState::Playing),
    ));
}

const FALL_DEATH_Y: f32 = -500.0;

fn check_fall_death(
    player_query: Query<&Transform, With<Player>>,
    mut game_data: ResMut<GameData>,
    mut next_state: ResMut<NextState<AppState>>,
    mut next_phase: ResMut<NextState<GamePhase>>,
) {
    let Ok(transform) = player_query.single() else {
        return;
    };

    if transform.translation.y < FALL_DEATH_Y {
        if game_data.lives > 0 {
            game_data.lives -= 1;
        }
        if game_data.lives == 0 {
            next_state.set(AppState::GameOver);
        } else {
            next_phase.set(GamePhase::Dying);
        }
    }
}

fn player_movement(
    mut reader: MessageReader<GameInputMessage>,
    mut query: Query<&mut TnuaController<PlayerScheme>, With<Player>>,
) {
    let mut move_dir = 0.0f32;
    let mut jump_pressed = false;
    let mut jump_held = false;

    for msg in reader.read() {
        if msg.move_left {
            move_dir -= 1.0;
        }
        if msg.move_right {
            move_dir += 1.0;
        }
        if msg.jump_pressed {
            jump_pressed = true;
        }
        if msg.jump_held {
            jump_held = true;
        }
    }

    for mut controller in query.iter_mut() {
        controller.basis = TnuaBuiltinWalk {
            desired_motion: bevy_tnua::math::Vector3::new(move_dir, 0.0, 0.0),
            ..default()
        };

        controller.initiate_action_feeding();
        if jump_pressed || jump_held {
            controller.action(PlayerScheme::Jump(TnuaBuiltinJump::default()));
        }
    }
}
