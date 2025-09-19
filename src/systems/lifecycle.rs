use bevy::prelude::*;
use crate::components::{Lifecycle, AntBehavior};

/// System for ant aging and energy management
pub fn ant_lifecycle_system(
    time: Res<Time>,
    mut commands: Commands,
    mut ant_query: Query<(Entity, &mut Lifecycle), With<AntBehavior>>,
) {
    for (entity, mut lifecycle) in ant_query.iter_mut() {
        // Age the ant
        lifecycle.age += time.delta_seconds();

        // Decrease energy over time (simplified)
        lifecycle.energy -= 2.0 * time.delta_seconds();

        // Check if ant should die
        if lifecycle.age >= lifecycle.max_age || lifecycle.energy <= 0.0 {
            commands.entity(entity).despawn();
            info!("Ant died at age {:.1}s with {:.1} energy", lifecycle.age, lifecycle.energy);
        }
    }
}