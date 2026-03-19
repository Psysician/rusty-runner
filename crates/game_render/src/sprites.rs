use bevy::prelude::*;
use game_core::level::{Coin, LevelGoal};
use game_core::enemy::{Enemy, EnemyType};
use game_core::boss::Boss;
use game_core::items::ItemType;
use game_core::wind::WindZone;
use game_core::platforms::MovingPlatform;
use game_core::state::AppState;

pub struct SpritesPlugin;

impl Plugin for SpritesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (add_coin_sprites, add_goal_sprite, add_enemy_sprites, add_boss_sprites, add_item_sprites, add_wind_sprites, add_platform_sprites)
                .run_if(in_state(AppState::Playing)),
        );
    }
}

fn add_coin_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    coins: Query<Entity, (With<Coin>, Without<Sprite>)>,
) {
    for entity in &coins {
        commands.entity(entity).insert(Sprite {
            image: asset_server.load("sprites/coin.png"),
            custom_size: Some(Vec2::new(16.0, 16.0)),
            ..default()
        });
    }
}

fn add_goal_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    goals: Query<Entity, (With<LevelGoal>, Without<Sprite>)>,
) {
    for entity in &goals {
        commands.entity(entity).insert(Sprite {
            image: asset_server.load("sprites/goal.png"),
            custom_size: Some(Vec2::new(32.0, 64.0)),
            ..default()
        });
    }
}

fn add_enemy_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    enemies: Query<(Entity, Option<&EnemyType>), (With<Enemy>, Without<Boss>, Without<Sprite>)>,
) {
    for (entity, enemy_type) in &enemies {
        let path = match enemy_type {
            Some(EnemyType::Walker) => "sprites/enemy_walker.png",
            Some(EnemyType::Jumper) => "sprites/enemy_jumper.png",
            Some(EnemyType::Flyer) => "sprites/enemy_flyer.png",
            Some(EnemyType::Spiker) => "sprites/enemy_spiker.png",
            None => "sprites/enemy_walker.png",
        };
        commands.entity(entity).insert(Sprite {
            image: asset_server.load(path),
            custom_size: Some(Vec2::new(24.0, 24.0)),
            ..default()
        });
    }
}

fn add_boss_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    bosses: Query<Entity, (With<Boss>, Without<Sprite>)>,
) {
    for entity in &bosses {
        commands.entity(entity).insert(Sprite {
            image: asset_server.load("sprites/boss.png"),
            custom_size: Some(Vec2::new(48.0, 48.0)),
            ..default()
        });
    }
}

fn add_item_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    items: Query<(Entity, &ItemType), Without<Sprite>>,
) {
    for (entity, item_type) in &items {
        let path = match item_type {
            ItemType::Growth => "sprites/powerup_growth.png",
            ItemType::Special => "sprites/powerup_dash.png",
            ItemType::Invincible => "sprites/powerup_invincible.png",
        };
        commands.entity(entity).insert(Sprite {
            image: asset_server.load(path),
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
