use crate::components::{
    ActiveDisasterEntry, ActiveDisastersPanel, DisasterDurationText, DisasterProgressBar,
    DisasterState, DisasterType, UITheme,
};
use bevy::prelude::*;

/// Setup active disasters display panel in the bottom-left corner with UITheme integration
pub fn setup_active_disasters_panel(mut commands: Commands, theme: Res<UITheme>) {
    // Main active disasters panel container - positioned at bottom-left with UITheme integration
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(theme.spacing.md),
                bottom: Val::Px(theme.spacing.md),
                width: Val::Px(320.0), // Keep fixed width for disaster entries
                max_height: Val::Px(250.0), // More height for improved spacing
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(theme.spacing.md)),
                row_gap: Val::Px(theme.spacing.sm),
                border: UiRect::all(Val::Px(theme.borders.width_medium)),
                ..default()
            },
            background_color: theme.colors.surface_primary.into(),
            border_color: theme.colors.action_warning.into(), // Use warning color for disaster context
            border_radius: BorderRadius::all(Val::Px(theme.borders.radius_medium)),
            visibility: Visibility::Hidden, // Initially hidden, shown when disasters are active
            ..default()
        })
        .with_children(|parent| {
            // Enhanced panel title with icon and UITheme integration
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(theme.spacing.xs),
                        margin: UiRect::bottom(Val::Px(theme.spacing.xs)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|title_parent| {
                    // Warning icon with theme color
                    title_parent.spawn(TextBundle::from_section(
                        "⚠️",
                        TextStyle {
                            font_size: theme.typography.heading_small,
                            color: theme.colors.action_warning,
                            ..default()
                        },
                    ));

                    // Panel title with theme typography
                    title_parent.spawn(TextBundle::from_section(
                        "Active Disasters",
                        TextStyle {
                            font_size: theme.typography.heading_medium,
                            color: theme.colors.text_primary,
                            ..default()
                        },
                    ));
                });
        })
        .insert(ActiveDisastersPanel);
}

/// Update the active disasters display based on current disaster state with UITheme integration
pub fn update_active_disasters_display(
    mut commands: Commands,
    disaster_state: Res<DisasterState>,
    theme: Res<UITheme>,
    mut panel_query: Query<(Entity, &mut Visibility), With<ActiveDisastersPanel>>,
    entry_query: Query<Entity, With<ActiveDisasterEntry>>,
) {
    let Ok((panel_entity, mut panel_visibility)) = panel_query.get_single_mut() else {
        return;
    };

    // Remove all existing disaster entries
    for entry_entity in entry_query.iter() {
        commands.entity(entry_entity).despawn_recursive();
    }

    // Check if any disasters are active
    if disaster_state.active_disasters.is_empty() {
        *panel_visibility = Visibility::Hidden;
        return;
    }

    // Show panel and add active disasters
    *panel_visibility = Visibility::Visible;

    commands.entity(panel_entity).with_children(|parent| {
        for (disaster_type, remaining_time) in disaster_state.active_disasters.iter() {
            create_disaster_entry(parent, *disaster_type, *remaining_time, &theme);
        }
    });
}

