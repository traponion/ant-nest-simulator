use crate::components::{ColonyStatistics, StatisticsPanel, StatisticsToggle, UITheme};
use bevy::prelude::*;

/// Setup the statistics display panel UI
pub fn setup_statistics_panel(mut commands: Commands, ui_theme: Res<UITheme>) {
    // Main statistics panel container (initially hidden)
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(20.0), // Bottom-left corner
                bottom: Val::Px(20.0),
                width: Val::Px(350.0), // Wide enough for comprehensive data
                height: Val::Auto,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(ui_theme.spacing.md)),
                row_gap: Val::Px(ui_theme.spacing.sm),
                display: Display::None, // Start hidden
                ..default()
            },
            background_color: ui_theme.colors.surface_primary.into(),
            border_color: ui_theme.colors.border_primary.into(),
            ..default()
        })
        .insert(StatisticsPanel)
        .insert(StatisticsToggle::default())
        .with_children(|parent| {
            // Panel Title
            parent.spawn(TextBundle::from_section(
                "Colony Statistics",
                TextStyle {
                    font_size: ui_theme.typography.heading_small,
                    color: ui_theme.colors.text_primary,
                    ..default()
                },
            ));

            // Population Section
            create_statistics_section(parent, "Population", &ui_theme);
            parent.spawn(create_stat_text(
                "Total Ants: 0",
                "population_total",
                &ui_theme,
            ));
            parent.spawn(create_stat_text("Queen: 0", "population_queen", &ui_theme));
            parent.spawn(create_stat_text("Eggs: 0", "population_eggs", &ui_theme));
            parent.spawn(create_stat_text(
                "Age Distribution: No ants",
                "population_age",
                &ui_theme,
            ));

            // Resource Section
            create_statistics_section(parent, "Resources", &ui_theme);
            parent.spawn(create_stat_text(
                "Food Sources: 0",
                "resource_food",
                &ui_theme,
            ));
            parent.spawn(create_stat_text(
                "Avg Energy: 0%",
                "resource_energy",
                &ui_theme,
            ));
            parent.spawn(create_stat_text(
                "Foraging Efficiency: 0%",
                "resource_efficiency",
                &ui_theme,
            ));
            parent.spawn(create_stat_text(
                "Carrying Food: 0",
                "resource_carrying",
                &ui_theme,
            ));

            // Environment Section
            create_statistics_section(parent, "Environment", &ui_theme);
            parent.spawn(create_stat_text(
                "Soil Moisture: 0%",
                "environment_moisture",
                &ui_theme,
            ));
            parent.spawn(create_stat_text(
                "Soil Temperature: 0°C",
                "environment_temperature",
                &ui_theme,
            ));
            parent.spawn(create_stat_text(
                "Soil Nutrition: 0%",
                "environment_nutrition",
                &ui_theme,
            ));
            parent.spawn(create_stat_text(
                "Active Disasters: 0",
                "environment_disasters",
                &ui_theme,
            ));

            // Behavior Section
            create_statistics_section(parent, "Behavior", &ui_theme);
            parent.spawn(create_stat_text(
                "Activity: No ants",
                "behavior_activity",
                &ui_theme,
            ));

            // Role Distribution Section
            create_statistics_section(parent, "Role Distribution");
            parent.spawn(create_stat_text(
                "General Workers: 0",
                "role_general_workers",
            ));
            parent.spawn(create_stat_text("Foragers: 0", "role_foragers"));
            parent.spawn(create_stat_text(
                "Nest Maintainers: 0",
                "role_nest_maintainers",
            ));
            parent.spawn(create_stat_text(
                "Nursery Workers: 0",
                "role_nursery_workers",
            ));
            parent.spawn(create_stat_text("Waste Managers: 0", "role_waste_managers"));
            parent.spawn(create_stat_text(
                "Storage Workers: 0",
                "role_storage_workers",
            ));
            parent.spawn(create_stat_text(
                "Specialization Rate: 0%",
                "role_specialization",
            ));

            // Controls hint
            parent.spawn(TextBundle::from_section(
                "Press S to toggle",
                TextStyle {
                    font_size: ui_theme.typography.body_small,
                    color: ui_theme.colors.text_secondary,
                    ..default()
                },
            ));
        });
}

/// Create a section header for the statistics panel
fn create_statistics_section(parent: &mut ChildBuilder, title: &str, ui_theme: &UITheme) {
    parent.spawn(TextBundle::from_section(
        title,
        TextStyle {
            font_size: ui_theme.typography.body_medium,
            color: ui_theme.colors.text_secondary,
            ..default()
        },
    ));
}

