use crate::components::{
    Ant, AntBehavior, AntState, DisasterState, Food, FoodSource, InvasiveSpecies, Inventory,
    Lifecycle, Position, Soil, SoilCell, SpatialGrid,
};
use bevy::prelude::*;
use rand::prelude::*;

/// System for ant movement and behavior
pub fn ant_movement_system(
    mut commands: Commands,
    time: Res<Time>,
    _disaster_state: Res<DisasterState>, // Keeping for future implementation
    spatial_grid: Res<SpatialGrid>,
    mut ant_query: Query<
        (
            &mut Position,
            &mut AntBehavior,
            &mut Transform,
            &mut Lifecycle,
            &mut Inventory,
        ),
        (With<Ant>, Without<Food>),
    >,
    food_query: Query<(&Position, &FoodSource), With<Food>>,
    invasive_query: Query<&Position, (With<InvasiveSpecies>, Without<Ant>)>,
    mut soil_query: Query<(Entity, &Position, &mut SoilCell), (With<Soil>, Without<Ant>)>,
) {
    let mut rng = thread_rng();

    for (mut position, mut behavior, mut transform, mut lifecycle, mut inventory) in
        ant_query.iter_mut()
    {
        // Check for nearby invasive species and implement defensive behavior
        let mut nearest_invasive_distance = f32::INFINITY;
        let mut avoidance_vector = (0.0, 0.0);

        for invasive_pos in invasive_query.iter() {
            let dx = invasive_pos.x - position.x;
            let dy = invasive_pos.y - position.y;
            let distance = (dx * dx + dy * dy).sqrt();

            if distance < nearest_invasive_distance {
                nearest_invasive_distance = distance;
            }

            // Calculate avoidance vector if invasive species is within threat range (15.0 units)
            if distance < 15.0 && distance > 0.0 {
                let avoidance_strength = 1.0 - (distance / 15.0); // Stronger avoidance when closer
                avoidance_vector.0 -= (dx / distance) * avoidance_strength;
                avoidance_vector.1 -= (dy / distance) * avoidance_strength;
            }
        }

        // Apply stress effects if invasive species are nearby
        if nearest_invasive_distance < 20.0 {
            let delta_time = time.delta_seconds();
            let stress_factor = 1.0 - (nearest_invasive_distance / 20.0);
            lifecycle.energy -= stress_factor * 1.5 * delta_time; // Additional energy loss due to stress
        }

        // Apply avoidance behavior if necessary
        let avoiding_invasive = avoidance_vector.0.abs() > 0.1 || avoidance_vector.1.abs() > 0.1;
        if avoiding_invasive {
            // Normalize avoidance vector
            let avoidance_length = (avoidance_vector.0 * avoidance_vector.0
                + avoidance_vector.1 * avoidance_vector.1)
                .sqrt();
            if avoidance_length > 0.0 {
                let normalized_avoidance = (
                    avoidance_vector.0 / avoidance_length,
                    avoidance_vector.1 / avoidance_length,
                );

                // Override target position with avoidance direction
                behavior.target_position = Some(Position {
                    x: position.x + normalized_avoidance.0 * 25.0,
                    y: position.y + normalized_avoidance.1 * 25.0,
                });
            }
        }

        match behavior.state {
            AntState::Foraging => {
                // Check if ant should look for food when energy is low
                if lifecycle.energy < 30.0 && behavior.target_position.is_none() {
                    // Look for nearest available food using spatial indexing (O(k) instead of O(n))
                    let mut nearest_food: Option<Position> = None;
                    let mut nearest_distance = f32::INFINITY;

                    // Use spatial grid to get only nearby food sources within a reasonable search radius
                    let search_radius = 100.0; // Adjust based on game balance
                    let nearby_food_entities =
                        spatial_grid.get_entities_in_radius(&position, search_radius);

                    for food_entity in nearby_food_entities {
                        // Get food data for this entity
                        if let Ok((food_pos, food_source)) = food_query.get(food_entity) {
                            if food_source.is_available {
                                let dx = food_pos.x - position.x;
                                let dy = food_pos.y - position.y;
                                let distance = (dx * dx + dy * dy).sqrt();

                                // Only consider food within the search radius (additional filtering)
                                if distance <= search_radius && distance < nearest_distance {
                                    nearest_distance = distance;
                                    nearest_food = Some(Position {
                                        x: food_pos.x,
                                        y: food_pos.y,
                                    });
                                }
                            }
                        }
                    }

                    // Set target to nearest food or random position if no food available
                    behavior.target_position = match nearest_food {
                        Some(food_pos) => Some(food_pos),
                        None => Some(Position {
                            x: position.x + rng.gen_range(-50.0..50.0),
                            y: position.y + rng.gen_range(-50.0..50.0),
                        }),
                    };
                } else if behavior.target_position.is_none() {
                    // Normal random foraging when energy is sufficient
                    behavior.target_position = Some(Position {
                        x: position.x + rng.gen_range(-50.0..50.0),
                        y: position.y + rng.gen_range(-50.0..50.0),
                    });
                }

                // Move toward target
                if let Some(target) = &behavior.target_position {
                    let dx = target.x - position.x;
                    let dy = target.y - position.y;
                    let distance = (dx * dx + dy * dy).sqrt();

                    if distance > 1.0 {
                        let delta_time = time.delta_seconds();
                        let speed_modifier = 1.0; // Simplified: no disaster speed modifications
                        let move_distance = behavior.speed * delta_time * speed_modifier;
                        position.x += (dx / distance) * move_distance;
                        position.y += (dy / distance) * move_distance;

                        // Update visual transform to match logical position
                        transform.translation.x = position.x;
                        transform.translation.y = position.y;
                    } else {
                        // Reached target, pick new one
                        behavior.target_position = None;
                    }
                }
            }
            AntState::CarryingFood => {
                // Move back to colony with food
                if let Some(target) = &behavior.target_position {
                    let dx = target.x - position.x;
                    let dy = target.y - position.y;
                    let distance = (dx * dx + dy * dy).sqrt();

                    if distance > 2.0 {
                        let delta_time = time.delta_seconds();
                        let speed_modifier = 1.0; // Simplified: no disaster speed modifications
                        let move_distance = behavior.speed * delta_time * speed_modifier;
                        position.x += (dx / distance) * move_distance;
                        position.y += (dy / distance) * move_distance;

                        // Update visual transform to match logical position
                        transform.translation.x = position.x;
                        transform.translation.y = position.y;
                    } else {
                        // Reached colony, deliver food and return to foraging
                        inventory.carried_food_value = 0.0;
                        behavior.state = AntState::Foraging;
                        behavior.target_position = None;
                        info!("Ant delivered food to colony!");
                    }
                }
            }
            AntState::Digging => {
                // Realistic Camponotus japonicus digging behavior
                if behavior.target_position.is_none() {
                    // Find nearby soil to excavate - prefer areas near existing tunnels
                    let mut target_soil: Option<Position> = None;
                    let mut closest_distance = f32::INFINITY;

                    // Search for soil within digging range
                    for (_soil_entity, soil_pos, soil_cell) in soil_query.iter() {
                        let dx = soil_pos.x - position.x;
                        let dy = soil_pos.y - position.y;
                        let distance = (dx * dx + dy * dy).sqrt();

                        // Only dig soil within reasonable range and not too hard
                        if distance <= 20.0 && distance < closest_distance {
                            // Prefer easier soil (higher moisture, moderate temperature)
                            let soil_suitability = soil_cell.moisture * 0.7
                                + (1.0 - (soil_cell.temperature - 20.0).abs() / 20.0) * 0.3;

                            if soil_suitability > 0.4 {
                                closest_distance = distance;
                                target_soil = Some(Position {
                                    x: soil_pos.x,
                                    y: soil_pos.y,
                                });
                            }
                        }
                    }

                    // Set target or pick random underground position if no suitable soil found
                    behavior.target_position = match target_soil {
                        Some(soil_pos) => Some(soil_pos),
                        None => Some(Position {
                            x: position.x + rng.gen_range(-15.0..15.0),
                            y: position.y + rng.gen_range(-10.0..-2.0), // Prefer going deeper
                        }),
                    };
                }

                // Move toward digging target
                if let Some(target) = &behavior.target_position {
                    let dx = target.x - position.x;
                    let dy = target.y - position.y;
                    let distance = (dx * dx + dy * dy).sqrt();

                    if distance > 2.0 {
                        // Move toward target soil
                        let delta_time = time.delta_seconds();
                        let digging_speed = behavior.speed * 0.6; // Slower when digging
                        let move_distance = digging_speed * delta_time;
                        position.x += (dx / distance) * move_distance;
                        position.y += (dy / distance) * move_distance;

                        // Update visual transform
                        transform.translation.x = position.x;
                        transform.translation.y = position.y;
                    } else {
                        // Reached target soil - attempt to excavate it
                        let mut excavated = false;

                        for (soil_entity, soil_pos, mut soil_cell) in soil_query.iter_mut() {
                            let dx = soil_pos.x - position.x;
                            let dy = soil_pos.y - position.y;
                            let distance = (dx * dx + dy * dy).sqrt();

                            // Excavate soil within close range
                            if distance <= 3.0 {
                                // Energy cost for digging - realistic ant excavation work
                                let digging_cost = 2.0 * time.delta_seconds();
                                lifecycle.energy -= digging_cost;

                                // Soil hardness affects excavation speed
                                let excavation_efficiency = soil_cell.moisture * 0.8 + 0.2;
                                soil_cell.nutrition -= excavation_efficiency * time.delta_seconds();

                                // Remove soil when it's sufficiently excavated
                                if soil_cell.nutrition <= 0.1 {
                                    commands.entity(soil_entity).despawn();
                                    excavated = true;
                                    info!(
                                        "Ant excavated soil at ({:.1}, {:.1}) - tunnel expanded!",
                                        soil_pos.x, soil_pos.y
                                    );
                                    break;
                                }
                            }
                        }

                        // Pick new target after excavating or if no more soil nearby
                        if excavated || lifecycle.energy < 20.0 {
                            behavior.target_position = None;

                            // Switch to resting if energy is low
                            if lifecycle.energy < 20.0 {
                                behavior.state = AntState::Resting;
                                info!("Ant stopped digging due to low energy, switching to rest");
                            }
                        }
                    }
                }
            }
            AntState::Resting => {
                // Resting ants slowly recover energy
                let energy_recovery = 15.0 * time.delta_seconds();
                lifecycle.energy = (lifecycle.energy + energy_recovery).min(lifecycle.max_energy);

                // Return to digging when energy is restored
                if lifecycle.energy > 70.0 {
                    behavior.state = AntState::Digging;
                    behavior.target_position = None;
                }
            }
            AntState::Returning => {
                // Ants returning home (simplified for now)
                if let Some(target) = &behavior.target_position {
                    let dx = target.x - position.x;
                    let dy = target.y - position.y;
                    let distance = (dx * dx + dy * dy).sqrt();

                    if distance > 2.0 {
                        let delta_time = time.delta_seconds();
                        let move_distance = behavior.speed * delta_time;
                        position.x += (dx / distance) * move_distance;
                        position.y += (dy / distance) * move_distance;

                        transform.translation.x = position.x;
                        transform.translation.y = position.y;
                    } else {
                        // Reached home, switch to resting
                        behavior.state = AntState::Resting;
                        behavior.target_position = None;
                    }
                }
            }
        }
    }
}
