use bevy_console::ConsoleCommand;
use clap::Parser;


#[derive(Parser, ConsoleCommand)]
#[command(name = "example")]
pub struct ExampleCommand {
    msg: String
}

pub fn example_command(
    mut log: ConsoleCommand<ExampleCommand>,
) {
    if let Some(Ok(ExampleCommand { msg })) = log.take() {
        ()
    }
}
