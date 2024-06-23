use bevy::{prelude::*, core_pipeline::{bloom::Bloom, tonemapping::Tonemapping}};
use crate::screen::{self, Screen};

pub fn run_main() {
    let mut app = App::new();
    app
        .add_plugins(DefaultPlugins)
        .init_state::<Screen>()
        .add_systems(Startup,  setup)
        .add_plugins((
            screen::title::title_plugin,
            screen::game::game_plugin,
        ));

    #[cfg(feature = "steam")] {
        app = steam::init_steam_app(app);
    }

    app.run();
}

fn setup(mut commands: Commands) {
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
