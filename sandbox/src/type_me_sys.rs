use bevy::{prelude::*, text::Text2dBounds};

use crate::resources;


pub fn change_words (
    words: Res<resources::Words>,
    mut text: Query<&mut Text, With<Text2dBounds>>,
) {
    let mut text = text.single_mut();
    let list = words.list();
    text.sections[0].value = list.get(words.current_index()).unwrap().to_string();
}
