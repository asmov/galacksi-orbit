use bevy::{
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
    prelude::*
};
use bevy_tiling_background::{BackgroundImageBundle, BackgroundMaterial, SetImageRepeatingExt, TilingBackgroundPlugin};
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
            .set(ImagePlugin::default_linear());
        let default_plugins = console::build_default_plugins(default_plugins);

        app
            .add_plugins((
                default_plugins,
                TilingBackgroundPlugin::<BackgroundMaterial>::default(),
            ))
            .insert_state::<Mode>(self.mode)
            .init_resource::<PlayerConfigs>()
            .add_systems(Startup, system_startup)
            .add_plugins((
                console::plugin_console,
                title::plugin_title,
                game::plugin_game,
                #[cfg(feature = "steam")] steam::plugin_steam,
            ));
    }
}

fn system_startup(
    mut commands: Commands,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
    asset_server: Res<AssetServer>)
{
    //let image_handle = asset_server.load("bg-stars.png");
    //commands.set_image_repeating(image_handle);
    let image_handle = asset_server.load("bg-stars.png");
    commands.set_image_repeating(image_handle.clone());

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

    commands.spawn(
        BackgroundImageBundle::from_image(image_handle, materials.as_mut())
            .with_movement_scale(0.3)
            .at_z_layer(0.1),
    );
}
