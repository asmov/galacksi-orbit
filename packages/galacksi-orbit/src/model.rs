use bevy::{prelude::*, utils::HashMap};
use crate::color::palette;

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

#[derive(Component, Default)]
pub struct Orb;

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

pub struct EquipmentItem {
    pub id: EquipmentID,
    pub cooldown_remaining: f32
}

impl EquipmentItem {
    pub fn new(id: EquipmentID) -> Self {
        Self {
            id,
            cooldown_remaining: 0.0,
        }
    }
}

#[derive(Component, Debug)]
pub struct OrbBlueprint {
    pub id: u32,
    pub max_velocity: f32,
    pub max_health: u16,
    pub max_shield: u16,
    pub color: Color,
}

impl Default for OrbBlueprint {
    fn default() -> Self {
        Self {
            id: 0,
            max_velocity: 0.0,
            max_health: 0,
            max_shield: 0,
            color: palette::BLOOM_WHITE,
        }
    }
}


#[derive(strum::Display)]
pub enum EquipmentCategory {
    Weapon
}

pub type EquipmentID = u8;

pub struct Equipment {
    pub id: EquipmentID,
    pub category: EquipmentCategory,
    pub name: &'static str,
    pub token: &'static str,
    /// Milliseconds
    pub cooldown: f32
}

pub const EQUIPMENT: &'static [Equipment;1] = &[
    Equipment {
        id: 0,
        category: EquipmentCategory::Weapon,
        name: "Gattling Gun",
        token: "gattling_gun",
        cooldown: 0.3
    }
];
