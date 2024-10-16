// eric

use bevy::{
    color::palettes::css::PURPLE, input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
    }, prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>
) {
    commands.spawn(Camera2dBundle::default());
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
                TextSection::new(
                    String::new(),
                    TextStyle {
                        font: asset_server.load("fonts/D2Coding.ttf"),
                        font_size: 30.0,
                        color: Color::BLACK,
                    }
                ),
            ]),
            background_color: BackgroundColor(Color::WHITE),
            style: Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(30.),
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

    commands.spawn(MaterialMesh2dBundle  {
        mesh: meshes.add(Rectangle::new(10.0, 30.0)).into(),
        material: materials.add(Color::from(PURPLE)),
        transform: Transform::from_xyz(0.,0., 0.),
        ..default()
    });
}

fn blinking_cursor (
    mut cursor: Query<&mut Transform, With<Mesh2dHandle>>,
    mut text_node: Query<(&mut Node, &mut GlobalTransform), With<Text>>,
) {

    let mut cursor = cursor.single_mut();
    cursor.translation.x = -100.;

    // let (node, gt) = text_node.single_mut();
    // let Rect{min, max: _} =  node.logical_rect(&gt);
    // println!("{:?}", min);
    //
    //  let cursor = cursor.single_mut().into_inner();
    //  cursor.translation = Vec3 {x: min.x, y: min.y, z: 0.0}
     
     

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
