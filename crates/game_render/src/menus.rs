use bevy::prelude::*;
use game_core::state::AppState;

pub struct MenusPlugin;

impl Plugin for MenusPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu);
        app.add_systems(
            Update,
            main_menu_input.run_if(in_state(AppState::MainMenu)),
        );

        app.add_systems(OnEnter(AppState::Paused), spawn_paused_menu);
        app.add_systems(
            Update,
            paused_input.run_if(in_state(AppState::Paused)),
        );

        app.add_systems(OnEnter(AppState::GameOver), spawn_game_over_menu);
        app.add_systems(
            Update,
            game_over_input.run_if(in_state(AppState::GameOver)),
        );

        app.add_systems(OnEnter(AppState::LevelComplete), spawn_level_complete_menu);
        app.add_systems(
            Update,
            level_complete_input.run_if(in_state(AppState::LevelComplete)),
        );

        app.add_systems(OnEnter(AppState::Victory), spawn_victory_menu);
        app.add_systems(
            Update,
            victory_input.run_if(in_state(AppState::Victory)),
        );

        app.add_systems(
            Update,
            pause_input.run_if(in_state(AppState::Playing)),
        );
    }
}

fn spawn_fullscreen_container(state: AppState) -> (Node, BackgroundColor, DespawnOnExit<AppState>) {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 0.95)),
        DespawnOnExit(state),
    )
}

fn spawn_title_text(title: &str) -> (Text, TextFont, TextColor) {
    (
        Text::new(title),
        TextFont::from_font_size(64.0),
        TextColor::WHITE,
    )
}

fn spawn_prompt_text(prompt: &str) -> (Text, TextFont, TextColor, Node) {
    (
        Text::new(prompt),
        TextFont::from_font_size(28.0),
        TextColor(Color::srgba(0.8, 0.8, 0.8, 1.0)),
        Node {
            margin: UiRect::top(Val::Px(40.0)),
            ..default()
        },
    )
}

fn spawn_main_menu(mut commands: Commands) {
    commands
        .spawn(spawn_fullscreen_container(AppState::MainMenu))
        .with_children(|parent| {
            parent.spawn(spawn_title_text("RUSTY RUNNER"));
            parent.spawn(spawn_prompt_text("Press SPACE to Start"));
        });
}

fn spawn_paused_menu(mut commands: Commands) {
    commands
        .spawn(spawn_fullscreen_container(AppState::Paused))
        .with_children(|parent| {
            parent.spawn(spawn_title_text("PAUSED"));
            parent.spawn(spawn_prompt_text("Press SPACE to Resume"));
        });
}

fn spawn_game_over_menu(mut commands: Commands) {
    commands
        .spawn(spawn_fullscreen_container(AppState::GameOver))
        .with_children(|parent| {
            parent.spawn(spawn_title_text("GAME OVER"));
            parent.spawn(spawn_prompt_text("Press SPACE to Restart"));
        });
}

fn spawn_level_complete_menu(mut commands: Commands) {
    commands
        .spawn(spawn_fullscreen_container(AppState::LevelComplete))
        .with_children(|parent| {
            parent.spawn(spawn_title_text("LEVEL COMPLETE!"));
            parent.spawn(spawn_prompt_text("Press SPACE to Continue"));
        });
}

fn spawn_victory_menu(mut commands: Commands) {
    commands
        .spawn(spawn_fullscreen_container(AppState::Victory))
        .with_children(|parent| {
            parent.spawn(spawn_title_text("VICTORY!"));
            parent.spawn(spawn_prompt_text("You defeated the boss!"));
        });
}

fn main_menu_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        next_state.set(AppState::Loading);
    }
}

fn paused_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        next_state.set(AppState::Playing);
    }
}

fn game_over_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        next_state.set(AppState::MainMenu);
    }
}

fn level_complete_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        next_state.set(AppState::Loading);
    }
}

fn victory_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        next_state.set(AppState::MainMenu);
    }
}

fn pause_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::Paused);
    }
}
