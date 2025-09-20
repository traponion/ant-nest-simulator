use crate::components::{
    CooldownTimer, DisasterControlButton, DisasterControlPanel, DisasterState, DisasterStatusIndicator,
    DisasterStatusBackground, DisasterTriggerFeedback, DisasterType, DisasterCooldownProgressBar, UITheme,
    Tooltip, TooltipTrigger, TooltipPosition,
};
use bevy::prelude::*;

/// Setup disaster control panel UI
pub fn setup_disaster_control_panel(mut commands: Commands) {
    // Main disaster control panel container
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Px(10.0),
                top: Val::Px(10.0),
                width: Val::Px(300.0), // Increased width to match Time Control UI
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(15.0)),
                row_gap: Val::Px(12.0), // Increased spacing to match Time Control UI
                border: UiRect::all(Val::Px(2.0)), // Added border like Time Control UI
                ..default()
            },
            background_color: Color::srgba(0.1, 0.1, 0.1, 0.9).into(), // Consistent with Time Control UI
            border_color: Color::srgb(0.3, 0.3, 0.3).into(), // Consistent border color
            border_radius: BorderRadius::all(Val::Px(8.0)),
            ..default()
        })
        .with_children(|parent| {
            // Panel title
            parent.spawn(TextBundle::from_section(
                "Disaster Controls",
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));

            // Create controls for each disaster type
            let disasters = [
                DisasterType::Rain,
                DisasterType::Drought,
                DisasterType::ColdSnap,
                DisasterType::InvasiveSpecies,
            ];

            for disaster_type in disasters.iter() {
                // Container for each disaster control (interactive button)
                parent
                    .spawn(ButtonBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            padding: UiRect::all(Val::Px(12.0)), // Increased padding like Time Control UI
                            row_gap: Val::Px(8.0), // Increased spacing
                            border: UiRect::all(Val::Px(2.0)), // 2px border like Time Control UI
                            margin: UiRect::bottom(Val::Px(6.0)), // Added margin for better spacing
                            ..default()
                        },
                        background_color: Color::srgba(0.15, 0.15, 0.15, 0.85).into(), // Enhanced consistent color
                        border_color: Color::srgb(0.4, 0.4, 0.4).into(), // Brighter border for better contrast
                        border_radius: BorderRadius::all(Val::Px(8.0)), // 8px radius like Time Control UI
                        ..default()
                    })
                    .with_children(|disaster_parent| {
                        // Disaster name and key
                        disaster_parent
                            .spawn(NodeBundle {
                                style: Style {
                                    flex_direction: FlexDirection::Row,
                                    justify_content: JustifyContent::SpaceBetween,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|header_parent| {
                                // Icon and disaster name container
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
                                    .with_children(|name_parent| {
                                        // Disaster icon with enhanced styling
                                        name_parent
                                            .spawn(NodeBundle {
                                                style: Style {
                                                    width: Val::Px(32.0),
                                                    height: Val::Px(32.0),
                                                    justify_content: JustifyContent::Center,
                                                    align_items: AlignItems::Center,
                                                    ..default()
                                                },
                                                background_color: Color::srgba(1.0, 1.0, 1.0, 0.1).into(), // Subtle icon background
                                                border_radius: BorderRadius::all(Val::Px(16.0)), // Circular background
                                                ..default()
                                            })
                                            .with_children(|icon_container| {
                                                icon_container.spawn(TextBundle::from_section(
                                                    disaster_type.get_icon(),
                                                    TextStyle {
                                                        font_size: 24.0, // Larger icon for better visibility
                                                        color: Color::WHITE,
                                                        ..default()
                                                    },
                                                ));
                                            });

                                        // Disaster name with enhanced typography
                                        name_parent.spawn(TextBundle::from_section(
                                            disaster_type.display_name(),
                                            TextStyle {
                                                font_size: 18.0, // Slightly larger for better readability
                                                color: Color::WHITE,
                                                ..default()
                                            },
                                        ));
                                    });

                                // Keyboard shortcut with enhanced styling (like Time Control UI)
                                header_parent
                                    .spawn(NodeBundle {
                                        style: Style {
                                            padding: UiRect::all(Val::Px(4.0)),
                                            ..default()
                                        },
                                        background_color: Color::srgba(0.0, 0.0, 0.0, 0.3).into(), // Same as Time Control UI
                                        border_radius: BorderRadius::all(Val::Px(4.0)),
                                        ..default()
                                    })
                                    .with_children(|shortcut_parent| {
                                        shortcut_parent.spawn(TextBundle::from_section(
                                            disaster_type.shortcut_key(),
                                            TextStyle {
                                                font_size: 12.0,
                                                color: Color::srgb(0.9, 0.9, 0.9), // Slightly brighter
                                                ..default()
                                            },
                                        ));
                                    });
                            });

                        // Status and timer row with enhanced styling
                        disaster_parent
                            .spawn(NodeBundle {
                                style: Style {
                                    flex_direction: FlexDirection::Row,
                                    justify_content: JustifyContent::SpaceBetween,
                                    align_items: AlignItems::Center,
                                    padding: UiRect::all(Val::Px(8.0)), // Increased padding
                                    margin: UiRect::top(Val::Px(4.0)), // Added top margin for separation
                                    ..default()
                                },
                                background_color: Color::srgba(0.05, 0.05, 0.05, 0.6).into(), // Slightly darker for better contrast
                                border_radius: BorderRadius::all(Val::Px(6.0)), // Larger radius for consistency
                                ..default()
                            })
                            .with_children(|status_parent| {
                                // Enhanced visual status indicator with background
                                status_parent
                                    .spawn((
                                        NodeBundle {
                                            style: Style {
                                                padding: UiRect::all(Val::Px(8.0)), // Increased padding
                                                border: UiRect::all(Val::Px(1.0)), // Added border for definition
                                                ..default()
                                            },
                                            background_color: Color::srgb(0.2, 0.8, 0.2).into(), // Enhanced available color
                                            border_color: Color::srgba(1.0, 1.0, 1.0, 0.2).into(), // Subtle white border
                                            border_radius: BorderRadius::all(Val::Px(8.0)), // Consistent with panel styling
                                            ..default()
                                        },
                                        DisasterStatusBackground {
                                            disaster_type: *disaster_type,
                                        },
                                    ))
                                    .with_children(|indicator_parent| {
                                        indicator_parent.spawn((
                                            TextBundle::from_section(
                                                "Available",
                                                TextStyle {
                                                    font_size: 12.0,
                                                    color: Color::BLACK,
                                                    ..default()
                                                },
                                            ),
                                            DisasterStatusIndicator {
                                                disaster_type: *disaster_type,
                                            },
                                        ));
                                    });

                                // Cooldown timer
                                status_parent.spawn((
                                    TextBundle::from_section(
                                        "",
                                        TextStyle {
                                            font_size: 12.0,
                                            color: Color::srgb(1.0, 0.6, 0.0),
                                            ..default()
                                        },
                                    ),
                                    CooldownTimer {
                                        disaster_type: *disaster_type,
                                    },
                                ));
                            });

                        // Enhanced cooldown progress bar (only visible during cooldown)
                        disaster_parent
                            .spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Px(8.0), // Slightly thicker for better visibility
                                    margin: UiRect::top(Val::Px(8.0)), // More spacing for separation
                                    border: UiRect::all(Val::Px(1.0)), // Added border like Time Control UI
                                    ..default()
                                },
                                background_color: Color::srgba(0.2, 0.2, 0.2, 0.8).into(), // Darker background for contrast
                                border_color: Color::srgba(0.4, 0.4, 0.4, 0.6).into(), // Subtle border
                                border_radius: BorderRadius::all(Val::Px(4.0)), // Consistent radius
                                visibility: Visibility::Hidden, // Initially hidden, shown during cooldown
                                ..default()
                            })
                            .with_children(|progress_parent| {
                                // Enhanced progress bar fill with better styling
                                progress_parent.spawn((
                                    NodeBundle {
                                        style: Style {
                                            width: Val::Percent(0.0), // Will be updated dynamically
                                            height: Val::Percent(100.0),
                                            ..default()
                                        },
                                        background_color: Color::srgba(0.8, 0.5, 0.2, 0.9).into(), // Enhanced orange for cooldown
                                        border_radius: BorderRadius::all(Val::Px(4.0)), // Consistent radius
                                        ..default()
                                    },
                                    DisasterCooldownProgressBar {
                                        disaster_type: *disaster_type,
                                        max_cooldown: get_disaster_cooldown_duration(*disaster_type),
                                    },
                                ));
                            });
                    })
                    .insert(DisasterControlButton {
                        disaster_type: *disaster_type,
                    });
            }

            // Enhanced instructions
            parent.spawn(TextBundle::from_section(
                "🎮 Press the keys to trigger disasters\n🟢 Available  🟠 Cooldown  🔴 Active\n\n🌧️ Rain (R)  ☀️ Drought (D)\n❄️ Cold Snap (C)  🐛 Invasive Species (I)",
                TextStyle {
                    font_size: 11.0,
                    color: Color::srgb(0.8, 0.8, 0.8),
                    ..default()
                },
            ));
        })
        .insert(DisasterControlPanel);
}

