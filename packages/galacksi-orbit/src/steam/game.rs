use bevy::prelude::*;
use super::*;

pub fn system_enter_game_steam_input(steamworks: Res<steamworks::Client>) {
    let game_input_handles = game_actionset_input_handles(&steamworks);
    steamworks.input()
        .activate_action_set_handle(SteamHandle::MAX, game_input_handles.actionset);
}

fn system_update_game_steam_input(
    steam_client: Res<steamworks::Client>,
    mut query: Query<&mut Position, With<Orb>>,
    mut controller_inputs: ResMut<ControllerInputCollection>,
) {
    controller_inputs.update(&steam_client);

    for controller_input in &mut controller_inputs.controller_inputs {
        let ControllerInput::Game(_, input) = controller_input else {
            if matches!(controller_input, ControllerInput::Connected{ .. }) {
                controller_input.set_type(ControllerInputType::Game);
            }

            continue
        };

        let mut position = query.iter_mut().next().unwrap();
/*
        if input.movement.xy != Vec2::ZERO {
            position.acceleration.
        } else {
            position.acceleration = Vec2::ZERO;
        }
        */


            /*
            if keyboard_input.pressed(KeyCode::KeyA) {
                position.acceleration.x = -DEFAULT_ACCELERATION;
            } else if keyboard_input.pressed(KeyCode::KeyD) {
                position.acceleration.x = DEFAULT_ACCELERATION;
            } else {
                position.acceleration.x = 0.0;
            }

            if keyboard_input.pressed(KeyCode::KeyW) {
                position.acceleration.y = DEFAULT_ACCELERATION;
            } else if keyboard_input.pressed(KeyCode::KeyS) {
                position.acceleration.y = -DEFAULT_ACCELERATION;
            } else {
                position.acceleration.y = 0.0;
            }

            if keyboard_input.pressed(KeyCode::KeyJ) {
                position.rotation = Quat::from_rotation_z(DEFAULT_ROTATION_SPEED);
            } else if keyboard_input.pressed(KeyCode::Semicolon) {
                position.rotation = Quat::from_rotation_z(-DEFAULT_ROTATION_SPEED);
            } else {
                position.rotation = Quat::IDENTITY;
            }
            */
    }
}
