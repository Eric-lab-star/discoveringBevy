use bevy::{prelude::*, window::PrimaryWindow};

use crate::resources;


pub fn enable_smooth_input_delete (
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


pub fn change_trig_backspace_state (
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
