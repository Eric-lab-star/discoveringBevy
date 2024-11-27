
//modules 
mod resources;

// external dependencies
use core::f32;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// bevy
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

/// egui
use bevy_egui::{ EguiContexts, EguiPlugin};
use bevy_egui::egui::{
    Align, RichText, TextEdit, TopBottomPanel, Vec2 as E_Vec2,
    text::LayoutJob, Color32, Ui, FontDefinitions,
    FontFamily, Key, FontData, FontId
};




fn main() {
    App::new()
        .init_resource::<resources::UIState>()
        .init_resource::<resources::ImeValue>()
        .init_resource::<resources::EditorLayoutJob>()
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
        .add_systems(Update, listen_ime_event)
        .add_systems(Update, text_editor_ui)
        .add_systems(Update, trigger_ime_event)
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

fn trigger_ime_event(
    mut ime_event_wr: EventWriter<Ime>,
    keyboard: Res<ButtonInput<KeyCode>>,
    win_entity: Query<Entity, With<PrimaryWindow>>,
    pri_window: Query<&Window, With<PrimaryWindow>>,
    mut ime_value: ResMut<resources::ImeValue>,
    mut ui_state: ResMut<resources::UIState>
) {
    let pri_window = pri_window.single();
    if pri_window.ime_enabled {
        if keyboard.just_pressed(KeyCode::Backspace)
            && ime_value.trig_backspace {
            ime_event_wr.send(
                Ime::Commit{
                    value: String::from(""),
                    window: win_entity.single(), 
                }
            );
            ui_state.text_edit.pop();
            ime_value.trig_backspace = false;
        };
    }
}

fn listen_ime_event (
    mut events: EventReader<Ime>,
    mut ime_value: ResMut<resources::ImeValue>
) {
    for event in events.read() {
        match event {
            Ime::Preedit { value, .. } => {
                if value.is_empty() {
                    ime_value.trig_backspace = true;
                }
            }
            _=> {}
        }
    }
}


fn text_editor_ui (
    mut uistate: ResMut<resources::UIState>,
    mut contexts: EguiContexts,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
    editor_layout_job: Res<resources::EditorLayoutJob>

) {
    let ctx = contexts.ctx_mut();

    TopBottomPanel::bottom("bottom")
        .min_height(100.0)
        .resizable(false)
        .show(ctx, |ui| {
            let mut layouter = |ui: &Ui, string: &str, wrap_width: f32| {
                let job = editor_layout_job.get(string, wrap_width);
                ui.fonts(|f| f.layout_job(job))
            };

            let textedit = TextEdit::singleline(&mut uistate.text_edit)
                .hint_text(RichText::new("Press Enter to Submit Your Answer").size(20.0))
                .layouter(&mut layouter)
                .frame(false)
                .min_size(E_Vec2{x: 100.0, y: 40.0})
                .desired_width(f32::INFINITY)
                .vertical_align(Align::Center);

            let response = ui.add(textedit);
            let mut window = primary_window.single_mut();

            if response.has_focus() {
                window.ime_enabled = true;
            } else {
                window.ime_enabled = false;
            }

            // print input to output area
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
