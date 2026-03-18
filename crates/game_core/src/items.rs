use avian2d::prelude::*;
use bevy::prelude::*;

use crate::player::{DashCooldown, InvincibilityTimer, Player, PowerUpState};
use crate::state::AppState;

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
                handle_growth_pickup,
                handle_dash_pickup,
                handle_invincible_pickup,
                tick_invincibility,
                tick_dash_cooldown,
            )
                .run_if(in_state(AppState::Playing)),
        );
    }
}

fn handle_growth_pickup(
    mut commands: Commands,
    collisions: Collisions,
    player_query: Query<(Entity, &PowerUpState), With<Player>>,
    item_query: Query<Entity, With<ItemType>>,
    item_type_query: Query<&ItemType>,
    mut power_up_query: Query<&mut PowerUpState, With<Player>>,
) {
    let Ok((player_entity, current_state)) = player_query.single() else {
        return;
    };

    for item_entity in item_query.iter() {
        if !collisions.contains(player_entity, item_entity) {
            continue;
        }

        let Ok(item_type) = item_type_query.get(item_entity) else {
            continue;
        };

        if !matches!(item_type, ItemType::Growth) {
            continue;
        }

        commands.entity(item_entity).despawn();

        if *current_state == PowerUpState::Small {
            if let Ok(mut power_up) = power_up_query.single_mut() {
                *power_up = PowerUpState::Big;
            }
        }
    }
}

fn handle_dash_pickup(
    mut commands: Commands,
    collisions: Collisions,
    player_query: Query<Entity, With<Player>>,
    item_query: Query<(Entity, &ItemType)>,
    mut power_up_query: Query<&mut PowerUpState, With<Player>>,
) {
    let Ok(player_entity) = player_query.single() else {
        return;
    };

    for (item_entity, item_type) in item_query.iter() {
        if !matches!(item_type, ItemType::Special) {
            continue;
        }

        if !collisions.contains(player_entity, item_entity) {
            continue;
        }

        commands.entity(item_entity).despawn();

        if let Ok(mut power_up) = power_up_query.single_mut() {
            *power_up = PowerUpState::Dash;
        }

        commands.entity(player_entity).insert(DashCooldown {
            timer: Timer::from_seconds(2.0, TimerMode::Once),
            available: true,
        });
    }
}

fn handle_invincible_pickup(
    mut commands: Commands,
    collisions: Collisions,
    player_query: Query<Entity, With<Player>>,
    item_query: Query<(Entity, &ItemType)>,
    mut power_up_query: Query<&mut PowerUpState, With<Player>>,
) {
    let Ok(player_entity) = player_query.single() else {
        return;
    };

    for (item_entity, item_type) in item_query.iter() {
        if !matches!(item_type, ItemType::Invincible) {
            continue;
        }

        if !collisions.contains(player_entity, item_entity) {
            continue;
        }

        commands.entity(item_entity).despawn();

        if let Ok(mut power_up) = power_up_query.single_mut() {
            *power_up = PowerUpState::Invincible;
        }

        commands.entity(player_entity).insert(InvincibilityTimer {
            timer: Timer::from_seconds(10.0, TimerMode::Once),
        });
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
