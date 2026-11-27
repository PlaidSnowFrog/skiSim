use bevy::prelude::*;
use rand::random_range;

use crate::combo::ComboNumber;
use crate::menu::GameState;
use crate::score::Score;
use crate::skier::{SKIER_SIZE, Skier};

const COIN_SIZE: Vec2 = Vec2::new(15.0, 15.0);
const COIN_SPEED_LOW: f32 = 150.0;
const COIN_SPEED_HIGH: f32 = 500.0;
const COIN_SPAWN_SECONDS: f32 = 2.0;

#[derive(Component)]
pub struct Coin {
    speed: f32,
}

#[derive(Resource)]
struct CoinSpawnTimer(Timer);

pub struct CoinPlugin;

impl Plugin for CoinPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CoinSpawnTimer(Timer::from_seconds(
            COIN_SPAWN_SECONDS,
            TimerMode::Repeating,
        )))
        .add_systems(
            Update,
            (spawn_coins, move_coins, collect_coins).run_if(in_state(GameState::Playing)),
        );
    }
}

fn spawn_coins(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<CoinSpawnTimer>,
    asset_server: Res<AssetServer>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let right_edge = crate::ARENA_WIDTH / 2.0;
    let max_y = (crate::ARENA_HEIGHT / 2.0) - 85.0 - SKIER_SIZE.y / 2.0;
    let y = random_range(-max_y..max_y);

    commands.spawn((
        Coin { speed: random_range(COIN_SPEED_LOW..COIN_SPEED_HIGH) },
        Sprite {
            image: asset_server.load("coin.png"),
            custom_size: Some(COIN_SIZE),
                ..default()
        },
        Transform::from_xyz(right_edge - COIN_SIZE.x / 2.0, y, 0.0),
    ));
}

fn move_coins(
    mut commands: Commands,
    time: Res<Time>,
    mut coins: Query<(Entity, &Coin, &mut Transform)>,
    mut score: ResMut<Score>,
) {
    let left_edge = -crate::ARENA_WIDTH / 2.0;

    for (entity, coin, mut transform) in &mut coins {
        transform.translation.x -= coin.speed * time.delta_secs();

        // Remove coins off the left edge
        if transform.translation.x < left_edge {
            commands.entity(entity).despawn();
            score.0 = score.0.saturating_sub(1);
        }
    }
}

fn collect_coins(
    mut commands: Commands,
    skier: Single<&Transform, With<Skier>>,
    coins: Query<(Entity, &Transform), With<Coin>>,
    mut score: ResMut<Score>,
    combo: Res<ComboNumber>,
) {
    let skier_box = Aabb2d::new(skier.translation.truncate(), SKIER_SIZE / 2.0);

    for (coin, transform) in &coins {
        let coin_box = Aabb2d::new(transform.translation.truncate(), COIN_SIZE / 2.0);
        if skier_box.intersects(&coin_box) {
            commands.entity(coin).despawn();
            score.0 += combo.multiplier();
        }
    }
}
