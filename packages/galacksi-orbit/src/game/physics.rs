use bevy::prelude::*;
use crate::*;
use super::*;

pub const MAX_VELOCITY_LENGTH: f32 = 500.;
pub const BULLET_SPEED: f32 = 1000.;

pub fn system_fixed_update_game_movement(
    mut query: Query<(&mut Motion, &mut LastPosition)>,
    time: Res<Time>,
) {
    let time_delta_secs = time.delta_secs();
    for (mut motion, mut last_position) in query.iter_mut() {
        let motion = &mut *motion;
        last_position.position = motion.position;
        let acceleration = if motion.velocity.length() > MAX_VELOCITY_LENGTH {
            Vec2::ZERO
        } else {
            motion.acceleration_vec * time_delta_secs
        };

        motion.velocity = motion.velocity + acceleration;
        motion.position += motion.velocity * time.delta_secs();
    }
}

pub fn system_fixed_update_game_actions(
    mut query: Query<(&Transform, &Motion, &UseActions, &mut InstalledEquipment, &mut LastUseActions)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
) {
    let delta_secs = time.delta_secs();
    for (transform, motion, use_actions, mut equipment, mut last_use_actions) in query.iter_mut() {
        for gear_index in 0..MAX_GEAR {
            let gear_item = use_actions.gear_use[gear_index];
            let using = gear_item.1;
            let installed = equipment.items.get_mut(&gear_item.0).unwrap();
            if installed.cooldown_remaining > 0. {
                installed.cooldown_remaining -= delta_secs;
            }

            if using && installed.cooldown_remaining <= 0. {
                // fire bullet
                last_use_actions.gear_use[gear_index].1 = true;
                let equip = &EQUIPMENT[installed.id as usize];
                installed.cooldown_remaining = equip.cooldown;

                BulletSpawner::from_orb(motion, transform)
                    .spawn(&mut commands, &mut meshes, &mut materials)
                    .expect("Bullet should spawn");
            }
        }
    }
}

pub fn system_fixed_update_game_transform_movement(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(&mut Transform, &mut Motion, &LastPosition)>,
) {
    for (mut transform, mut motion, last_position) in &mut query {
        let overstep = fixed_time.overstep_fraction();
        transform.translation = last_position.position
            .lerp(motion.position, overstep)
            .extend(1.);

        transform.rotate_z(motion.rotation_amount);
        motion.rotation_amount = 0.;
    }
}
