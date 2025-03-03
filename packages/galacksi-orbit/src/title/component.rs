use bevy::prelude::*;

#[derive(Component)]
pub struct OnMenuScreen;

/// (index, current_interaction)
#[derive(Component, Debug)]
pub struct Selection(pub usize, pub Interaction);

#[derive(Component, strum::EnumCount)]
pub enum MenuAction {
    Simulate = 0,
    Quit = 1
}
