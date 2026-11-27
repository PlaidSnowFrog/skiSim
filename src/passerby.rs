use bevy::prelude::*;
use rand::{random, random_range};
use crate::menu::GameState;
use crate::score::Score;
use crate::skier::{Skier, SKIER_SIZE};

const SPEED: f32 = 500.0;
const SPAWN_SECONDS: f32 = 5.0;
const SIZE: Vec2 = Vec2::new(60.0, 60.0);

#[derive(Component)]
pub struct Passerby;

/// Marks a passerby that has already collided with the skier.
#[derive(Component)]
struct Hit;

#[derive(Resource)]
struct PasserbyTimer(Timer);

pub struct PasserbyPlugin;

impl Plugin for PasserbyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PasserbyTimer(Timer::from_seconds(
            SPAWN_SECONDS,
            TimerMode::Repeating,
        )))
            .add_systems(
                Update,
                (spawn_passerby, move_passerby, remove_points).run_if(in_state(GameState::Playing)),
            );
    }
}

fn spawn_passerby(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<PasserbyTimer>,
    asset_server: Res<AssetServer>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let left_edge = -(crate::ARENA_WIDTH / 2.0);
    let max_y = (crate::ARENA_HEIGHT / 2.0) - 85.0 - SIZE.y / 2.0;

    let y = random_range(-max_y..max_y);

    commands.spawn((
        Passerby,
        Sprite {
            image: asset_server.load("passerby.png"),
            custom_size: Some(SIZE),
                ..default()
        },
        Transform::from_xyz(left_edge - SIZE.x / 2.0, y, 0.0),
        ));
}

fn move_passerby(
    mut commands: Commands,
    time: Res<Time>,
    mut passerbies: Query<(Entity, &mut Transform), With<Passerby>>,
) {
    let right_edge = crate::ARENA_WIDTH / 2.0 + SIZE.x / 2.0;

    for (entity, mut transform) in &mut passerbies {
        transform.translation.x += SPEED * time.delta_secs();

        if transform.translation.x > right_edge {
            commands.entity(entity).despawn();
        }
    }
}

fn remove_points(
    mut commands: Commands,
    skier: Single<&Transform, With<Skier>>,
    passerbies: Query<(Entity, &Transform), (With<Passerby>, Without<Hit>)>,
    mut score: ResMut<Score>,
) {
    let skier_box = Aabb2d::new(skier.translation.truncate(), SKIER_SIZE / 2.0);

    for (passerby, transform) in &passerbies {
        let passerby_box = Aabb2d::new(transform.translation.truncate(), SIZE / 2.0);
        if skier_box.intersects(&passerby_box) {
            score.0 = score.0.saturating_sub(3);
            // Mark so this passerby is excluded from the query on later frames.
            commands.entity(passerby).insert(Hit);
        }
    }
}