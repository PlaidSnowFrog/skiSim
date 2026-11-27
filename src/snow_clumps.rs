use bevy::prelude::*;
use rand::random_range;

use crate::menu::GameState;

const SPEED: f32 = 400.0;

const SIZE: Vec2 = Vec2::new(100.0, 30.0);
const CLUMP_SPAWN_SECONDS: f32 = 1.5;

#[derive(Component)]
pub struct SnowClump;

#[derive(Resource)]
struct SnowClumpsTimer(Timer);

pub struct SnowClumpPlugin;

impl Plugin for SnowClumpPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SnowClumpsTimer(Timer::from_seconds(
            CLUMP_SPAWN_SECONDS,
            TimerMode::Repeating,
        )))
        .add_systems(Update, (spawn, move_clumps).run_if(in_state(GameState::Playing)));
    }
}

fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut timer: ResMut<SnowClumpsTimer>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let right_edge = crate::ARENA_WIDTH / 2.0;

    let max_y = crate::ARENA_HEIGHT / 2.0;
    let min_y = max_y - 200.0;
    let y = random_range(-max_y..max_y);

    commands.spawn((
        SnowClump,
        Sprite {
            image: asset_server.load("snow_clump.png"),
            custom_size: Some(SIZE),
            ..default()
        },
        Transform::from_xyz(right_edge - SIZE.x / 2.0, y, -1.0),
    ));
}

fn move_clumps(
    mut commands: Commands,
    time: Res<Time>,
    mut clumps: Query<(Entity, &mut Transform), With<SnowClump>>,
) {
    // Fully off screen once the clump's right side passes the left edge.
    let left_edge = -crate::ARENA_WIDTH / 2.0 - SIZE.x / 2.0;

    for (entity, mut transform) in &mut clumps {
        transform.translation.x -= SPEED * time.delta_secs();

        if transform.translation.x < left_edge {
            commands.entity(entity).despawn();
        }
    }
}
