use bevy::prelude::*;
use super::*;
use crate::{color::*, model::*, util::*, Mode};

const MAX_VELOCITY_LENGTH: f32 = 500.;
const BULLET_VELOCITY: f32 = 1000.;

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
        .add_systems(FixedUpdate, (
            system_fixed_update_game_movement,
            system_fixed_update_game_actions,
            system_fixed_update_game_transform_movement,
        ).run_if(in_state(Mode::Game)));
}

fn system_enter_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    // spawn orbs for local players
    let p1_color = Palette::rand_bloom();
    let p2_color = Palette::rand_bloom_not(vec![p1_color]);

    spawn_orb(Some(1), Some(p1_color), &mut commands, &mut meshes, &mut color_materials);
    spawn_orb(Some(2), Some(p2_color), &mut commands, &mut meshes, &mut color_materials);
}

fn spawn_orb(
    local_player_num: Option<u8>,
    color: Option<Color>,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let color = match color {
        Some(c) => c,
        None => Palette::rand_bloom()
    };

    match local_player_num {
        Some(1) => {
            commands
                .spawn((
                    Orb,
                    Mesh2d(meshes.add(Circle::new(10.))),
                    MeshMaterial2d(materials.add(color)),
                    Transform::from_translation(Vec3::new(0.,0.,1.)),
                    LocalPlayer1,
                    LocalPlayer {
                        num: 1,
                        gamepad_id: None,
                        orb_id: Some(1),
                    },
                    Motion::default(),
                    LastPosition::default(),
                    UseActions::default(),
                    LastUseActions::default(),
                    InstalledEquipment {
                        items: [(0, EquipmentItem::new(0))]
                            .into_iter().collect()
                    },
                    Gear {
                       items: [Some(0), None, None, None]
                    },
                    OnGameScreen
                ))
                .with_children(|commands| {
                    commands.spawn((
                        Mesh2d(meshes.add(Rectangle::new(30.,1.))),
                        MeshMaterial2d(materials.add(color)),
                        Transform::from_translation(Vec3::new(0.,0.,1.)),
                    ));
                });
        },
        Some(player_num) => {
            let position = match player_num {
                1 => Vec2::ZERO,
                2 => Vec2::new(30., -30.),
                3 => Vec2::new(-30., -30.),
                4 => Vec2::new(60., -60.),
                5 => Vec2::new(-60., -60.),
                6 => Vec2::new(90., -90.),
                7 => Vec2::new(-90., -90.),
                8 => Vec2::new(0., -90.),
                _ => unimplemented!("More than 8 local players is not currently supported")
            };

            commands
                .spawn((
                    Mesh2d(meshes.add(Circle::new(10.))),
                    MeshMaterial2d(materials.add(color)),
                    Transform::from_translation(position.extend(0.)),
                    LocalPlayer {
                        num: player_num,
                        gamepad_id: None,
                        orb_id: Some(1),
                    },
                    Orb,
                    Motion {
                        position,
                        ..default()
                    },
                    LastPosition {
                        position,
                    },
                    OnGameScreen
                ))
                .with_children(|commands| {
                    commands.spawn((
                        Mesh2d(meshes.add(Rectangle::new(30.,1.))),
                        MeshMaterial2d(materials.add(color)),
                        Transform::from_translation(Vec3::ZERO),
                    ));
                });
        },
        None => {
            commands
                .spawn((
                    Mesh2d(meshes.add(Circle::new(10.))),
                    MeshMaterial2d(materials.add(color)),
                    Transform::from_translation(Vec3::ZERO),
                    Orb,
                    Motion::default(),
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
        }
    }
}

fn system_exit_game(query: Query<Entity, With<OnGameScreen>>, commands: Commands) {
    despawn_entities(query, commands);
}

fn system_fixed_update_game_movement(
    mut query: Query<(&mut Motion, &mut LastPosition)>,
    time: Res<Time>,
) {

    for (mut motion, mut last_position) in query.iter_mut() {
        let motn = &mut *motion;
        last_position.position = motn.position;
        motn.velocity = (motn.velocity + (motn.acceleration_vec * time.delta_secs()))
            .clamp_length_max(MAX_VELOCITY_LENGTH);
        motn.position += motn.velocity * time.delta_secs();
    }
}

fn system_fixed_update_game_actions(
    mut query: Query<(&Transform, &Motion, &UseActions, &mut InstalledEquipment, &mut LastUseActions)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
) {
    let delta_secs = time.delta_secs();
    for (transform, motion, use_actions, mut equipment, mut last_use_actions) in query.iter_mut() {
        let bullet_velocity = motion.velocity.length() + BULLET_VELOCITY;
        for gear_index in 0..MAX_GEAR {
            let gear_item = use_actions.gear_use[gear_index];
            let using = gear_item.1;
            let installed = equipment.items.get_mut(&gear_item.0).unwrap();
            if installed.cooldown_remaining > 0. {
                installed.cooldown_remaining -= delta_secs;
            }

            if using && installed.cooldown_remaining <= 0. {
                last_use_actions.gear_use[gear_index].1 = true;
                let equip = &EQUIPMENT[installed.id as usize];
                installed.cooldown_remaining = equip.cooldown;
                let color = Palette::rand_bloom();
                commands.spawn((
                    OrbBullet,
                    Mesh2d(meshes.add(Circle::new(1.))),
                    MeshMaterial2d(materials.add(color)),
                    Transform::from_translation(transform.translation),
                    Motion {
                        position: motion.position,
                        velocity: ((transform.rotation * Vec3::Y) * bullet_velocity).truncate(),
                        ..default()
                    },
                    LastPosition {
                        position: motion.position,
                        ..default()
                    },
                    OnGameScreen
                ));
            }
        }
    }
}

fn system_fixed_update_game_transform_movement(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(&mut Transform, &mut Motion, &LastPosition)>,
) {
    for (mut transform, mut motion, last_position) in &mut query {
        let overstep = fixed_time.overstep_fraction();
        transform.translation = last_position.position
            .lerp(motion.position, overstep)
            .extend(0.0);

        transform.rotate_z(motion.rotation_amount);
        motion.rotation_amount = 0.;
    }
}

fn update_camera(
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<LocalPlayer1>)>,
    player: Query<(&Transform, &Motion), (With<LocalPlayer1>, Without<Camera2d>)>,
    //time: Res<Time>,
) {
    let Ok(mut camera) = camera.get_single_mut() else {
        return;
    };

    let Ok((transform, _position)) = player.get_single() else {
        return;
    };

    //let Vec3 { x, y, .. } = transform.translation;
    //let direction = Vec3::new(x, y, camera.translation.z);

    // Applies a smooth effect to camera movement using stable interpolation
    // between the camera position and the player position on the x and y axes.
    camera.translation = transform.translation;
    //camera.translation
    //    .smooth_nudge(&direction, position.velocity.length(), time.delta_secs());
}
