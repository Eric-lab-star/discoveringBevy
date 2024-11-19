// sand box
use core::f32;
use std::sync::{Arc, Mutex};
use bevy::input::keyboard::{KeyboardInput, Key as BKey};
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::egui::{FontData, FontId};
use bevy_egui::{ EguiContexts, EguiPlugin};
use bevy_egui::egui::{
    Align, RichText, TextEdit, TopBottomPanel, Vec2,
    text::LayoutJob, TextFormat, Color32, Ui, FontDefinitions,
    FontFamily, Key
};

#[derive(Default, Resource)]
struct UIState {
    input: Arc<Mutex<String>>,
}

fn main() {
    App::new()
        .init_resource::<UIState>()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, ui_example_system)
        .add_systems(Update, keyboard_input)
        .add_systems(Update, ime_toggle)
        .add_systems(Update, ime_input)
        .run();
}



fn setup(
    mut contexts: EguiContexts,
) {
    let ctx = contexts.ctx_mut();
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "korean".to_owned(),
        FontData::from_static(include_bytes!("../assets/fonts/korean/NotoSansKR-Bold.ttf")));

    fonts.families.get_mut(&FontFamily::Proportional).unwrap()
        .insert(0, "korean".to_owned());
    ctx.set_fonts(fonts);
}

fn keyboard_input(
    mut evr_kbd: EventReader<KeyboardInput>,
    mut string: Local<String>,

) {
    for ev in evr_kbd.read() {
        if ev.state == ButtonState::Released {
            continue;
        }
        match &ev.logical_key {
            BKey::Enter => {
                info!("Text Input: {}", &*string);
                string.clear();
            }
            BKey::Backspace => {
                string.pop();
            }
            BKey::Character(input) => {
                if input.chars().any(|c| c.is_control()) {
                    continue;
                }
                string.push_str(&input);
            }
            _ => {}
        }
    }
}

fn ime_toggle(
    mousebtn: Res<ButtonInput<MouseButton>>,
    mut q_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    if mousebtn.just_pressed(MouseButton::Left) {
        let mut window = q_window.single_mut();

        // toggle "IME mode"
        window.ime_enabled = !window.ime_enabled;

        // We need to tell the OS the on-screen coordinates where the text will
        // be displayed; for this simple example, let's just use the mouse cursor.
        // In a real app, this might be the position of a UI text field, etc.
        window.ime_position = window.cursor_position().unwrap();
    }
}

fn ime_input(
    mut evr_ime: EventReader<Ime>,
) {
    for ev in evr_ime.read() {
        match ev {
            Ime::Commit { value, .. } => {
                println!("IME confirmed text: {}", value);
            }
            Ime::Preedit { value, cursor, .. } => {
                println!("IME buffer: {:?}, cursor: {:?}", value, cursor);
            }
            Ime::Enabled { .. } => {
                println!("IME mode enabled!");
            }
            Ime::Disabled { .. } => {
                println!("IME mode disabled!");
            }
        }
    }
}


fn ui_example_system(
    mut uistate: ResMut<UIState>,
    mut contexts: EguiContexts,
    mut user_input: Local<String>
) {
    let ctx = contexts.ctx_mut();



    TopBottomPanel::bottom("bottom")
        .min_height(100.0)
        .resizable(false)
        .show(ctx, |ui| {
            ui.label("안녕하세요");

            let mut job = LayoutJob::default();
            job.append(
                "안녕하세요",
                0.0,
                TextFormat {
                    font_id: FontId {
                        size: 20.0,
                        family: FontFamily::Proportional,
                    },
                    color: Color32::WHITE,
                    ..Default::default()
                }
            );

            job.append(
                "Human",
                10.0,
                TextFormat {
                    font_id: FontId::monospace(20.0),
                    color: Color32::RED,
                    ..Default::default()
                }
            );

            ui.label(job);
            let mut layouter = |ui: &Ui, string: &str, wrap_width: f32| {
                let mut input_layout_job = LayoutJob::simple_singleline(
                    String::from(string),
                    FontId::proportional(20.0),
                    Color32::WHITE
                );
                input_layout_job.wrap.max_width = wrap_width;
                ui.fonts(|f| f.layout_job(input_layout_job))
            };

            let textedit = TextEdit::singleline(&mut *user_input)
                .hint_text(RichText::new("Press Enter to Submit Your Answer").size(19.0))
                .layouter(&mut layouter)
                .frame(true)
                .min_size(Vec2{x: 100.0, y: 40.0})
                .desired_width(f32::INFINITY)
                .vertical_align(Align::Center);

            let response = ui.add(textedit);
            if response.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
                uistate.input = Arc::new(Mutex::new(String::new()));
                match uistate.input.lock() {
                    Ok(mut input) => {
                        input.push_str(&*user_input);
                    },
                    Err(e) => {
                        println!("Error: {:?}", e);
                    }
                }
                *user_input = String::new();
                response.request_focus();
            }

            match uistate.input.lock() {
                Ok(input) => {
                    ui.label(&*input);
                },
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
        });

}
