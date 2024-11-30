
use bevy::prelude::*;
use bevy_egui::egui::{text::LayoutJob, Color32, FontId};
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


#[derive( Resource)]
pub struct EditorLayoutJob {
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
    pub fn get(&self, key: &str, wrap_width: f32) -> LayoutJob {
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

