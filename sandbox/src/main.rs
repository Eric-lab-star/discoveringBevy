use std::sync::{Arc, Mutex};

use bevy::prelude::*;
use bevy_egui::{ EguiContexts, EguiPlugin};
use bevy_egui::egui::{
    TopBottomPanel,
    Key
};

#[derive(Default, Resource)]
struct UIState {
    input: Arc<Mutex<String>>,
    output: Arc<Mutex<String>>,
}

fn main() {
    App::new()
        .init_resource::<UIState>()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_systems(Update, ui_example_system)
        .run();
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
            ui.label("Score");

            let response = ui.text_edit_singleline(&mut *user_input);
            if response.changed() {
                uistate.input = Arc::new(Mutex::new(String::new()));
                uistate.input.lock().unwrap().push_str(&*user_input);
            }

            if response.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
                uistate.output = Arc::clone(&uistate.input);   
                *user_input = String::new();
                response.request_focus();
            }
            ui.label(&*uistate.output.lock().unwrap());
        });

}