/// Create a single disaster entry in the active disasters panel with UITheme integration
fn create_disaster_entry(
    parent: &mut ChildBuilder,
    disaster_type: DisasterType,
    remaining_time: f32,
    theme: &UITheme,
) {
    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(theme.spacing.sm)),
                margin: UiRect::bottom(Val::Px(theme.spacing.xs)),
                border: UiRect::all(Val::Px(theme.borders.width_thin)),
                ..default()
            },
            background_color: theme.colors.surface_secondary.into(),
            border_color: disaster_type.get_active_color().with_alpha(0.4).into(), // Colored border with theme alpha
            border_radius: BorderRadius::all(Val::Px(theme.borders.radius_small)),
            ..default()
        })
        .with_children(|disaster_parent| {
            // Disaster name and time row with UITheme spacing
            disaster_parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(theme.spacing.xs)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|header_parent| {
                    // Disaster icon and name container with theme spacing
                    header_parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                column_gap: Val::Px(theme.spacing.xs),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|name_container| {
                            // Disaster icon with theme typography
                            name_container.spawn(TextBundle::from_section(
                                disaster_type.get_icon(),
                                TextStyle {
                                    font_size: theme.typography.heading_small,
                                    color: theme.colors.text_primary,
                                    ..default()
                                },
                            ));

                            // Disaster name with disaster color and theme typography
                            name_container.spawn(TextBundle::from_section(
                                disaster_type.display_name(),
                                TextStyle {
                                    font_size: theme.typography.body_large,
                                    color: disaster_type.get_active_color(),
                                    ..default()
                                },
                            ));
                        });

                    // Enhanced duration text with visual emphasis and UITheme integration
                    header_parent
                        .spawn(NodeBundle {
                            style: Style {
                                padding: UiRect::all(Val::Px(theme.spacing.xs)),
                                ..default()
                            },
                            background_color: theme.colors.surface_elevated.into(),
                            border_radius: BorderRadius::all(Val::Px(theme.borders.radius_small)),
                            ..default()
                        })
                        .with_children(|duration_parent| {
                            duration_parent.spawn((
                                TextBundle::from_section(
                                    format_duration(remaining_time),
                                    TextStyle {
                                        font_size: theme.typography.body_small,
                                        color: get_duration_color(remaining_time, theme),
                                        ..default()
                                    },
                                ),
                                DisasterDurationText { disaster_type },
                            ));
                        });
                });

            // Enhanced progress bar with UITheme integration
            disaster_parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(14.0), // Slightly taller for better visibility
                        border: UiRect::all(Val::Px(theme.borders.width_thin)),
                        ..default()
                    },
                    background_color: theme.colors.surface_elevated.into(),
                    border_color: theme.colors.border_secondary.into(),
                    border_radius: BorderRadius::all(Val::Px(theme.borders.radius_small)),
                    ..default()
                })
                .with_children(|progress_parent| {
                    // Progress bar fill with enhanced styling and theme integration
                    progress_parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0), // Will be updated dynamically
                                height: Val::Percent(100.0),
                                margin: UiRect::all(Val::Px(theme.borders.width_thin)),
                                ..default()
                            },
                            background_color: disaster_type
                                .get_active_color()
                                .with_alpha(0.8)
                                .into(),
                            border_radius: BorderRadius::all(Val::Px(theme.borders.radius_small)),
                            ..default()
                        },
                        DisasterProgressBar {
                            disaster_type,
                            max_duration: get_disaster_max_duration(disaster_type),
                        },
                    ));
                });
        })
        .insert(ActiveDisasterEntry { disaster_type });
}

/// Update progress bars based on current disaster timers
pub fn update_disaster_progress_bars(
    disaster_state: Res<DisasterState>,
    mut progress_query: Query<(&DisasterProgressBar, &mut Style)>,
) {
    for (progress_bar, mut style) in progress_query.iter_mut() {
        if let Some(remaining_time) = disaster_state.get_remaining_time(progress_bar.disaster_type)
        {
            let progress_ratio = remaining_time / progress_bar.max_duration;
            let progress_percentage = (progress_ratio * 100.0).clamp(0.0, 100.0);
            style.width = Val::Percent(progress_percentage);
        }
    }
}

/// Update duration text displays with enhanced formatting and theme integration
pub fn update_disaster_duration_text(
    disaster_state: Res<DisasterState>,
    theme: Res<UITheme>,
    mut text_query: Query<(&DisasterDurationText, &mut Text)>,
) {
    for (duration_text, mut text) in text_query.iter_mut() {
        if let Some(remaining_time) = disaster_state.get_remaining_time(duration_text.disaster_type)
        {
            // Update both text value and color based on remaining time
            text.sections[0].value = format_duration(remaining_time);
            text.sections[0].style.color = get_duration_color(remaining_time, &theme);
        }
    }
}

/// Helper function to get the maximum duration for each disaster type
/// This should match the durations set in the disaster input system
fn get_disaster_max_duration(disaster_type: DisasterType) -> f32 {
    match disaster_type {
        DisasterType::Rain => 20.0,
        DisasterType::Drought => 45.0,
        DisasterType::ColdSnap => 30.0,
        DisasterType::InvasiveSpecies => 60.0,
    }
}

/// Format duration for human-readable display
fn format_duration(remaining_time: f32) -> String {
    if remaining_time < 10.0 {
        // Show "Ending soon!" for < 10 seconds
        "Ending soon!".to_string()
    } else if remaining_time < 60.0 {
        // Show seconds for < 1 minute
        format!("{}s", remaining_time.round() as u32)
    } else {
        // Show minutes and seconds for >= 1 minute
        let minutes = (remaining_time / 60.0).floor() as u32;
        let seconds = (remaining_time % 60.0).round() as u32;
        if seconds == 0 {
            format!("{}m", minutes)
        } else {
            format!("{}m {}s", minutes, seconds)
        }
    }
}

/// Get color for duration text based on remaining time and theme
fn get_duration_color(remaining_time: f32, theme: &UITheme) -> Color {
    if remaining_time < 10.0 {
        // Red for ending soon
        theme.colors.action_danger
    } else if remaining_time < 30.0 {
        // Yellow/orange for moderate time
        theme.colors.action_warning
    } else {
        // Normal color for plenty of time
        theme.colors.text_secondary
    }
}
