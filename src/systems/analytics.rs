use crate::components::{
    AgeGroup, AnalyticsDashboard, Ant, ColonyAnalytics, Egg, FoodSource, Lifecycle, MetricDisplay,
    MetricType, Queen, SimulationTime,
};
use bevy::prelude::*;

/// Setup analytics dashboard UI in the top-left corner
pub fn setup_analytics_dashboard(mut commands: Commands) {
    // Main analytics dashboard container - positioned at top-left
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                top: Val::Px(10.0),
                width: Val::Px(280.0),
                max_height: Val::Px(400.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(15.0)),
                row_gap: Val::Px(8.0),
                ..default()
            },
            background_color: Color::srgba(0.0, 0.0, 0.0, 0.8).into(),
            border_radius: BorderRadius::all(Val::Px(8.0)),
            visibility: Visibility::Visible, // Start visible
            ..default()
        })
        .with_children(|parent| {
            // Panel title with toggle hint
            parent.spawn(TextBundle::from_section(
                "Colony Analytics (A to toggle)",
                TextStyle {
                    font_size: 18.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));

            // Population section
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(8.0)),
                        row_gap: Val::Px(4.0),
                        ..default()
                    },
                    background_color: Color::srgba(0.2, 0.2, 0.2, 0.6).into(),
                    border_radius: BorderRadius::all(Val::Px(4.0)),
                    ..default()
                })
                .with_children(|section_parent| {
                    // Section title
                    section_parent.spawn(TextBundle::from_section(
                        "Population",
                        TextStyle {
                            font_size: 16.0,
                            color: Color::srgb(0.8, 1.0, 0.8),
                            ..default()
                        },
                    ));

                    // Total population
                    section_parent.spawn((
                        TextBundle::from_section(
                            "Total: 0",
                            TextStyle {
                                font_size: 14.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ),
                        MetricDisplay {
                            metric_type: MetricType::Population,
                        },
                    ));

                    // Birth rate
                    section_parent.spawn((
                        TextBundle::from_section(
                            "Birth Rate: 0.0/min",
                            TextStyle {
                                font_size: 12.0,
                                color: Color::srgb(0.8, 1.0, 0.8),
                                ..default()
                            },
                        ),
                        MetricDisplay {
                            metric_type: MetricType::BirthRate,
                        },
                    ));

                    // Death rate
                    section_parent.spawn((
                        TextBundle::from_section(
                            "Death Rate: 0.0/min",
                            TextStyle {
                                font_size: 12.0,
                                color: Color::srgb(1.0, 0.8, 0.8),
                                ..default()
                            },
                        ),
                        MetricDisplay {
                            metric_type: MetricType::DeathRate,
                        },
                    ));
                });

            // Resources section
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(8.0)),
                        row_gap: Val::Px(4.0),
                        ..default()
                    },
                    background_color: Color::srgba(0.2, 0.2, 0.2, 0.6).into(),
                    border_radius: BorderRadius::all(Val::Px(4.0)),
                    ..default()
                })
                .with_children(|section_parent| {
                    // Section title
                    section_parent.spawn(TextBundle::from_section(
                        "Resources",
                        TextStyle {
                            font_size: 16.0,
                            color: Color::srgb(1.0, 1.0, 0.8),
                            ..default()
                        },
                    ));

                    // Food collected
                    section_parent.spawn((
                        TextBundle::from_section(
                            "Food Collected: 0.0",
                            TextStyle {
                                font_size: 14.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ),
                        MetricDisplay {
                            metric_type: MetricType::FoodCollected,
                        },
                    ));

                    // Average energy
                    section_parent.spawn((
                        TextBundle::from_section(
                            "Avg Energy: 0.0%",
                            TextStyle {
                                font_size: 12.0,
                                color: Color::srgb(0.8, 0.8, 1.0),
                                ..default()
                            },
                        ),
                        MetricDisplay {
                            metric_type: MetricType::AverageEnergy,
                        },
                    ));

                    // Foraging success
                    section_parent.spawn((
                        TextBundle::from_section(
                            "Foraging Success: 0.0%",
                            TextStyle {
                                font_size: 12.0,
                                color: Color::srgb(1.0, 0.8, 1.0),
                                ..default()
                            },
                        ),
                        MetricDisplay {
                            metric_type: MetricType::ForagingSuccess,
                        },
                    ));
                });
        })
        .insert(AnalyticsDashboard);
}

