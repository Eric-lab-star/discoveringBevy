use bevy::{prelude::*, window::PrimaryWindow};

use std::sync::{Arc, Mutex};

/// egui
use bevy_egui::{ EguiContexts};
use bevy_egui::egui::text::LayoutJob;
use bevy_egui::egui::{
    Align, Color32, FontId, Galley, Key, RichText, TextEdit, TextFormat, TopBottomPanel, Ui, Vec2 
};

use crate::{resources, Score};

#[derive(Component)]
pub struct TextEditor<'a> {
    editor: TextEdit<'a>, 
}

impl<'a> TextEditor<'a> {
    pub fn new(
        text_buffer: &'a mut String,
        layouter: &'a mut dyn FnMut(&Ui, &str, f32) -> Arc<Galley>
    ) -> Self 
    {
        let editor = TextEdit::singleline(text_buffer)
            .hint_text(RichText::new("Press Enter to Submit Your Answer").size(20.0))
            .layouter(layouter)
            .frame(false)
            .min_size(Vec2{x: 100.0, y: 40.0})
            .desired_width(f32::INFINITY)
            .vertical_align(Align::Center);

        TextEditor {
            editor 
        }

    }
}

pub fn text_editor_ui (
    mut uistate: ResMut<resources::UIState>,
    mut contexts: EguiContexts,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
    layout_cache: Res<resources::TextEditorLayoutJobCache>,
    mut score: Query<&mut Score>,
    mut words: ResMut<resources::Words>,

) {
    let ctx = contexts.ctx_mut();

    let mut score = score.single_mut();

    TopBottomPanel::bottom("bottom")
        .min_height(100.0)
        .resizable(false)
        .show(ctx, |ui| {
            let mut textarea_layoutjob = layout_cache.textarea_layouter();

            let textedit = TextEdit::singleline(&mut uistate.text_edit)
                .hint_text(RichText::new("Press Enter to Submit Your Answer").size(20.0))
                .layouter(&mut textarea_layoutjob)
                .frame(false)
                .min_size(Vec2{x: 100.0, y: 40.0})
                .desired_width(f32::INFINITY)
                .vertical_align(Align::Center);

            let response = ui.add(textedit);
            let mut window = primary_window.single_mut();

            if response.has_focus() {
                window.ime_enabled = true;
            } else {
                window.ime_enabled = false;
            }

            if response.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
                uistate.output = Arc::new(Mutex::new(String::new()));
                match uistate.output.lock() {
                    Ok(mut output) => {
                        let input = &*uistate.text_edit;
                        output.push_str(input);
                        let correct_word = words.list().get(words.current_index()).unwrap();
                        if input ==  correct_word {
                            score.0 += 1;
                            words.next_word();

                            
                        }
                    },

                    Err(e) => {
                        println!("Error: {:?}", e);
                    }
                }
                uistate.text_edit.clear();
                response.request_focus();
            }

            match uistate.output.lock() {
                Ok(output) => {
                    ui.label(&*output);
                },

                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }

            let mut job =  LayoutJob::default();
            job.append (
                "Score: ",
                0.0,
                TextFormat {
                    font_id: FontId::proportional(15.0),
                    ..default()
                }
            );

            job.append (
                &score.0.to_string(),
                0.0,
                TextFormat {
                    color: Color32::RED,
                    font_id: FontId::proportional(15.0),
                    ..default()
                }
            );

            ui.label(job);
        });
}
