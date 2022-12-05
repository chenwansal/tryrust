use bevy::prelude::*;

pub fn tick_keyboard_input(input: Res<Input<KeyCode>>) {
    if input.pressed(KeyCode::A) {
        println!("A is pressed");
    }

    if input.just_pressed(KeyCode::B) {
        println!("B is just pressed");
    }

    if input.just_released(KeyCode::C) {
        println!("C is just released");
    }

}