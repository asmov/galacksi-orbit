use bevy::{app::PluginGroupBuilder, log::{self, LogPlugin}, prelude::*};
use bevy_console::{AddConsoleCommand, ConsoleConfiguration, ConsolePlugin, make_filtered_layer};
use super::*;

pub fn plugin_console(app: &mut App) {
    app
        .add_plugins(ConsolePlugin)
        .insert_resource(ConsoleConfiguration {
            title_name: "Galacksi Orbit Console".to_string(),
            background_color: bevy_egui::egui::Color32::from_black_alpha(245),
            ..default()
        })
        .add_console_command::<TeleportCmd, _>(teleport_cmd);
}

pub fn build_default_plugins(default_plugins: PluginGroupBuilder) -> PluginGroupBuilder {
    default_plugins.set(LogPlugin {
        filter: log::DEFAULT_FILTER.to_string(),
        level: log::Level::INFO,
        custom_layer: |app: &mut App | make_filtered_layer(app, "galacksi_orbit=info,warn,debug,error".to_string())
    })
}
