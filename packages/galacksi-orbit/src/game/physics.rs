use bevy::prelude::*;
use consts::DEFAULT_ROTATION_SPEED;
use crate::*;
use super::*;

pub fn system_fixed_update_game_movement(
    mut query: Query<(&mut Motion, &BodyAction, &mut LastPosition)>,
    time: Res<Time>,
) {
    for (mut motion, action, mut last_position) in query.iter_mut() {
        let motion = &mut *motion;
        last_position.0 = motion.position;

        motion.velocity = if action.deaccelerating() {
            Vec2::ZERO //todo: deaccelerate
        } else {
            let mut velocity = motion.velocity + action.acceleration() * time.delta_secs();
            if let Some(max_speed) = motion.max_speed {
                velocity = velocity.clamp_length_max(max_speed);
            }

            velocity
        };

        motion.position += motion.velocity * time.delta_secs();
    }
}

pub fn system_fixed_update_game_equipment(
    mut query: Query<(&Transform, &Motion, &mut EquipmentStates)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time<Fixed>>,
    blueprints: Res<Blueprints>,
) {
    for (transform, motion, mut equipment_inventory) in query.iter_mut() {
        for equipment_state in equipment_inventory.iter_mut() {
            if !equipment_state.using {
                continue;
            }

            let equipment = equipment_state.equipment();

            let time_elapsed = time.elapsed_secs();
            if (time_elapsed - equipment_state.last_used) < equipment.cooldown {
                continue;
            }

            equipment_state.last_used = time_elapsed;

            BulletSpawner::from_orb(motion, &transform, &blueprints)
                .spawn(&mut commands, &mut meshes, &mut materials)
                .expect("Bullet should spawn");
        }
    }
}

pub fn system_fixed_update_game_transform_movement(
    mut query: Query<(&mut Transform, &Motion, &mut BodyAction, &LastPosition)>,
    fixed_time: Res<Time<Fixed>>,
) {
    for (mut transform, motion, mut action, last_position) in &mut query {
        let overstep = fixed_time.overstep_fraction();
        // interpolation
        transform.translation = last_position
            .lerp(motion.position, overstep)
            .extend(1.);

        // extrapolation
        //let predicted = motion.position + motion.velocity * motion.acceleration * fixed_time.delta_secs();
        //transform.translation = motion.position.lerp(predicted, overstep).extend(1.);

        if let Some(action_rotation_amount) = action.rotation_amount {
            //todo: blueprint
            let rotation_amount = action_rotation_amount.clamp(-DEFAULT_ROTATION_SPEED, DEFAULT_ROTATION_SPEED);
            transform.rotate_z(rotation_amount);
            action.rotate(action_rotation_amount - rotation_amount);
        }
    }
}
