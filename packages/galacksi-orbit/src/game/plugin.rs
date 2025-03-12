use bevy::{log, prelude::*};
use crate::*;
use super::*;

pub fn plugin_game(app: &mut App) {
    app
        .add_systems(OnEnter(Mode::Game), (
            system_enter_game,
        ))
        .add_systems(OnExit(Mode::Game), (
            system_exit_game,
        ))
        .add_systems(Update, (
            system_update_game_input_keyboard_mouse,
            update_camera,
        ).run_if(in_state(Mode::Game)))
        .add_systems(FixedUpdate,
            (
                system_fixed_update_game_equipment,
                system_fixed_update_game_movement,
                system_fixed_update_game_transform_movement
            )
                .chain()
                .run_if(in_state(Mode::Game))
        );
}

fn system_enter_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    // spawn orbs for local players
    let p1_orb = OrbSpawner::local_player1()
        .spawn(&mut commands, &mut meshes, &mut color_materials)
        .expect("Should spawn P1 orb");

    OrbSpawner::local_player(2, None, p1_orb.transform.as_ref().unwrap())
        .color_not(vec![p1_orb.color.unwrap()])
        .spawn(&mut commands, &mut meshes, &mut color_materials)
        .expect("Should spawn P2 orb");

    log::info!("Simulation started");
}

fn system_exit_game(query: Query<Entity, With<OnGameScreen>>, commands: Commands) {
    despawn_entities(query, commands);
}

fn update_camera(
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<LocalPlayer1>)>,
    player: Query<(&Transform, &Motion), (With<LocalPlayer1>, Without<Camera2d>)>,
) {
    let Ok(mut camera) = camera.get_single_mut() else {
        return;
    };

    let Ok((transform, _position)) = player.get_single() else {
        return;
    };

    camera.translation = transform.translation;
}
