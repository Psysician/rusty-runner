use bevy::prelude::*;
use game_core::enemy::{Enemy, EnemyPlugin, EnemyType, PatrolConfig, PatrolDirection};

#[test]
fn walker_reverses_direction_at_patrol_limit() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(bevy::state::app::StatesPlugin);
    app.add_plugins(game_core::state::StatePlugin);
    app.add_plugins(EnemyPlugin);

    app.update();
    app.world_mut()
        .resource_mut::<NextState<game_core::state::AppState>>()
        .set(game_core::state::AppState::Playing);
    app.update();
    app.update();

    app.world_mut().spawn((
        Enemy,
        EnemyType::Walker,
        PatrolDirection { right: true },
        PatrolConfig {
            speed: 80.0,
            distance: 50.0,
            origin_x: 100.0,
        },
        Transform::from_xyz(100.0, 0.0, 0.0),
    ));

    for _ in 0..60 {
        app.update();
    }

    let count = app.world_mut().query::<&Enemy>().iter(app.world()).count();
    assert_eq!(count, 1, "Walker should still exist");
}
