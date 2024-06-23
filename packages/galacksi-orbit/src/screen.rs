pub mod title;
pub mod game;

use bevy::prelude::*;

#[derive(Resource, Default, Copy, Clone, States, Debug, Hash, Eq, PartialEq)]
pub enum Screen {
    #[default]
    Title,
    Game
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub(crate) fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
