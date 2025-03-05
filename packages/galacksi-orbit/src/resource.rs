use bevy::prelude::*;
use crate::*;

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
    pub fn for_num(&self, num: u8) -> &PlayerConfig {
        self.configs.get(num as usize - 1)
            .expect("PlayerConfig not initialized for num: {num}")
    }
}
