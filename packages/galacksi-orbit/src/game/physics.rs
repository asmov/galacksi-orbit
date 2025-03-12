use bevy::prelude::*;
use crate::*;
use super::*;

pub fn system_fixed_update_game_movement(
    mut query: Query<(&mut Motion, &mut LastPosition)>,
    time: Res<Time>,
) {
    for (mut motion, mut last_position) in query.iter_mut() {
        let motion = &mut *motion;
        last_position.0 = motion.position;

        motion.velocity = motion.velocity + motion.acceleration_vec * time.delta_secs();
        if let Some(max_speed) = motion.max_speed {
            motion.velocity = motion.velocity.clamp_length_max(max_speed);
        }

        motion.position += motion.velocity * time.delta_secs();
    }
}

pub fn system_fixed_update_game_equipment(
    mut query: Query<(&Transform, &Motion, &mut EquipmentInventory)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time<Fixed>>,
) {
    for (transform, motion, mut equipment_inventory) in query.iter_mut() {
        for (equipment_id, equipment_installation) in equipment_inventory.iter_mut() {
            if !equipment_installation.using {
                continue;
            }

            let equipment = &EQUIPMENT[*equipment_id];

            let time_elapsed = time.elapsed_secs();
            if (time_elapsed - equipment_installation.last_used) < equipment.cooldown {
                continue;
            }

            // fire bullet
            equipment_installation.last_used = time_elapsed;

            BulletSpawner::from_orb(motion, &transform)
                .spawn(&mut commands, &mut meshes, &mut materials)
                .expect("Bullet should spawn");
        }
    }
}

pub fn system_fixed_update_game_transform_movement(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(&mut Transform, &mut Motion, &LastPosition)>,
) {
    for (mut transform, mut motion, last_position) in &mut query {
        let overstep = fixed_time.overstep_fraction();
        transform.translation = last_position
            .lerp(motion.position, overstep)
            .extend(1.);

        transform.rotate_z(motion.rotation_amount);
        motion.rotation_amount = 0.;
    }
}
