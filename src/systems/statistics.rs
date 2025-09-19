use crate::components::{
    Ant, AntBehavior, AntState, ColonyStatistics, DisasterState, Egg, FoodSource,
    InvasiveSpecies, Lifecycle, Queen, ReproductionState, SoilCell,
    StatisticsPanel, StatisticsText, StatisticsSectionType,
};
use bevy::prelude::*;

/// System for calculating colony statistics based on current entity states
pub fn colony_statistics_calculation_system(
    mut statistics: ResMut<ColonyStatistics>,
    time: Res<Time>,
    ant_query: Query<(&Lifecycle, &AntBehavior), With<Ant>>,
    queen_query: Query<&ReproductionState, With<Queen>>,
    egg_query: Query<&Egg>,
    food_query: Query<&FoodSource>,
    soil_query: Query<&SoilCell>,
    invasive_query: Query<Entity, With<InvasiveSpecies>>,
    disaster_state: Res<DisasterState>,
) {
    // Update timestamp
    statistics.last_updated = time.elapsed_seconds_f64();

    // Reset counters for this frame
    statistics.total_ant_count = 0;
    statistics.young_ants = 0;
    statistics.adult_ants = 0;
    statistics.elderly_ants = 0;
    statistics.foraging_ants = 0;
    statistics.returning_ants = 0;
    statistics.resting_ants = 0;
    statistics.digging_ants = 0;
    statistics.carrying_ants = 0;

    let mut total_energy = 0.0;
    let mut min_energy = f32::MAX;
    let mut max_energy = f32::MIN;
    let mut total_age = 0.0;

    // Calculate population and behavioral statistics
    for (lifecycle, behavior) in ant_query.iter() {
        statistics.total_ant_count += 1;

        // Age distribution (assuming max_age is around 100.0)
        let age_ratio = lifecycle.age / lifecycle.max_age;
        if age_ratio < 0.3 {
            statistics.young_ants += 1;
        } else if age_ratio < 0.7 {
            statistics.adult_ants += 1;
        } else {
            statistics.elderly_ants += 1;
        }

        // Behavioral statistics
        match behavior.state {
            AntState::Foraging => statistics.foraging_ants += 1,
            AntState::Returning => statistics.returning_ants += 1,
            AntState::Resting => statistics.resting_ants += 1,
            AntState::Digging => statistics.digging_ants += 1,
            AntState::CarryingFood => statistics.carrying_ants += 1,
        }

        // Energy statistics
        total_energy += lifecycle.energy;
        min_energy = min_energy.min(lifecycle.energy);
        max_energy = max_energy.max(lifecycle.energy);
        total_age += lifecycle.age;
    }

    // Calculate averages
    if statistics.total_ant_count > 0 {
        statistics.average_energy = total_energy / statistics.total_ant_count as f32;
        statistics.average_ant_age = total_age / statistics.total_ant_count as f32;
        statistics.min_energy = min_energy;
        statistics.max_energy = max_energy;
    } else {
        statistics.average_energy = 0.0;
        statistics.average_ant_age = 0.0;
        statistics.min_energy = 0.0;
        statistics.max_energy = 0.0;
    }

    // Queen and egg statistics
    statistics.queen_count = queen_query.iter().count() as u32;
    statistics.egg_count = egg_query.iter().count() as u32;

    // Calculate queen reproduction rate
    statistics.queen_reproduction_rate = 0.0;
    for reproduction_state in queen_query.iter() {
        if reproduction_state.egg_laying_interval > 0.0 {
            statistics.queen_reproduction_rate += 1.0 / reproduction_state.egg_laying_interval;
        }
    }

    // Food source statistics
    let mut available_food = 0;
    let mut total_food = 0;

    for food_source in food_query.iter() {
        total_food += 1;
        if food_source.is_available {
            available_food += 1;
        }
    }

    statistics.available_food_sources = available_food;
    statistics.total_food_sources = total_food;

    // Environmental statistics
    let mut total_moisture = 0.0;
    let mut total_temperature = 0.0;
    let mut total_nutrition = 0.0;
    let mut min_moisture = f32::MAX;
    let mut max_moisture = f32::MIN;
    let mut min_temperature = f32::MAX;
    let mut max_temperature = f32::MIN;
    let mut min_nutrition = f32::MAX;
    let mut max_nutrition = f32::MIN;
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
        let soil_count_f32 = soil_count as f32;
        statistics.average_soil_moisture = total_moisture / soil_count_f32;
        statistics.average_soil_temperature = total_temperature / soil_count_f32;
        statistics.average_soil_nutrition = total_nutrition / soil_count_f32;
        statistics.soil_moisture_range = (min_moisture, max_moisture);
        statistics.soil_temperature_range = (min_temperature, max_temperature);
        statistics.soil_nutrition_range = (min_nutrition, max_nutrition);
    }

    // Active disasters
    statistics.active_disasters = disaster_state.active_disasters.keys().cloned().collect();

    // Invasive species count
    statistics.invasive_species_count = invasive_query.iter().count() as u32;

    // Calculate foraging efficiency (simple metric: available food / foraging ants)
    if statistics.foraging_ants > 0 {
        statistics.foraging_efficiency =
            statistics.available_food_sources as f32 / statistics.foraging_ants as f32;
    } else {
        statistics.foraging_efficiency = 0.0;
    }
}

/// System to handle statistics panel toggle input (S key)
pub fn statistics_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut statistics: ResMut<ColonyStatistics>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyS) {
        statistics.toggle_visibility();
    }
}

