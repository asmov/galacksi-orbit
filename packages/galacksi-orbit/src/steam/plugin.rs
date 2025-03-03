use bevy::prelude::*;
use bevy_steamworks as steamworks;
use crate::*;
use super::*;

pub const STEAM_APP_ID: u32 = 3110770;

pub fn plugin_steam(app: &mut App) {
    app
        .add_plugins(
            steamworks::SteamworksPlugin::init_app(STEAM_APP_ID)
                .expect("Failed to initialize Steam API")
        )
        .add_systems(Startup, system_startup_steam)
        .add_systems(OnEnter(Mode::Title),
            system_enter_title_steam_input
        )
        .add_systems(Update, (
            system_update_title_steam_input
        ).run_if(in_state(Mode::Title)))
        .add_systems(OnEnter(Mode::Game), (
            system_enter_game_steam_input
        ))
        .add_systems(Update, (
            system_update_game_steam_input,
        ).run_if(in_state(Mode::Game)));
}

pub fn system_startup_steam(mut commands: Commands, steamworks: Res<steamworks::Client>) {
    commands.insert_resource(ControllerInputCollection::default());
    steamworks.input().init(false);
}

fn system_exit_steam(mut commands: Commands) {
    commands.remove_resource::<ControllerInputCollection>();
}
