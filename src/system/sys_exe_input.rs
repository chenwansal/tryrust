use crate::component::com_global_input::ComGlobalInput;
use bevy::prelude::*;

pub fn tick_keyboard_input(
    input: Res<Input<KeyCode>>,
    mut input_entity: Query<&mut ComGlobalInput>,
) {
    let mut input_mut = input_entity.single_mut();
    if input.pressed(KeyCode::A) {
        input_mut.move_dir.x = -1.0;
    } else if input.pressed(KeyCode::D) {
        input_mut.move_dir.x = 1.0;
    } else {
        input_mut.move_dir.x = 0.0;
    }

    if input.pressed(KeyCode::W) {
        input_mut.move_dir.y = 1.0;
    } else if input.pressed(KeyCode::S) {
        input_mut.move_dir.y = -1.0;
    } else {
        input_mut.move_dir.y = 0.0;
    }
}
