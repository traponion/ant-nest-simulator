use crate::components::{
    CooldownTimer, DisasterControlButton, DisasterControlPanel, DisasterState, DisasterStatusIndicator,
    DisasterStatusBackground, DisasterTriggerFeedback, DisasterType, DisasterCooldownProgressBar,
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
                width: Val::Px(280.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(15.0)),
                row_gap: Val::Px(10.0),
                ..default()
            },
            background_color: Color::srgba(0.0, 0.0, 0.0, 0.8).into(),
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
                            padding: UiRect::all(Val::Px(8.0)),
                            row_gap: Val::Px(4.0),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        background_color: Color::srgba(0.2, 0.2, 0.2, 0.6).into(),
                        border_color: Color::srgba(0.3, 0.3, 0.3, 0.4).into(),
                        border_radius: BorderRadius::all(Val::Px(4.0)),
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
                                        // Disaster icon
                                        name_parent.spawn(TextBundle::from_section(
                                            disaster_type.get_icon(),
                                            TextStyle {
                                                font_size: 20.0,
                                                color: Color::WHITE,
                                                ..default()
                                            },
                                        ));

                                        // Disaster name
                                        name_parent.spawn(TextBundle::from_section(
                                            disaster_type.display_name(),
                                            TextStyle {
                                                font_size: 16.0,
                                                color: Color::WHITE,
                                                ..default()
                                            },
                                        ));
                                    });

                                // Key shortcut
                                header_parent.spawn(TextBundle::from_section(
                                    format!("Key: {}", disaster_type.shortcut_key()),
                                    TextStyle {
                                        font_size: 14.0,
                                        color: Color::srgb(0.8, 0.8, 0.8),
                                        ..default()
                                    },
                                ));
                            });

                        // Status and timer row
                        disaster_parent
                            .spawn(NodeBundle {
                                style: Style {
                                    flex_direction: FlexDirection::Row,
                                    justify_content: JustifyContent::SpaceBetween,
                                    align_items: AlignItems::Center,
                                    padding: UiRect::all(Val::Px(4.0)),
                                    ..default()
                                },
                                background_color: Color::srgba(0.1, 0.1, 0.1, 0.3).into(),
                                border_radius: BorderRadius::all(Val::Px(3.0)),
                                ..default()
                            })
                            .with_children(|status_parent| {
                                // Visual status indicator with background
                                status_parent
                                    .spawn((
                                        NodeBundle {
                                            style: Style {
                                                padding: UiRect::all(Val::Px(6.0)),
                                                ..default()
                                            },
                                            background_color: Color::srgb(0.3, 1.0, 0.3).into(), // Default available color
                                            border_radius: BorderRadius::all(Val::Px(12.0)),
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

                        // Cooldown progress bar (only visible during cooldown)
                        disaster_parent
                            .spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Px(6.0),
                                    margin: UiRect::top(Val::Px(4.0)),
                                    ..default()
                                },
                                background_color: Color::srgba(0.3, 0.3, 0.3, 0.6).into(),
                                border_radius: BorderRadius::all(Val::Px(3.0)),
                                visibility: Visibility::Hidden, // Initially hidden, shown during cooldown
                                ..default()
                            })
                            .with_children(|progress_parent| {
                                // Progress bar fill
                                progress_parent.spawn((
                                    NodeBundle {
                                        style: Style {
                                            width: Val::Percent(0.0), // Will be updated dynamically
                                            height: Val::Percent(100.0),
                                            ..default()
                                        },
                                        background_color: Color::srgb(1.0, 0.6, 0.0).into(), // Orange for cooldown
                                        border_radius: BorderRadius::all(Val::Px(3.0)),
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
            Color::srgb(1.0, 0.3, 0.3) // Red for active
        } else if disaster_state.is_on_cooldown(disaster_type) {
            Color::srgb(1.0, 0.6, 0.0) // Orange for cooldown
        } else {
            Color::srgb(0.3, 1.0, 0.3) // Green for available
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
                // Hover effect - lighten the color based on disaster state
                if disaster_state.is_active(disaster_type) {
                    *background_color = Color::srgba(0.3, 0.3, 0.3, 0.7).into();
                    *border_color = Color::srgba(0.5, 0.5, 0.5, 0.7).into();
                } else if disaster_state.is_on_cooldown(disaster_type) {
                    *background_color = Color::srgba(0.3, 0.3, 0.3, 0.7).into();
                    *border_color = Color::srgba(0.5, 0.5, 0.5, 0.7).into();
                } else {
                    // Available - more prominent hover effect
                    *background_color = Color::srgba(0.3, 0.3, 0.3, 0.8).into();
                    *border_color = Color::srgba(0.5, 0.5, 0.5, 1.0).into();
                }
            }
            Interaction::None => {
                // Reset to normal color
                *background_color = Color::srgba(0.2, 0.2, 0.2, 0.6).into();
                *border_color = Color::srgba(0.3, 0.3, 0.3, 0.4).into();
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
