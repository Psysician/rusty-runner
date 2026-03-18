use bevy::prelude::*;
use game_core::state::{AppState, GameData};

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct LivesText;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playing), spawn_hud)
            .add_systems(Update, update_hud.run_if(in_state(AppState::Playing)));
    }
}

fn spawn_hud(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Auto,
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(12.0)),
                ..default()
            },
            DespawnOnExit(AppState::Playing),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Score: 0  Coins: 0"),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                ScoreText,
            ));

            parent.spawn((
                Text::new("Lives: 3"),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                LivesText,
            ));
        });
}

fn update_hud(
    game_data: Res<GameData>,
    mut score_q: Query<&mut Text, (With<ScoreText>, Without<LivesText>)>,
    mut lives_q: Query<&mut Text, (With<LivesText>, Without<ScoreText>)>,
) {
    if let Ok(mut text) = score_q.single_mut() {
        **text = format!("Score: {}  Coins: {}", game_data.score, game_data.coins);
    }
    if let Ok(mut text) = lives_q.single_mut() {
        **text = format!("Lives: {}", game_data.lives);
    }
}
