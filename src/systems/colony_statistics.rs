use crate::components::{
    Ant, AntBehavior, AntState, ColonyStatistics, DisasterState, Egg, FoodSource, Inventory,
    Lifecycle, Queen, ReproductionState, SoilCell, TimeControl,
};
use crate::systems::time_control::effective_delta_time;
use bevy::prelude::*;

/// System for calculating and updating colony statistics in real-time
pub fn colony_statistics_calculation_system(
    time: Res<Time>,
    time_control: Res<TimeControl>,
    mut colony_stats: ResMut<ColonyStatistics>,
    disaster_state: Res<DisasterState>,
    ant_query: Query<(&AntBehavior, &Lifecycle, &Inventory), (With<Ant>, Without<Queen>)>,
    queen_query: Query<(&Lifecycle, &ReproductionState), (With<Queen>, With<Ant>)>,
    egg_query: Query<&Egg>,
    food_query: Query<&FoodSource>,
    soil_query: Query<&SoilCell>,
) {
    let delta_time = effective_delta_time(&time, &time_control);

    // Update last update time
    colony_stats.last_update_time += delta_time;

    // Reset statistics for fresh calculation
    colony_stats.reset();

    // Population Statistics
    calculate_population_stats(&mut colony_stats, &ant_query, &queen_query, &egg_query);

    // Resource Management
    calculate_resource_stats(&mut colony_stats, &ant_query, &food_query);

    // Environmental Status
    calculate_environmental_stats(&mut colony_stats, &disaster_state, &soil_query);

    // Behavioral Insights
    calculate_behavioral_stats(&mut colony_stats, &ant_query);

    // Queen Reproduction Statistics
    calculate_reproduction_stats(&mut colony_stats, &queen_query);
}

/// Calculate population-related statistics
fn calculate_population_stats(
    stats: &mut ColonyStatistics,
    ant_query: &Query<(&AntBehavior, &Lifecycle, &Inventory), (With<Ant>, Without<Queen>)>,
    queen_query: &Query<(&Lifecycle, &ReproductionState), (With<Queen>, With<Ant>)>,
    egg_query: &Query<&Egg>,
) {
    // Count ants and calculate age distribution
    stats.total_ant_count = ant_query.iter().count();
    stats.queen_count = queen_query.iter().count();
    stats.egg_count = egg_query.iter().count();

    // Calculate age distribution
    let mut total_incubation_time = 0.0;
    for egg in egg_query.iter() {
        total_incubation_time += egg.incubation_time;
    }

    if stats.egg_count > 0 {
        stats.average_incubation_time = total_incubation_time / stats.egg_count as f32;
    }

    // Age distribution calculation
    for (_, lifecycle, _) in ant_query.iter() {
        let age_ratio = lifecycle.age / lifecycle.max_age;

        if age_ratio < 0.3 {
            stats.young_ants += 1;
        } else if age_ratio < 0.7 {
            stats.adult_ants += 1;
        } else {
            stats.elderly_ants += 1;
        }
    }

    // Note: recent_births and recent_deaths would need to be tracked
    // separately in a more sophisticated system with event tracking
    stats.recent_births = 0; // Placeholder
    stats.recent_deaths = 0; // Placeholder
}

