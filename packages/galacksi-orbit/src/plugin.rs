use bevy::prelude::*;
use bevy_console::ConsoleSet;
use bevy_tiling_background::{BackgroundMaterial, TilingBackgroundPlugin};
use crate::*;

#[derive(Default)]
pub struct OrbitPlugin {
    mode: Mode
}

impl OrbitPlugin {
    pub fn new(mode: Mode) -> Self {
        Self { mode }
    }
}

impl Plugin for OrbitPlugin {
    fn build(&self, app: &mut App) {
        let default_plugins = DefaultPlugins.build()
            .set(ImagePlugin::default_linear())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Galacksi Orbit".into(),
                    name: Some("orbit.galacksi.app".into()),
                    ..default()
                }),
                ..default()
            });

        let default_plugins = console::build_default_plugins(default_plugins);

        app
            .add_plugins((
                default_plugins,
                TilingBackgroundPlugin::<BackgroundMaterial>::default(),
            ))
            .insert_state::<Mode>(self.mode)
            .init_resource::<MousePosition>()
            .init_resource::<PlayerConfigs>()
            .add_systems(Startup, system_startup)
            .add_systems(Update, system_update_mouse_world_position)
            .add_plugins((
                console::plugin_console,
                title::plugin_title,
                game::plugin_game,
                #[cfg(feature = "steam")] steam::plugin_steam,
            ))
            .add_systems(Startup, system_startup_greet_console.after(ConsoleSet::ConsoleUI));

        #[cfg(debug_assertions)] {
            app.add_plugins((
                bevy::diagnostic::FrameTimeDiagnosticsPlugin,
                bevy::diagnostic::LogDiagnosticsPlugin::default()
            ));
        }
    }
}