/// Update disaster status displays in real-time
pub fn update_disaster_status_system(
    disaster_state: Res<DisasterState>,
    mut status_query: Query<(&DisasterStatusIndicator, &mut Text)>,
    mut background_query: Query<(&DisasterStatusBackground, &mut BackgroundColor)>,
) {
    // Update status text
    for (status_indicator, mut text) in status_query.iter_mut() {
        let disaster_type = status_indicator.disaster_type;

        let (status_text, text_color) = if disaster_state.is_active(disaster_type) {
            ("ACTIVE", Color::WHITE)
        } else if disaster_state.is_on_cooldown(disaster_type) {
            ("Cooldown", Color::WHITE)
        } else {
            ("Available", Color::BLACK)
        };

        text.sections[0].value = status_text.to_string();
        text.sections[0].style.color = text_color;
    }

    // Update background colors
    for (status_background, mut background_color) in background_query.iter_mut() {
        let disaster_type = status_background.disaster_type;

        let bg_color = if disaster_state.is_active(disaster_type) {
            Color::srgb(0.8, 0.2, 0.2) // Enhanced red for active (more professional)
        } else if disaster_state.is_on_cooldown(disaster_type) {
            Color::srgb(0.8, 0.5, 0.2) // Enhanced orange for cooldown (consistent with progress bar)
        } else {
            Color::srgb(0.2, 0.8, 0.2) // Enhanced green for available (consistent with new styling)
        };

        background_color.0 = bg_color;
    }
}