/// Calculate resource management statistics
fn calculate_resource_stats(
    stats: &mut ColonyStatistics,
    ant_query: &Query<(&AntBehavior, &Lifecycle, &Inventory), (With<Ant>, Without<Queen>)>,
    food_query: &Query<&FoodSource>,
) {
    // Food source statistics
    for food_source in food_query.iter() {
        if food_source.is_available {
            stats.available_food_sources += 1;
            stats.total_food_nutrition += food_source.nutrition_value;
        }
    }

    // Ant energy statistics
    let mut total_energy = 0.0;
    let mut min_energy = f32::INFINITY;
    let mut max_energy: f32 = 0.0;
    let mut total_carried_food = 0.0;
    let mut carrying_count = 0;

    for (_, lifecycle, inventory) in ant_query.iter() {
        total_energy += lifecycle.energy;
        min_energy = min_energy.min(lifecycle.energy);
        max_energy = max_energy.max(lifecycle.max_energy);

        if inventory.carried_food_value > 0.0 {
            total_carried_food += inventory.carried_food_value;
            carrying_count += 1;
        }
    }

    if stats.total_ant_count > 0 {
        stats.average_ant_energy = total_energy / stats.total_ant_count as f32;
        stats.min_ant_energy = if min_energy == f32::INFINITY { 0.0 } else { min_energy };
        stats.max_ant_energy = max_energy;
    }

    stats.ants_carrying_food = carrying_count;
    stats.total_carried_food_value = total_carried_food;
}

/// Calculate environmental statistics
fn calculate_environmental_stats(
    stats: &mut ColonyStatistics,
    disaster_state: &DisasterState,
    soil_query: &Query<&SoilCell>,
) {
    // Disaster count
    stats.active_disasters_count = disaster_state.active_disasters.len();

    // Soil statistics
    let mut total_moisture = 0.0;
    let mut total_temperature = 0.0;
    let mut total_nutrition = 0.0;
    let mut min_moisture = f32::INFINITY;
    let mut max_moisture = f32::NEG_INFINITY;
    let mut min_temperature = f32::INFINITY;
    let mut max_temperature = f32::NEG_INFINITY;
    let mut min_nutrition = f32::INFINITY;
    let mut max_nutrition = f32::NEG_INFINITY;

    let soil_count = soil_query.iter().count();

    for soil in soil_query.iter() {
        total_moisture += soil.moisture;
        total_temperature += soil.temperature;
        total_nutrition += soil.nutrition;

        min_moisture = min_moisture.min(soil.moisture);
        max_moisture = max_moisture.max(soil.moisture);
        min_temperature = min_temperature.min(soil.temperature);
        max_temperature = max_temperature.max(soil.temperature);
        min_nutrition = min_nutrition.min(soil.nutrition);
        max_nutrition = max_nutrition.max(soil.nutrition);
    }

    if soil_count > 0 {
        stats.average_soil_moisture = total_moisture / soil_count as f32;
        stats.average_soil_temperature = total_temperature / soil_count as f32;
        stats.average_soil_nutrition = total_nutrition / soil_count as f32;

        stats.min_soil_moisture = min_moisture;
        stats.max_soil_moisture = max_moisture;
        stats.min_soil_temperature = min_temperature;
        stats.max_soil_temperature = max_temperature;
        stats.min_soil_nutrition = min_nutrition;
        stats.max_soil_nutrition = max_nutrition;
    }
}

/// Calculate behavioral insights statistics
fn calculate_behavioral_stats(
    stats: &mut ColonyStatistics,
    ant_query: &Query<(&AntBehavior, &Lifecycle, &Inventory), (With<Ant>, Without<Queen>)>,
) {
    for (behavior, _, _) in ant_query.iter() {
        match behavior.state {
            AntState::Foraging => stats.ants_foraging += 1,
            AntState::Returning => stats.ants_returning += 1,
            AntState::Resting => stats.ants_resting += 1,
            AntState::Digging => stats.ants_digging += 1,
            AntState::CarryingFood => stats.ants_carrying += 1,
        }
    }
}

/// Calculate reproduction statistics
fn calculate_reproduction_stats(
    stats: &mut ColonyStatistics,
    queen_query: &Query<(&Lifecycle, &ReproductionState), (With<Queen>, With<Ant>)>,
) {
    // Get queen reproduction statistics if queen exists
    if let Ok((_, reproduction_state)) = queen_query.get_single() {
        stats.queen_reproduction_capacity = reproduction_state.reproductive_capacity;
        stats.time_since_last_egg = reproduction_state.time_since_last_egg;
    }
}