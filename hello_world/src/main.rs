// eric
use std::{mem, time::Duration};

use bevy::{
    input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
    },
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set( WindowPlugin {
            primary_window : Some(Window {
                title: String::from("FIRST BEVY APP"),
                ime_position: Vec2::ZERO,
                ..default()
            }),
                
            ..default()
        }))
        .add_systems(Startup, setup_scene)
        .add_systems(
            Update,
            (
                toggle_ime,
                listen_ime_events,
                listen_keyboard_input_events,
                bubbling_text,
            ),
        )
        .run();
}

fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let font = asset_server.load("fonts/D2Coding.ttf");

    commands.spawn(
        TextBundle::from_sections([
            TextSection {
                value: String::from("한국어 지원"),
                style: TextStyle {
                    font: font.clone_weak(),
                    ..default()
                },
            },
            TextSection {
                value: String::from("\n"),
                style: TextStyle {
                    font: font.clone_weak(),
                    font_size: 30.0,
                    ..default()
                },
            },
            TextSection {
                value: "IME Active: ".to_string(),
                style: TextStyle {
                    font: font.clone_weak(),
                    ..default()
                },
            },
            TextSection {
                value: String::from("불가능\n"),
                style: TextStyle {
                    font: font.clone_weak(),
                    font_size: 30.0,
                    ..default()
                },
            },
            TextSection {
                value: "click to toggle IME, press return to start a new line\n\n".to_string(),
                style: TextStyle {
                    font: font.clone_weak(),
                    font_size: 18.0,
                    ..default()
                },
            },
            TextSection {
                value: "".to_string(),
                style: TextStyle {
                    font,
                    font_size: 25.0,
                    ..default()
                },
            },
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    );

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "".to_string(),
            TextStyle {
                font: asset_server.load("fonts/D2Coding.ttf"),
                font_size: 100.0,
                color: Color::Srgba(Srgba::GREEN)
            },
        ),
        ..default()
    });
}

fn toggle_ime(
    input: Res<ButtonInput<MouseButton>>,
    mut windows: Query<&mut Window>,
    mut text: Query<&mut Text, With<Node>>,
) {
    if input.just_pressed(MouseButton::Left) {
        let mut window = windows.single_mut();

        window.ime_position = window.cursor_position().unwrap();
        window.ime_enabled = !window.ime_enabled;
        let msg = format!("{}", window.ime_position);
        // let ime_state = if window.ime_enabled {
        //     String::from("가능\n") 
        // } else {
        //     String::from("불가능\n")
        // };

        let mut text = text.single_mut();
        text.sections[1].value = msg;
    }
}

#[derive(Component)]
struct Bubble {
    timer: Timer,
}

fn bubbling_text(
    mut commands: Commands,
    mut bubbles: Query<(Entity, &mut Transform, &mut Bubble)>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut bubble) in bubbles.iter_mut() {
        if bubble.timer.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn();
        }
        transform.translation.y += time.delta_seconds() * 100.0;
    }
}

fn listen_ime_events(
    mut events: EventReader<Ime>,
    mut status_text: Query<&mut Text, With<Node>>,
    mut edit_text: Query<&mut Text, (Without<Node>, Without<Bubble>)>,
) {
    for event in events.read() {
        match event {
            Ime::Preedit { value, cursor, .. } if !cursor.is_none() => {
                status_text.single_mut().sections[5].value = format!("IME buffer: {value}");
            }
            Ime::Preedit { cursor, .. } if cursor.is_none() => {
                status_text.single_mut().sections[5].value = "".to_string();
            }
            Ime::Commit { value, .. } => {
                edit_text.single_mut().sections[0].value.push_str(value);
            }
            Ime::Enabled { .. } => {
                status_text.single_mut().sections[3].value = "가능\n".to_string();
            }
            Ime::Disabled { .. } => {
                status_text.single_mut().sections[3].value = "불가능\n".to_string();
            }
            _ => (),
        }
    }
}

fn listen_keyboard_input_events(
    mut commands: Commands,
    mut events: EventReader<KeyboardInput>,
    mut edit_text: Query<&mut Text, (Without<Node>, Without<Bubble>)>,
) {
    for event in events.read() {
        // Only trigger changes when the key is first pressed.
        if event.state == ButtonState::Released {
            continue;
        }

        match &event.logical_key {
            Key::Enter => {
                let mut text = edit_text.single_mut();
                if text.sections[0].value.is_empty() {
                    continue;
                }
                let old_value = mem::take(&mut text.sections[0].value);

                commands.spawn((
                    Text2dBundle {
                        text: Text::from_section(old_value, text.sections[0].style.clone()),
                        ..default()
                    },
                    Bubble {
                        timer: Timer::from_seconds(5.0, TimerMode::Once),
                    },
                ));
            }
            Key::Space => {
                edit_text.single_mut().sections[0].value.push(' ');
            }
            Key::Backspace => {
                edit_text.single_mut().sections[0].value.pop();
            }
            Key::Character(character) => {
                edit_text.single_mut().sections[0].value.push_str(character);
            }
            _ => continue,
        }
    }
}