/// Update cooldown timer displays
pub fn update_cooldown_timers_system(
    disaster_state: Res<DisasterState>,
    mut timer_query: Query<(&CooldownTimer, &mut Text)>,
) {
    for (cooldown_timer, mut text) in timer_query.iter_mut() {
        let disaster_type = cooldown_timer.disaster_type;

        if disaster_state.is_on_cooldown(disaster_type) {
            if let Some(cooldown_time) = disaster_state.cooldown_timers.get(&disaster_type) {
                if *cooldown_time > 0.0 {
                    text.sections[0].value = format!("{:.1}s", cooldown_time);
                } else {
                    text.sections[0].value = "".to_string();
                }
            }
        } else if disaster_state.is_active(disaster_type) {
            if let Some(remaining_time) = disaster_state.get_remaining_time(disaster_type) {
                text.sections[0].value = format!("Active: {:.1}s", remaining_time);
                text.sections[0].style.color = Color::srgb(1.0, 0.3, 0.3);
            }
        } else {
            text.sections[0].value = "".to_string();
        }
    }
}

/// Handle visual feedback when disasters are triggered
pub fn disaster_trigger_feedback_system(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    _disaster_state: Res<DisasterState>,
    mut feedback_query: Query<(Entity, &mut DisasterTriggerFeedback, &mut BackgroundColor)>,
    time: Res<Time>,
) {
    // Update existing feedback timers
    for (entity, mut feedback, mut background_color) in feedback_query.iter_mut() {
        feedback.fade_timer -= time.delta_seconds();

        // Fade out effect
        let alpha = (feedback.fade_timer / 0.5).max(0.0);
        background_color.0 = Color::srgba(1.0, 1.0, 0.0, alpha * 0.3);

        // Remove expired feedback
        if feedback.fade_timer <= 0.0 {
            commands.entity(entity).despawn();
        }
    }

    // Check for new key presses and create feedback
    let key_mappings = [
        (KeyCode::KeyR, DisasterType::Rain),
        (KeyCode::KeyD, DisasterType::Drought),
        (KeyCode::KeyC, DisasterType::ColdSnap),
        (KeyCode::KeyI, DisasterType::InvasiveSpecies),
    ];

    for (key, disaster_type) in key_mappings.iter() {
        if input.just_pressed(*key) {
            // Create visual feedback overlay
            commands.spawn((
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: Color::srgba(1.0, 1.0, 0.0, 0.3).into(),
                    ..default()
                },
                DisasterTriggerFeedback {
                    disaster_type: *disaster_type,
                    fade_timer: 0.5, // Fade out over 0.5 seconds
                },
            ));
        }
    }
}

