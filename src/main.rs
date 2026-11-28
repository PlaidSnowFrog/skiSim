mod coin;
mod score;
mod skier;
mod snow_clumps;
pub mod passerby;
pub mod pine;
pub mod trail;
pub mod menu;
pub mod pause;
pub mod combo;
pub mod parry;

use bevy::camera::ScalingMode;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use coin::CoinPlugin;
use score::ScorePlugin;
use skier::SkierPlugin;
use snow_clumps::SnowClumpPlugin;
use passerby::PasserbyPlugin;
use pine::PinePlugin;
use trail::TrailPlugin;
use menu::MenuPlugin;
use pause::PausePlugin;
use combo::ComboPlugin;
use crate::parry::ParryPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.68, 0.85, 0.9)))
        .add_plugins((DefaultPlugins
                          .set(ImagePlugin::default_nearest()) // Pixel art sharp
                          .set(WindowPlugin {
                              primary_window: Some(Window {
                                  title: "Skiing Simulator".into(),
                                  // On the web, resize the canvas to fill its parent (the page / itch.io iframe)
                                  fit_canvas_to_parent: true,
                                  ..default()
                              }),
                              ..default()
                          }),
                      CoinPlugin,
                      ScorePlugin,
                      SnowClumpPlugin,
                      SkierPlugin,
                      PasserbyPlugin,
                      PinePlugin,
                      TrailPlugin,
                      MenuPlugin,
                      PausePlugin,
                      ComboPlugin,
                      ParryPlugin,))
        .add_systems(Startup, setup_camera)
        .add_systems(Update, scale_ui_to_window)
        .run();
}

/// The game world is always this size in world units, whatever the window/canvas size is.
pub const ARENA_WIDTH: f32 = 1280.0;
pub const ARENA_HEIGHT: f32 = 720.0;

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        // Zoom the camera so the whole 1280x720 arena is always visible, at any canvas size.
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: ARENA_WIDTH,
                min_height: ARENA_HEIGHT,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}

/// Scale UI (score, menus) along with the world so it looks the same at any canvas size.
fn scale_ui_to_window(window: Single<&Window, With<PrimaryWindow>>, mut ui_scale: ResMut<UiScale>) {
    let scale = (window.width() / ARENA_WIDTH).min(window.height() / ARENA_HEIGHT);
    if scale > 0.0 && (ui_scale.0 - scale).abs() > f32::EPSILON {
        ui_scale.0 = scale;
    }
}
