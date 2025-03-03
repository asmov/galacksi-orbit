use bevy::{prelude::*, core_pipeline::{bloom::Bloom, tonemapping::Tonemapping}};
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
        app
            .add_plugins(DefaultPlugins)
            .insert_state::<Mode>(self.mode)
            .add_systems(Startup,  system_startup)
            .add_plugins((
                title::plugin_title,
                game::plugin_game,
                #[cfg(feature = "steam")] steam::plugin_steam,
            ));
    }
}

fn system_startup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            hdr: true, // bloom requires HDR
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        Tonemapping::TonyMcMapface,
        Bloom::default(), // enable bloom for the camera
    ));
}
