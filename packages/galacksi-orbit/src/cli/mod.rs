pub use clap::Parser;
use clap::Subcommand;
use crate::{Mode, plugin::OrbitPlugin};

#[derive(Parser)]
#[command(version, about, author)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>
}

#[derive(Subcommand)]
pub enum Command {
    Simulate(SimulateCommand)
}

/// Launches the game into simulation mode
#[derive(Parser)]
pub struct SimulateCommand {}

impl OrbitPlugin {
    pub fn from_cli(cli: &Cli) -> OrbitPlugin {
        let mode = match cli.command {
            Some(Command::Simulate(_)) => Mode::Game,
            None => Mode::default(),
        };

        OrbitPlugin::new(mode)
    }
}
