use bevy::prelude::*;
use crate::components::{Position, AntBehavior, AntState, Ant, Lifecycle, Food, FoodSource, Inventory, TimeControl};
use crate::systems::time_control::effective_delta_time;

/// System for handling food consumption and energy recovery
pub fn food_consumption_system(
    mut ant_query: Query<(Entity, &Position, &mut AntBehavior, &mut Lifecycle, &mut Inventory), With<Ant>>,
    mut food_query: Query<(Entity, &Position, &mut FoodSource), With<Food>>,
) {
    for (_ant_entity, ant_pos, mut ant_behavior, mut ant_lifecycle, mut inventory) in ant_query.iter_mut() {
        // Only process ants that are foraging
        if ant_behavior.state != AntState::Foraging {
            continue;
        }

        // Check for food sources within consumption range (2.0 units)
        for (_food_entity, food_pos, mut food_source) in food_query.iter_mut() {
            if !food_source.is_available {
                continue;
            }

            let dx = food_pos.x - ant_pos.x;
            let dy = food_pos.y - ant_pos.y;
            let distance = (dx * dx + dy * dy).sqrt();

            // If ant is close enough to consume food
            if distance <= 2.0 {
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
                    ant_lifecycle.energy, ant_lifecycle.max_energy, inventory.carried_food_value
                );

                // Only consume one food source per frame per ant
                break;
            }
        }
    }
}

/// System for handling food regeneration over time
pub fn food_regeneration_system(
    time: Res<Time>,
    time_control: Res<TimeControl>,
    mut food_query: Query<&mut FoodSource, With<Food>>,
) {
    let delta_time = effective_delta_time(&time, &time_control);

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