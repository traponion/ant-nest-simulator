use crate::components::{AntBehavior, Lifecycle};
use bevy::prelude::*;

/// System for ant aging and energy management
pub fn ant_lifecycle_system(
    time: Res<Time>,
    mut commands: Commands,
    mut ant_query: Query<(Entity, &mut Lifecycle), With<AntBehavior>>,
) {
    let delta_time = time.delta_seconds();

    for (entity, mut lifecycle) in ant_query.iter_mut() {
        // Age the ant
        lifecycle.age += delta_time;

        // Decrease energy over time (much slower for debugging)
        lifecycle.energy -= 0.2 * delta_time;

        // Check if ant should die
        if lifecycle.age >= lifecycle.max_age || lifecycle.energy <= 0.0 {
            commands.entity(entity).despawn();
            info!(
                "Ant died at age {:.1}s with {:.1} energy",
                lifecycle.age, lifecycle.energy
            );
        }
    }
}
