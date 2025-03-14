use bevy::{
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping}, prelude::*, window::PrimaryWindow
};
use bevy_console::PrintConsoleLine;
use bevy_tiling_background::{BackgroundImageBundle, BackgroundMaterial, SetImageRepeatingExt};
use crate::*;


pub fn system_startup(
    mut commands: Commands,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
    asset_server: Res<AssetServer>,
)
{
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

pub fn system_startup_greet_console(mut console_line: EventWriter<PrintConsoleLine>) {
    let text = format!("Welcome to {galacksi_orbit}\nUse {help} for more information\n\n",
        galacksi_orbit = ansi_term::Color::Green.paint("Galacksi Orbit"),
        help = ansi_term::Color::Yellow.paint("help")
    );
    console_line.send(PrintConsoleLine::new(text));
}

pub fn system_update_mouse_world_position(
    mut mouse_position: ResMut<MousePosition>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>
) {
    let window = window.single();
    let (camera, camera_global_transform) = camera.single();

    mouse_position.window_position = window.cursor_position();
    mouse_position.position = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_global_transform, cursor).ok())
        .map(|ray| ray.origin.truncate());

}
