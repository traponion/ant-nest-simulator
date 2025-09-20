use crate::components::{
    TimeControl, TimeControlPanel, PlayPauseButton, PlayPauseIcon, PlayPauseText,
    SpeedButton, SpeedDisplay, SpeedSlider, SpeedSliderTrack, SpeedSliderHandle, SpeedSliderProgress,
    UITheme, UIAnimation, EasingFunction, AnimationTarget, AnimationValue, Tooltip, FocusIndicator,
    AccessibilityFeatures, AnimationSettings,
};
use bevy::prelude::*;

/// Button for speed presets (enhanced version with speed value)
#[derive(Component)]
pub struct SpeedPresetButton(pub f32);

/// Setup time control UI panel with enhanced interactive design
pub fn setup_time_control_ui(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                top: Val::Px(10.0),
                width: Val::Px(280.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(15.0)),
                row_gap: Val::Px(10.0),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: Color::srgba(0.1, 0.1, 0.1, 0.9).into(),
            border_color: Color::srgb(0.3, 0.3, 0.3).into(),
            border_radius: BorderRadius::all(Val::Px(8.0)),
            ..default()
        })
        .with_children(|parent| {
            // Panel title
            parent.spawn(TextBundle::from_section(
                "Time Control",
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));

            // Speed display
            parent.spawn((
                TextBundle::from_section(
                    "Speed: 1.0x",
                    TextStyle {
                        font_size: 18.0,
                        color: Color::srgb(0.8, 1.0, 0.8),
                        ..default()
                    },
                ),
                SpeedDisplay,
            ));

            // Play/Pause button (enhanced from both versions)
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    background_color: Color::srgba(0.2, 0.6, 0.2, 0.8).into(),
                    border_color: Color::srgb(0.3, 0.7, 0.3).into(),
                    border_radius: BorderRadius::all(Val::Px(4.0)),
                    ..default()
                })
                .with_children(|button_parent| {
                    button_parent.spawn(TextBundle::from_section(
                        "▶ Playing (SPACE to pause)",
                        TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                })
                .insert(PlayPauseButton);

            // Speed control section header
            parent.spawn(TextBundle::from_section(
                "Speed Presets:",
                TextStyle {
                    font_size: 14.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ));

            // Compact speed preset buttons (horizontal layout for main speeds)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(5.0),
                        align_items: AlignItems::Center,
                        flex_wrap: FlexWrap::Wrap,
                        row_gap: Val::Px(5.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|buttons_parent| {
                    // Main speed presets (compact horizontal layout)
                    let main_speed_presets =
                        [(1.0, "1x"), (5.0, "5x"), (20.0, "20x"), (100.0, "Max")];

                    for (speed, label) in main_speed_presets {
                        buttons_parent
                            .spawn((
                                ButtonBundle {
                                    style: Style {
                                        width: Val::Px(55.0),
                                        height: Val::Px(32.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        border: UiRect::all(Val::Px(1.0)),
                                        ..default()
                                    },
                                    background_color: Color::srgb(0.3, 0.3, 0.6).into(),
                                    border_color: Color::srgb(0.4, 0.4, 0.7).into(),
                                    border_radius: BorderRadius::all(Val::Px(4.0)),
                                    ..default()
                                },
                                SpeedPresetButton(speed),
                            ))
                            .with_children(|button| {
                                button.spawn(TextBundle::from_section(
                                    label,
                                    TextStyle {
                                        font_size: 12.0,
                                        color: Color::WHITE,
                                        ..default()
                                    },
                                ));
                            });
                    }
                });

            // Additional speed buttons (full width)
            let additional_speed_presets = [
                (2.0, "2x (Key: 2)"),
                (10.0, "10x (Key: 4)"),
                (50.0, "50x (Key: 7)"),
            ];

            for (speed, label) in additional_speed_presets {
                parent
                    .spawn(ButtonBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Px(28.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::bottom(Val::Px(2.0)),
                            ..default()
                        },
                        background_color: Color::srgba(0.3, 0.3, 0.3, 0.6).into(),
                        border_radius: BorderRadius::all(Val::Px(4.0)),
                        ..default()
                    })
                    .with_children(|speed_parent| {
                        speed_parent.spawn(TextBundle::from_section(
                            label,
                            TextStyle {
                                font_size: 12.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ));
                    })
                    .insert(SpeedButton {
                        target_speed: speed,
                    });
            }

            // Instructions
            parent.spawn(TextBundle::from_section(
                "Click buttons or use keyboard shortcuts (SPACE=Pause, 1-9=Speed)",
                TextStyle {
                    font_size: 11.0,
                    color: Color::srgb(0.7, 0.7, 0.7),
                    ..default()
                },
            ));
        })
        .insert(TimeControlPanel);
}

/// Handle keyboard input for time control
pub fn time_control_input_system(
    input: Res<ButtonInput<KeyCode>>,
    mut time_control: ResMut<TimeControl>,
) {
    // Pause/unpause with space
    if input.just_pressed(KeyCode::Space) {
        time_control.is_paused = !time_control.is_paused;
        if time_control.is_paused {
            info!("Simulation paused");
        } else {
            info!(
                "Simulation resumed at {}x speed",
                time_control.speed_multiplier
            );
        }
    }

    // Speed presets with number keys
    if input.just_pressed(KeyCode::Digit0) {
        time_control.speed_multiplier = 1.0;
        time_control.is_paused = false;
        info!("Speed set to normal (1x)");
    }

    if input.just_pressed(KeyCode::Digit1) {
        time_control.speed_multiplier = 1.0;
        time_control.is_paused = false;
        info!("Speed set to 1x");
    }

    if input.just_pressed(KeyCode::Digit2) {
        time_control.speed_multiplier = 2.0;
        time_control.is_paused = false;
        info!("Speed set to 2x");
    }

    if input.just_pressed(KeyCode::Digit3) {
        time_control.speed_multiplier = 5.0;
        time_control.is_paused = false;
        info!("Speed set to 5x");
    }

    if input.just_pressed(KeyCode::Digit4) {
        time_control.speed_multiplier = 10.0;
        time_control.is_paused = false;
        info!("Speed set to 10x");
    }

    if input.just_pressed(KeyCode::Digit5) {
        time_control.speed_multiplier = 20.0;
        time_control.is_paused = false;
        info!("Speed set to 20x");
    }

    if input.just_pressed(KeyCode::Digit6) {
        time_control.speed_multiplier = 30.0;
        time_control.is_paused = false;
        info!("Speed set to 30x");
    }

    if input.just_pressed(KeyCode::Digit7) {
        time_control.speed_multiplier = 50.0;
        time_control.is_paused = false;
        info!("Speed set to 50x");
    }

    if input.just_pressed(KeyCode::Digit8) {
        time_control.speed_multiplier = 75.0;
        time_control.is_paused = false;
        info!("Speed set to 75x");
    }

    if input.just_pressed(KeyCode::Digit9) {
        time_control.speed_multiplier = 100.0;
        time_control.is_paused = false;
        info!("Speed set to maximum (100x)");
    }
}

/// Handle enhanced button interactions for time control with improved hover effects
pub fn handle_time_control_buttons(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&PlayPauseButton>,
            Option<&SpeedPresetButton>,
        ),
        (
            Changed<Interaction>,
            Or<(With<PlayPauseButton>, With<SpeedPresetButton>)>,
        ),
    >,
    mut time_control: ResMut<TimeControl>,
) {
    for (interaction, mut background_color, play_pause_button, speed_preset_button) in
        &mut interaction_query
    {
        match *interaction {
            Interaction::Pressed => {
                if play_pause_button.is_some() {
                    // Toggle play/pause with enhanced pressed state
                    time_control.is_paused = !time_control.is_paused;
                    if time_control.is_paused {
                        // Pressed state for paused (amber)
                        *background_color = Color::srgba(0.7, 0.5, 0.2, 0.95).into();
                        info!("Simulation paused");
                    } else {
                        // Pressed state for playing (bright green)
                        *background_color = Color::srgba(0.2, 0.7, 0.2, 0.95).into();
                        info!(
                            "Simulation resumed at {}x speed",
                            time_control.speed_multiplier
                        );
                    }
                } else if let Some(SpeedPresetButton(speed)) = speed_preset_button {
                    // Set speed preset with enhanced pressed state
                    *background_color = Color::srgba(0.5, 0.5, 0.8, 0.95).into();
                    time_control.speed_multiplier = *speed;
                    time_control.is_paused = false;
                    info!("Speed set to {}x", speed);
                }
            }
            Interaction::Hovered => {
                // Enhanced hover effects with better visual feedback
                if play_pause_button.is_some() {
                    if time_control.is_paused {
                        // Hover state for paused button (bright amber)
                        *background_color = Color::srgba(0.7, 0.5, 0.2, 0.95).into();
                    } else {
                        // Hover state for playing button (bright green)
                        *background_color = Color::srgba(0.2, 0.7, 0.2, 0.95).into();
                    }
                } else if speed_preset_button.is_some() {
                    // Enhanced hover for speed preset buttons
                    *background_color = Color::srgba(0.4, 0.4, 0.8, 0.9).into();
                }
            }
            Interaction::None => {
                // Reset to enhanced normal colors
                if play_pause_button.is_some() {
                    if time_control.is_paused {
                        // Normal state for paused button (amber)
                        *background_color = Color::srgba(0.6, 0.4, 0.15, 0.9).into();
                    } else {
                        // Normal state for playing button (green)
                        *background_color = Color::srgba(0.15, 0.6, 0.15, 0.9).into();
                    }
                } else if speed_preset_button.is_some() {
                    // Normal state for speed preset buttons
                    *background_color = Color::srgba(0.3, 0.3, 0.6, 0.8).into();
                }
            }
        }
    }
}

