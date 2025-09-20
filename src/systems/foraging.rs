use crate::components::{
    Ant, AntBehavior, AntState, Food, FoodSource, InvasiveSpecies, Inventory, Lifecycle, Position,
    SpatialGrid,
};
use bevy::prelude::*;

/// System for handling food consumption and energy recovery
pub fn food_consumption_system(
    spatial_grid: Res<SpatialGrid>,
    mut ant_query: Query<
        (
            Entity,
            &Position,
            &mut AntBehavior,
            &mut Lifecycle,
            &mut Inventory,
        ),
        With<Ant>,
    >,
    mut food_query: Query<(Entity, &Position, &mut FoodSource), With<Food>>,
) {
    for (_ant_entity, ant_pos, mut ant_behavior, mut ant_lifecycle, mut inventory) in
        ant_query.iter_mut()
    {
        // Only process ants that are foraging
        if ant_behavior.state != AntState::Foraging {
            continue;
        }

        // Check for food sources within consumption range (wider range for debugging) using spatial indexing
        let consumption_radius = 10.0;
        let nearby_food_entities = spatial_grid.get_entities_in_radius(ant_pos, consumption_radius);

        for food_entity in nearby_food_entities {
            if let Ok((_entity, food_pos, mut food_source)) = food_query.get_mut(food_entity) {
                if !food_source.is_available {
                    continue;
                }

                let dx = food_pos.x - ant_pos.x;
                let dy = food_pos.y - ant_pos.y;
                let distance = (dx * dx + dy * dy).sqrt();

                // If ant is close enough to consume food (same as detection radius)
                if distance <= consumption_radius {
                    // Consume the food
                    food_source.is_available = false;
                    food_source.regeneration_timer = food_source.regeneration_time;

                    // Recover energy
                    ant_lifecycle.energy += food_source.nutrition_value;
                    ant_lifecycle.energy = ant_lifecycle.energy.min(ant_lifecycle.max_energy); // Cap at max energy

                    // Set inventory to carry food value back to colony
                    inventory.carried_food_value = food_source.nutrition_value;

                    // Change state to carrying food back to colony
                    ant_behavior.state = AntState::CarryingFood;
                    ant_behavior.target_position = Some(inventory.home_position.clone());

                    info!(
                        "Ant consumed food! Energy: {:.1}/{:.1}, Carrying: {:.1}",
                        ant_lifecycle.energy,
                        ant_lifecycle.max_energy,
                        inventory.carried_food_value
                    );

                    // Only consume one food source per frame per ant
                    break;
                }
            }
        }
    }
}

/// System for handling food regeneration over time
pub fn food_regeneration_system(
    time: Res<Time>,
    mut food_query: Query<&mut FoodSource, With<Food>>,
) {
    let delta_time = time.delta_seconds();

    for mut food_source in food_query.iter_mut() {
        if !food_source.is_available && food_source.regeneration_timer > 0.0 {
            food_source.regeneration_timer -= delta_time;

            // Regenerate food source when timer reaches zero
            if food_source.regeneration_timer <= 0.0 {
                food_source.is_available = true;
                food_source.regeneration_timer = 0.0;
                info!("Food source regenerated!");
            }
        }
    }
}

/// System for invasive species food consumption (more aggressive than ants)
pub fn invasive_species_food_consumption_system(
    invasive_query: Query<(&Position, &InvasiveSpecies)>,
    mut food_query: Query<(Entity, &Position, &mut FoodSource), With<Food>>,
) {
    for (invasive_pos, invasive) in invasive_query.iter() {
        // Check for food sources within consumption range
        for (_food_entity, food_pos, mut food_source) in food_query.iter_mut() {
            if !food_source.is_available {
                continue;
            }

            let dx = food_pos.x - invasive_pos.x;
            let dy = food_pos.y - invasive_pos.y;
            let distance = (dx * dx + dy * dy).sqrt();

            // Invasive species have a slightly larger consumption range
            if distance <= 3.0 {
                // Consume food more aggressively based on consumption rate
                food_source.is_available = false;
                // Invasive species damage food sources - longer regeneration time
                food_source.regeneration_timer =
                    food_source.regeneration_time * invasive.food_consumption_rate * 1.5;

                info!(
                    "Invasive species consumed food! Regeneration extended to: {:.1}s",
                    food_source.regeneration_timer
                );

                // Invasive species can consume multiple food sources per frame
                // This makes them more damaging to the ecosystem
            }
        }
    }
}
