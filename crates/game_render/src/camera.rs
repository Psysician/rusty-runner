use bevy::prelude::*;
use game_core::player::Player;
use game_core::state::AppState;

#[derive(Component)]
pub struct GameCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(
                Update,
                follow_player.run_if(in_state(AppState::Playing)),
            );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        GameCamera,
        Transform::from_xyz(0.0, 0.0, 100.0),
    ));
}

fn follow_player(
    player_q: Query<&Transform, (With<Player>, Without<GameCamera>)>,
    mut camera_q: Query<&mut Transform, With<GameCamera>>,
    time: Res<Time>,
) {
    let Ok(player_transform) = player_q.single() else {
        return;
    };
    let Ok(mut camera_transform) = camera_q.single_mut() else {
        return;
    };

    let target = Vec3::new(
        player_transform.translation.x,
        player_transform.translation.y + 50.0,
        camera_transform.translation.z,
    );

    let smoothing = 5.0;
    camera_transform.translation = camera_transform
        .translation
        .lerp(target, smoothing * time.delta_secs());
}