/// Update speed display and play/pause button icon
pub fn update_speed_display_system(
    time_control: Res<TimeControl>,
    mut speed_display_query: Query<&mut Text, With<SpeedDisplay>>,
    _play_pause_button_query: Query<Entity, With<PlayPauseButton>>,
    _children_query: Query<&Children>,
    _text_query: Query<&mut Text, Without<SpeedDisplay>>,
) {
    // Update speed display
    if let Ok(mut text) = speed_display_query.get_single_mut() {
        if time_control.is_paused {
            text.sections[0].value = "Speed: PAUSED".to_string();
            text.sections[0].style.color = Color::srgb(1.0, 0.6, 0.6);
        } else {
            text.sections[0].value = format!("Speed: {:.1}x", time_control.speed_multiplier);
            text.sections[0].style.color = Color::srgb(0.8, 1.0, 0.8);
        }
    }
}

/// Update play/pause button display with enhanced layout
pub fn update_play_pause_button_system(
    time_control: Res<TimeControl>,
    mut icon_query: Query<
        &mut Text,
        (
            With<PlayPauseIcon>,
            Without<PlayPauseText>,
            Without<SpeedDisplay>,
        ),
    >,
    mut text_query: Query<
        &mut Text,
        (
            With<PlayPauseText>,
            Without<PlayPauseIcon>,
            Without<SpeedDisplay>,
        ),
    >,
    mut button_query: Query<&mut BackgroundColor, With<PlayPauseButton>>,
) {
    // Update icon
    if let Ok(mut icon_text) = icon_query.get_single_mut() {
        if time_control.is_paused {
            icon_text.sections[0].value = "▶".to_string();
        } else {
            icon_text.sections[0].value = "⏸".to_string();
        }
    }

    // Update text
    if let Ok(mut button_text) = text_query.get_single_mut() {
        if time_control.is_paused {
            button_text.sections[0].value = "Paused".to_string();
        } else {
            button_text.sections[0].value = "Playing".to_string();
        }
    }

    // Update button background color based on state
    if let Ok(mut background_color) = button_query.get_single_mut() {
        if time_control.is_paused {
            *background_color = Color::srgba(0.6, 0.4, 0.15, 0.9).into(); // Amber for paused
        } else {
            *background_color = Color::srgba(0.15, 0.6, 0.15, 0.9).into(); // Green for playing
        }
    }
}

