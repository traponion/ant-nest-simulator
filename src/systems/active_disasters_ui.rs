use crate::components::{
    ActiveDisasterEntry, ActiveDisastersPanel, DisasterDurationText, DisasterProgressBar,
    DisasterState, DisasterType,
};
use bevy::prelude::*;

/// Setup active disasters display panel in the bottom-left corner
pub fn setup_active_disasters_panel(mut commands: Commands) {
    // Main active disasters panel container - positioned at bottom-left
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                bottom: Val::Px(10.0),
                width: Val::Px(320.0), // Slightly wider for better content fit
                max_height: Val::Px(250.0), // More height for improved spacing
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(16.0)), // More padding
                row_gap: Val::Px(10.0),              // More spacing between items
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: Color::srgba(0.05, 0.05, 0.1, 0.92).into(), // Slightly bluer dark background
            border_color: Color::srgba(0.8, 0.4, 0.2, 0.6).into(),        // Warm orange border
            border_radius: BorderRadius::all(Val::Px(10.0)),              // More rounded corners
            visibility: Visibility::Hidden, // Initially hidden, shown when disasters are active
            ..default()
        })
        .with_children(|parent| {
            // Enhanced panel title with icon
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(8.0),
                        margin: UiRect::bottom(Val::Px(4.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|title_parent| {
                    // Warning icon
                    title_parent.spawn(TextBundle::from_section(
                        "⚠️",
                        TextStyle {
                            font_size: 20.0,
                            color: Color::srgb(1.0, 0.8, 0.2),
                            ..default()
                        },
                    ));

                    // Panel title
                    title_parent.spawn(TextBundle::from_section(
                        "Active Disasters",
                        TextStyle {
                            font_size: 18.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
        })
        .insert(ActiveDisastersPanel);
}

/// Update the active disasters display based on current disaster state
pub fn update_active_disasters_display(
    mut commands: Commands,
    disaster_state: Res<DisasterState>,
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
            create_disaster_entry(parent, *disaster_type, *remaining_time);
        }
    });
}

/// Create a single disaster entry in the active disasters panel
fn create_disaster_entry(
    parent: &mut ChildBuilder,
    disaster_type: DisasterType,
    remaining_time: f32,
) {
    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(12.0)), // More padding for better spacing
                margin: UiRect::bottom(Val::Px(6.0)), // More space between entries
                border: UiRect::all(Val::Px(1.0)),   // Add border for better definition
                ..default()
            },
            background_color: Color::srgba(0.15, 0.15, 0.15, 0.85).into(), // Slightly darker background
            border_color: disaster_type.get_active_color().with_alpha(0.3).into(), // Colored border
            border_radius: BorderRadius::all(Val::Px(6.0)),                // Slightly more rounded
            ..default()
        })
        .with_children(|disaster_parent| {
            // Disaster name and time row
            disaster_parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(4.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|header_parent| {
                    // Disaster icon and name container
                    header_parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                column_gap: Val::Px(8.0),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|name_container| {
                            // Disaster icon
                            name_container.spawn(TextBundle::from_section(
                                disaster_type.get_icon(),
                                TextStyle {
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                    ..default()
                                },
                            ));

                            // Disaster name with color
                            name_container.spawn(TextBundle::from_section(
                                disaster_type.display_name(),
                                TextStyle {
                                    font_size: 16.0,
                                    color: disaster_type.get_active_color(),
                                    ..default()
                                },
                            ));
                        });

                    // Enhanced duration text with visual emphasis
                    header_parent
                        .spawn(NodeBundle {
                            style: Style {
                                padding: UiRect::all(Val::Px(4.0)),
                                ..default()
                            },
                            background_color: Color::srgba(0.0, 0.0, 0.0, 0.4).into(),
                            border_radius: BorderRadius::all(Val::Px(4.0)),
                            ..default()
                        })
                        .with_children(|duration_parent| {
                            duration_parent.spawn((
                                TextBundle::from_section(
                                    format!("{:.1}s", remaining_time),
                                    TextStyle {
                                        font_size: 14.0,
                                        color: Color::srgb(1.0, 1.0, 0.8), // Slightly warm white
                                        ..default()
                                    },
                                ),
                                DisasterDurationText { disaster_type },
                            ));
                        });
                });

            // Enhanced progress bar with better visual design
            disaster_parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(12.0), // Slightly taller for better visibility
                        border: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    background_color: Color::srgba(0.1, 0.1, 0.1, 0.9).into(), // Darker background
                    border_color: Color::srgba(0.4, 0.4, 0.4, 0.6).into(),
                    border_radius: BorderRadius::all(Val::Px(6.0)),
                    ..default()
                })
                .with_children(|progress_parent| {
                    // Progress bar fill with enhanced styling
                    progress_parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0), // Will be updated dynamically
                                height: Val::Percent(100.0),
                                margin: UiRect::all(Val::Px(1.0)), // Small margin for border effect
                                ..default()
                            },
                            background_color: disaster_type
                                .get_active_color()
                                .with_alpha(0.8)
                                .into(),
                            border_radius: BorderRadius::all(Val::Px(4.0)),
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

/// Update duration text displays
pub fn update_disaster_duration_text(
    disaster_state: Res<DisasterState>,
    mut text_query: Query<(&DisasterDurationText, &mut Text)>,
) {
    for (duration_text, mut text) in text_query.iter_mut() {
        if let Some(remaining_time) = disaster_state.get_remaining_time(duration_text.disaster_type)
        {
            text.sections[0].value = format!("{:.1}s", remaining_time);
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
