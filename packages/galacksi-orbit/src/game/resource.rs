use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct LastMouseGamePosition{
    pub window_position: Option<Vec2>,
    pub position: Option<Vec2>,
}

impl LastMouseGamePosition {
    pub fn pair(&self) -> Option<(&Vec2, &Vec2)> {
        match (&self.position, &self.window_position) {
            (Some(position), Some(window_position)) => Some((position, window_position)),
            _ => None,
        }
    }
}