/// Handle mouse clicks on UI buttons
pub fn button_click_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&SpeedButton>,
            Option<&PlayPauseButton>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut time_control: ResMut<TimeControl>,
) {
    for (interaction, mut color, speed_button, play_pause_button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(speed_button) = speed_button {
                    // Speed button clicked
                    time_control.speed_multiplier = speed_button.target_speed;
                    time_control.is_paused = false;
                    info!(
                        "Speed set to {}x via button click",
                        speed_button.target_speed
                    );
                } else if play_pause_button.is_some() {
                    // Play/pause button clicked
                    time_control.is_paused = !time_control.is_paused;
                    if time_control.is_paused {
                        info!("Simulation paused via button click");
                    } else {
                        info!(
                            "Simulation resumed via button click at {}x speed",
                            time_control.speed_multiplier
                        );
                    }
                }
            }
            Interaction::Hovered => {
                *color = Color::srgba(0.5, 0.5, 0.5, 0.8).into();
            }
            Interaction::None => {
                if speed_button.is_some() {
                    *color = Color::srgba(0.3, 0.3, 0.3, 0.6).into();
                } else if play_pause_button.is_some() {
                    *color = Color::srgba(0.2, 0.6, 0.2, 0.8).into();
                }
            }
        }
    }
}

/// Calculate effective delta time with speed multiplier
pub fn effective_delta_time(time: &Res<Time>, time_control: &Res<TimeControl>) -> f32 {
    if time_control.is_paused {
        0.0
    } else {
        time.delta_seconds() * time_control.speed_multiplier
    }
}

/// Setup modern time control UI panel with slider-based speed control
pub fn setup_time_control_ui_with_slider(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                top: Val::Px(10.0),
                width: Val::Px(300.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(15.0)),
                row_gap: Val::Px(12.0),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: Color::srgba(0.1, 0.1, 0.1, 0.9).into(),
            border_color: Color::srgb(0.3, 0.3, 0.3).into(),
            border_radius: BorderRadius::all(Val::Px(8.0)),
            ..default()
        })
        .with_children(|parent| {
            // Panel title
            parent.spawn(TextBundle::from_section(
                "Time Control",
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));

            // Enhanced Play/Pause button with improved visual design
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(50.0), // Slightly taller for better proportions
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        margin: UiRect::bottom(Val::Px(8.0)), // More spacing
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(10.0),
                        ..default()
                    },
                    background_color: Color::srgba(0.15, 0.6, 0.15, 0.9).into(), // Enhanced green
                    border_color: Color::srgb(0.2, 0.8, 0.2).into(),             // Brighter border
                    border_radius: BorderRadius::all(Val::Px(8.0)),              // More rounded
                    ..default()
                })
                .with_children(|button_parent| {
                    // Icon section with enhanced styling
                    button_parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Px(24.0),
                                height: Val::Px(24.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: Color::srgba(1.0, 1.0, 1.0, 0.15).into(), // Subtle icon background
                            border_radius: BorderRadius::all(Val::Px(12.0)),
                            ..default()
                        })
                        .with_children(|icon_parent| {
                            icon_parent.spawn((
                                TextBundle::from_section(
                                    "▶", // Will be updated dynamically
                                    TextStyle {
                                        font_size: 20.0,
                                        color: Color::WHITE,
                                        ..default()
                                    },
                                ),
                                PlayPauseIcon,
                            ));
                        });

                    // Text section with better typography
                    button_parent.spawn((
                        TextBundle::from_section(
                            "Playing",
                            TextStyle {
                                font_size: 16.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ),
                        PlayPauseText,
                    ));

                    // Keyboard shortcut hint with subtle styling
                    button_parent
                        .spawn(NodeBundle {
                            style: Style {
                                padding: UiRect::all(Val::Px(4.0)),
                                ..default()
                            },
                            background_color: Color::srgba(0.0, 0.0, 0.0, 0.3).into(),
                            border_radius: BorderRadius::all(Val::Px(4.0)),
                            ..default()
                        })
                        .with_children(|shortcut_parent| {
                            shortcut_parent.spawn(TextBundle::from_section(
                                "SPACE",
                                TextStyle {
                                    font_size: 12.0,
                                    color: Color::srgb(0.9, 0.9, 0.9),
                                    ..default()
                                },
                            ));
                        });
                })
                .insert(PlayPauseButton);

            // Speed slider section
            parent.spawn(TextBundle::from_section(
                "Speed Control:",
                TextStyle {
                    font_size: 14.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ));

            // Speed display
            parent.spawn((
                TextBundle::from_section(
                    "Speed: 1.0x",
                    TextStyle {
                        font_size: 16.0,
                        color: Color::srgb(0.8, 1.0, 0.8),
                        ..default()
                    },
                ),
                SpeedDisplay,
            ));

            // Enhanced speed slider container
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(50.0), // More height for better touch targets
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        padding: UiRect::horizontal(Val::Px(8.0)),
                        margin: UiRect::vertical(Val::Px(4.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|slider_container| {
                    // Speed zone indicators (visual markers)
                    slider_container
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Px(12.0),
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceBetween,
                                margin: UiRect::bottom(Val::Px(6.0)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|markers_parent| {
                            let speed_markers = [
                                ("1x", Color::srgb(0.5, 0.8, 0.5)),   // Slow - Green
                                ("5x", Color::srgb(0.8, 0.8, 0.5)),   // Normal - Yellow
                                ("20x", Color::srgb(0.8, 0.6, 0.4)),  // Fast - Orange
                                ("100x", Color::srgb(0.8, 0.4, 0.4)), // Ultra - Red
                            ];

                            for (label, color) in speed_markers {
                                markers_parent.spawn(TextBundle::from_section(
                                    label,
                                    TextStyle {
                                        font_size: 10.0,
                                        color,
                                        ..default()
                                    },
                                ));
                            }
                        });

                    // Enhanced slider track with gradient effect
                    slider_container
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Px(10.0), // Thicker track
                                position_type: PositionType::Relative,
                                border: UiRect::all(Val::Px(1.0)),
                                ..default()
                            },
                            background_color: Color::srgba(0.2, 0.2, 0.2, 0.9).into(), // Darker background
                            border_color: Color::srgba(0.4, 0.4, 0.4, 0.8).into(),
                            border_radius: BorderRadius::all(Val::Px(5.0)),
                            ..default()
                        })
                        .with_children(|track_parent| {
                            // Gradient progress fill (visual indicator of current speed zone)
                            track_parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(0.0), // Will be updated based on current speed
                                        height: Val::Percent(100.0),
                                        position_type: PositionType::Absolute,
                                        left: Val::Px(0.0),
                                        top: Val::Px(0.0),
                                        ..default()
                                    },
                                    background_color: Color::srgba(0.2, 0.6, 0.8, 0.6).into(), // Blue progress
                                    border_radius: BorderRadius::all(Val::Px(5.0)),
                                    ..default()
                                })
                                .insert(SpeedSliderProgress);

                            // Enhanced slider handle with better visual design
                            track_parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Px(24.0), // Larger handle
                                        height: Val::Px(24.0),
                                        position_type: PositionType::Absolute,
                                        left: Val::Px(0.0), // Will be updated based on value
                                        top: Val::Px(-7.0), // Center on track
                                        border: UiRect::all(Val::Px(2.0)),
                                        ..default()
                                    },
                                    background_color: Color::srgb(0.9, 0.9, 1.0).into(), // Brighter handle
                                    border_color: Color::srgb(0.6, 0.6, 0.8).into(),
                                    border_radius: BorderRadius::all(Val::Px(12.0)),
                                    ..default()
                                })
                                .with_children(|handle_parent| {
                                    // Inner handle dot for better visibility
                                    handle_parent.spawn(NodeBundle {
                                        style: Style {
                                            width: Val::Px(8.0),
                                            height: Val::Px(8.0),
                                            position_type: PositionType::Absolute,
                                            left: Val::Px(6.0), // Center in handle
                                            top: Val::Px(6.0),
                                            ..default()
                                        },
                                        background_color: Color::srgb(0.3, 0.5, 0.8).into(),
                                        border_radius: BorderRadius::all(Val::Px(4.0)),
                                        ..default()
                                    });
                                })
                                .insert(SpeedSliderHandle);
                        })
                        .insert(SpeedSliderTrack);
                })
                .insert(SpeedSlider {
                    current_value: 1.0,
                    is_dragging: false,
                });

            // Quick speed presets (compact)
            parent.spawn(TextBundle::from_section(
                "Quick Presets:",
                TextStyle {
                    font_size: 12.0,
                    color: Color::srgb(0.8, 0.8, 0.8),
                    ..default()
                },
            ));

            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(5.0),
                        align_items: AlignItems::Center,
                        flex_wrap: FlexWrap::Wrap,
                        row_gap: Val::Px(5.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|buttons_parent| {
                    let speed_presets = [(1.0, "1x"), (5.0, "5x"), (20.0, "20x"), (100.0, "Max")];

                    for (speed, label) in speed_presets {
                        buttons_parent
                            .spawn(ButtonBundle {
                                style: Style {
                                    width: Val::Px(60.0),
                                    height: Val::Px(28.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    border: UiRect::all(Val::Px(1.0)),
                                    ..default()
                                },
                                background_color: Color::srgb(0.3, 0.3, 0.6).into(),
                                border_color: Color::srgb(0.4, 0.4, 0.7).into(),
                                border_radius: BorderRadius::all(Val::Px(4.0)),
                                ..default()
                            })
                            .with_children(|button| {
                                button.spawn(TextBundle::from_section(
                                    label,
                                    TextStyle {
                                        font_size: 12.0,
                                        color: Color::WHITE,
                                        ..default()
                                    },
                                ));
                            })
                            .insert(SpeedPresetButton(speed));
                    }
                });

            // Instructions
            parent.spawn(TextBundle::from_section(
                "Drag slider or use presets. SPACE=Pause, 1-9=Speed shortcuts",
                TextStyle {
                    font_size: 10.0,
                    color: Color::srgb(0.7, 0.7, 0.7),
                    ..default()
                },
            ));
        })
        .insert(TimeControlPanel);
}

