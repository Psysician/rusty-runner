use bevy::prelude::*;
use game_core::level::{Coin, LevelGoal};
use game_core::enemy::Enemy;
use game_core::items::ItemType;
use game_core::wind::WindZone;
use game_core::platforms::MovingPlatform;
use game_core::state::AppState;

pub struct SpritesPlugin;

impl Plugin for SpritesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (add_coin_sprites, add_goal_sprite, add_enemy_sprites, add_item_sprites, add_wind_sprites, add_platform_sprites)
                .run_if(in_state(AppState::Playing)),
        );
    }
}

fn add_coin_sprites(
    mut commands: Commands,
    coins: Query<Entity, (With<Coin>, Without<Sprite>)>,
) {
    for entity in &coins {
        commands.entity(entity).insert(Sprite {
            color: Color::srgb(1.0, 0.85, 0.0),
            custom_size: Some(Vec2::new(16.0, 16.0)),
            ..default()
        });
    }
}

fn add_goal_sprite(
    mut commands: Commands,
    goals: Query<Entity, (With<LevelGoal>, Without<Sprite>)>,
) {
    for entity in &goals {
        commands.entity(entity).insert(Sprite {
            color: Color::srgb(0.0, 1.0, 0.3),
            custom_size: Some(Vec2::new(32.0, 64.0)),
            ..default()
        });
    }
}

fn add_enemy_sprites(
    mut commands: Commands,
    enemies: Query<Entity, (With<Enemy>, Without<Sprite>)>,
) {
    for entity in &enemies {
        commands.entity(entity).insert(Sprite {
            color: Color::srgb(0.9, 0.2, 0.2),
            custom_size: Some(Vec2::new(24.0, 24.0)),
            ..default()
        });
    }
}

fn add_item_sprites(
    mut commands: Commands,
    items: Query<(Entity, &ItemType), Without<Sprite>>,
) {
    for (entity, item_type) in &items {
        let color = match item_type {
            ItemType::Growth => Color::srgb(0.2, 0.8, 0.2),
            ItemType::Special => Color::srgb(0.0, 0.6, 1.0),
            ItemType::Invincible => Color::srgb(1.0, 1.0, 0.0),
        };
        commands.entity(entity).insert(Sprite {
            color,
            custom_size: Some(Vec2::new(20.0, 20.0)),
            ..default()
        });
    }
}

fn add_wind_sprites(
    mut commands: Commands,
    winds: Query<Entity, (With<WindZone>, Without<Sprite>)>,
) {
    for entity in &winds {
        commands.entity(entity).insert(Sprite {
            color: Color::srgba(0.7, 0.85, 1.0, 0.3),
            custom_size: Some(Vec2::new(100.0, 200.0)),
            ..default()
        });
    }
}

fn add_platform_sprites(
    mut commands: Commands,
    platforms: Query<Entity, (With<MovingPlatform>, Without<Sprite>)>,
) {
    for entity in &platforms {
        commands.entity(entity).insert(Sprite {
            color: Color::srgb(0.5, 0.4, 0.3),
            custom_size: Some(Vec2::new(128.0, 16.0)),
            ..default()
        });
    }
}
