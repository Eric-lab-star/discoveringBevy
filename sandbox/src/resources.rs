
use bevy::prelude::*;
use bevy_egui::egui::{text::LayoutJob};
use bevy_egui::egui::{
    Ui, FontId, Color32, Galley
};
use std::{collections::HashMap, sync::{Arc, Mutex}};

#[derive(Default, Resource)]
pub struct UIState {
   pub output: Arc<Mutex<String>>,
   pub text_edit: String,
}

#[derive(Resource, Default)]
pub struct ImeValue {
    pub trig_backspace: bool,
}


#[derive(Resource)]
pub struct  TextEditorLayoutJobCache {
    cache: Arc<Mutex<HashMap<String, LayoutJob>>>
}

impl Default for TextEditorLayoutJobCache {
    fn default() -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new()))
        }
    }
}

impl TextEditorLayoutJobCache {
    pub fn cache(&self) -> Arc<Mutex<HashMap<String, LayoutJob>>>{
        let cache = Arc::clone(&self.cache);
        cache
    }

    /// print hashmap size
    /// with info!
    fn _print_len(&self)  {
        let hashmap = self.cache.lock().unwrap();
        info!("{}",hashmap.len());
    }

    pub fn textarea_layouter<'a> (
        &'a self
    ) -> impl Fn(&Ui, &str, f32) -> Arc<Galley> +'a {
        |ui: &Ui, text: &str, wrap_width: f32| {
            ui.fonts(|f| {
                let cache = &self.cache();
                let cache = cache.lock().unwrap().entry(text.to_string()).or_insert_with(|| {
                    let mut textarea_layoutjob = LayoutJob::simple_singleline(
                        text.to_string(),
                        FontId::proportional(20.0),
                        Color32::WHITE);
                    textarea_layoutjob.wrap.max_width = wrap_width;
                    textarea_layoutjob
                }).clone();

                f.layout_job(cache)
            })
        }
    }
}

#[derive(Resource)]
pub struct Words {
    list: Vec<String>,
    current_index: usize,
}

impl Default for Words {
    fn default()-> Self {
        let word_list =   vec![
            "모모".to_string(),
            "사나".to_string(),
            "정연".to_string(),
            "미나".to_string(),
            "채영".to_string(),
            "지효".to_string(),
            "다현".to_string(),
        ];

        Words  {
            list: word_list,
            current_index: 0,
        }
    }

}

impl Words {
    pub fn list(&self) -> &Vec<String> {
        &self.list
    }

    pub fn current_index(&self) -> usize{
        self.current_index
    }

    pub fn next_word(&mut self) {
        let current = self.current_index;
        if current < self.list.len() - 1 {
            self.current_index = self.current_index + 1;
        } else {
            self.current_index = 0;
        }

    }

}

