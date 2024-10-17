// eric


use bevy::{
    color::palettes::css::{BLACK, WHITE}, input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
    }, prelude::*
};

#[derive(Component)]
struct BlinKTimer {
    timer: Timer,
}


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
                blinking_cursor ,
                listen_ime_events,
                ime_enable,
                listen_keyboard_input_events,
            ),
        )
        .run();
}

fn setup_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(BlinKTimer {
        timer: Timer::from_seconds(0.4, TimerMode::Repeating),
    });
    commands.spawn((
        TextBundle {
            text: Text::from_sections([
                TextSection::new (
                    String::new(), 
                    TextStyle {
                        font: asset_server.load("fonts/D2Coding.ttf"),
                        font_size: 30.0,
                        color: Color::BLACK,
                    }
                ),

                TextSection::new (
                    String::new(),
                    TextStyle {
                        font: asset_server.load("fonts/D2Coding.ttf"),
                        font_size: 30.0,
                        color: Color::BLACK,
                    }
                ),

                // 2 Text input cursor
                TextSection::new (
                    String::from("|"),
                    TextStyle {
                        font: asset_server.load("fonts/Inter-Regular.ttf"),
                        font_size: 30.0,
                        color: Color::BLACK,
                    }
                ),

                TextSection::new (
                    String::new(), 
                    TextStyle {
                        font: asset_server.load("fonts/D2Coding.ttf"),
                        font_size: 30.0,
                        color: Color::BLACK,
                    }
                )
            ]),
            background_color: BackgroundColor(Color::WHITE),
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(80.),
                bottom: Val::Px(10.),
                margin: UiRect {
                    left: Val::Percent(10.),
                    right: Val::Percent(10.),
                    ..default()
                },
                ..default()
            },
            ..default()
        }, 
    ));
}

fn blinking_cursor (
    mut text_cursor: Query<&mut Text, With<Node>>,
    time: Res<Time>,
    mut timer: Query<&mut BlinKTimer>,
) {
    let text_cursor_style = &mut text_cursor.single_mut().sections[2].style;
    let mut timer = timer.single_mut();
    if timer.timer.tick(time.delta()).just_finished() {
        match text_cursor_style.color {
            Color::Srgba(BLACK) => {
                text_cursor_style.color = Color::Srgba(WHITE);
            }
            _ => {
                text_cursor_style.color = Color::Srgba(BLACK);
            }
        }
    }

}

fn ime_enable(
    mut window: Query<&mut Window>,
){
    let mut window = window.single_mut();
    window.ime_enabled = true;
}

fn listen_ime_events (
    mut events: EventReader<Ime>,
    mut edit_text: Query<&mut Text, With<Node>>,
) {
    for event in events.read() {
        match event {
            Ime::Preedit { value,  .. }  => {
                edit_text.single_mut().sections[1].value = String::from(value);
            }
            Ime::Commit { value, .. } => {
                edit_text.single_mut().sections[0].value.push_str(value);
            }
            Ime::Enabled { .. } => {
                println!("IME Active")
            }
            Ime::Disabled { .. } => {
                println!("IME Inactive")
            }
        }
    }
}

fn listen_keyboard_input_events(
    mut events: EventReader<KeyboardInput>,
    mut edit_text: Query<&mut Text, With<Node>>,
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
                text.sections[0].value = String::new();
            }
            Key::Backspace => {
                let mut text = edit_text.single_mut();
                if text.sections[1].value.is_empty() {
                    text.sections[0].value.pop();
                }
            }
            Key::Space => {
                edit_text.single_mut().sections[0].value.push_str(" ");
            }
            Key::Character(character) => {
                edit_text.single_mut().sections[0].value.push_str(character);
            }
            _ => continue,
        }
    }
}
