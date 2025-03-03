use bevy::prelude::*;
use crate::{model::*, consts::*};
use super::*;

/// Handles keyboard and mouse input for the game.
///
/// Default controls:
/// WASD controls directional acceleration relative to the player's current direction.
/// KL controls rotation (direction).
/// J primary fire.
/// ; secondary fire
/// H' uses first and second utilities
/// F sets acceleration to 100%
/// G sets acceleration to 50%
/// V sets acceleration to 1%
/// Space starts deacceleration
/// U sets rotation speed to 100%
/// N sets rotation speed to 50%
/// M sets rotation speed to 25%
///
pub fn system_update_game_input_keyboard_mouse(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    //mouse_input: Res<Input<MouseButton>>,
    mut query: Query<(&LocalPlayer, &mut Motion, &mut UseActions),With<Orb>>
) {
    let (_local_player, mut motion, mut use_action) = query.iter_mut()
        .find(|(local_player, _, _)| local_player.num == 1)
        .expect("No local player #1 found");

    // handle rotation
    if keyboard_input.pressed(KeyCode::KeyK) {
        motion.rotation = motion.rotation_speed;
    } else if keyboard_input.pressed(KeyCode::KeyL) {
        motion.rotation = -motion.rotation_speed;
    }

    // handle thrust forward / backward
    if keyboard_input.pressed(KeyCode::KeyW) {
        motion.acceleration_vec.y = motion.thrust_amount;
    } else if keyboard_input.pressed(KeyCode::KeyS) {
        motion.acceleration_vec.y = -motion.thrust_amount;
    } else {
        motion.acceleration_vec.y = 0.0;
    }

    // handle thrust left / right
    if keyboard_input.pressed(KeyCode::KeyA) {
        motion.acceleration_vec.x = -motion.thrust_amount;
    } else if keyboard_input.pressed(KeyCode::KeyD) {
        motion.acceleration_vec.x = motion.thrust_amount;
    } else {
        motion.acceleration_vec.x = 0.0;
    }

    // handle deacceleration
    if keyboard_input.just_pressed(KeyCode::Space) {
        //todo: don't touch velocity and don't just stop
        motion.velocity = Vec2::ZERO;
        motion.acceleration_vec = Vec2::ZERO;
    }

    // handle rotation speed
    if keyboard_input.just_pressed(KeyCode::KeyU) {
        motion.rotation_speed = DEFAULT_ROTATION_SPEED;
    } else if keyboard_input.just_pressed(KeyCode::KeyN) {
        motion.rotation_speed = DEFAULT_ROTATION_SPEED * 0.5;
    } else if keyboard_input.just_pressed(KeyCode::KeyM) {
        motion.rotation_speed = DEFAULT_ROTATION_SPEED * 0.25;
    }

    // handle thrust amount
    if keyboard_input.just_pressed(KeyCode::KeyF) {
        motion.thrust_amount = DEFAULT_ACCELERATION;
    } else if keyboard_input.just_pressed(KeyCode::KeyG) {
        motion.thrust_amount = DEFAULT_ACCELERATION * 0.5;
    } else if keyboard_input.just_pressed(KeyCode::KeyV) {
        motion.thrust_amount = DEFAULT_ACCELERATION * 0.25;
    }

    use_action.reset();

    // handle primary gear
    if keyboard_input.pressed(KeyCode::KeyH) {
        use_action.gear_use[0].1 = true;
    }
    // handle secondary gear
    if keyboard_input.pressed(KeyCode::Quote) {
        use_action.gear_use[1].1 = true;
    }
}

pub fn _gamepad_input(
    _button_inputs: Res<ButtonInput<GamepadButton>>,
    _button_axes: Res<Axis<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    query_gamepads: Query<&Gamepad>,
    mut query: Query<(&mut Orb, &mut Transform), With<Orb>>
) {
    for _gamepad in query_gamepads.iter() {
        //todo
        /*let local_players = vec![LocalPlayer { num: LOCAL_PLAYER_1, gamepad_id: Some(gamepad.id), orb_id: Some(1)}];
        let player = local_players.iter().find(|p| p.gamepad_id == Some(gamepad.id)).unwrap();
        if player.orb_id != Some(orb.id) {
            continue;
        }*/

        let (mut _orb, mut orb_transform) = query.iter_mut().next().unwrap();

        let left_stick_x = axes.get(GamepadAxis::LeftStickX).unwrap();
        let left_stick_y = axes.get(GamepadAxis::LeftStickY).unwrap();
        let right_stick_x = axes.get(GamepadAxis::RightStickX).unwrap();
        let _right_stick_y = axes.get(GamepadAxis::RightStickX).unwrap();

        if left_stick_x > DEFAULT_LEFT_STICK_DEADZONE {
            orb_transform.translation.x += DEFAULT_ACCELERATION;
            orb_transform.translation.x += DEFAULT_ACCELERATION;
        } else if left_stick_x < -DEFAULT_LEFT_STICK_DEADZONE {
            orb_transform.translation.x -= DEFAULT_ACCELERATION;
            orb_transform.translation.x -= DEFAULT_ACCELERATION;
        }

        if left_stick_y > DEFAULT_LEFT_STICK_DEADZONE {
            orb_transform.translation.y += DEFAULT_ACCELERATION;
            orb_transform.translation.y += DEFAULT_ACCELERATION;
        } else if left_stick_y < -DEFAULT_LEFT_STICK_DEADZONE {
            orb_transform.translation.y -= DEFAULT_ACCELERATION;
            orb_transform.translation.y -= DEFAULT_ACCELERATION;
        }

        //todo
        if right_stick_x > DEFAULT_RIGHT_STICK_DEADZONE {
            orb_transform.rotate_z(DEFAULT_ROTATION_SPEED);
        } else if right_stick_x < -DEFAULT_RIGHT_STICK_DEADZONE {
            orb_transform.rotate_z(-DEFAULT_ROTATION_SPEED);
        }
    }
}
