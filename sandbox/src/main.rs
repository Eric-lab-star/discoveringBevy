// sand box
use core::f32;
use std::sync::{Arc, Mutex};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{ EguiContexts, EguiPlugin};
use bevy_egui::egui::{
    Align, RichText, TextEdit, TopBottomPanel, Vec2 as E_Vec2,
    text::LayoutJob, TextFormat, Color32, Ui, FontDefinitions,
    FontFamily, Key, FontData, FontId
};

#[derive(Default, Resource)]
struct UIState {
    output: Arc<Mutex<String>>,
    text_edit: String,
}

fn main() {
    App::new()
        .init_resource::<UIState>()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: String::from("Tower Defence"),
                    ..default()
                }),
                ..default()
            })
        )
        .add_plugins(EguiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, ui_example_system)
        .add_systems(Update, listen_ime_event)
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

fn listen_ime_event (
    mut events: EventReader<Ime>,
) {
    for event in events.read() {
        match event {
            Ime::Preedit { value, .. } => {
                info!("IME Preedit {}", value);
            }
            Ime::Enabled { .. } => {
                info!("IME Enabled");

            }
            Ime::Disabled { .. } => { info!("IME Disabled") }
            Ime::Commit { value, ..} => { info!("IME Commit {}", value) }
        }
    }
}


fn ui_example_system(
    mut uistate: ResMut<UIState>,
    mut contexts: EguiContexts,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
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
                20.0,
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

            let textedit = TextEdit::singleline(&mut uistate.text_edit)
                .hint_text(RichText::new("Press Enter to Submit Your Answer").size(20.0))
                .layouter(&mut layouter)
                .frame(true)
                .min_size(E_Vec2{x: 100.0, y: 40.0})
                .desired_width(f32::INFINITY)
                .vertical_align(Align::Center);

            let response = ui.add(textedit);

            let mut window = primary_window.single_mut();

            if response.has_focus() {
                window.ime_enabled = true;
            }

            if response.has_focus() && ui.input(|i| i.key_pressed(Key::Backspace)) && window.ime_enabled == true {
                println!("Backspace");
                let text_edit_state = TextEdit::load_state(&ctx, response.id);
                match text_edit_state {
                    Some(state) => {
                        match state.cursor.char_range() {
                            Some(mut r) => {
                                println!("{}, {}", r.primary.index, r.secondary.index);
                                r.primary.index += 1;
                                r.secondary.index += 1;
                                TextEdit::store_state(&ctx, response.id, state)
                            },
                            None => {println!("error");}
                        }
                    },
                    None =>{println!("error");}
                }
            }

            if response.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
                uistate.output = Arc::new(Mutex::new(String::new()));
                match uistate.output.lock() {
                    Ok(mut input) => {
                        input.push_str(&*uistate.text_edit);
                    },
                    Err(e) => {
                        println!("Error: {:?}", e);
                    }
                }
                uistate.text_edit.clear();
                response.request_focus();
            }

            match uistate.output.lock() {
                Ok(input) => {
                    ui.label(&*input);
                },
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
        });
}
