use bevy::{app::PluginGroupBuilder, log::{self, LogPlugin}, prelude::*};
use bevy_console::{make_layer, AddConsoleCommand, ConsoleConfiguration, ConsolePlugin};
use super::*;

pub fn plugin_console(app: &mut App) {
    app
        .add_plugins(ConsolePlugin)
        .insert_resource(ConsoleConfiguration {
            background_color: bevy_egui::egui::Color32::from_black_alpha(245),
            ..default()
        })
        .add_console_command::<ExampleCommand, _>(example_command);
}

pub fn build_default_plugins(default_plugins: PluginGroupBuilder) -> PluginGroupBuilder {
    default_plugins.set(LogPlugin {
        filter: "error,capture_bevy_logs=info".to_string(),
        level: log::Level::INFO,
        custom_layer: make_layer
    })
}
