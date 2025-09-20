use crate::components::{
    SimulationTime, SimulationTimeDisplay, TimeControl, TimeDisplayFormat, UITheme,
};
use bevy::prelude::*;

/// Setup simulation time display UI panel
pub fn setup_simulation_time_display(mut commands: Commands, theme: Res<UITheme>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(theme.spacing.md),
                right: Val::Px(theme.spacing.md + 320.0), // Position to the left of disaster control panel
                width: Val::Px(280.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(theme.spacing.md)),
                row_gap: Val::Px(theme.spacing.sm),
                border: UiRect::all(Val::Px(theme.borders.width_medium)),
                ..default()
            },
            background_color: theme.colors.surface_primary.into(),
            border_color: theme.colors.border_primary.into(),
            border_radius: BorderRadius::all(Val::Px(theme.borders.radius_medium)),
            ..default()
        })
        .with_children(|parent| {
            // Panel title
            parent.spawn(TextBundle::from_section(
                "⏰ Simulation Time",
                TextStyle {
                    font_size: theme.typography.heading_medium,
                    color: theme.colors.text_primary,
                    ..default()
                },
            ));

            // Main time display
            parent.spawn((
                TextBundle::from_section(
                    "Day 1, 06:00",
                    TextStyle {
                        font_size: theme.typography.heading_small,
                        color: theme.colors.text_accent,
                        ..default()
                    },
                ),
                SimulationTimeDisplay,
                TimeDisplayFormat {
                    show_day: true,
                    show_time_of_day: true,
                    show_elapsed_time: false,
                    show_speed_indicator: false,
                },
                Name::new("main_time_display"),
            ));

            // Elapsed time display
            parent.spawn((
                TextBundle::from_section(
                    "Runtime: 0m",
                    TextStyle {
                        font_size: theme.typography.body_medium,
                        color: theme.colors.text_secondary,
                        ..default()
                    },
                ),
                SimulationTimeDisplay,
                TimeDisplayFormat {
                    show_day: false,
                    show_time_of_day: false,
                    show_elapsed_time: true,
                    show_speed_indicator: false,
                },
                Name::new("elapsed_time_display"),
            ));

            // Speed indicator
            parent.spawn((
                TextBundle::from_section(
                    "Speed: 1.0x",
                    TextStyle {
                        font_size: theme.typography.body_medium,
                        color: theme.colors.text_muted,
                        ..default()
                    },
                ),
                SimulationTimeDisplay,
                TimeDisplayFormat {
                    show_day: false,
                    show_time_of_day: false,
                    show_elapsed_time: false,
                    show_speed_indicator: true,
                },
                Name::new("speed_indicator_display"),
            ));

            // Time of day indicator with visual elements
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(32.0),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceBetween,
                        padding: UiRect::all(Val::Px(theme.spacing.xs)),
                        margin: UiRect::vertical(Val::Px(theme.spacing.xs)),
                        border: UiRect::all(Val::Px(theme.borders.width_thin)),
                        ..default()
                    },
                    background_color: theme.colors.surface_secondary.into(),
                    border_color: theme.colors.border_secondary.into(),
                    border_radius: BorderRadius::all(Val::Px(theme.borders.radius_small)),
                    ..default()
                })
                .with_children(|time_indicator_parent| {
                    // Time period indicator (Dawn/Day/Dusk/Night)
                    time_indicator_parent.spawn((
                        TextBundle::from_section(
                            "🌅 Dawn",
                            TextStyle {
                                font_size: theme.typography.body_small,
                                color: theme.colors.text_primary,
                                ..default()
                            },
                        ),
                        SimulationTimeDisplay,
                        Name::new("time_period_display"),
                    ));

                    // Visual time progress bar
                    time_indicator_parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Px(120.0),
                                height: Val::Px(6.0),
                                border: UiRect::all(Val::Px(1.0)),
                                ..default()
                            },
                            background_color: theme.colors.border_secondary.into(),
                            border_color: theme.colors.border_primary.into(),
                            border_radius: BorderRadius::all(Val::Px(3.0)),
                            ..default()
                        })
                        .with_children(|progress_parent| {
                            // Progress fill
                            progress_parent.spawn((
                                NodeBundle {
                                    style: Style {
                                        width: Val::Percent(25.0), // Will be updated dynamically
                                        height: Val::Percent(100.0),
                                        ..default()
                                    },
                                    background_color: theme.colors.accent_orange.into(),
                                    border_radius: BorderRadius::all(Val::Px(2.0)),
                                    ..default()
                                },
                                Name::new("time_progress_fill"),
                            ));
                        });
                });

            // Simulation statistics section
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(theme.spacing.xs),
                        padding: UiRect::all(Val::Px(theme.spacing.sm)),
                        margin: UiRect::top(Val::Px(theme.spacing.sm)),
                        border: UiRect::all(Val::Px(theme.borders.width_thin)),
                        ..default()
                    },
                    background_color: theme.colors.surface_elevated.into(),
                    border_color: theme.colors.border_secondary.into(),
                    border_radius: BorderRadius::all(Val::Px(theme.borders.radius_small)),
                    ..default()
                })
                .with_children(|stats_parent| {
                    // Section title
                    stats_parent.spawn(TextBundle::from_section(
                        "📊 Time Statistics",
                        TextStyle {
                            font_size: theme.typography.body_small,
                            color: theme.colors.text_secondary,
                            ..default()
                        },
                    ));

                    // Total simulation days
                    stats_parent.spawn((
                        TextBundle::from_section(
                            "Total Days: 1",
                            TextStyle {
                                font_size: theme.typography.caption,
                                color: theme.colors.text_muted,
                                ..default()
                            },
                        ),
                        Name::new("total_days_display"),
                    ));

                    // Day progress percentage
                    stats_parent.spawn((
                        TextBundle::from_section(
                            "Day Progress: 25%",
                            TextStyle {
                                font_size: theme.typography.caption,
                                color: theme.colors.text_muted,
                                ..default()
                            },
                        ),
                        Name::new("day_progress_display"),
                    ));
                });

            // Instructions
            parent.spawn(TextBundle::from_section(
                "Time flows based on simulation speed. Pause with SPACE.",
                TextStyle {
                    font_size: theme.typography.caption,
                    color: theme.colors.text_muted,
                    ..default()
                },
            ));
        });
}

