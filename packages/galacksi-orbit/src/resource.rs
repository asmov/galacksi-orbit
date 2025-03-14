use bevy::prelude::*;
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
