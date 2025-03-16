use bevy::prelude::*;
use crate::*;

/// (index, current_interaction)
#[derive(Component)]
pub struct Selection(pub usize, pub Interaction);

#[derive(Component, Deref)]
pub struct MenuActionEnum(pub Box<dyn MenuActionEnumTrait>);

impl MenuActionEnum {
    pub fn new(value: impl MenuActionEnumTrait) -> Self {
        Self(Box::new(value))
    }
}

#[derive(Component)]
pub struct OnMenuScreen;
