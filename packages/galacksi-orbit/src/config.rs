#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PlayerConfig {
    pub keyboard: KeyboardConfig,
}

pub enum BindAction {
    ThrustForward,
    ThrustBackward,
    ThrustLeft,
    ThrustRight,
    RotateClockwise,
    RotateCounterClockwise,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThrustOrientation {
    /// Relative to the ship's current direction.
    #[default]
    Relative,
    /// Absolute against the screen. Up is up, down is down.
    Absolute
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct KeyboardConfig {
   pub thrust_orientation: ThrustOrientation
}
