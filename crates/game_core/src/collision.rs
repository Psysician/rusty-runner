use avian2d::prelude::*;
use bevy::prelude::*;

use crate::boss::{Boss, BossPhase, Shockwave};
use crate::enemy::Enemy;
use crate::level::{Coin, LevelGoal};
use crate::player::{Player, PowerUpState, InvincibilityTimer};
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
                detect_boss_stomp,
                detect_shockwave_hit,
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
    mut commands: Commands,
    mut reader: MessageReader<PlayerHitByEnemy>,
    mut game_data: ResMut<GameData>,
    mut next_state: ResMut<NextState<AppState>>,
    mut next_phase: ResMut<NextState<GamePhase>>,
    mut player_query: Query<(Entity, Option<&InvincibilityTimer>, &mut PowerUpState), With<Player>>,
) {
    let Some(_msg) = reader.read().next() else { return };

    let Ok((entity, invincibility, mut power_up)) = player_query.single_mut() else {
        return;
    };

    if invincibility.is_some() {
        return;
    }

    if *power_up != PowerUpState::Small {
        *power_up = PowerUpState::Small;
        commands.entity(entity).remove::<crate::player::DashCooldown>();
        return;
    }

    if game_data.lives > 0 {
        game_data.lives -= 1;
    }

    if game_data.lives == 0 {
        next_state.set(AppState::GameOver);
    } else {
        next_phase.set(GamePhase::Dying);
    }
}

fn detect_boss_stomp(
    collisions: Collisions,
    mut player_query: Query<(Entity, &Transform, &mut LinearVelocity), With<Player>>,
    mut boss_query: Query<(Entity, &Transform, &mut Boss, &mut BossPhase)>,
    mut next_state: ResMut<NextState<AppState>>,
    mut game_data: ResMut<GameData>,
) {
    let Ok((player_entity, player_transform, mut player_vel)) = player_query.single_mut() else {
        return;
    };

    for (boss_entity, boss_transform, mut boss, mut phase) in boss_query.iter_mut() {
        if !collisions.contains(player_entity, boss_entity) {
            continue;
        }

        if !matches!(*phase, BossPhase::PausedAtWall { .. }) {
            continue;
        }

        let player_bottom = player_transform.translation.y - PLAYER_HALF_HEIGHT;
        let boss_top = boss_transform.translation.y + ENEMY_HALF_HEIGHT;

        if player_bottom >= boss_top - STOMP_TOLERANCE {
            boss.hp = boss.hp.saturating_sub(1);
            game_data.score += 500;
            player_vel.y = STOMP_BOUNCE_VELOCITY;

            if boss.hp == 0 {
                next_state.set(AppState::Victory);
            } else {
                let boss_x = boss_transform.translation.x;
                *phase = BossPhase::Charging {
                    direction: if boss_x < 0.0 { 1.0 } else { -1.0 },
                };
            }
        }
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

fn detect_shockwave_hit(
    collisions: Collisions,
    player_query: Query<Entity, With<Player>>,
    shockwave_query: Query<Entity, With<Shockwave>>,
    mut hit_writer: MessageWriter<PlayerHitByEnemy>,
) {
    let Ok(player_entity) = player_query.single() else { return };
    for shockwave_entity in shockwave_query.iter() {
        if collisions.contains(player_entity, shockwave_entity) {
            hit_writer.write(PlayerHitByEnemy { enemy: shockwave_entity });
        }
    }
}
