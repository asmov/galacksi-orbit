use bevy::prelude::*;
use crate::*;

pub type InventoryIndex = u8;
pub type BlueprintID = u32;
pub type EquipmentID = u8;
pub type AmmunitionID = u8;

pub trait Transport: Send {}

#[derive(Debug, Default)]
pub struct Action {
    pub flags: u8,
    /// the amount of thrust applied, if not blueprint maximum (or 0)
    pub acceleration: Option<Vec2>,
    pub rotation_amount: Option<f32>,
    /// the index of the equipment being used within [Blueprint::equipment]
    pub using_equipment_ids: Option<Vec<EquipmentID>>,
}

impl Action {
    /// Based on [Blueprint], so [Action::accelerating_amount] is None
    pub const FLAG_RESERVED_01: u8          = 0x001;
    /// Equipment is specified by [Action::using_equipment_index]
    pub const FLAG_RESERVED_02: u8          = 0x002;
    /// Amount is specified by [Action::acceleration_amount]
    pub const FLAG_DEACCELERATING: u8       = 0x004;
    pub const FLAG_RESERVED_04: u8          = 0x008;
    pub const FLAG_RESERVED_05: u8          = 0x010;
    pub const FLAG_RESERVED_06: u8          = 0x020;
    pub const FLAG_RESERVED_07: u8          = 0x040;
    pub const FLAG_RESERVED_08: u8          = 0x080;

    pub fn rotate(&mut self, amount: f32) {
        if amount != 0. {
            self.rotation_amount = Some(amount);
        } else {
            self.rotation_amount = None;
        }
    }

    pub fn stop_rotating(&mut self) {
        self.rotation_amount = None;
    }

    pub fn rotating(&self) -> bool {
        self.rotation_amount.is_some()
    }

    /// Clamps length to [Blueprint::max_acceleration_length].
    pub fn accelerate(&mut self, blueprint: &Blueprint, vec: Vec2) {
        if vec == Vec2::ZERO {
            self.acceleration = None;
        } else {
            self.acceleration = Some(vec.clamp_length_max(blueprint.max_acceleration_length));
        }
    }

    pub fn stop_accelerating(&mut self) {
        self.acceleration = None;
    }

    pub fn accelerating(&self) -> bool {
        self.acceleration.is_some()
    }

    pub fn accelerate_x(&mut self, blueprint: &Blueprint, x: f32) {
        let vec = if let Some(mut vec) = self.acceleration.take() {
            vec.x = x;
            vec
        } else {
            Vec2::new(x, 0.)
        };

        self.accelerate(blueprint, vec);
    }

    pub fn accelerate_y(&mut self, blueprint: &Blueprint, y: f32) {
        let vec = if let Some(mut vec) = self.acceleration.take() {
            vec.y = y;
            vec
        } else {
            Vec2::new(0., y)
        };

        self.accelerate(blueprint, vec);
    }

    pub fn accelerate_max_x(&mut self, blueprint: &Blueprint) {
        self.accelerate_x(blueprint, blueprint.max_acceleration_length);
    }

    pub fn accelerate_max_y(&mut self, blueprint: &Blueprint) {
        self.accelerate_y(blueprint, blueprint.max_acceleration_length);
    }

    pub fn accelerate_max_x_inverse(&mut self, blueprint: &Blueprint) {
        self.accelerate_x(blueprint, -blueprint.max_acceleration_length);
    }

    pub fn accelerate_max_y_inverse(&mut self, blueprint: &Blueprint) {
        self.accelerate_y(blueprint, -blueprint.max_acceleration_length);
    }

    pub fn acceleration(&self) -> &Vec2 {
        self.acceleration.as_ref().unwrap_or(&Vec2::ZERO)
    }

    pub fn accelerate_relative_x(&mut self, blueprint: &Blueprint, rotation: &Quat, x: f32) {
        let vec = (*rotation * Vec3::X * x).truncate();
        self.accelerate(blueprint, vec);
    }

    pub fn accelerate_relative_max_x(&mut self, blueprint: &Blueprint, rotation: &Quat) {
        self.accelerate_relative_x(blueprint, rotation, blueprint.max_acceleration_length);
    }

    pub fn accelerate_relative_max_x_inverse(&mut self, blueprint: &Blueprint, rotation: &Quat) {
        self.accelerate_relative_x(blueprint, rotation, -blueprint.max_acceleration_length);
    }

    pub fn accelerate_relative_y(&mut self, blueprint: &Blueprint, rotation: &Quat, y: f32) {
        let vec = (*rotation * Vec3::Y * y).truncate();
        self.accelerate(blueprint, vec);
    }

    pub fn accelerate_relative_max_y(&mut self, blueprint: &Blueprint, rotation: &Quat) {
        self.accelerate_relative_y(blueprint, rotation, blueprint.max_acceleration_length);
    }

    pub fn accelerate_relative_max_y_inverse(&mut self, blueprint: &Blueprint, rotation: &Quat) {
        self.accelerate_relative_y(blueprint, rotation, -blueprint.max_acceleration_length);
    }

    pub fn deaccelerate(&mut self) {
        self.acceleration = None;
        self.flags |= Self::FLAG_DEACCELERATING;
    }

    pub fn stop_deaccelerating(&mut self) {
        self.flags &= !Self::FLAG_DEACCELERATING;
    }

    pub fn deaccelerating(&self) -> bool {
        self.flags & Self::FLAG_DEACCELERATING != 0
    }
}

impl Transport for Action {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GalacksiElement {
    Alpha,
    Beta,
    Gamma,
}

impl GalacksiElement {
    pub fn bloom_color(&self) -> Color {
        match self {
            GalacksiElement::Alpha => palette::BLOOM_RED,
            GalacksiElement::Beta => palette::BLOOM_BLUE,
            GalacksiElement::Gamma => palette::BLOOM_GREEN,
        }
    }

    pub fn color(&self) -> Color {
        match self {
            GalacksiElement::Alpha => palette::RED,
            GalacksiElement::Beta => palette::BLUE,
            GalacksiElement::Gamma => palette::GREEN,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BodyKind {
    Orb,
    Projectile,
    Structure
}

#[derive(Debug, Clone)]
pub struct Blueprint {
    pub id: BlueprintID,
    pub kind: BodyKind,
    pub name: String,
    pub token: String,
    pub max_speed: f32,
    pub max_acceleration_length: f32,
    pub max_rotation_speed: f32,
    /// Health. Non-recoverable. <= 0 is destruction
    pub max_integrity: u16,
    /// [GalacksiElement::Gamma]
    pub max_energy: u16,
    pub equipment: Vec<EquipmentID>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InventoryCategory {
    Equipment,
    Ammunition,
    Cargo
}

#[derive(Debug, strum::Display)]
pub enum EquipmentCategory {
    Weapon
}

#[derive(Debug)]
pub struct Equipment {
    pub id: usize,
    pub category: EquipmentCategory,
    //pub consumes: Option<AmmunitionID>,
    pub name: &'static str,
    pub token: &'static str,
    /// Seconds
    pub cooldown: f32
}

pub const EQUIPMENT_ID_GATTLING_GUN: EquipmentID = 0;

pub const BASE_EQUIPMENT: [Equipment;1] = [
    Equipment {
        id: 0,
        category: EquipmentCategory::Weapon,
        name: "Gattling Gun",
        token: "gattling_gun",
        cooldown: 0.3
    },
];