/// System to update simulation time based on time control
pub fn update_simulation_time_system(
    mut simulation_time: ResMut<SimulationTime>,
    time_control: Res<TimeControl>,
    time: Res<Time>,
) {
    simulation_time.update(time.delta_seconds(), &time_control);
}

/// System to update time display UI elements
pub fn update_time_display_system(
    simulation_time: Res<SimulationTime>,
    time_control: Res<TimeControl>,
    mut query: Query<(&mut Text, &TimeDisplayFormat, &Name), With<SimulationTimeDisplay>>,
    mut progress_fill_query: Query<&mut Style, (With<Name>, Without<SimulationTimeDisplay>)>,
    theme: Res<UITheme>,
) {
    for (mut text, format, name) in &mut query {
        match name.as_str() {
            "main_time_display" => {
                if format.show_day && format.show_time_of_day {
                    text.sections[0].value = simulation_time.format_time();
                    // Update color based on time of day
                    let time_fraction = simulation_time.get_time_of_day_fraction();
                    text.sections[0].style.color = get_time_of_day_color(time_fraction, &theme);
                }
            }
            "elapsed_time_display" => {
                if format.show_elapsed_time {
                    text.sections[0].value =
                        format!("Runtime: {}", simulation_time.format_elapsed_time());
                }
            }
            "speed_indicator_display" => {
                if format.show_speed_indicator {
                    if time_control.is_paused {
                        text.sections[0].value = "Speed: PAUSED".to_string();
                        text.sections[0].style.color = theme.colors.action_warning;
                    } else {
                        text.sections[0].value =
                            format!("Speed: {:.1}x", time_control.speed_multiplier);
                        text.sections[0].style.color =
                            get_speed_color(time_control.speed_multiplier, &theme);
                    }
                }
            }
            "time_period_display" => {
                let (period_icon, period_name) =
                    get_time_period(simulation_time.get_time_of_day_fraction());
                text.sections[0].value = format!("{} {}", period_icon, period_name);
                text.sections[0].style.color =
                    get_time_of_day_color(simulation_time.get_time_of_day_fraction(), &theme);
            }
            "total_days_display" => {
                text.sections[0].value = format!("Total Days: {}", simulation_time.current_day);
            }
            "day_progress_display" => {
                let progress = (simulation_time.get_time_of_day_fraction() * 100.0) as u8;
                text.sections[0].value = format!("Day Progress: {}%", progress);
            }
            _ => {}
        }
    }

    // Update time progress bar
    if let Ok(mut style) = progress_fill_query.get_single_mut() {
        let progress_percentage = simulation_time.get_time_of_day_fraction() * 100.0;
        style.width = Val::Percent(progress_percentage);
    }
}

/// Get color based on time of day
fn get_time_of_day_color(time_fraction: f32, theme: &UITheme) -> Color {
    match time_fraction {
        f if f < 0.25 => theme.colors.accent_blue, // Night (00:00-06:00) - Blue
        f if f < 0.5 => theme.colors.accent_orange, // Morning (06:00-12:00) - Orange
        f if f < 0.75 => theme.colors.accent_orange, // Afternoon (12:00-18:00) - Orange
        _ => theme.colors.accent_blue,             // Evening (18:00-24:00) - Blue
    }
}

/// Get time period information
fn get_time_period(time_fraction: f32) -> (&'static str, &'static str) {
    match time_fraction {
        f if f < 0.25 => ("🌙", "Night"),
        f if f < 0.5 => ("🌅", "Dawn"),
        f if f < 0.75 => ("☀️", "Day"),
        _ => ("🌆", "Dusk"),
    }
}

/// Get color based on simulation speed
fn get_speed_color(speed: f32, theme: &UITheme) -> Color {
    match speed {
        s if s <= 1.0 => theme.colors.text_muted, // Normal speed
        s if s <= 5.0 => theme.colors.text_secondary, // Moderate speed
        s if s <= 20.0 => theme.colors.text_accent, // Fast speed
        s if s <= 50.0 => theme.colors.action_warning, // Very fast speed
        _ => theme.colors.action_danger,          // Ultra fast speed
    }
}

/// System to initialize simulation time start time
pub fn initialize_simulation_time_system(
    mut simulation_time: ResMut<SimulationTime>,
    time: Res<Time>,
) {
    if simulation_time.start_time == 0.0 {
        simulation_time.start_time = time.elapsed_seconds_f64();
    }
}