/// Create a statistics text element with identifier for updates
fn create_stat_text(
    initial_text: &str,
    identifier: &str,
    ui_theme: &UITheme,
) -> (TextBundle, Name) {
    (
        TextBundle::from_section(
            initial_text,
            TextStyle {
                font_size: ui_theme.typography.body_small,
                color: ui_theme.colors.text_primary,
                ..default()
            },
        ),
        Name::new(identifier.to_string()),
    )
}

/// System for updating statistics display in real-time
pub fn update_statistics_display(
    colony_stats: Res<ColonyStatistics>,
    mut text_query: Query<(&mut Text, &Name)>,
    toggle_query: Query<&StatisticsToggle, With<StatisticsPanel>>,
) {
    // Only update if panel is visible
    if let Ok(toggle) = toggle_query.get_single() {
        if !toggle.is_visible {
            return;
        }
    }

    for (mut text, name) in text_query.iter_mut() {
        let new_text = match name.as_str() {
            "population_total" => format!("Total Ants: {}", colony_stats.total_ant_count),
            "population_queen" => format!("Queen: {}", colony_stats.queen_count),
            "population_eggs" => format!(
                "Eggs: {} (Avg hatch: {:.1}s)",
                colony_stats.egg_count, colony_stats.average_incubation_time
            ),
            "population_age" => format!("Age: {}", colony_stats.age_distribution_text()),

            "resource_food" => format!(
                "Food Sources: {} (Total: {:.0})",
                colony_stats.available_food_sources, colony_stats.total_food_nutrition
            ),
            "resource_energy" => format!(
                "Avg Energy: {:.0}% ({:.0}-{:.0})",
                colony_stats.average_energy_percentage(),
                colony_stats.min_ant_energy,
                colony_stats.max_ant_energy
            ),
            "resource_efficiency" => format!(
                "Foraging Efficiency: {:.1}%",
                colony_stats.foraging_efficiency()
            ),
            "resource_carrying" => format!(
                "Carrying Food: {} (Value: {:.0})",
                colony_stats.ants_carrying_food, colony_stats.total_carried_food_value
            ),

            "environment_moisture" => format!(
                "Soil Moisture: {:.1}% ({:.1}-{:.1})",
                colony_stats.average_soil_moisture * 100.0,
                colony_stats.min_soil_moisture * 100.0,
                colony_stats.max_soil_moisture * 100.0
            ),
            "environment_temperature" => format!(
                "Soil Temperature: {:.1}°C ({:.1}-{:.1})",
                colony_stats.average_soil_temperature,
                colony_stats.min_soil_temperature,
                colony_stats.max_soil_temperature
            ),
            "environment_nutrition" => format!(
                "Soil Nutrition: {:.1}% ({:.1}-{:.1})",
                colony_stats.average_soil_nutrition * 100.0,
                colony_stats.min_soil_nutrition * 100.0,
                colony_stats.max_soil_nutrition * 100.0
            ),
            "environment_disasters" => {
                format!("Active Disasters: {}", colony_stats.active_disasters_count)
            }

            "behavior_activity" => colony_stats.behavior_distribution_text(),

            "role_general_workers" => {
                format!("General Workers: {}", colony_stats.role_general_workers)
            }
            "role_foragers" => format!("Foragers: {}", colony_stats.role_foragers),
            "role_nest_maintainers" => {
                format!("Nest Maintainers: {}", colony_stats.role_nest_maintainers)
            }
            "role_nursery_workers" => {
                format!("Nursery Workers: {}", colony_stats.role_nursery_workers)
            }
            "role_waste_managers" => {
                format!("Waste Managers: {}", colony_stats.role_waste_managers)
            }
            "role_storage_workers" => {
                format!("Storage Workers: {}", colony_stats.role_storage_workers)
            }
            "role_specialization" => format!(
                "Specialization Rate: {:.1}%",
                colony_stats.specialization_rate()
            ),

            _ => continue,
        };

        if !text.sections.is_empty() {
            text.sections[0].value = new_text;
        }
    }
}

/// System for handling statistics panel toggle input (S key)
pub fn statistics_toggle_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut panel_query: Query<(&mut Style, &mut StatisticsToggle), With<StatisticsPanel>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyS) {
        if let Ok((mut style, mut toggle)) = panel_query.get_single_mut() {
            toggle.is_visible = !toggle.is_visible;

            style.display = if toggle.is_visible {
                Display::Flex
            } else {
                Display::None
            };

            if toggle.is_visible {
                info!("Statistics panel shown");
            } else {
                info!("Statistics panel hidden");
            }
        }
    }
}
