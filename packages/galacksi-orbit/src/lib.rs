pub mod plugin;
pub mod model;
pub mod color;
pub mod consts;
pub mod util;
pub mod title;
pub mod game;
pub mod resource;
pub mod config;
pub mod console;
mod run;

pub use {console::*, model::*, resource::*, config::*, run::run};

#[cfg(feature = "steam")]
pub mod steam;

#[cfg(feature = "cli")]
pub mod cli;

#[derive(bevy::prelude::Resource, Default, Copy, Clone, bevy::prelude::States, Debug, Hash, Eq, PartialEq)]
pub enum Mode {
    #[default]
    Title,
    Game
}
