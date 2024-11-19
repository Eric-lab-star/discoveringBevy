// sand box
use core::f32;
use std::sync::{Arc, Mutex};
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
    output: Arc<Mutex<String>>,
    text_edit: String,
}

fn main() {
    App::new()
        .init_resource::<UIState>()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, ui_example_system)
        .add_systems(Update, ime_toggle)
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

fn ime_toggle(
    mut q_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut window = q_window.single_mut();
    window.ime_enabled =  true;
}




fn ui_example_system(
    mut uistate: ResMut<UIState>,
    mut contexts: EguiContexts,
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
                .min_size(Vec2{x: 100.0, y: 40.0})
                .desired_width(f32::INFINITY)
                .vertical_align(Align::Center);

            let response = ui.add(textedit);

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
