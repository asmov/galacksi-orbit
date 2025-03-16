use bevy::prelude::*;

mod component;
mod resource;
mod plugin;
mod main;
mod connect;

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Copy, Default)]
pub enum TitleScreen {
    #[default]
    Inactive,
    Main,
    Connect,
}

pub use self::{main::*, connect::*, plugin::*};
