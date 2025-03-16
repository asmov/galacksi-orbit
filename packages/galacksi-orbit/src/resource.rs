use bevy::{prelude::*, utils::HashMap};
use crate::*;

#[derive(Resource, Default)]
pub struct MousePosition {
    pub window_position: Option<Vec2>,
    pub position: Option<Vec2>,
}

impl MousePosition {
    pub fn pair(&self) -> Option<(&Vec2, &Vec2)> {
        match (&self.position, &self.window_position) {
            (Some(position), Some(window_position)) => Some((position, window_position)),
            _ => None,
        }
    }
}

#[derive(Resource)]
pub struct PlayerConfigs {
    pub configs: Vec<PlayerConfig>,
}

impl Default for PlayerConfigs {
    fn default() -> Self {
        Self {
            configs: vec![PlayerConfig::default()]
        }
    }
}

impl PlayerConfigs {
    pub fn for_num(&self, num: usize) -> &PlayerConfig {
        self.configs.get(num)
            .expect("PlayerConfig not initialized for num: {num}")
    }
}

#[derive(Resource, Deref)]
pub struct Blueprints(HashMap<BlueprintID, Blueprint>);

impl Blueprints {
    pub const ID_MID: BlueprintID = 1;
    pub const ID_GATTLING_GUN_BULLET: BlueprintID = 2;
}

impl Default for Blueprints {
    fn default() -> Self {
        Self([
            (Self::ID_MID, Blueprint {
                kind: BodyKind::Orb,
                id: Self::ID_MID,
                name: "Mid".to_string(),
                token: "mid".to_string(),
                max_acceleration_length: 200.,
                max_speed: 500.,
                max_rotation_speed: 0.15,
                max_integrity: 1000,
                max_energy: 1000,
                equipment: vec![EQUIPMENT_ID_GATTLING_GUN],
            }),
            (Self::ID_GATTLING_GUN_BULLET, Blueprint {
                            kind: BodyKind::Projectile,
                            id: Self::ID_GATTLING_GUN_BULLET,
                            name: "Gattling Gun Bullet".to_string(),
                            token: "gattling_gun_bullet".to_string(),
                            max_acceleration_length: 0.,
                            max_speed: 700.,
                            max_rotation_speed: 0.,
                            max_integrity: 0,
                            max_energy: 0,
                            equipment: vec![],
                        })
            ].into())
    }
}
