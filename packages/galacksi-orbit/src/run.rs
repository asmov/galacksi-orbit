use bevy::prelude::*;
use crate::plugin::OrbitPlugin;

pub fn run() {
    #[allow(unused_assignments)]
    let mut app = None;

    #[cfg(feature = "cli")] {
        app = Some(cli_app());
    }

    if app.is_none() {
        app = Some(default_app());
    };

    app.expect("Expected app").run();
}

pub fn default_app() -> App {
    let mut app = App::new();
    app.add_plugins(OrbitPlugin::default());
    app
}

#[cfg(feature = "cli")]
pub fn cli_app() -> App {
    use crate::cli::{Cli, Parser};

    let cli = Cli::parse();
    let mut app = App::new();
    app.add_plugins(OrbitPlugin::from_cli(&cli));
    app
}
