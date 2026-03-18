use avian2d::prelude::*;
use bevy::prelude::*;

use crate::enemy::Enemy;
use crate::level::{Coin, LevelGoal};
use crate::player::Player;
use crate::state::{AppState, GameData, GamePhase};

const STOMP_TOLERANCE: f32 = 6.0;
const STOMP_BOUNCE_VELOCITY: f32 = 500.0;
const PLAYER_HALF_HEIGHT: f32 = 12.0;
const ENEMY_HALF_HEIGHT: f32 = 12.0;
const COIN_SCORE: u32 = 100;

#[derive(Message, Clone, Debug)]
pub struct PlayerStompedEnemy {
    pub enemy: Entity,
}

#[derive(Message, Clone, Debug)]
pub struct PlayerHitByEnemy {
    pub enemy: Entity,
}

#[derive(Message, Clone, Debug)]
pub struct PlayerCollectedCoin {
    pub coin: Entity,
}

#[derive(Message, Clone, Debug)]
pub struct PlayerReachedGoal {
    pub goal: Entity,
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<PlayerStompedEnemy>();
        app.add_message::<PlayerHitByEnemy>();
        app.add_message::<PlayerCollectedCoin>();
        app.add_message::<PlayerReachedGoal>();

        app.add_systems(
            Update,
            (
                detect_player_enemy_collision,
                detect_coin_collection,
                detect_goal_reached,
            )
                .run_if(in_state(GamePhase::Active)),
        );
        app.add_systems(
            Update,
            (
                handle_stomp,
                handle_player_hit,
            )
                .run_if(in_state(GamePhase::Active)),
        );
    }
}

fn detect_player_enemy_collision(
    collisions: Collisions,
    player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    mut stomp_writer: MessageWriter<PlayerStompedEnemy>,
    mut hit_writer: MessageWriter<PlayerHitByEnemy>,
) {
    let Ok((player_entity, player_transform)) = player_query.single() else {
        return;
    };

    for (enemy_entity, enemy_transform) in enemy_query.iter() {
        if !collisions.contains(player_entity, enemy_entity) {
            continue;
        }

        let player_bottom = player_transform.translation.y - PLAYER_HALF_HEIGHT;
        let enemy_top = enemy_transform.translation.y + ENEMY_HALF_HEIGHT;

        if player_bottom >= enemy_top - STOMP_TOLERANCE {
            stomp_writer.write(PlayerStompedEnemy {
                enemy: enemy_entity,
            });
        } else {
            hit_writer.write(PlayerHitByEnemy {
                enemy: enemy_entity,
            });
        }
    }
}

fn handle_stomp(
    mut commands: Commands,
    mut reader: MessageReader<PlayerStompedEnemy>,
    mut player_query: Query<&mut LinearVelocity, With<Player>>,
    mut game_data: ResMut<GameData>,
) {
    for msg in reader.read() {
        commands.entity(msg.enemy).despawn();

        if let Ok(mut velocity) = player_query.single_mut() {
            velocity.y = STOMP_BOUNCE_VELOCITY;
        }

        game_data.score += 200;
    }
}

fn handle_player_hit(
    mut reader: MessageReader<PlayerHitByEnemy>,
    mut game_data: ResMut<GameData>,
    mut next_state: ResMut<NextState<AppState>>,
    mut next_phase: ResMut<NextState<GamePhase>>,
) {
    for _msg in reader.read() {
        if game_data.lives > 0 {
            game_data.lives -= 1;
        }

        if game_data.lives == 0 {
            next_state.set(AppState::GameOver);
        } else {
            next_phase.set(GamePhase::Dying);
        }
        break;
    }
}

fn detect_coin_collection(
    mut commands: Commands,
    collisions: Collisions,
    player_query: Query<Entity, With<Player>>,
    coin_query: Query<Entity, With<Coin>>,
    mut game_data: ResMut<GameData>,
    mut writer: MessageWriter<PlayerCollectedCoin>,
) {
    let Ok(player_entity) = player_query.single() else {
        return;
    };

    for coin_entity in coin_query.iter() {
        if collisions.contains(player_entity, coin_entity) {
            writer.write(PlayerCollectedCoin { coin: coin_entity });
            commands.entity(coin_entity).despawn();
            game_data.coins += 1;
            game_data.score += COIN_SCORE;
        }
    }
}

fn detect_goal_reached(
    collisions: Collisions,
    player_query: Query<Entity, With<Player>>,
    goal_query: Query<Entity, With<LevelGoal>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut writer: MessageWriter<PlayerReachedGoal>,
) {
    let Ok(player_entity) = player_query.single() else {
        return;
    };

    for goal_entity in goal_query.iter() {
        if collisions.contains(player_entity, goal_entity) {
            writer.write(PlayerReachedGoal { goal: goal_entity });
            next_state.set(AppState::LevelComplete);
            break;
        }
    }
}