/// Handle speed slider interactions and dragging
pub fn handle_speed_slider_system(
    mut slider_query: Query<&mut SpeedSlider>,
    handle_query: Query<&Interaction, (With<SpeedSliderHandle>, Changed<Interaction>)>,
    mut time_control: ResMut<TimeControl>,
    windows: Query<&Window>,
    mut handle_style_query: Query<&mut Style, With<SpeedSliderHandle>>,
    track_query: Query<&Node, With<SpeedSliderTrack>>,
) {
    if let Ok(mut slider) = slider_query.get_single_mut() {
        // Handle mouse interaction with slider handle
        for interaction in &handle_query {
            if interaction == &Interaction::Pressed {
                slider.is_dragging = true;
            }
        }

        // Handle dragging
        if slider.is_dragging {
            if let Ok(window) = windows.get_single() {
                if let Some(cursor_pos) = window.cursor_position() {
                    if let Ok(track_node) = track_query.get_single() {
                        // Calculate relative position on track (simplified)
                        let track_width = track_node.size().x - 20.0; // Account for handle width
                        let relative_x = (cursor_pos.x - 50.0).max(0.0).min(track_width); // Simplified positioning
                        let percentage = relative_x / track_width;

                        // Convert to logarithmic speed scale (1x to 100x)
                        let min_speed = 1.0;
                        let max_speed = 100.0;
                        slider.current_value = min_speed + (max_speed - min_speed) * percentage;

                        // Update time control
                        time_control.speed_multiplier = slider.current_value;
                        time_control.is_paused = false;

                        // Update handle position
                        if let Ok(mut handle_style) = handle_style_query.get_single_mut() {
                            handle_style.left = Val::Px(relative_x);
                        }
                    }
                }
            }

            // Stop dragging on mouse release (simplified - should check mouse button state)
            // For now, we'll implement this in the button system
        }
    }
}

/// Update slider handle position based on current speed
pub fn update_slider_handle_position_system(
    mut slider_query: Query<&mut SpeedSlider>,
    mut handle_style_query: Query<&mut Style, With<SpeedSliderHandle>>,
    track_query: Query<&Node, With<SpeedSliderTrack>>,
    time_control: Res<TimeControl>,
) {
    if let (Ok(mut slider), Ok(mut handle_style), Ok(track_node)) = (
        slider_query.get_single_mut(),
        handle_style_query.get_single_mut(),
        track_query.get_single(),
    ) {
        // Update slider value from time control when not dragging
        if !slider.is_dragging {
            slider.current_value = time_control.speed_multiplier;

            // Calculate handle position
            let track_width = track_node.size().x - 20.0; // Account for handle width
            let min_speed = 1.0;
            let max_speed = 100.0;
            let percentage =
                ((slider.current_value - min_speed) / (max_speed - min_speed)).clamp(0.0, 1.0);
            let position = percentage * track_width;

            handle_style.left = Val::Px(position);
        }
    }
}

