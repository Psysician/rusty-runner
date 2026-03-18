use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;
use game_core::enemy::Enemy;
use game_core::level::LevelGoal;
use game_core::player::Player;

#[derive(Debug, Clone)]
pub struct GameWorldState {
    pub player_pos: Vec2,
    pub player_velocity: Vec2,
    pub nearby_enemies: Vec<(Vec2, String)>,
    pub ground_ahead: bool,
    pub gap_ahead: bool,
    pub enemy_ahead: bool,
    pub goal_pos: Vec2,
}

#[derive(Resource)]
pub struct GameWorldStateResource(pub GameWorldState);

impl Default for GameWorldStateResource {
    fn default() -> Self {
        Self(GameWorldState {
            player_pos: Vec2::ZERO,
            player_velocity: Vec2::ZERO,
            nearby_enemies: vec![],
            ground_ahead: true,
            gap_ahead: false,
            enemy_ahead: false,
            goal_pos: Vec2::new(1000.0, 0.0),
        })
    }
}

pub fn build_world_state(
    player_q: Query<(&Transform, Option<&LinearVelocity>), With<Player>>,
    enemy_q: Query<&Transform, With<Enemy>>,
    goal_q: Query<&Transform, With<LevelGoal>>,
    mut world_state: ResMut<GameWorldStateResource>,
) {
    let Ok((player_transform, player_vel)) = player_q.single() else { return };
    let player_pos = player_transform.translation.truncate();
    let player_velocity = player_vel.map(|v| v.0).unwrap_or(Vec2::ZERO);

    let nearby_enemies: Vec<_> = enemy_q
        .iter()
        .filter(|t| t.translation.truncate().distance(player_pos) < 200.0)
        .map(|t| (t.translation.truncate(), "walker".into()))
        .collect();

    let enemy_ahead = nearby_enemies
        .iter()
        .any(|(pos, _)| pos.x > player_pos.x && (pos.x - player_pos.x) < 100.0);

    let goal_pos = goal_q
        .iter()
        .next()
        .map(|t| t.translation.truncate())
        .unwrap_or(Vec2::new(1000.0, 0.0));

    world_state.0 = GameWorldState {
        player_pos,
        player_velocity,
        nearby_enemies,
        ground_ahead: true,
        gap_ahead: false,
        enemy_ahead,
        goal_pos,
    };
}
