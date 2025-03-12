use bevy::{prelude::*, utils::HashMap};
use crate::consts::*;

pub const MAX_MOUNTED_EQUIPMENT: usize = 4;

#[derive(Component, Default)]
pub struct Orb;

#[derive(Component)]
pub struct OnGameScreen;

#[derive(Component)]
pub struct OrbCursor;

/// Installed equipment
#[derive(Component, Default, Deref, DerefMut)]
pub struct EquipmentInventory(pub HashMap<usize, InstalledEquipment>);

impl EquipmentInventory {
    pub fn mounted_at(&self, index: usize) -> Option<&InstalledEquipment> {
        self.iter().find(|(_, equipment)| equipment.mounted_at == Some(index))
            .map(|(_, equipment)| equipment)
    }

    pub fn mounted_at_mut(&mut self, index: usize) -> Option<&mut InstalledEquipment> {
        self.iter_mut().find(|(_, equipment)| equipment.mounted_at == Some(index))
            .map(|(_, equipment)| equipment)
    }

    pub fn reset_use(&mut self) {
        for (_, equipment) in self.iter_mut() {
            equipment.using = false;
        }
    }
}

#[derive(Default)]
pub struct InstalledEquipment {
    pub id: usize,
    pub using: bool,
    pub mounted_at: Option<usize>,
    pub last_used: f32,
}

impl InstalledEquipment {
    pub fn new_mounted(id: usize, mounted_at: usize) -> Self {
        Self {
            id,
            mounted_at: Some(mounted_at),
            ..default()
        }
    }
}

#[derive(Component, Default)]
pub struct OrbBullet;

pub const LOCAL_PLAYER_1: usize = 1;

#[derive(Component, Default)]
pub struct LocalPlayer1;

#[derive(Component, Clone)]
pub struct LocalPlayer {
    pub num: usize,
    pub gamepad_id: Option<usize>,
}

#[derive(Component, Default)]
pub struct SpawnMotion;

#[derive(Component, Clone)]
pub struct Motion {
    pub position: Vec2,
    pub rotation_amount: f32,
    pub rotation_speed: f32,
    pub thrust_amount: f32,
    pub acceleration_vec: Vec2,
    pub velocity: Vec2,
    pub max_speed: Option<f32>
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
            max_speed: None
        }
    }
}

#[derive(Component, Default, Deref, DerefMut)]
pub struct LastPosition(pub Vec2);
