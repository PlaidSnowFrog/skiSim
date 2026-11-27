use bevy::prelude::*;
use crate::menu::GameState;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, toggle_pause);
    }
}

fn toggle_pause(keys: Res<ButtonInput<KeyCode>>, mut time: ResMut<Time<Virtual>>) {
    if keys.just_pressed(KeyCode::Escape) {
        if time.is_paused() {
            time.unpause();
        } else {
            time.pause();
        }
    }
}