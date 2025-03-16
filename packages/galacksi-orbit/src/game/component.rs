use bevy::prelude::*;
use crate::*;
use crate::consts::*;

#[derive(Component, Default, Deref, DerefMut)]
pub struct BodyAction(Action);

#[derive(Component)]
pub struct Orb {
    pub blueprint_id: BlueprintID,
}

impl Orb {
    pub fn new(blueprint_id: BlueprintID) -> Self {
        Self {
            blueprint_id,
        }
    }
}

#[derive(Component)]
pub struct OnGameScreen;

#[derive(Component)]
pub struct OrbCursor;

/// Installed equipment
#[derive(Component, Default, Deref, DerefMut)]
pub struct EquipmentStates(pub Vec<EquipmentState>);

impl EquipmentStates {
    pub fn mounted_at(&self, index: usize) -> Option<&EquipmentState> {
        self.iter().find(|state| state.mount_point == Some(index))
    }

    pub fn mounted_at_mut(&mut self, index: usize) -> Option<&mut EquipmentState> {
        self.iter_mut().find(|state| state.mount_point == Some(index))
    }
}

#[derive(Default)]
pub struct EquipmentState {
    pub equipment_id: EquipmentID,
    pub using: bool,
    pub mount_point: Option<usize>,
    pub last_used: f32,
}

impl EquipmentState {
    pub fn new_mounted(equipment_id: EquipmentID, mount_point: usize) -> Self {
        Self {
            equipment_id,
            mount_point: Some(mount_point),
            ..default()
        }
    }

    pub fn equipment(&self) -> &Equipment {
        &BASE_EQUIPMENT[self.equipment_id as usize]
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
