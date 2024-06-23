use bevy::prelude::*;
use bevy_steamworks as steamworks;

pub const STEAM_APP_ID: u32 = 3110770;

pub fn init_steam_app(app: App) -> App {
    app
        .add_plugins(steam::SteamworksPlugin::init_app(STEAM_APP_ID)
            .expect("Failed to initialize Steam API"))
        .add_systems(Startup, setup_steam);

    app
}

pub fn setup_steam(steam: Res<steamworks::Client>) {
    steamworks.input().init(false);
}
