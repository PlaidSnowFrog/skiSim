use bevy::log::tracing_subscriber::fmt::time;
use bevy::prelude::*;
use crate::menu::GameState;
use crate::skier::Skier;

const WORLD_SPEED: f32 = 250.0;
const TRAIL_UPDATE_SPEED: f32 = 0.2; // 0.2 seconds

#[derive(Component)]
struct TrailMark;

#[derive(Resource)]
struct TrailMarkTimer(Timer);

pub struct TrailPlugin;

impl Plugin for TrailPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TrailMarkTimer(Timer::from_seconds(
            TRAIL_UPDATE_SPEED,
            TimerMode::Repeating
        )));
        app.add_systems(Update, (spawn_trail, move_trail).run_if(in_state(GameState::Playing)));
    }
}

fn spawn_trail(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<TrailMarkTimer>,
    skier: Single<&Transform, With<Skier>>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    commands.spawn((
        TrailMark,
        Sprite {
            color: Color::srgba(0.9, 0.9, 0.99, 0.8),
            custom_size: Some(Vec2::new(8.0, 3.0)),
            ..default()
        },
        Transform::from_xyz(skier.translation.x - 20.0, skier.translation.y - 28.0, -0.5),
    ));
}

fn move_trail(
    mut commands: Commands,
    time: Res<Time>,
    mut trailmarks: Query<(Entity, &mut Transform), With<TrailMark>>,
) {
    let left_edge = -crate::ARENA_WIDTH / 2.0;

    for (entity,mut transform) in &mut trailmarks {
        transform.translation.x -= WORLD_SPEED * time.delta_secs();

        if transform.translation.x < left_edge {
            commands.entity(entity).despawn();
        }
    }
}