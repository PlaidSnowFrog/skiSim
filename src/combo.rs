use bevy::prelude::*;
use rand::seq::IndexedRandom;

use crate::menu::GameState;
use crate::skier::{Skier, SKIER_SIZE};

const COMBO_KEYS: [KeyCode; 4] = [KeyCode::KeyI, KeyCode::KeyJ, KeyCode::KeyK, KeyCode::KeyL];

fn random_key() -> KeyCode {
    *COMBO_KEYS.choose(&mut rand::rng()).unwrap()
}

fn key_label(key: KeyCode) -> &'static str {
    match key {
        KeyCode::KeyI => "I",
        KeyCode::KeyJ => "J",
        KeyCode::KeyK => "K",
        KeyCode::KeyL => "L",
        _ => "?",
    }
}

#[derive(Resource, Default)]
pub struct ComboNumber(pub u32);

impl ComboNumber {
    /// Points multiplier for the current combo streak.
    pub fn multiplier(&self) -> u32 {
        match self.0 {
            0..10 => 1,
            10..20 => 2,
            20..30 => 4,
            _ => 6,
        }
    }
}

#[derive(Resource)]
pub struct NextComboKey(pub KeyCode);

impl Default for NextComboKey {
    fn default() -> Self {
        Self(random_key())
    }
}

const COMBO_TIMEOUT_SECONDS: f32 = 1.7;

/// Counts down from each correct press; the combo is lost when it runs out.
#[derive(Resource)]
struct ComboTimer(Timer);

impl Default for ComboTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(COMBO_TIMEOUT_SECONDS, TimerMode::Once))
    }
}

#[derive(Component)]
struct Combo;

#[derive(Component)]
struct NextKey;

#[derive(Component)]
struct MultiplierLabel;

pub struct ComboPlugin;

impl Plugin for ComboPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ComboNumber>()
            .init_resource::<NextComboKey>()
            .init_resource::<ComboTimer>()
            .add_systems(OnEnter(GameState::Playing), spawn_combo_menu)
            .add_systems(
                Update,
                (handle_combo_input, update_combo_text, follow_skier)
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn spawn_combo_menu(
    mut commands: Commands,
    combo: Res<ComboNumber>,
    next: Res<NextComboKey>,
) {
    // Combo counter stays in the corner (UI, screen space).
    commands.spawn((
        Combo,
        Text::new(format!("Combo: {}", combo.0)),
        TextFont::from_font_size(32.0),
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            right: Val::Px(12.0),
            ..default()
        },
        DespawnOnExit(GameState::Playing),
    ));

    // Next-key letter lives in world space so it can sit on the skier.
    commands.spawn((
        NextKey,
        Text2d::new(key_label(next.0)),
        TextFont::from_font_size(32.0),
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, 0.0, 1.0),
        DespawnOnExit(GameState::Playing),
    ));

    // Multiplier sits under the skier, also in world space.
    commands.spawn((
        MultiplierLabel,
        Text2d::new(format!("x{}", combo.multiplier())),
        TextFont::from_font_size(24.0),
        TextColor(Color::srgb(1.0, 0.85, 0.2)),
        Transform::from_xyz(0.0, 0.0, 1.0),
        DespawnOnExit(GameState::Playing),
    ));
}

const KEY_LABEL_OFFSET: f32 = SKIER_SIZE.y / 2.0 + 20.0;
const MULTIPLIER_LABEL_OFFSET: f32 = -(SKIER_SIZE.y / 2.0 + 16.0);

fn follow_skier(
    skier: Single<&Transform, (With<Skier>, Without<NextKey>, Without<MultiplierLabel>)>,
    mut key_label: Single<&mut Transform, (With<NextKey>, Without<MultiplierLabel>)>,
    mut mult_label: Single<&mut Transform, (With<MultiplierLabel>, Without<NextKey>)>,
) {
    key_label.translation = skier.translation + Vec3::new(0.0, KEY_LABEL_OFFSET, 1.0);
    mult_label.translation = skier.translation + Vec3::new(0.0, MULTIPLIER_LABEL_OFFSET, 1.0);
}

fn handle_combo_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut combo: ResMut<ComboNumber>,
    mut next: ResMut<NextComboKey>,
    mut timer: ResMut<ComboTimer>,
    time: Res<Time>,
) {
    // Only ticks while Playing, so pausing freezes the countdown.
    timer.0.tick(time.delta());

    if keys.just_pressed(next.0) {
        combo.0 += 1;
        next.0 = random_key();
        timer.0.reset();
    } else if COMBO_KEYS.iter().any(|k| keys.just_pressed(*k)) {
        // Wrong combo key breaks the combo.
        combo.0 = 0;
    } else if combo.0 > 0 && timer.0.is_finished() {
        // Too slow: the combo expires.
        combo.0 = 0;
    }
}

fn update_combo_text(
    combo: Res<ComboNumber>,
    next: Res<NextComboKey>,
    mut combo_text: Single<&mut Text, With<Combo>>,
    mut key_text: Single<&mut Text2d, (With<NextKey>, Without<MultiplierLabel>)>,
    mut mult_text: Single<&mut Text2d, (With<MultiplierLabel>, Without<NextKey>)>,
) {
    if combo.is_changed() || next.is_changed() {
        combo_text.0 = format!("Combo: {}", combo.0);
        key_text.0 = key_label(next.0).to_string();
        mult_text.0 = format!("x{}", combo.multiplier());
    }
}