
//modules 
mod resources;
mod type_me_sys;
mod ime_sys;
mod bottom_panel_sys;

// external dependencies

// bevy
use bevy::prelude::*;

/// egui
use bevy_egui::{ EguiContexts, EguiPlugin};
use bevy_egui::egui::{
    FontData, FontDefinitions, FontFamily, 
};
use bottom_panel_sys::TextEditor;

#[derive(Component)]
struct Score(i32);

impl Default for Score {
    fn default() -> Self {
        Self(0)
    }
}

fn main() {
    App::new()
        .init_resource::<resources::UIState>()
        .init_resource::<resources::ImeValue>()
        .init_resource::<resources::TextEditorLayoutJobCache>()
        .init_resource::<resources::Words>()
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
        .add_systems(Startup, egui_setup)
        .add_systems(Update, ime_sys::change_trig_backspace_state)
        .add_systems(Update, bottom_panel_sys::text_editor_ui)
        .add_systems(Update, ime_sys::enable_smooth_input_delete)
        .add_systems(Update, type_me_sys::change_words)
        .run();
}

fn setup (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let text_style = TextStyle  {
        font: asset_server.load("../assets/fonts/korean/NotoSansKR-Bold.ttf"),
        font_size: 60.0,
        color: Color::WHITE,
    };

    commands.spawn(Camera2dBundle::default());
    commands.spawn(Score::default());
    commands.spawn(
        Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: "안녕하세요".to_string(),
                    style: text_style,                                       
                }],
                justify:JustifyText::Center,
                ..default()
            },
            ..default()
        }
    );
}

fn egui_setup (
    mut contexts: EguiContexts,
    mut commands: Commands,
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


