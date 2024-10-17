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
                height: Val::Px(33.0),
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


// text section 0: input before cursor 
// text section 1: ime buffer
// text section 2: cursor
// text section 3: input after cursor
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

            Key::ArrowLeft => {
                move_cursor(Direction::Left, &mut edit_text)
                // let mut text_node = edit_text.single_mut();
                // let left = &mut text_node.sections[0];
                // let pop_char = left.value.pop().unwrap_or_default();
                // let msg = &text_node.sections[3].value;
                // let msg = format!("{pop_char}{msg}");
                // text_node.sections[3].value = msg
            }

            Key::ArrowRight => {
                move_cursor(Direction::Right, &mut edit_text)
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

enum Direction {
    Left,
    Right,
}

fn move_cursor (
    direction: Direction,
    edit_text: &mut Query<&mut Text, With<Node>>,
) {

    let mut text_node = edit_text.single_mut();

    let (pop_from, insert_to) = match direction {
        Direction::Left => (0, 3),
        Direction::Right => (3, 0),
    };

    let src_text = &mut text_node.sections[pop_from];
    let pop_char = if pop_from == 0 {
        match src_text.value.pop() {
            Some(ch) => {
                String::from(ch)
            }
            None => {
                String::new()
            }
        }

    } else {
        let mut iter = src_text.value.chars();
        let first_char = iter.next();
        match first_char {
            Some(ch) => {
                src_text.value = src_text.value[ch.len_utf8()..].to_string();
                String::from(ch)
            }
            None => {
                String::new()
            }

        }
    };
    let msg = &text_node.sections[insert_to].value;
    let msg = if pop_from == 0 {
        format!("{pop_char}{msg}")
    } else {
        format!("{msg}{pop_char}")
    };
    text_node.sections[insert_to].value = msg;

}
