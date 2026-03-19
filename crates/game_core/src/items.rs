use avian2d::prelude::*;
use bevy::prelude::*;

use crate::player::{DashCooldown, InvincibilityTimer, Player, PowerUpState};
use crate::state::GamePhase;

#[derive(Component, Debug, Clone)]
pub enum ItemType {
    Growth,
    Special,
    Invincible,
}

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_item_pickup,
                tick_invincibility,
                tick_dash_cooldown,
            )
                .run_if(in_state(GamePhase::Active)),
        );
    }
}

fn handle_item_pickup(
    mut commands: Commands,
    collisions: Collisions,
    mut player_query: Query<(Entity, &mut PowerUpState), With<Player>>,
    item_query: Query<(Entity, &ItemType)>,
) {
    let Ok((player_entity, mut power_up)) = player_query.single_mut() else {
        return;
    };

    for (item_entity, item_type) in item_query.iter() {
        if !collisions.contains(player_entity, item_entity) {
            continue;
        }

        commands.entity(item_entity).despawn();

        match item_type {
            ItemType::Growth => {
                if *power_up == PowerUpState::Small {
                    *power_up = PowerUpState::Big;
                }
            }
            ItemType::Special => {
                *power_up = PowerUpState::Dash;
                commands.entity(player_entity).insert(DashCooldown {
                    timer: Timer::from_seconds(2.0, TimerMode::Once),
                    available: true,
                });
            }
            ItemType::Invincible => {
                *power_up = PowerUpState::Invincible;
                commands.entity(player_entity).insert(InvincibilityTimer {
                    timer: Timer::from_seconds(10.0, TimerMode::Once),
                });
            }
        }
    }
}

fn tick_invincibility(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut InvincibilityTimer, &mut PowerUpState), With<Player>>,
) {
    for (entity, mut inv_timer, mut power_up) in query.iter_mut() {
        inv_timer.timer.tick(time.delta());
        if inv_timer.timer.is_finished() {
            *power_up = PowerUpState::Small;
            commands.entity(entity).remove::<InvincibilityTimer>();
        }
    }
}

fn tick_dash_cooldown(
    time: Res<Time>,
    mut query: Query<&mut DashCooldown, With<Player>>,
) {
    for mut cooldown in query.iter_mut() {
        cooldown.timer.tick(time.delta());
        if cooldown.timer.is_finished() {
            cooldown.available = true;
            cooldown.timer.reset();
        }
    }
}
