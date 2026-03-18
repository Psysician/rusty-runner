use avian2d::prelude::*;
use bevy::prelude::*;

use crate::player::Player;
use crate::state::AppState;

#[derive(Component)]
pub struct WindZone {
    pub force: Vec2,
}

pub struct WindPlugin;

impl Plugin for WindPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            apply_wind.run_if(in_state(AppState::Playing)),
        );
    }
}

fn apply_wind(
    collisions: Collisions,
    wind_q: Query<(Entity, &WindZone)>,
    mut player_q: Query<(Entity, &mut LinearVelocity), With<Player>>,
    time: Res<Time>,
) {
    let Ok((player_entity, mut velocity)) = player_q.single_mut() else {
        return;
    };

    for (wind_entity, wind) in &wind_q {
        if collisions.contains(player_entity, wind_entity) {
            velocity.x += wind.force.x * time.delta_secs();
            velocity.y += wind.force.y * time.delta_secs();
        }
    }
}
