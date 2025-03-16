use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct Selected(pub Option<(usize, Interaction)>);