/// System for collecting analytics data from colony entities
pub fn analytics_data_collection_system(
    mut analytics: ResMut<ColonyAnalytics>,
    time: Res<Time>,
    _simulation_time: Res<SimulationTime>,
    ant_query: Query<&Lifecycle, (With<Ant>, Without<Queen>)>,
    queen_query: Query<&Lifecycle, With<Queen>>,
    egg_query: Query<&Egg>,
    food_query: Query<&FoodSource>,
) {
    let delta_time = time.delta_seconds();
    analytics.update_timer += delta_time;

    // Only update analytics at the specified interval (e.g., once per second)
    if analytics.update_timer < analytics.update_interval {
        return;
    }

    // Reset timer
    analytics.update_timer = 0.0;

    // Count population by age groups
    let mut total_population = 0;
    analytics.population_by_age.clear();
    analytics.population_by_age.insert(AgeGroup::Egg, 0);
    analytics.population_by_age.insert(AgeGroup::Young, 0);
    analytics.population_by_age.insert(AgeGroup::Adult, 0);
    analytics.population_by_age.insert(AgeGroup::Elderly, 0);

    // Count eggs
    let egg_count = egg_query.iter().count();
    analytics.population_by_age.insert(AgeGroup::Egg, egg_count);
    total_population += egg_count;

    // Count ants by age
    for lifecycle in ant_query.iter() {
        let age_ratio = lifecycle.age / lifecycle.max_age;
        let age_group = if age_ratio < 0.25 {
            AgeGroup::Young
        } else if age_ratio < 0.75 {
            AgeGroup::Adult
        } else {
            AgeGroup::Elderly
        };

        *analytics.population_by_age.get_mut(&age_group).unwrap() += 1;
        total_population += 1;
    }

    // Count queen
    if !queen_query.is_empty() {
        let queen_lifecycle = queen_query.single();
        let age_ratio = queen_lifecycle.age / queen_lifecycle.max_age;
        let age_group = if age_ratio < 0.25 {
            AgeGroup::Young
        } else if age_ratio < 0.75 {
            AgeGroup::Adult
        } else {
            AgeGroup::Elderly
        };

        *analytics.population_by_age.get_mut(&age_group).unwrap() += 1;
        total_population += 1;
    }

    analytics.total_population = total_population;

    // Calculate average energy
    let mut total_energy = 0.0;
    let mut energy_count = 0;

    for lifecycle in ant_query.iter() {
        if lifecycle.max_energy > 0.0 {
            total_energy += lifecycle.energy / lifecycle.max_energy;
            energy_count += 1;
        }
    }

    for lifecycle in queen_query.iter() {
        if lifecycle.max_energy > 0.0 {
            total_energy += lifecycle.energy / lifecycle.max_energy;
            energy_count += 1;
        }
    }

    analytics.average_energy = if energy_count > 0 {
        (total_energy / energy_count as f32) * 100.0
    } else {
        0.0
    };

    // Calculate total food available
    let total_food: f32 = food_query
        .iter()
        .filter(|food| food.is_available)
        .map(|food| food.nutrition_value)
        .sum();

    analytics.food_collected = total_food;

    // Add snapshots to history
    let current_time = time.elapsed_seconds();
    analytics.add_population_snapshot(current_time);
    analytics.add_resource_snapshot(current_time);
}

/// System for handling analytics dashboard toggle input
pub fn analytics_toggle_system(
    input: Res<ButtonInput<KeyCode>>,
    mut analytics: ResMut<ColonyAnalytics>,
    mut dashboard_query: Query<&mut Visibility, With<AnalyticsDashboard>>,
) {
    if input.just_pressed(KeyCode::KeyA) {
        analytics.is_visible = !analytics.is_visible;

        for mut visibility in dashboard_query.iter_mut() {
            *visibility = if analytics.is_visible {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }

        info!(
            "Analytics dashboard {}",
            if analytics.is_visible {
                "shown"
            } else {
                "hidden"
            }
        );
    }
}

/// System for updating metric displays in the analytics dashboard
pub fn update_analytics_display_system(
    analytics: Res<ColonyAnalytics>,
    mut metric_query: Query<(&MetricDisplay, &mut Text)>,
) {
    for (metric_display, mut text) in metric_query.iter_mut() {
        let new_text = match metric_display.metric_type {
            MetricType::Population => {
                format!("Total: {}", analytics.total_population)
            }
            MetricType::FoodCollected => {
                format!("Food Available: {:.1}", analytics.food_collected)
            }
            MetricType::AverageEnergy => {
                format!("Avg Energy: {:.1}%", analytics.average_energy)
            }
            MetricType::ForagingSuccess => {
                format!(
                    "Foraging Success: {:.1}%",
                    analytics.foraging_success_rate()
                )
            }
            MetricType::BirthRate => {
                format!("Birth Rate: {:.1}/min", analytics.birth_rate())
            }
            MetricType::DeathRate => {
                format!("Death Rate: {:.1}/min", analytics.death_rate())
            }
        };

        text.sections[0].value = new_text;
    }
}

/// System for tracking foraging attempts and successes (to be called from foraging system)
pub fn track_foraging_attempt(analytics: &mut ResMut<ColonyAnalytics>) {
    analytics.foraging_attempts += 1;
}

/// System for tracking successful foraging (to be called from foraging system)
pub fn track_foraging_success(analytics: &mut ResMut<ColonyAnalytics>) {
    analytics.successful_foraging += 1;
}

/// System for tracking births (to be called from reproduction system)
pub fn track_birth(analytics: &mut ResMut<ColonyAnalytics>) {
    analytics.birth_count += 1;
}

/// System for tracking deaths (to be called from lifecycle system)
pub fn track_death(analytics: &mut ResMut<ColonyAnalytics>) {
    analytics.death_count += 1;
}
