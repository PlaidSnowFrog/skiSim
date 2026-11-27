use std::process::exit;
use bevy::prelude::*;
use bevy::ui_widgets::{Activate, Button};
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
    Paused,
}

#[derive(Component)]
pub struct PlayButton;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(OnEnter(GameState::Menu), spawn_menu);
    }
}

fn spawn_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("Main_Menu_Song.wav")),
        PlaybackSettings::LOOP,
        DespawnOnExit(GameState::Menu),
    ));

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(24.0),
            ..default()
        },
        DespawnOnExit(GameState::Menu),
    ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Skiing Simulator"),
                TextFont::from_font_size(72.0),
                TextColor(Color::WHITE),
            ));
            parent.spawn((
                PlayButton,
                Button,
                Node {
                    padding: UiRect::axes(Val::Px(32.0), Val::Px(12.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.4, 0.7)),
            ))
                .with_child((Text::new("Play"), TextFont::from_font_size(40.0)))
                .observe(|_: On<Activate>, mut next_state: ResMut<NextState<GameState>>| {
                    next_state.set(GameState::Playing);
                });

            parent
                .spawn((
                    Button,
                    Node {
                        padding: UiRect::axes(Val::Px(32.0), Val::Px(12.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.4, 0.7)),
                ))
                .with_child((Text::new("Quit"), TextFont::from_font_size(40.0)))
                .observe(|_: On<Activate>, mut exit: MessageWriter<AppExit>| {
                    exit.write(AppExit::Success);
                });
        });
}