/// Update slider progress bar based on current speed
pub fn update_slider_progress_system(
    time_control: Res<TimeControl>,
    mut progress_query: Query<(&mut Style, &mut BackgroundColor), With<SpeedSliderProgress>>,
) {
    if let Ok((mut style, mut background_color)) = progress_query.get_single_mut() {
        // Calculate percentage (1.0 to 100.0 maps to 0% to 100%)
        let min_speed = 1.0;
        let max_speed = 100.0;
        let current_speed = time_control.speed_multiplier.clamp(min_speed, max_speed);
        let percentage = ((current_speed - min_speed) / (max_speed - min_speed)) * 100.0;

        // Update progress bar width
        style.width = Val::Percent(percentage);

        // Update color based on speed zone
        let color = if current_speed <= 2.0 {
            Color::srgba(0.2, 0.6, 0.8, 0.8) // Blue for slow speeds
        } else if current_speed <= 10.0 {
            Color::srgba(0.2, 0.8, 0.6, 0.8) // Green for normal speeds
        } else if current_speed <= 50.0 {
            Color::srgba(0.8, 0.6, 0.2, 0.8) // Orange for fast speeds
        } else {
            Color::srgba(0.8, 0.2, 0.2, 0.8) // Red for ultra speeds
        };

        *background_color = color.into();
    }
}

/// Handle speed preset button clicks for slider UI
pub fn handle_speed_preset_buttons_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &SpeedPresetButton),
        (Changed<Interaction>, With<SpeedPresetButton>),
    >,
    mut time_control: ResMut<TimeControl>,
    mut slider_query: Query<&mut SpeedSlider>,
) {
    for (interaction, mut background_color, SpeedPresetButton(speed)) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // Set speed preset
                time_control.speed_multiplier = *speed;
                time_control.is_paused = false;

                // Update slider value
                if let Ok(mut slider) = slider_query.get_single_mut() {
                    slider.current_value = *speed;
                    slider.is_dragging = false; // Stop any current dragging
                }

                info!("Speed set to {}x via preset button", speed);
            }
            Interaction::Hovered => {
                *background_color = Color::srgb(0.4, 0.4, 0.7).into();
            }
            Interaction::None => {
                *background_color = Color::srgb(0.3, 0.3, 0.6).into();
            }
        }
    }
}

/// Setup enhanced time control UI panel using the unified design system
pub fn setup_themed_time_control_ui(mut commands: Commands, theme: Res<UITheme>) {
    commands
        .spawn(NodeBundle {
            style: theme.create_panel_style(Val::Px(320.0), Val::Auto),
            background_color: theme.colors.surface_primary.into(),
            border_color: theme.colors.border_primary.into(),
            border_radius: BorderRadius::all(Val::Px(theme.borders.radius_medium)),
            ..default()
        })
        .with_children(|parent| {
            // Panel title with improved typography
            parent.spawn(TextBundle::from_section(
                "Time Control",
                TextStyle {
                    font_size: theme.typography.heading_medium,
                    color: theme.colors.text_primary,
                    ..default()
                },
            ));

            // Speed display with accent color
            parent.spawn((
                TextBundle::from_section(
                    "Speed: 1.0x",
                    TextStyle {
                        font_size: theme.typography.body_large,
                        color: theme.colors.text_accent,
                        ..default()
                    },
                ),
                SpeedDisplay,
            ));

            // Enhanced Play/Pause button
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(48.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(theme.borders.width_medium)),
                        margin: UiRect::vertical(Val::Px(theme.spacing.sm)),
                        ..default()
                    },
                    background_color: theme.colors.action_primary.into(),
                    border_color: theme.get_hover_color(theme.colors.action_primary).into(),
                    border_radius: BorderRadius::all(Val::Px(theme.borders.radius_medium)),
                    ..default()
                })
                .with_children(|button_parent| {
                    button_parent.spawn(TextBundle::from_section(
                        "▶ Playing (SPACE)",
                        TextStyle {
                            font_size: theme.typography.body_large,
                            color: theme.colors.text_primary,
                            ..default()
                        },
                    ));
                })
                .insert(PlayPauseButton);

            // Speed control section with better spacing
            parent.spawn(TextBundle::from_section(
                "Speed Control:",
                TextStyle {
                    font_size: theme.typography.body_medium,
                    color: theme.colors.text_secondary,
                    ..default()
                },
            ));

            // Enhanced speed slider container
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(50.0),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(theme.spacing.sm)),
                        margin: UiRect::vertical(Val::Px(theme.spacing.sm)),
                        ..default()
                    },
                    background_color: theme.colors.surface_secondary.into(),
                    border_radius: BorderRadius::all(Val::Px(theme.borders.radius_small)),
                    ..default()
                })
                .with_children(|slider_container| {
                    // Enhanced slider track
                    slider_container
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Px(8.0),
                                position_type: PositionType::Relative,
                                margin: UiRect::horizontal(Val::Px(theme.spacing.sm)),
                                ..default()
                            },
                            background_color: theme.colors.border_secondary.into(),
                            border_radius: BorderRadius::all(Val::Px(theme.borders.radius_small)),
                            ..default()
                        })
                        .with_children(|track_parent| {
                            // Enhanced slider handle
                            track_parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Px(24.0),
                                        height: Val::Px(24.0),
                                        position_type: PositionType::Absolute,
                                        left: Val::Px(0.0),
                                        top: Val::Px(-8.0),
                                        border: UiRect::all(Val::Px(theme.borders.width_medium)),
                                        ..default()
                                    },
                                    background_color: theme.colors.accent_blue.into(),
                                    border_color: theme.colors.text_primary.into(),
                                    border_radius: BorderRadius::all(Val::Px(
                                        theme.borders.radius_round,
                                    )),
                                    ..default()
                                })
                                .insert(SpeedSliderHandle);
                        })
                        .insert(SpeedSliderTrack);
                })
                .insert(SpeedSlider {
                    current_value: 1.0,
                    is_dragging: false,
                });

            // Enhanced quick speed presets section
            parent.spawn(TextBundle::from_section(
                "Quick Presets:",
                TextStyle {
                    font_size: theme.typography.body_small,
                    color: theme.colors.text_secondary,
                    ..default()
                },
            ));

            // Enhanced preset buttons with better layout
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(theme.spacing.sm),
                        align_items: AlignItems::Center,
                        flex_wrap: FlexWrap::Wrap,
                        row_gap: Val::Px(theme.spacing.sm),
                        margin: UiRect::vertical(Val::Px(theme.spacing.sm)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|buttons_parent| {
                    let speed_presets = [(1.0, "1×"), (5.0, "5×"), (20.0, "20×"), (100.0, "Max")];

                    for (speed, label) in speed_presets {
                        buttons_parent
                            .spawn(ButtonBundle {
                                style: Style {
                                    width: Val::Px(65.0),
                                    height: Val::Px(36.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    border: UiRect::all(Val::Px(theme.borders.width_thin)),
                                    ..default()
                                },
                                background_color: theme.colors.action_secondary.into(),
                                border_color: theme.colors.border_primary.into(),
                                border_radius: BorderRadius::all(Val::Px(
                                    theme.borders.radius_small,
                                )),
                                ..default()
                            })
                            .with_children(|button| {
                                button.spawn(TextBundle::from_section(
                                    label,
                                    TextStyle {
                                        font_size: theme.typography.body_small,
                                        color: theme.colors.text_primary,
                                        ..default()
                                    },
                                ));
                            })
                            .insert(SpeedPresetButton(speed));
                    }
                });

            // Enhanced instructions with better typography
            parent.spawn(TextBundle::from_section(
                "Drag slider or use presets • SPACE=Pause • 1-9=Speed shortcuts",
                TextStyle {
                    font_size: theme.typography.caption,
                    color: theme.colors.text_muted,
                    ..default()
                },
            ));
        })
        .insert(TimeControlPanel);
}

