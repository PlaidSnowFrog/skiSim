use bevy::prelude::*;
use bevy::text::TextEdit::Up;
use rand::random_range;

use crate::menu::GameState;

// Parallax file
const WORLD_SPEED: f32 = 250.0;
const PINE_SIZE: Vec2 = Vec2::new(60.0, 80.0);

struct Layer {
    factor: f32,
    z: f32,
    scale: f32,
    spacing: f32,
    inset: f32,
    tint: Color
}

const LAYERS: [Layer; 2] = [
    // Far
    Layer {factor: 0.6, z: -2.0, scale: 0.7, spacing: 30.0, inset: 40.0, tint: Color::srgb(0.75, 0.82, 0.9)},
    // Near
    Layer {factor: 1.4, z: 2.0, scale: 1.4, spacing: 70.0, inset: 10.0, tint: Color::WHITE},
];

#[derive(Component)]
struct Pine {
    factor: f32,
    strip_width: f32,
    half_width: f32,
}

pub struct PinePlugin;

impl Plugin for PinePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_pines)
            .add_systems(Update, scroll_pines.run_if(in_state(GameState::Playing)));
    }
}

fn spawn_pines(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let half_width = crate::ARENA_WIDTH / 2.0;
    let half_height = crate::ARENA_HEIGHT / 2.0;
    let image = asset_server.load("pine.png");

    for layer in &LAYERS {
        let size = PINE_SIZE * layer.scale;
        let count = (crate::ARENA_WIDTH / layer.spacing).ceil() as usize + 2;
        let strip_width = count as f32 * layer.spacing;

        for i in 0..count {
            for edge in [1.0, -1.0] { // top and bottom
                let x = -half_width + i as f32 * layer.spacing + random_range(-15.0..15.0);
                let y = edge * (half_height - layer.inset);
                commands.spawn((
                    Pine { factor: layer.factor, strip_width, half_width: size.x / 2.0 },
                    Sprite {
                        image: image.clone(),
                        custom_size: Some(size),
                        color: layer.tint,
                        flip_x: random_range(0.0..1.0) < 0.5, // cheap variety
                        ..default()
                    },
                    Transform::from_xyz(x, y, layer.z),
                ));
            }
        }
    }
}

fn scroll_pines(
    time: Res<Time>,
    mut pines: Query<(&Pine, &mut Transform)>,
) {
    let left_edge = - crate::ARENA_WIDTH / 2.0;

    for (pine, mut transform) in &mut pines {
        transform.translation.x -= WORLD_SPEED * pine.factor * time.delta_secs();

        if transform.translation.x < left_edge - pine.half_width {
            transform.translation.x += pine.strip_width;
        }
    }
}