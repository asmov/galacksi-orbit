use bevy::prelude::*;
use super::*;
use crate::title::*;

pub fn system_update_title_steam_input(
    steam_client: Res<steamworks::Client>,
    mut ui_query: Query<&mut Selection, With<Button>>,
    mut selected: ResMut<Selected>,
    mut controller_inputs: ResMut<ControllerInputCollection>,
) {
    controller_inputs.update(&steam_client);

    for controller_input in &mut controller_inputs.controller_inputs {
        let ControllerInput::Menu(_, input) = controller_input else {
            if matches!(controller_input, ControllerInput::Connected{ .. }) {
                controller_input.set_type(ControllerInputType::Menu);
            }

            continue
        };

        let menu_down = input.down.just_pressed;
        let menu_up = input.up.just_pressed;
        let menu_select = input.select.just_pressed;

        let index;
        let index_interaction;
        if menu_down || menu_up {
            index_interaction = Interaction::Hovered;
            index = match selected.0 {
                None => if menu_down { 0 } else { MenuAction::COUNT - 1 },
                Some((index,_)) => if menu_down {
                    if index >= MenuAction::COUNT - 1 { 0 } else { index + 1 }
                } else {
                    if index <= 0 { MenuAction::COUNT - 1 } else { index - 1 }
                }
            };
        } else if menu_select {
            index_interaction = Interaction::Pressed;
            index = match selected.0 {
                Some((index,_)) => index,
                None => continue
            };
        } else {
            continue;
        }

        selected.0 = Some((index, index_interaction));

        for mut selection in &mut ui_query.iter_mut() {
            if selection.0 == index {
                selection.1 = index_interaction;
            } else {
                selection.1 = Interaction::None;
            }
        }
    }
}

pub fn system_enter_title_steam_input(steam: Res<steamworks::Client>) {
    let menu_input_handles = menu_input_handles(&steam);
    steam.input()
        .activate_action_set_handle(SteamHandle::MAX, menu_input_handles.actionset);
}
