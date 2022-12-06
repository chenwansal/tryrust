use crate::component::com_global_input::ComGlobalInput;
use bevy::prelude::*;

pub fn tick_keyboard_input(
    input: Res<Input<KeyCode>>,
    mut input_entity: Query<&mut ComGlobalInput>,
) {
    let mut input_mut = input_entity.single_mut();
    let dir_ptr = &mut input_mut.move_dir;
    if input.pressed(KeyCode::A) {
        dir_ptr.x = -1.0;
    } else if input.pressed(KeyCode::D) {
        dir_ptr.x = 1.0;
    } else {
        dir_ptr.x = 0.0;
    }

    if input.pressed(KeyCode::W) {
        dir_ptr.y = 1.0;
    } else if input.pressed(KeyCode::S) {
        dir_ptr.y = -1.0;
    } else {
        dir_ptr.y = 0.0;
    }
}