/// Handle hover effects for disaster control buttons
pub fn handle_disaster_control_interactions(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor, &DisasterControlButton),
        Changed<Interaction>,
    >,
    disaster_state: Res<DisasterState>,
) {
    for (interaction, mut background_color, mut border_color, button) in &mut interaction_query {
        let disaster_type = button.disaster_type;

        match *interaction {
            Interaction::Pressed => {
                // Trigger disaster (this will be handled by existing keyboard input system)
                // Just provide visual feedback for the press
                *background_color = Color::srgba(0.4, 0.4, 0.4, 0.8).into();
                *border_color = Color::srgba(0.6, 0.6, 0.6, 0.8).into();
            }
            Interaction::Hovered => {
                // Enhanced hover effect with better visual feedback (matching Time Control UI style)
                if disaster_state.is_active(disaster_type) {
                    *background_color = Color::srgba(0.2, 0.2, 0.2, 0.9).into(); // Slightly lighter for active
                    *border_color = Color::srgba(0.6, 0.6, 0.6, 0.9).into(); // Brighter border
                } else if disaster_state.is_on_cooldown(disaster_type) {
                    *background_color = Color::srgba(0.2, 0.2, 0.2, 0.9).into(); // Slightly lighter for cooldown
                    *border_color = Color::srgba(0.6, 0.6, 0.6, 0.9).into(); // Brighter border
                } else {
                    // Available - more prominent hover effect with green tint
                    *background_color = Color::srgba(0.18, 0.22, 0.18, 0.9).into(); // Subtle green tint
                    *border_color = Color::srgba(0.5, 0.7, 0.5, 1.0).into(); // Green-tinted border
                }
            }
            Interaction::None => {
                // Reset to enhanced normal colors (consistent with new design)
                *background_color = Color::srgba(0.15, 0.15, 0.15, 0.85).into(); // Match new button background
                *border_color = Color::srgb(0.4, 0.4, 0.4).into(); // Match new border color
            }
        }
    }
}

