use bevy::prelude::*;

#[derive(Resource)]
pub struct ResoTimer {
    pub value: Timer,
}