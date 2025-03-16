use bevy::prelude::*;
use super::*;
use crate::*;

pub fn plugin_title(app: &mut App) {
    app
        .init_state::<TitleScreen>()
        .add_event::<TitleMenuActionEvent>()
        .add_systems(OnEnter(Mode::Title), system_enter_title)
        .add_systems(OnEnter(TitleScreen::Main), system_enter_title_main)
        .add_systems(OnExit(TitleScreen::Main), system_exit_menuscreen)
        .add_systems(OnEnter(TitleScreen::Connect), system_enter_title_connect)
        .add_systems(OnExit(TitleScreen::Connect), system_exit_menuscreen)
        .add_systems(Update, (
                system_update_menu_action_selection,
                system_update_menu_action_button,
                event_title_menu_action
            ).run_if(in_state(Mode::Title))
        );
        /*.add_systems(Update, (
            system_update_title_connect_mouse,
            system_update_title_connect_button
            ).run_if(in_state(MenuScreen::Connect))
        );*/
}
