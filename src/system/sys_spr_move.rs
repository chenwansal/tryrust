use crate::component::com_global_input::ComGlobalInput;
use bevy::prelude::*;

pub fn tick_spr_move(
    input_entity: Query<&ComGlobalInput>,
    mut sprite_entity: Query<&mut Transform, With<Handle<Image>>>,
) {
    let mut sprite_mut = sprite_entity.single_mut();

    sprite_mut.translation.x += input_entity.single().move_dir.x;
    sprite_mut.translation.y += input_entity.single().move_dir.y;
}