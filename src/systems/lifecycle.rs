use bevy::prelude::*;
use crate::components::{Lifecycle, AntBehavior, TimeControl};
use crate::systems::time_control::effective_delta_time;

/// System for ant aging and energy management
pub fn ant_lifecycle_system(
    time: Res<Time>,
    time_control: Res<TimeControl>,
    mut commands: Commands,
    mut ant_query: Query<(Entity, &mut Lifecycle), With<AntBehavior>>,
) {
    let delta_time = effective_delta_time(&time, &time_control);

    for (entity, mut lifecycle) in ant_query.iter_mut() {
        // Age the ant
        lifecycle.age += delta_time;

        // Decrease energy over time (simplified)
        lifecycle.energy -= 2.0 * delta_time;

        // Check if ant should die
        if lifecycle.age >= lifecycle.max_age || lifecycle.energy <= 0.0 {
            commands.entity(entity).despawn();
            info!("Ant died at age {:.1}s with {:.1} energy", lifecycle.age, lifecycle.energy);
        }
    }
}