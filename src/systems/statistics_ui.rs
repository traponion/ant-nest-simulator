use crate::components::{ColonyStatistics, StatisticsPanel, StatisticsToggle, UITheme};
use bevy::prelude::*;

/// Setup the statistics display panel UI with enhanced theming
pub fn setup_statistics_panel(mut commands: Commands, theme: Res<UITheme>) {
    // Main statistics panel container (initially hidden) with theme styling
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(theme.spacing.lg),
                bottom: Val::Px(theme.spacing.lg),
                width: Val::Px(380.0),  // Slightly wider for better content fit
                height: Val::Auto,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(theme.spacing.lg)),
                row_gap: Val::Px(theme.spacing.md),
                border: UiRect::all(Val::Px(theme.borders.width_medium)),
                display: Display::None, // Start hidden
                ..default()
            },
            background_color: theme.colors.surface_primary.into(),
            border_color: theme.colors.border_primary.into(),
            border_radius: BorderRadius::all(Val::Px(theme.borders.radius_medium)),
            ..default()
        })
        .insert(StatisticsPanel)
        .insert(StatisticsToggle::default())
        .with_children(|parent| {
            // Enhanced Panel Title with theme typography
            parent.spawn(TextBundle::from_section(
                "Colony Statistics",
                TextStyle {
                    font_size: theme.typography.heading_medium,
                    color: theme.colors.text_primary,
                    ..default()
                },
            ));

            // Population Section with enhanced theming
            create_themed_statistics_section(parent, "Population", &theme);
            parent.spawn(create_themed_stat_text("Total Ants: 0", "population_total", &theme));
            parent.spawn(create_themed_stat_text("Queen: 0", "population_queen", &theme));
            parent.spawn(create_themed_stat_text("Eggs: 0", "population_eggs", &theme));
            parent.spawn(create_themed_stat_text("Age Distribution: No ants", "population_age", &theme));

            // Resource Section with enhanced theming
            create_themed_statistics_section(parent, "Resources", &theme);
            parent.spawn(create_themed_stat_text("Food Sources: 0", "resource_food", &theme));
            parent.spawn(create_themed_stat_text("Avg Energy: 0%", "resource_energy", &theme));
            parent.spawn(create_themed_stat_text("Foraging Efficiency: 0%", "resource_efficiency", &theme));
            parent.spawn(create_themed_stat_text("Carrying Food: 0", "resource_carrying", &theme));

            // Environment Section with enhanced theming
            create_themed_statistics_section(parent, "Environment", &theme);
            parent.spawn(create_themed_stat_text("Soil Moisture: 0%", "environment_moisture", &theme));
            parent.spawn(create_themed_stat_text("Soil Temperature: 0°C", "environment_temperature", &theme));
            parent.spawn(create_themed_stat_text("Soil Nutrition: 0%", "environment_nutrition", &theme));
            parent.spawn(create_themed_stat_text("Active Disasters: 0", "environment_disasters", &theme));

            // Behavior Section with enhanced theming
            create_themed_statistics_section(parent, "Behavior", &theme);
            parent.spawn(create_themed_stat_text("Activity: No ants", "behavior_activity", &theme));

            // Enhanced controls hint with better styling
            parent
                .spawn(NodeBundle {
                    style: Style {
                        padding: UiRect::all(Val::Px(theme.spacing.sm)),
                        margin: UiRect::top(Val::Px(theme.spacing.md)),
                        border: UiRect::all(Val::Px(theme.borders.width_thin)),
                        ..default()
                    },
                    background_color: theme.colors.surface_secondary.into(),
                    border_color: theme.colors.border_secondary.into(),
                    border_radius: BorderRadius::all(Val::Px(theme.borders.radius_small)),
                    ..default()
                })
                .with_children(|hint_parent| {
                    hint_parent.spawn(TextBundle::from_section(
                        "💡 Press S to toggle statistics panel",
                        TextStyle {
                            font_size: theme.typography.caption,
                            color: theme.colors.text_muted,
                            ..default()
                        },
                    ));
                });
        });
}

/// Create a themed section header for the statistics panel
fn create_themed_statistics_section(parent: &mut ChildBuilder, title: &str, theme: &UITheme) {
    parent
        .spawn(NodeBundle {
            style: Style {
                padding: UiRect::all(Val::Px(theme.spacing.sm)),
                margin: UiRect::vertical(Val::Px(theme.spacing.sm)),
                border: UiRect::bottom(Val::Px(theme.borders.width_thin)),
                ..default()
            },
            background_color: Color::NONE.into(),
            border_color: theme.colors.accent_blue.into(),
            ..default()
        })
        .with_children(|section_parent| {
            section_parent.spawn(TextBundle::from_section(
                title,
                TextStyle {
                    font_size: theme.typography.heading_small,
                    color: theme.colors.accent_blue,
                    ..default()
                },
            ));
        });
}

/// Create a themed statistics text element with identifier for updates
fn create_themed_stat_text(initial_text: &str, identifier: &str, theme: &UITheme) -> (TextBundle, Name) {
    (
        TextBundle::from_section(
            initial_text,
            TextStyle {
                font_size: theme.typography.body_small,
                color: theme.colors.text_secondary,
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
            "population_eggs" => format!("Eggs: {} (Avg hatch: {:.1}s)",
                colony_stats.egg_count, colony_stats.average_incubation_time),
            "population_age" => format!("Age: {}", colony_stats.age_distribution_text()),

            "resource_food" => format!("Food Sources: {} (Total: {:.0})",
                colony_stats.available_food_sources, colony_stats.total_food_nutrition),
            "resource_energy" => format!("Avg Energy: {:.0}% ({:.0}-{:.0})",
                colony_stats.average_energy_percentage(),
                colony_stats.min_ant_energy, colony_stats.max_ant_energy),
            "resource_efficiency" => format!("Foraging Efficiency: {:.1}%",
                colony_stats.foraging_efficiency()),
            "resource_carrying" => format!("Carrying Food: {} (Value: {:.0})",
                colony_stats.ants_carrying_food, colony_stats.total_carried_food_value),

            "environment_moisture" => format!("Soil Moisture: {:.1}% ({:.1}-{:.1})",
                colony_stats.average_soil_moisture * 100.0,
                colony_stats.min_soil_moisture * 100.0,
                colony_stats.max_soil_moisture * 100.0),
            "environment_temperature" => format!("Soil Temperature: {:.1}°C ({:.1}-{:.1})",
                colony_stats.average_soil_temperature,
                colony_stats.min_soil_temperature,
                colony_stats.max_soil_temperature),
            "environment_nutrition" => format!("Soil Nutrition: {:.1}% ({:.1}-{:.1})",
                colony_stats.average_soil_nutrition * 100.0,
                colony_stats.min_soil_nutrition * 100.0,
                colony_stats.max_soil_nutrition * 100.0),
            "environment_disasters" => format!("Active Disasters: {}",
                colony_stats.active_disasters_count),

            "behavior_activity" => colony_stats.behavior_distribution_text(),

            _ => continue,
        };

        if text.sections.len() > 0 {
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