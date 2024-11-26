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

#[derive(Default, Resource)]
struct UIState {
    output: Arc<Mutex<String>>,
    text_edit: String,
}

#[derive(Resource, Default)]
struct ImeValue {
    trig_backspace: bool,
}

#[derive( Resource)]
struct EditorLayoutJob {
    cache: Arc<Mutex<HashMap<String, LayoutJob>>>
}

impl Default for EditorLayoutJob {
    fn default() -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new()))
        }
    }
}

impl EditorLayoutJob {
    fn get(&self, key: &str, wrap_width: f32) -> LayoutJob {
        let cache = Arc::clone(&self.cache);
        let mut hashmap = cache.lock().unwrap();
        let layoutjob = hashmap.get(key);
        match layoutjob {
            Some(value) => {
                value.clone()
            }
            None => {
                let mut new_job = LayoutJob::simple_singleline(
                    key.to_string(),
                    FontId::proportional(20.0),
                    Color32::WHITE
                );

                new_job.wrap.max_width = wrap_width; 

                let clone = new_job.clone();
                if hashmap.len() > 100 {
                    hashmap.clear();
                }
                hashmap.insert(key.to_string(), new_job);
                clone
            }
        }
    }
}

fn main() {
    App::new()
        .init_resource::<UIState>()
        .init_resource::<ImeValue>()
        .init_resource::<EditorLayoutJob>()
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
    mut ime_value: ResMut<ImeValue>,
    mut ui_state: ResMut<UIState>
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
    mut ime_value: ResMut<ImeValue>
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
    mut uistate: ResMut<UIState>,
    mut contexts: EguiContexts,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
    editor_layout_job: Res<EditorLayoutJob>

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