/// Enhanced button interaction system with theme-aware hover effects
pub fn handle_themed_time_control_buttons(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&PlayPauseButton>,
            Option<&SpeedPresetButton>,
        ),
        (
            Changed<Interaction>,
            Or<(With<PlayPauseButton>, With<SpeedPresetButton>)>,
        ),
    >,
    mut time_control: ResMut<TimeControl>,
    theme: Res<UITheme>,
) {
    for (interaction, mut background_color, play_pause_button, speed_preset_button) in
        &mut interaction_query
    {
        match *interaction {
            Interaction::Pressed => {
                if play_pause_button.is_some() {
                    // Toggle play/pause
                    time_control.is_paused = !time_control.is_paused;
                    if time_control.is_paused {
                        info!("Simulation paused");
                    } else {
                        info!(
                            "Simulation resumed at {}x speed",
                            time_control.speed_multiplier
                        );
                    }
                } else if let Some(SpeedPresetButton(speed)) = speed_preset_button {
                    // Set speed preset
                    time_control.speed_multiplier = *speed;
                    time_control.is_paused = false;
                    info!("Speed set to {}x", speed);
                }
            }
            Interaction::Hovered => {
                // Enhanced hover effect using theme colors
                if play_pause_button.is_some() {
                    *background_color = theme.get_hover_color(theme.colors.action_primary).into();
                } else if speed_preset_button.is_some() {
                    *background_color = theme.get_hover_color(theme.colors.action_secondary).into();
                }
            }
            Interaction::None => {
                // Reset to normal theme colors
                if play_pause_button.is_some() {
                    *background_color = theme.colors.action_primary.into();
                } else if speed_preset_button.is_some() {
                    *background_color = theme.colors.action_secondary.into();
                }
            }
        }
    }
}

