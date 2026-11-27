use std::fs::File;
use std::io::{Read, Write};
use bevy::prelude::*;

use crate::menu::GameState;

const HIGHSCORE_FILE_NAME: &str = "assets/highscore.dat";

#[derive(Resource, Default)]
pub struct Score(pub u32);

#[derive(Resource, Default)]
pub struct HighScore(pub u32);

#[derive(Component)]
struct Scoreboard;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .insert_resource(get_high_score())
            .add_systems(OnEnter(GameState::Playing), spawn_scoreboard)
            .add_systems(Update, (update_high_score, update_scoreboard)
                .run_if(in_state(GameState::Playing)));
    }
}

fn spawn_scoreboard(mut commands: Commands, highscore: Res<HighScore>) {
    commands.spawn((
        Scoreboard,
        Text::new(format!("Score: 0\n High: {}", highscore.0)),
        TextFont::from_font_size(32.0),
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            right: Val::Px(12.0),
            ..default()
        },
    ));
}

fn update_scoreboard(score: Res<Score>, mut text: Single<&mut Text, With<Scoreboard>>, highscore: Res<HighScore>) {
    if score.is_changed() || highscore.is_changed() {
        text.0 = format!("Score: {}\nHigh: {}", score.0, highscore.0);
    }
}

fn update_high_score(score: Res<Score>, mut high_score: ResMut<HighScore>) {
    if !score.is_changed() || score.0 <= high_score.0 {
        return;
    }

    high_score.0 = score.0;
    let mut file = match File::create(HIGHSCORE_FILE_NAME) {
        Ok(f) => f,
        Err(e) => {eprintln!("error creating file: {}", e); return;},
    };

    match file.write_all(format!("{}", high_score.0.to_string()).as_bytes()) {
        Ok(_) => {},
        Err(e) => {eprintln!("error writing to file: {}", e); return;},
    };
}

fn get_high_score() -> HighScore {
    let value = std::fs::read_to_string(HIGHSCORE_FILE_NAME)
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0);

    return HighScore(value);
}