/// Update cooldown progress bars based on current cooldown timers
pub fn update_cooldown_progress_bars_system(
    disaster_state: Res<DisasterState>,
    mut progress_query: Query<(&DisasterCooldownProgressBar, &mut Style)>,
) {
    for (progress_bar, mut style) in progress_query.iter_mut() {
        let disaster_type = progress_bar.disaster_type;

        if disaster_state.is_on_cooldown(disaster_type) {
            if let Some(cooldown_time) = disaster_state.cooldown_timers.get(&disaster_type) {
                if *cooldown_time > 0.0 {
                    // Calculate progress ratio (inverted for cooldown - starts at 100% and goes to 0%)
                    let progress_ratio = *cooldown_time / progress_bar.max_cooldown;
                    let progress_percentage = (progress_ratio * 100.0).clamp(0.0, 100.0);
                    style.width = Val::Percent(progress_percentage);
                } else {
                    // Cooldown finished, hide progress bar
                    style.width = Val::Percent(0.0);
                }
            }
        } else {
            // Not on cooldown, hide progress bar
            style.width = Val::Percent(0.0);
        }
    }
}

/// Helper function to get the cooldown duration for each disaster type
/// This should match the cooldown durations set in the disaster system
fn get_disaster_cooldown_duration(disaster_type: DisasterType) -> f32 {
    match disaster_type {
        DisasterType::Rain => 10.0,
        DisasterType::Drought => 15.0,
        DisasterType::ColdSnap => 12.0,
        DisasterType::InvasiveSpecies => 20.0,
    }
}

/// Get detailed description for disaster type tooltip
fn get_disaster_description(disaster_type: DisasterType) -> String {
    match disaster_type {
        DisasterType::Rain => "Heavy rainfall increases soil moisture and can help plants grow, but may flood tunnels and slow ant movement temporarily.".to_string(),
        DisasterType::Drought => "Extended dry period reduces soil moisture and food availability, making foraging more challenging for the colony.".to_string(),
        DisasterType::ColdSnap => "Sudden temperature drop slows ant metabolism and movement, reducing overall colony activity until conditions improve.".to_string(),
        DisasterType::InvasiveSpecies => "Foreign insects invade the territory, competing for food sources and potentially attacking colony members.".to_string(),
    }
}

/// Handle disaster control button animations and interactions with UITheme integration
pub fn handle_disaster_control_button_interactions(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &DisasterControlButton),
        (Changed<Interaction>, With<Button>),
    >,
    theme: Res<UITheme>,
) {
    for (interaction, mut background_color, _button) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                *background_color = theme.get_hover_color(theme.colors.surface_elevated).into();
            }
            Interaction::Pressed => {
                *background_color = theme.get_active_color(theme.colors.surface_elevated).into();
            }
            Interaction::None => {
                *background_color = theme.colors.surface_elevated.into();
            }
        }
    }
}

