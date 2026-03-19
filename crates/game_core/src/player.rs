use avian2d::prelude::*;
use bevy::prelude::*;

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

#[derive(Component)]
pub struct Grounded(pub bool);

const MOVE_SPEED: f32 = 300.0;
const JUMP_VELOCITY: f32 = 500.0;
const GROUND_Y: f32 = 0.0;
const PLAYER_HALF_H: f32 = 12.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playing), spawn_player);
        app.add_systems(
            FixedUpdate,
            (player_movement, check_grounded)
                .chain()
                .run_if(in_state(AppState::Playing)),
        );
        app.add_systems(
            Update,
            check_fall_death.run_if(in_state(GamePhase::Active)),
        );
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Player
    commands.spawn((
        Player,
        Grounded(false),
        PowerUpState::default(),
        RigidBody::Dynamic,
        Collider::rectangle(16.0, 24.0),
        LockedAxes::ROTATION_LOCKED,
        LinearVelocity::default(),
        Transform::from_xyz(-400.0, 30.0, 1.0),
        Sprite {
            image: asset_server.load("sprites/player.png"),
            custom_size: Some(Vec2::new(32.0, 48.0)),
            ..default()
        },
        DespawnOnExit(AppState::Playing),
    ));

    // Ground — flat brown path
    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(1200.0, 200.0),
        Transform::from_xyz(0.0, -100.0, 0.0),
        Sprite {
            color: Color::srgb(0.45, 0.3, 0.15),
            custom_size: Some(Vec2::new(1200.0, 200.0)),
            ..default()
        },
        DespawnOnExit(AppState::Playing),
    ));

    // Ant enemy — bounces left/right
    commands.spawn((
        crate::enemy::Enemy,
        crate::enemy::EnemyType::Walker,
        crate::enemy::PatrolDirection { right: true },
        crate::enemy::PatrolConfig {
            speed: 80.0,
            distance: 150.0,
            origin_x: 200.0,
        },
        RigidBody::Kinematic,
        Collider::rectangle(24.0, 24.0),
        CollisionEventsEnabled,
        Transform::from_xyz(200.0, 12.0, 1.0),
        Sprite {
            image: asset_server.load("sprites/enemy_walker.png"),
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        DespawnOnExit(AppState::Playing),
    ));

    // Second ant
    commands.spawn((
        crate::enemy::Enemy,
        crate::enemy::EnemyType::Walker,
        crate::enemy::PatrolDirection { right: false },
        crate::enemy::PatrolConfig {
            speed: 60.0,
            distance: 100.0,
            origin_x: 0.0,
        },
        RigidBody::Kinematic,
        Collider::rectangle(24.0, 24.0),
        CollisionEventsEnabled,
        Transform::from_xyz(0.0, 12.0, 1.0),
        Sprite {
            image: asset_server.load("sprites/enemy_walker.png"),
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        DespawnOnExit(AppState::Playing),
    ));

    // Goal
    commands.spawn((
        crate::level::LevelGoal,
        Collider::rectangle(32.0, 64.0),
        Sensor,
        CollisionEventsEnabled,
        Transform::from_xyz(500.0, 32.0, 1.0),
        Sprite {
            image: asset_server.load("sprites/goal.png"),
            custom_size: Some(Vec2::new(32.0, 64.0)),
            ..default()
        },
        DespawnOnExit(AppState::Playing),
    ));

    // Coin
    commands.spawn((
        crate::level::Coin,
        Collider::rectangle(16.0, 16.0),
        Sensor,
        CollisionEventsEnabled,
        Transform::from_xyz(-100.0, 40.0, 1.0),
        Sprite {
            image: asset_server.load("sprites/coin.png"),
            custom_size: Some(Vec2::new(16.0, 16.0)),
            ..default()
        },
        DespawnOnExit(AppState::Playing),
    ));
}

fn check_grounded(mut query: Query<(&Transform, &mut Grounded), With<Player>>) {
    for (transform, mut grounded) in &mut query {
        grounded.0 = transform.translation.y <= GROUND_Y + PLAYER_HALF_H + 2.0;
    }
}

fn player_movement(
    keys: Option<Res<ButtonInput<KeyCode>>>,
    mut reader: MessageReader<GameInputMessage>,
    mut query: Query<(&mut LinearVelocity, &Grounded), With<Player>>,
) {
    let mut move_dir = 0.0f32;
    let mut jump = false;

    // AI bot messages
    for msg in reader.read() {
        if msg.move_left { move_dir -= 1.0; }
        if msg.move_right { move_dir += 1.0; }
        if msg.jump_pressed { jump = true; }
    }

    // Direct keyboard
    if let Some(keys) = &keys {
        if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
            move_dir -= 1.0;
        }
        if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
            move_dir += 1.0;
        }
        if keys.just_pressed(KeyCode::Space)
            || keys.just_pressed(KeyCode::KeyW)
            || keys.just_pressed(KeyCode::ArrowUp)
        {
            jump = true;
        }
    }

    move_dir = move_dir.clamp(-1.0, 1.0);

    for (mut velocity, grounded) in &mut query {
        velocity.x = move_dir * MOVE_SPEED;
        if jump && grounded.0 {
            velocity.y = JUMP_VELOCITY;
        }
    }
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
