use bevy::prelude::*;
use crate::{model::*, input::consts::*};

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
pub fn game_keyboard_mouse_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    //mouse_input: Res<Input<MouseButton>>,
    mut query: Query<(&mut Transform, &mut Position), With<Orb>>
) {
    let (mut transform, mut position) = query.iter_mut().next().unwrap();

    // handle rotation
    if keyboard_input.pressed(KeyCode::KeyK) {
        transform.rotate_z(-position.rotation_speed);
    } else if keyboard_input.pressed(KeyCode::KeyL) {
        transform.rotate_z(position.rotation_speed);
    }

    // handle thrust forward / backward
    if keyboard_input.pressed(KeyCode::KeyW) {
        position.acceleration_vec.y = position.thrust_amount;
    } else if keyboard_input.pressed(KeyCode::KeyS) {
        position.acceleration_vec.y = -position.thrust_amount;
    }

    // handle thrust left / right
    if keyboard_input.pressed(KeyCode::KeyA) {
        position.acceleration_vec.x = -position.thrust_amount;
    } else if keyboard_input.pressed(KeyCode::KeyD) {
        position.acceleration_vec.x = position.thrust_amount;
    }

    // handle deacceleration
    if keyboard_input.just_pressed(KeyCode::Space) {
        //todo: don't touch velocity and don't just stop
        position.velocity = Vec2::ZERO;
        position.acceleration_vec = Vec2::ZERO;
    }

    // handle rotation speed
    if keyboard_input.just_pressed(KeyCode::KeyU) {
        position.rotation_speed = DEFAULT_ROTATION_SPEED;
    } else if keyboard_input.just_pressed(KeyCode::KeyN) {
        position.rotation_speed = DEFAULT_ROTATION_SPEED * 0.5;
    } else if keyboard_input.just_pressed(KeyCode::KeyM) {
        position.rotation_speed = DEFAULT_ROTATION_SPEED * 0.25;
    }

    // handle thrust amount
    if keyboard_input.just_pressed(KeyCode::KeyF) {
        position.thrust_amount = DEFAULT_ACCELERATION;
    } else if keyboard_input.just_pressed(KeyCode::KeyG) {
        position.thrust_amount = DEFAULT_ACCELERATION * 0.5;
    } else if keyboard_input.just_pressed(KeyCode::KeyV) {
        position.thrust_amount = DEFAULT_ACCELERATION * 0.25;
    }

}

pub fn gamepad_input(
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