/// Enhanced Time Control UI v3 with animations, tooltips, and accessibility improvements
/// Implements all requirements from Issue #60
pub fn setup_enhanced_time_control_ui_v3(mut commands: Commands, theme: Res<UITheme>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                top: Val::Px(10.0),
                width: Val::Px(340.0), // Slightly wider for better responsive design
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
            // Panel title with improved typography
            parent.spawn(TextBundle::from_section(
                "Time Control",
                TextStyle {
                    font_size: theme.typography.heading_medium,
                    color: theme.colors.text_primary,
                    ..default()
                },
            ));

            // Speed display with enhanced styling
            parent.spawn((
                TextBundle::from_section(
                    "Speed: 1.0×",
                    TextStyle {
                        font_size: theme.typography.body_large,
                        color: theme.colors.text_accent,
                        ..default()
                    },
                ),
                SpeedDisplay,
                UIAnimation::new(
                    0.3,
                    AnimationTarget::BackgroundColor,
                    AnimationValue::Color(theme.colors.text_accent),
                    AnimationValue::Color(theme.colors.accent_blue),
                    EasingFunction::EaseInOut,
                ),
            ));

            // Enhanced Play/Pause button with animation and accessibility
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(56.0), // Larger touch target for accessibility
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(theme.borders.width_medium)),
                        margin: UiRect::vertical(Val::Px(theme.spacing.sm)),
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(theme.spacing.md),
                        ..default()
                    },
                    background_color: theme.colors.action_primary.into(),
                    border_color: theme.colors.border_primary.into(),
                    border_radius: BorderRadius::all(Val::Px(theme.borders.radius_medium)),
                    ..default()
                })
                .with_children(|button_parent| {
                    // Enhanced icon with background circle
                    button_parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Px(32.0),
                                height: Val::Px(32.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: Color::srgba(1.0, 1.0, 1.0, 0.2).into(),
                            border_radius: BorderRadius::all(Val::Px(16.0)),
                            ..default()
                        })
                        .with_children(|icon_parent| {
                            icon_parent.spawn((
                                TextBundle::from_section(
                                    "▶",
                                    TextStyle {
                                        font_size: 18.0,
                                        color: Color::WHITE,
                                        ..default()
                                    },
                                ),
                                PlayPauseIcon,
                                UIAnimation::new(
                                    0.2,
                                    AnimationTarget::Scale,
                                    AnimationValue::Float(1.0),
                                    AnimationValue::Float(1.1),
                                    EasingFunction::EaseInOut,
                                ),
                            ));
                        });

                    // Text with improved layout
                    button_parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Start,
                                row_gap: Val::Px(2.0),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|text_parent| {
                            // Main text
                            text_parent.spawn((
                                TextBundle::from_section(
                                    "Playing",
                                    TextStyle {
                                        font_size: theme.typography.body_large,
                                        color: Color::WHITE,
                                        ..default()
                                    },
                                ),
                                PlayPauseText,
                            ));

                            // Keyboard shortcut hint
                            text_parent.spawn(TextBundle::from_section(
                                "Press SPACE",
                                TextStyle {
                                    font_size: theme.typography.body_small,
                                    color: Color::srgba(1.0, 1.0, 1.0, 0.7),
                                    ..default()
                                },
                            ));
                        });
                })
                .insert(PlayPauseButton)
                .insert(Tooltip::new("Toggle simulation playback".to_string()))
                .insert(FocusIndicator::new(&theme))
                .insert(AccessibilityFeatures::button("Play/Pause simulation".to_string(), 1))
                .insert(UIAnimation::new(
                    0.15,
                    AnimationTarget::BackgroundColor,
                    AnimationValue::Color(theme.colors.action_primary),
                    AnimationValue::Color(theme.get_hover_color(theme.colors.action_primary)),
                    EasingFunction::EaseOut,
                ));

            // Speed control section
            parent.spawn(TextBundle::from_section(
                "Speed Control:",
                TextStyle {
                    font_size: theme.typography.body_medium,
                    color: theme.colors.text_secondary,
                    ..default()
                },
            ));

            // Enhanced speed slider with gradient track and improved visual feedback
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(60.0), // More height for better touch targets
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        padding: UiRect::all(Val::Px(theme.spacing.sm)),
                        margin: UiRect::vertical(Val::Px(theme.spacing.sm)),
                        ..default()
                    },
                    background_color: theme.colors.surface_secondary.into(),
                    border_radius: BorderRadius::all(Val::Px(theme.borders.radius_small)),
                    ..default()
                })
                .with_children(|slider_container| {
                    // Speed zone markers with improved visibility
                    slider_container
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Px(16.0),
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceBetween,
                                margin: UiRect::bottom(Val::Px(theme.spacing.xs)),
                                padding: UiRect::horizontal(Val::Px(theme.spacing.sm)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|markers_parent| {
                            let speed_markers = [
                                ("1×", Color::srgb(0.6, 0.9, 0.6)),   // Slow - Green
                                ("5×", Color::srgb(0.9, 0.9, 0.6)),   // Normal - Yellow
                                ("20×", Color::srgb(0.9, 0.7, 0.5)),  // Fast - Orange
                                ("100×", Color::srgb(0.9, 0.5, 0.5)), // Ultra - Red
                            ];

                            for (label, color) in speed_markers {
                                markers_parent.spawn((
                                    TextBundle::from_section(
                                        label,
                                        TextStyle {
                                            font_size: theme.typography.body_small,
                                            color,
                                            ..default()
                                        },
                                    ),
                                    UIAnimation::new(
                                        0.3,
                                        AnimationTarget::Opacity,
                                        AnimationValue::Float(0.7),
                                        AnimationValue::Float(1.0),
                                        EasingFunction::EaseInOut,
                                    ),
                                ));
                            }
                        });

                    // Enhanced slider track with gradient background
                    slider_container
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Px(12.0), // Thicker track for accessibility
                                position_type: PositionType::Relative,
                                border: UiRect::all(Val::Px(1.0)),
                                margin: UiRect::horizontal(Val::Px(theme.spacing.sm)),
                                ..default()
                            },
                            background_color: Color::srgba(0.15, 0.15, 0.15, 0.9).into(),
                            border_color: theme.colors.border_secondary.into(),
                            border_radius: BorderRadius::all(Val::Px(6.0)),
                            ..default()
                        })
                        .with_children(|track_parent| {
                            // Animated progress fill with speed-based colors
                            track_parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(0.0), // Will be updated based on speed
                                        height: Val::Percent(100.0),
                                        position_type: PositionType::Absolute,
                                        left: Val::Px(0.0),
                                        top: Val::Px(0.0),
                                        ..default()
                                    },
                                    background_color: Color::srgba(0.3, 0.7, 0.9, 0.8).into(),
                                    border_radius: BorderRadius::all(Val::Px(6.0)),
                                    ..default()
                                })
                                .insert(SpeedSliderProgress)
                                .insert(UIAnimation::new(
                                    0.25,
                                    AnimationTarget::BackgroundColor,
                                    AnimationValue::Color(Color::srgba(0.3, 0.7, 0.9, 0.8)),
                                    AnimationValue::Color(Color::srgba(0.9, 0.3, 0.3, 0.8)),
                                    EasingFunction::EaseInOut,
                                ));

                            // Enhanced slider handle with accessibility improvements
                            track_parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Px(28.0), // Larger handle for touch
                                        height: Val::Px(28.0),
                                        position_type: PositionType::Absolute,
                                        left: Val::Px(0.0),
                                        top: Val::Px(-8.0),
                                        border: UiRect::all(Val::Px(2.0)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    background_color: Color::srgb(0.95, 0.95, 1.0).into(),
                                    border_color: theme.colors.accent_blue.into(),
                                    border_radius: BorderRadius::all(Val::Px(14.0)),
                                    ..default()
                                })
                                .with_children(|handle_parent| {
                                    // Inner handle indicator for better visibility
                                    handle_parent.spawn(NodeBundle {
                                        style: Style {
                                            width: Val::Px(10.0),
                                            height: Val::Px(10.0),
                                            ..default()
                                        },
                                        background_color: theme.colors.accent_blue.into(),
                                        border_radius: BorderRadius::all(Val::Px(5.0)),
                                        ..default()
                                    });
                                })
                                .insert(SpeedSliderHandle)
                                .insert(FocusIndicator::new(&theme))
                                .insert(AccessibilityFeatures::slider("Speed control slider".to_string(), 2))
                                .insert(Tooltip::new("Drag to adjust simulation speed".to_string()))
                                .insert(UIAnimation::new(
                                    0.15,
                                    AnimationTarget::Scale,
                                    AnimationValue::Float(1.0),
                                    AnimationValue::Float(1.2),
                                    EasingFunction::EaseOut,
                                ));
                        })
                        .insert(SpeedSliderTrack);
                })
                .insert(SpeedSlider {
                    current_value: 1.0,
                    is_dragging: false,
                });

            // Enhanced speed presets with tooltips and animations
            parent.spawn(TextBundle::from_section(
                "Quick Presets:",
                TextStyle {
                    font_size: theme.typography.body_small,
                    color: theme.colors.text_secondary,
                    ..default()
                },
            ));

            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(theme.spacing.sm),
                        row_gap: Val::Px(theme.spacing.sm),
                        align_items: AlignItems::Center,
                        flex_wrap: FlexWrap::Wrap,
                        margin: UiRect::vertical(Val::Px(theme.spacing.sm)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|buttons_parent| {
                    let speed_presets = [
                        (1.0, "1×", "Normal speed"),
                        (5.0, "5×", "Fast speed"),
                        (20.0, "20×", "Very fast speed"),
                        (100.0, "Max", "Maximum speed"),
                    ];

                    for (i, (speed, label, tooltip_text)) in speed_presets.iter().enumerate() {
                        buttons_parent
                            .spawn(ButtonBundle {
                                style: Style {
                                    width: Val::Px(68.0),
                                    height: Val::Px(40.0), // Larger for better accessibility
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    border: UiRect::all(Val::Px(theme.borders.width_thin)),
                                    ..default()
                                },
                                background_color: theme.colors.action_secondary.into(),
                                border_color: theme.colors.border_primary.into(),
                                border_radius: BorderRadius::all(Val::Px(theme.borders.radius_small)),
                                ..default()
                            })
                            .with_children(|button| {
                                button.spawn(TextBundle::from_section(
                                    *label,
                                    TextStyle {
                                        font_size: theme.typography.body_small,
                                        color: theme.colors.text_primary,
                                        ..default()
                                    },
                                ));
                            })
                            .insert(SpeedPresetButton(*speed))
                            .insert(Tooltip::new(tooltip_text.to_string()))
                            .insert(FocusIndicator::new(&theme))
                            .insert(AccessibilityFeatures::button(
                                format!("Set speed to {}", label),
                                3 + i as i32,
                            ))
                            .insert(UIAnimation::new(
                                0.15,
                                AnimationTarget::BackgroundColor,
                                AnimationValue::Color(theme.colors.action_secondary),
                                AnimationValue::Color(theme.get_hover_color(theme.colors.action_secondary)),
                                EasingFunction::EaseOut,
                            ));
                    }
                });

            // Enhanced instructions with better accessibility
            parent.spawn(TextBundle::from_section(
                "💡 Drag slider or use presets • SPACE=Pause • 1-9=Speed shortcuts",
                TextStyle {
                    font_size: theme.typography.caption,
                    color: theme.colors.text_muted,
                    ..default()
                },
            ));
        })
        .insert(TimeControlPanel)
        .insert(AccessibilityFeatures::button("Time Control Panel".to_string(), 0));
}