/// System to setup the statistics panel UI
pub fn setup_statistics_panel(mut commands: Commands) {
    // Create the statistics panel container
    commands
        .spawn((
            StatisticsPanel,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(10.0),
                    bottom: Val::Px(10.0),
                    width: Val::Px(350.0),
                    height: Val::Auto,
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(15.0)),
                    row_gap: Val::Px(8.0),
                    ..default()
                },
                background_color: Color::srgba(0.1, 0.1, 0.1, 0.9).into(),
                border_color: Color::srgba(0.3, 0.3, 0.3, 1.0).into(),
                visibility: Visibility::Hidden, // Start hidden
                ..default()
            },
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                TextBundle::from_section(
                    "Colony Statistics (S to toggle)",
                    TextStyle {
                        font_size: 16.0,
                        color: Color::srgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ),
                StatisticsText {
                    section_type: StatisticsSectionType::Population,
                },
            ));

            // Population Section
            parent.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 12.0,
                        color: Color::srgb(0.8, 0.8, 0.8),
                        ..default()
                    },
                ),
                StatisticsText {
                    section_type: StatisticsSectionType::Population,
                },
            ));

            // Resources Section
            parent.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 12.0,
                        color: Color::srgb(0.8, 0.8, 0.8),
                        ..default()
                    },
                ),
                StatisticsText {
                    section_type: StatisticsSectionType::Resources,
                },
            ));

            // Environment Section
            parent.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 12.0,
                        color: Color::srgb(0.8, 0.8, 0.8),
                        ..default()
                    },
                ),
                StatisticsText {
                    section_type: StatisticsSectionType::Environment,
                },
            ));

            // Behavior Section
            parent.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 12.0,
                        color: Color::srgb(0.8, 0.8, 0.8),
                        ..default()
                    },
                ),
                StatisticsText {
                    section_type: StatisticsSectionType::Behavior,
                },
            ));
        });
}

/// System to update statistics panel visibility and content
pub fn update_statistics_display_system(
    statistics: Res<ColonyStatistics>,
    mut panel_query: Query<&mut Visibility, With<StatisticsPanel>>,
    mut text_query: Query<(&mut Text, &StatisticsText), Without<StatisticsPanel>>,
) {
    // Update panel visibility
    for mut visibility in panel_query.iter_mut() {
        *visibility = if statistics.is_visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }

    // Update text content if visible
    if !statistics.is_visible {
        return;
    }

    for (mut text, stats_text) in text_query.iter_mut() {
        let content = match stats_text.section_type {
            StatisticsSectionType::Population => {
                format!(
                    "📊 POPULATION\n• Total Ants: {}\n• Queen: {}\n• Eggs: {}\n• Young/Adult/Elderly: {}/{}/{}\n• Average Age: {:.1}",
                    statistics.total_ant_count,
                    statistics.queen_count,
                    statistics.egg_count,
                    statistics.young_ants,
                    statistics.adult_ants,
                    statistics.elderly_ants,
                    statistics.average_ant_age
                )
            }
            StatisticsSectionType::Resources => {
                format!(
                    "🍯 RESOURCES\n• Food Sources: {}/{}\n• Energy Avg/Min/Max: {:.1}/{:.1}/{:.1}\n• Foraging Efficiency: {:.2}\n• Reproduction Rate: {:.3}/s",
                    statistics.available_food_sources,
                    statistics.total_food_sources,
                    statistics.average_energy,
                    statistics.min_energy,
                    statistics.max_energy,
                    statistics.foraging_efficiency,
                    statistics.queen_reproduction_rate
                )
            }
            StatisticsSectionType::Environment => {
                let disasters_str = if statistics.active_disasters.is_empty() {
                    "None".to_string()
                } else {
                    statistics.active_disasters
                        .iter()
                        .map(|d| d.display_name())
                        .collect::<Vec<_>>()
                        .join(", ")
                };

                format!(
                    "🌍 ENVIRONMENT\n• Soil Moisture: {:.2} ({:.2}-{:.2})\n• Soil Temperature: {:.1} ({:.1}-{:.1})\n• Soil Nutrition: {:.2} ({:.2}-{:.2})\n• Active Disasters: {}\n• Invasive Species: {}",
                    statistics.average_soil_moisture,
                    statistics.soil_moisture_range.0,
                    statistics.soil_moisture_range.1,
                    statistics.average_soil_temperature,
                    statistics.soil_temperature_range.0,
                    statistics.soil_temperature_range.1,
                    statistics.average_soil_nutrition,
                    statistics.soil_nutrition_range.0,
                    statistics.soil_nutrition_range.1,
                    disasters_str,
                    statistics.invasive_species_count
                )
            }
            StatisticsSectionType::Behavior => {
                let (foraging_pct, returning_pct, resting_pct, digging_pct, carrying_pct) =
                    statistics.get_ant_state_percentages();

                format!(
                    "🐜 BEHAVIOR\n• Foraging: {} ({:.1}%)\n• Returning: {} ({:.1}%)\n• Resting: {} ({:.1}%)\n• Digging: {} ({:.1}%)\n• Carrying: {} ({:.1}%)",
                    statistics.foraging_ants, foraging_pct,
                    statistics.returning_ants, returning_pct,
                    statistics.resting_ants, resting_pct,
                    statistics.digging_ants, digging_pct,
                    statistics.carrying_ants, carrying_pct
                )
            }
        };

        text.sections[0].value = content;
    }
}