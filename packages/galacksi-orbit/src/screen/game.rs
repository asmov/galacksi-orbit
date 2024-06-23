use bevy::prelude::*;

use crate::{input, color::*, model::*, screen::*};

pub(crate) fn game_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(Screen::Game), (
            setup_game,
            #[cfg(feature = "steam")] steam::setup_game
        ))
        .add_systems(OnExit(Screen::Game), (
            teardown_game,
            #[cfg(feature = "steam")] teardown_steam_game
        ))
        .add_systems(Update, (
                #[cfg(feature = "steam")] steam::input,
                input::game::game_keyboard_mouse_input,
                movement,
                transform_movement)
            .run_if(in_state(Screen::Game)));
}

#[derive(Component)]
struct OnGameScreen;

fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let color = Palette::rand_bloom();
    let orb = Orb {
        core_color: color,
        ..Orb::default()
    };

    meshes.add(Circle::new(10.));
    commands
        .spawn((
            Mesh2d(meshes.add(Circle::new(10.))),
            MeshMaterial2d(materials.add(color)),
            Transform::from_translation(Vec3::ZERO),
            orb,
            Position::default(),
            LastPosition::default(),
            OnGameScreen
        ))
        .with_children(|commands| {
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::new(30.,1.))),
                MeshMaterial2d(materials.add(color)),
                Transform::from_translation(Vec3::ZERO),
            ));
        });

    #[cfg(feature = "steam")] {
    }
}

fn teardown_game(query: Query<Entity, With<OnGameScreen>>, commands: Commands) {
    despawn_screen(query, commands);
}

fn movement(
    time: Res<Time>,
    mut query: Query<(&mut Position, &mut LastPosition)>
) {
    for (mut position, mut last_position) in query.iter_mut() {
        let pos = &mut *position;
        last_position.position = pos.position;
        pos.velocity += pos.acceleration_vec * time.delta_secs();
        pos.position += pos.velocity * time.delta_secs();
    }
}

fn transform_movement(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(&mut Transform, &Position, &LastPosition)>
) {
    for (mut transform, position, last_position) in &mut query {
        let overstep = fixed_time.overstep_fraction();
        transform.translation = last_position.position
            .lerp(position.position, overstep)
            .extend(0.0);

        if position.rotation != Quat::IDENTITY {
            transform.rotate(position.rotation);
        }
    }
}
