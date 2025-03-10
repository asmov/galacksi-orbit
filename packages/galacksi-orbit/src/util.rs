use bevy::prelude::*;

// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_entities<T: Component>(entities: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &entities {
        commands.entity(entity).despawn_recursive();
    }
}
