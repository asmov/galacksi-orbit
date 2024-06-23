#![allow(dead_code)]

use bevy::prelude::*;
use crate::input;

#[derive(Component)]
pub struct Position {
    pub position: Vec2,
    pub rotation: Quat,
    pub rotation_speed: f32,
    pub thrust_amount: f32,
    pub acceleration_vec: Vec2,
    pub velocity: Vec2,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            position: Vec2::default(),
            rotation: Quat::default(),
            rotation_speed: input::consts::DEFAULT_ROTATION_SPEED,
            thrust_amount: input::consts::DEFAULT_ACCELERATION,
            acceleration_vec: Vec2::default(),
            velocity: Vec2::default(),
        }
    }
}


#[derive(Component, Default)]
pub struct LastPosition {
    pub position: Vec2
}


#[derive(Debug)]
pub enum Energy {
    Red,
    Orange,
    Yellow,
}

impl Energy {
    const RED: EnergyDef = EnergyDef::new(Energy::Red, Color::srgb(1.0, 0.0, 0.0));
    const ORANGE: EnergyDef = EnergyDef::new(Energy::Orange, Color::srgb(1.0, 0.5, 0.0));
    const YELLOW: EnergyDef = EnergyDef::new(Energy::Yellow, Color::srgb(1.0, 1.0, 0.0));

    pub const fn def(&self) -> &'static EnergyDef {
        match self {
            Energy::Red => &Self::RED,
            Energy::Orange => &Self::ORANGE,
            Energy::Yellow => &Self::YELLOW,
        }
    }
}

#[derive(Debug)]
pub struct EnergyDef {
    energy_type: Energy,
    color: Color,
}

impl EnergyDef {
    pub const fn new(energy_type: Energy, color: Color) -> Self {
        Self {
            energy_type,
            color,
        }
    }
}

#[derive(Component, Debug)]
pub struct Orb {
    pub id: u16,
    pub position: Vec2,
    pub acceleration: Vec3,
    pub aim_angle: f32,
    pub velocity: Vec2,
    pub max_velocity: f32,
    pub visibile: bool,
    pub health: u16,
    pub max_health: u16,
    pub shield_health: u8,
    pub shield_energy: Energy,
    pub core_color: Color,
    pub radiation_color: Color,
    pub equipment_1: Equipment,
    pub equipment_2: Equipment
}

impl Default for Orb {
    fn default() -> Self {
        Self {
            id: 0,
            position: Vec2::new(0.0, 0.0),
            acceleration: Vec3::new(0.0, 0.0, 0.0),
            aim_angle: 0.0,
            velocity: Vec2::new(0.0, 0.0),
            max_velocity: 0.0,
            visibile: false,
            health: 0,
            max_health: 0,
            shield_health: 0,
            shield_energy: Energy::RED.energy_type,
            core_color: Color::srgb(7.5, 0.0, 7.5),
            radiation_color: Color::srgb(0.0, 0.0, 0.0),
            equipment_1: Equipment::Shield,
            equipment_2: Equipment::Laser
        }
    }

}

#[derive(Component)]
struct OrbCursor;


#[derive(Debug)]
pub enum Equipment {
    Shield,
    Laser
}

impl Equipment {
    const SHIELD: EquipmentDef = EquipmentDef::new(Equipment::Shield, "Shield", "shield");
    const LASER: EquipmentDef = EquipmentDef::new(Equipment::Laser, "Laser", "laser");

    pub const fn def(&self) -> &'static EquipmentDef {
        match self {
            Equipment::Shield => &Self::SHIELD,
            Equipment::Laser => &Self::LASER,
        }
    }
}

#[derive(Debug)]
pub struct EquipmentDef {
    module_type: Equipment,
    name: &'static str,
    token: &'static str
}

impl EquipmentDef {
    pub const fn new(module_type: Equipment, name: &'static str, token: &'static str) -> Self {
        Self {
            module_type,
            name,
            token
        }
    }
}

pub const LOCAL_PLAYER_1: u8 = 1;

pub struct LocalPlayer {
    pub num: u8,
    pub gamepad_id: Option<usize>,
    pub orb_id: Option<u16>
}
