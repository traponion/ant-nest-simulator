use crate::components::{Ant, Position, Soil};
use bevy::prelude::*;
use rand::prelude::*;

/// MVP: Simple ant movement with gravity and basic digging
pub fn ant_movement_system(
    mut commands: Commands,
    time: Res<Time>,
    mut ant_query: Query<(&mut Position, &mut Transform), With<Ant>>,
    soil_query: Query<(Entity, &Position), (With<Soil>, Without<Ant>)>,
) {
    let mut rng = thread_rng();
    let delta_time = time.delta_seconds();

    for (mut position, mut transform) in ant_query.iter_mut() {
        // MVP: Apply gravity - ants fall down unless supported by soil
        let gravity_force = -20.0 * delta_time; // Downward force
        let new_y = position.y + gravity_force;

        // Check if there's soil at the new position to stop falling
        let mut can_fall = true;
        for (_soil_entity, soil_position) in soil_query.iter() {
            let dx = soil_position.x - position.x;
            let dy = soil_position.y - new_y;
            let distance = (dx * dx + dy * dy).sqrt();

            // If ant is close to soil, it can't fall further
            if distance < 4.0 {
                can_fall = false;
                break;
            }
        }

        // Apply gravity if ant can fall
        if can_fall && new_y > -100.0 {
            // Don't fall below a certain depth
            position.y = new_y;
        }

        // MVP: Simple random movement
        if rng.gen_bool(0.1) {
            // 10% chance to move each frame
            let move_x = rng.gen_range(-8.0..8.0) * delta_time;
            let move_y = rng.gen_range(-4.0..4.0) * delta_time;

            position.x += move_x;
            position.y += move_y;

            // Keep ants within reasonable bounds
            position.x = position.x.clamp(-100.0, 100.0);
            position.y = position.y.clamp(-80.0, 20.0);
        }

        // MVP: Basic digging - remove soil that ants walk through
        if rng.gen_bool(0.05) {
            // 5% chance to dig each frame
            for (soil_entity, soil_position) in soil_query.iter() {
                let dx = soil_position.x - position.x;
                let dy = soil_position.y - position.y;
                let distance = (dx * dx + dy * dy).sqrt();

                // If ant is very close to soil, dig it out
                if distance < 3.0 {
                    commands.entity(soil_entity).despawn();
                    break; // Only dig one soil cell at a time
                }
            }
        }

        // Update transform to match position
        transform.translation.x = position.x;
        transform.translation.y = position.y;
    }
}