/// System to update UI animations for smooth transitions
pub fn update_ui_animations_system(
    mut animation_query: Query<(Entity, &mut UIAnimation, &mut BackgroundColor, &mut Style, &mut Transform)>,
    time: Res<Time>,
    animation_settings: Res<AnimationSettings>,
) {
    if !animation_settings.animations_enabled {
        return;
    }

    let delta_time = time.delta_seconds() * animation_settings.speed_multiplier;

    for (_entity, mut animation, mut background_color, mut style, mut transform) in &mut animation_query {
        if animation.is_playing {
            animation.update(delta_time);
            let current_value = animation.get_current_value();

            // Apply animation based on target property
            match animation.target {
                AnimationTarget::BackgroundColor => {
                    if let AnimationValue::Color(color) = current_value {
                        *background_color = color.into();
                    }
                }
                AnimationTarget::Scale => {
                    if let AnimationValue::Float(scale) = current_value {
                        transform.scale = Vec3::splat(scale);
                    }
                }
                AnimationTarget::Position => {
                    if let AnimationValue::Vec2(pos) = current_value {
                        style.left = Val::Px(pos.x);
                        style.top = Val::Px(pos.y);
                    }
                }
                AnimationTarget::Opacity => {
                    if let AnimationValue::Float(alpha) = current_value {
                        let current_color = background_color.0;
                        *background_color = Color::srgba(
                            current_color.to_srgba().red,
                            current_color.to_srgba().green,
                            current_color.to_srgba().blue,
                            alpha,
                        ).into();
                    }
                }
                _ => {} // Other targets can be implemented as needed
            }
        }
    }
}

/// System to handle tooltip display based on hover state
pub fn tooltip_system(
    mut commands: Commands,
    mut tooltip_query: Query<(Entity, &mut Tooltip, &Interaction, &GlobalTransform), Changed<Interaction>>,
    theme: Res<UITheme>,
    time: Res<Time>,
) {
    for (entity, mut tooltip, interaction, global_transform) in &mut tooltip_query {
        let is_hovered = matches!(interaction, Interaction::Hovered);
        tooltip.update(is_hovered, time.delta_seconds());

        if tooltip.is_visible {
            // Spawn tooltip UI element
            let tooltip_entity = commands
                .spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        left: Val::Px(global_transform.translation().x + tooltip.offset.x),
                        top: Val::Px(global_transform.translation().y + tooltip.offset.y),
                        padding: UiRect::all(Val::Px(theme.spacing.sm)),
                        ..default()
                    },
                    background_color: tooltip.background_color.into(),
                    border_radius: BorderRadius::all(Val::Px(theme.borders.radius_small)),
                    z_index: ZIndex::Global(1000), // Ensure tooltip appears on top
                    ..default()
                })
                .with_children(|tooltip_parent| {
                    tooltip_parent.spawn(TextBundle::from_section(
                        tooltip.text.clone(),
                        TextStyle {
                            font_size: theme.typography.body_small,
                            color: tooltip.text_color,
                            ..default()
                        },
                    ));
                })
                .id();

            // Store tooltip entity for cleanup
            commands.entity(entity).insert(TooltipEntity(tooltip_entity));
        }
    }
}

/// Component to track spawned tooltip entities for cleanup
#[derive(Component)]
pub struct TooltipEntity(pub Entity);

/// System to handle focus indicators for keyboard navigation
pub fn focus_indicator_system(
    mut focus_query: Query<(Entity, &mut FocusIndicator, &Interaction), Changed<Interaction>>,
    mut commands: Commands,
    theme: Res<UITheme>,
) {
    for (entity, mut focus_indicator, interaction) in &mut focus_query {
        let is_focused = matches!(interaction, Interaction::Hovered);

        if focus_indicator.is_focused != is_focused {
            focus_indicator.set_focus(is_focused);

            if is_focused {
                // Add visual focus indicator
                commands.entity(entity).insert(Outline::new(
                    Val::Px(focus_indicator.outline_width),
                    Val::Px(2.0),
                    focus_indicator.focus_color,
                ));
            } else {
                // Remove focus indicator
                commands.entity(entity).remove::<Outline>();
            }
        }

        // Update focus animation if present
        if let Some(ref mut animation) = focus_indicator.focus_animation {
            if animation.is_playing {
                animation.update(0.016); // Approximate frame time for smooth animation
            }
        }
    }
}
