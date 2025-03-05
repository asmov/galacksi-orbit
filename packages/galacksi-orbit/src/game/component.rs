use bevy::{prelude::*, utils::HashMap};
use crate::{model::*, consts::*};

pub const MAX_GEAR: usize = 4;

#[derive(Component, Default)]
pub struct Orb;

#[derive(Component)]
pub struct OnGameScreen;

#[derive(Component)]
pub struct OrbCursor;

/// Gear is active usable equipment
#[derive(Component, Default)]
pub struct Gear {
    pub items: [Option<EquipmentID>; 4] // total of 4 active slots
}

/// Installed equipment
#[derive(Component, Default)]
pub struct InstalledEquipment {
    pub items: HashMap<EquipmentID, EquipmentItem>
}

#[derive(Component, Default)]
pub struct UseActions {
    pub gear_use: GearUse
}

#[derive(Component, Default)]
pub struct LastUseActions {
    pub gear_use: GearUse
}

pub type GearUse = [(EquipmentID, bool); MAX_GEAR];

impl UseActions {
    pub fn reset(&mut self) {
        for i in 0..MAX_GEAR {
            self.gear_use[i].1 = false;
        }
    }
}

#[derive(Component, Default)]
pub struct OrbBullet;

pub const LOCAL_PLAYER_1: u8 = 1;

#[derive(Component, Default)]
pub struct LocalPlayer1;

#[derive(Component)]
pub struct LocalPlayer {
    pub num: u8,
    pub gamepad_id: Option<usize>,
    pub orb_id: Option<u16>
}

#[derive(Component)]
pub struct Motion {
    pub position: Vec2,
    pub rotation_amount: f32,
    pub rotation_speed: f32,
    pub thrust_amount: f32,
    pub acceleration_vec: Vec2,
    pub velocity: Vec2,
}

impl Default for Motion {
    fn default() -> Self {
        Self {
            position: Vec2::default(),
            rotation_amount: 0.,
            rotation_speed: DEFAULT_ROTATION_SPEED,
            thrust_amount: DEFAULT_ACCELERATION,
            acceleration_vec: Vec2::default(),
            velocity: Vec2::default(),
        }
    }
}

#[derive(Component, Default)]
pub struct LastPosition {
    pub position: Vec2
}
