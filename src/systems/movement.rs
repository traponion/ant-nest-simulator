use crate::components::{Ant, FoundingState, Position, Queen, Soil, SoilCell};
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

/// Queen ant founding behavior system
pub fn queen_founding_system(
    mut commands: Commands,
    mut queen_query: Query<(&mut Queen, &mut Position, &mut Transform), With<Ant>>,
    soil_query: Query<(Entity, &Position, &SoilCell), (With<Soil>, Without<Ant>)>,
) {
    let mut rng = thread_rng();

    for (mut queen, mut position, mut transform) in queen_query.iter_mut() {
        match queen.founding_state {
            FoundingState::Seeking => {
                // Queen seeks optimal founding location
                let current_site_quality = evaluate_founding_site(&position, &soil_query);

                if current_site_quality > 0.7 {
                    // Good site found, start digging
                    queen.founding_state = FoundingState::Digging;
                    info!(
                        "Queen found suitable founding site at ({}, {}) with quality {:.2}",
                        position.x, position.y, current_site_quality
                    );
                } else {
                    // Keep searching - move in random direction
                    if rng.gen_bool(0.3) {
                        let move_x = rng.gen_range(-12.0..12.0);
                        let move_y = rng.gen_range(-2.0..2.0); // Prefer staying near surface

                        position.x += move_x;
                        position.y += move_y;

                        // Keep queen within bounds and near surface
                        position.x = position.x.clamp(-90.0, 90.0);
                        position.y = position.y.clamp(-5.0, 5.0);
                    }
                }
            }

            FoundingState::Digging => {
                // Queen digs 3x3 founding chamber
                let mut dug_count = 0;

                // Remove soil in 3x3 area around queen
                for (soil_entity, soil_position, _soil_cell) in soil_query.iter() {
                    let dx = (soil_position.x - position.x).abs();
                    let dy = (soil_position.y - position.y).abs();

                    // Check if soil is within 3x3 area (allowing for 4-pixel spacing)
                    if dx <= 6.0 && dy <= 6.0 {
                        commands.entity(soil_entity).despawn();
                        dug_count += 1;
                    }
                }

                if dug_count > 0 {
                    info!("Queen dug {} soil cells for founding chamber", dug_count);
                }

                // After digging, establish the chamber
                queen.founding_state = FoundingState::Established;

                // Move queen slightly underground into the chamber
                position.y -= 8.0; // Move down into the dug chamber
            }

            FoundingState::Established => {
                // Queen stays in chamber - minimal movement
                if rng.gen_bool(0.02) {
                    // Very small movements within chamber
                    let move_x = rng.gen_range(-2.0..2.0);
                    let move_y = rng.gen_range(-2.0..2.0);

                    position.x += move_x;
                    position.y += move_y;
                }
            }
        }

        // Update transform to match position
        transform.translation.x = position.x;
        transform.translation.y = position.y;
    }
}

/// Evaluate the quality of a founding site based on soil conditions
fn evaluate_founding_site(
    queen_position: &Position,
    soil_query: &Query<(Entity, &Position, &SoilCell), (With<Soil>, Without<Ant>)>,
) -> f32 {
    let mut total_quality = 0.0;
    let mut sample_count = 0;

    // Sample soil conditions in a small area around queen
    for (_entity, soil_position, soil_cell) in soil_query.iter() {
        let dx = (soil_position.x - queen_position.x).abs();
        let dy = (soil_position.y - queen_position.y).abs();

        // Check soil within a reasonable founding area
        if dx <= 12.0 && dy <= 12.0 {
            // Evaluate soil quality based on Camponotus japonicus preferences
            let moisture_score = if soil_cell.moisture >= 0.4 && soil_cell.moisture <= 0.7 {
                1.0
            } else {
                0.3
            };
            let temp_score = if soil_cell.temperature >= 19.0 && soil_cell.temperature <= 21.0 {
                1.0
            } else {
                0.5
            };
            let nutrition_score = if soil_cell.nutrition >= 0.3 { 1.0 } else { 0.4 };

            let site_quality = (moisture_score + temp_score + nutrition_score) / 3.0;
            total_quality += site_quality;
            sample_count += 1;
        }
    }

    if sample_count > 0 {
        total_quality / sample_count as f32
    } else {
        0.0 // No soil nearby - poor site
    }
}
