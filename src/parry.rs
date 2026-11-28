use bevy::prelude::*;

use crate::score;
use crate::score::{LastScoreDrop, Score};

#[derive(Resource)]
pub struct ParryIFramesTimer(Timer);

pub struct ParryPlugin;

impl Plugin for ParryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ParryIFramesTimer(Timer::from_seconds(0.4, TimerMode::Once)))
            .add_systems(Update, handle_parry_input);
    }
}

fn handle_parry_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<ParryIFramesTimer>,
    mut score: ResMut<Score>,
    mut last_score_drop: ResMut<LastScoreDrop>
) {
    if keys.just_pressed(KeyCode::Space) && last_score_drop.within(time.elapsed_secs(), 0.4) {
        let lost = last_score_drop.amount;
        score.0 += lost;
        last_score_drop.time = None; // consume the drop so it can't be parried twice
    }
}