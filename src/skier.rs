use bevy::prelude::*;
use bevy::winit::winit_window_position;
use crate::menu::GameState;

pub const SKIER_SIZE: Vec2 = Vec2::new(60.0, 60.0);
const SKIER_SPEED: f32 = 500.0; // pixels per second
const SKIER_X: f32 = 450.0; // distance from the centre of the screen
const ARENA_HALF_HEIGHT: f32 = 360.0; // 1280x720 window

#[derive(Component)]
pub struct Skier {
    up: KeyCode,
    down: KeyCode,
}

pub struct SkierPlugin;

impl Plugin for SkierPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_skier)
            .add_systems(Update, move_skier.run_if(in_state(GameState::Playing)));
    }
}

fn spawn_skier(
    mut commands: Commands,
    asset_server : Res<AssetServer>,
) {
    commands.spawn((
        Sprite {
            image: asset_server.load("skier.png"),
            custom_size: Some(SKIER_SIZE),
            ..default()
        },
        Transform::from_xyz(-SKIER_X, 0.0, 0.0),
        Skier { up: KeyCode::KeyW, down: KeyCode::KeyS },
    ));
}

fn move_skier(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut skiers: Query<(&Skier, &mut Transform)>,
) {
    for (skier, mut transform) in &mut skiers {
        let mut direction = 0.0;

        let max_y = (crate::ARENA_HEIGHT / 2.0) - 85.0;
        let y = transform.translation.y;

        if keys.pressed(skier.up) && y < max_y {
            direction += 1.0;
        }
        if keys.pressed(skier.down) && y > -max_y {
            direction -= 1.0;
        }

        // Multiply by delta time so speed is the same at any frame rate.
        transform.translation.y += direction * SKIER_SPEED * time.delta_secs();

        // Keep the skier fully on screen.
        let limit = ARENA_HALF_HEIGHT - SKIER_SIZE.y / 2.0;
        transform.translation.y = transform.translation.y.clamp(-limit, limit);
    }
}