/// Setup enhanced disaster control panel UI with UITheme integration (Phase 1)
pub fn setup_enhanced_disaster_control_ui_v3(mut commands: Commands, theme: Res<UITheme>) {
    // Main disaster control panel container with UITheme
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Px(theme.spacing.md),
                top: Val::Px(theme.spacing.md),
                width: Val::Px(320.0), // Slightly wider for better layout
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(theme.spacing.lg)),
                row_gap: Val::Px(theme.spacing.md),
                border: UiRect::all(Val::Px(theme.borders.width_medium)),
                ..default()
            },
            background_color: theme.colors.surface_primary.into(),
            border_color: theme.colors.border_primary.into(),
            border_radius: BorderRadius::all(Val::Px(theme.borders.radius_medium)),
            ..default()
        })
        .with_children(|parent| {
            // Panel title with UITheme typography
            parent.spawn(TextBundle::from_section(
                "Disaster Controls",
                TextStyle {
                    font_size: theme.typography.heading_medium,
                    color: theme.colors.text_primary,
                    ..default()
                },
            ));

            // Create controls for each disaster type
            let disasters = [
                DisasterType::Rain,
                DisasterType::Drought,
                DisasterType::ColdSnap,
                DisasterType::InvasiveSpecies,
            ];

            for disaster_type in disasters.iter() {
                // Container for each disaster control (interactive button)
                parent
                    .spawn(ButtonBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            padding: UiRect::all(Val::Px(theme.spacing.md)),
                            row_gap: Val::Px(theme.spacing.sm),
                            border: UiRect::all(Val::Px(theme.borders.width_medium)),
                            margin: UiRect::bottom(Val::Px(theme.spacing.sm)),
                            min_height: Val::Px(48.0), // Touch-friendly minimum height
                            ..default()
                        },
                        background_color: theme.colors.surface_elevated.into(),
                        border_color: theme.colors.border_secondary.into(),
                        border_radius: BorderRadius::all(Val::Px(theme.borders.radius_medium)),
                        ..default()
                    })
                    .with_children(|disaster_parent| {
                        // Disaster name and key header
                        disaster_parent
                            .spawn(NodeBundle {
                                style: Style {
                                    flex_direction: FlexDirection::Row,
                                    justify_content: JustifyContent::SpaceBetween,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|header_parent| {
                                // Icon and disaster name container
                                header_parent
                                    .spawn(NodeBundle {
                                        style: Style {
                                            flex_direction: FlexDirection::Row,
                                            align_items: AlignItems::Center,
                                            column_gap: Val::Px(theme.spacing.sm),
                                            ..default()
                                        },
                                        ..default()
                                    })
                                    .with_children(|name_parent| {
                                        // Disaster icon with UITheme styling
                                        name_parent
                                            .spawn(NodeBundle {
                                                style: Style {
                                                    width: Val::Px(theme.spacing.xxl),
                                                    height: Val::Px(theme.spacing.xxl),
                                                    justify_content: JustifyContent::Center,
                                                    align_items: AlignItems::Center,
                                                    ..default()
                                                },
                                                background_color: theme.colors.surface_secondary.into(),
                                                border_radius: BorderRadius::all(Val::Px(theme.borders.radius_round)),
                                                ..default()
                                            })
                                            .with_children(|icon_container| {
                                                icon_container.spawn(TextBundle::from_section(
                                                    disaster_type.get_icon(),
                                                    TextStyle {
                                                        font_size: theme.typography.heading_small,
                                                        color: theme.colors.text_primary,
                                                        ..default()
                                                    },
                                                ));
                                            });

                                        // Disaster name with UITheme typography
                                        name_parent.spawn(TextBundle::from_section(
                                            disaster_type.display_name(),
                                            TextStyle {
                                                font_size: theme.typography.body_large,
                                                color: theme.colors.text_primary,
                                                ..default()
                                            },
                                        ));
                                    });

                                // Keyboard shortcut with UITheme styling
                                header_parent
                                    .spawn(NodeBundle {
                                        style: Style {
                                            padding: UiRect::all(Val::Px(theme.spacing.xs)),
                                            ..default()
                                        },
                                        background_color: theme.colors.surface_secondary.into(),
                                        border_radius: BorderRadius::all(Val::Px(theme.borders.radius_small)),
                                        ..default()
                                    })
                                    .with_children(|shortcut_parent| {
                                        shortcut_parent.spawn(TextBundle::from_section(
                                            disaster_type.shortcut_key(),
                                            TextStyle {
                                                font_size: theme.typography.body_small,
                                                color: theme.colors.text_secondary,
                                                ..default()
                                            },
                                        ));
                                    });
                            });

                        // Status and timer row with UITheme styling
                        disaster_parent
                            .spawn(NodeBundle {
                                style: Style {
                                    flex_direction: FlexDirection::Row,
                                    justify_content: JustifyContent::SpaceBetween,
                                    align_items: AlignItems::Center,
                                    padding: UiRect::all(Val::Px(theme.spacing.sm)),
                                    margin: UiRect::top(Val::Px(theme.spacing.xs)),
                                    ..default()
                                },
                                background_color: theme.colors.surface_secondary.into(),
                                border_radius: BorderRadius::all(Val::Px(theme.borders.radius_small)),
                                ..default()
                            })
                            .with_children(|status_parent| {
                                // Status indicator with UITheme colors
                                status_parent
                                    .spawn((
                                        NodeBundle {
                                            style: Style {
                                                padding: UiRect::all(Val::Px(theme.spacing.sm)),
                                                border: UiRect::all(Val::Px(theme.borders.width_thin)),
                                                ..default()
                                            },
                                            background_color: theme.colors.accent_green.into(), // Default: Available
                                            border_color: theme.colors.border_secondary.into(),
                                            border_radius: BorderRadius::all(Val::Px(theme.borders.radius_medium)),
                                            ..default()
                                        },
                                        DisasterStatusBackground {
                                            disaster_type: *disaster_type,
                                        },
                                    ))
                                    .with_children(|indicator_parent| {
                                        indicator_parent.spawn((
                                            TextBundle::from_section(
                                                "Available",
                                                TextStyle {
                                                    font_size: theme.typography.body_small,
                                                    color: theme.colors.text_primary,
                                                    ..default()
                                                },
                                            ),
                                            DisasterStatusIndicator {
                                                disaster_type: *disaster_type,
                                            },
                                        ));
                                    });

                                // Cooldown timer with UITheme typography
                                status_parent.spawn((
                                    TextBundle::from_section(
                                        "",
                                        TextStyle {
                                            font_size: theme.typography.body_small,
                                            color: theme.colors.accent_orange,
                                            ..default()
                                        },
                                    ),
                                    CooldownTimer {
                                        disaster_type: *disaster_type,
                                    },
                                ));
                            });

                        // Cooldown progress bar with UITheme styling
                        disaster_parent
                            .spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Px(theme.spacing.sm),
                                    margin: UiRect::top(Val::Px(theme.spacing.sm)),
                                    border: UiRect::all(Val::Px(theme.borders.width_thin)),
                                    ..default()
                                },
                                background_color: theme.colors.surface_secondary.into(),
                                border_color: theme.colors.border_secondary.into(),
                                border_radius: BorderRadius::all(Val::Px(theme.borders.radius_small)),
                                visibility: Visibility::Hidden, // Initially hidden, shown during cooldown
                                ..default()
                            })
                            .with_children(|progress_parent| {
                                // Progress bar fill with UITheme accent color
                                progress_parent.spawn((
                                    NodeBundle {
                                        style: Style {
                                            width: Val::Percent(0.0), // Will be updated dynamically
                                            height: Val::Percent(100.0),
                                            ..default()
                                        },
                                        background_color: theme.colors.accent_orange.into(),
                                        border_radius: BorderRadius::all(Val::Px(theme.borders.radius_small)),
                                        ..default()
                                    },
                                    DisasterCooldownProgressBar {
                                        disaster_type: *disaster_type,
                                        max_cooldown: get_disaster_cooldown_duration(*disaster_type),
                                    },
                                ));
                            });
                    })
                    .insert(DisasterControlButton {
                        disaster_type: *disaster_type,
                    })
                    .insert(Tooltip {
                        text: get_disaster_description(*disaster_type),
                        shortcut: Some(disaster_type.shortcut_key().to_string()),
                        position: TooltipPosition::Left,
                    })
                    .insert(TooltipTrigger::default());
            }

            // Instructions with UITheme typography
            parent.spawn(TextBundle::from_section(
                "🎮 Press the keys to trigger disasters\n🟢 Available  🟠 Cooldown  🔴 Active\n\n🌧️ Rain (R)  ☀️ Drought (D)\n❄️ Cold Snap (C)  🐛 Invasive Species (I)",
                TextStyle {
                    font_size: theme.typography.caption,
                    color: theme.colors.text_muted,
                    ..default()
                },
            ));
        })
        .insert(DisasterControlPanel);
}
