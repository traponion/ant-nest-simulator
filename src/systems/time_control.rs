use crate::components::{
    TimeControl, TimeControlPanel, PlayPauseButton, SpeedButton, SpeedDisplay,
    SpeedSlider, SpeedSliderTrack, SpeedSliderHandle,
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
                    let main_speed_presets = [
                        (1.0, "1x"),
                        (5.0, "5x"),
                        (20.0, "20x"),
                        (100.0, "Max"),
                    ];

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
                    .insert(SpeedButton { target_speed: speed });
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

/// Handle button clicks for time control
pub fn handle_time_control_buttons(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&PlayPauseButton>, Option<&SpeedPresetButton>),
        (Changed<Interaction>, Or<(With<PlayPauseButton>, With<SpeedPresetButton>)>),
    >,
    mut time_control: ResMut<TimeControl>,
) {
    for (interaction, mut background_color, play_pause_button, speed_preset_button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if play_pause_button.is_some() {
                    // Toggle play/pause
                    time_control.is_paused = !time_control.is_paused;
                    if time_control.is_paused {
                        info!("Simulation paused");
                    } else {
                        info!("Simulation resumed at {}x speed", time_control.speed_multiplier);
                    }
                } else if let Some(SpeedPresetButton(speed)) = speed_preset_button {
                    // Set speed preset
                    time_control.speed_multiplier = *speed;
                    time_control.is_paused = false;
                    info!("Speed set to {}x", speed);
                }
            }
            Interaction::Hovered => {
                // Hover effect - lighten the color
                if play_pause_button.is_some() {
                    *background_color = Color::srgb(0.3, 0.7, 0.3).into();
                } else if speed_preset_button.is_some() {
                    *background_color = Color::srgb(0.4, 0.4, 0.7).into();
                }
            }
            Interaction::None => {
                // Reset to normal color
                if play_pause_button.is_some() {
                    *background_color = Color::srgb(0.2, 0.6, 0.2).into();
                } else if speed_preset_button.is_some() {
                    *background_color = Color::srgb(0.3, 0.3, 0.6).into();
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

/// Update play/pause button display
pub fn update_play_pause_button_system(
    time_control: Res<TimeControl>,
    mut button_query: Query<&mut Text, (With<Node>, Without<SpeedDisplay>)>,
    play_pause_query: Query<&Children, With<PlayPauseButton>>,
) {
    if let Ok(children) = play_pause_query.get_single() {
        for &child in children.iter() {
            if let Ok(mut text) = button_query.get_mut(child) {
                if time_control.is_paused {
                    text.sections[0].value = "▶ Paused (SPACE to play)".to_string();
                } else {
                    text.sections[0].value = "⏸ Playing (SPACE to pause)".to_string();
                }
            }
        }
    }
}

/// Handle mouse clicks on UI buttons
pub fn button_click_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SpeedButton>, Option<&PlayPauseButton>),
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
                    info!("Speed set to {}x via button click", speed_button.target_speed);
                } else if play_pause_button.is_some() {
                    // Play/pause button clicked
                    time_control.is_paused = !time_control.is_paused;
                    if time_control.is_paused {
                        info!("Simulation paused via button click");
                    } else {
                        info!("Simulation resumed via button click at {}x speed", time_control.speed_multiplier);
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

            // Play/Pause button with icons
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(45.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        margin: UiRect::bottom(Val::Px(5.0)),
                        ..default()
                    },
                    background_color: Color::srgba(0.2, 0.6, 0.2, 0.8).into(),
                    border_color: Color::srgb(0.3, 0.7, 0.3).into(),
                    border_radius: BorderRadius::all(Val::Px(6.0)),
                    ..default()
                })
                .with_children(|button_parent| {
                    button_parent.spawn(TextBundle::from_section(
                        "▶ Playing (SPACE)",
                        TextStyle {
                            font_size: 18.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
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

            // Speed slider container
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(40.0),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        padding: UiRect::horizontal(Val::Px(5.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|slider_container| {
                    // Slider track
                    slider_container
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Px(6.0),
                                position_type: PositionType::Relative,
                                ..default()
                            },
                            background_color: Color::srgb(0.3, 0.3, 0.3).into(),
                            border_radius: BorderRadius::all(Val::Px(3.0)),
                            ..default()
                        })
                        .with_children(|track_parent| {
                            // Slider handle
                            track_parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Px(20.0),
                                        height: Val::Px(20.0),
                                        position_type: PositionType::Absolute,
                                        left: Val::Px(0.0), // Will be updated based on value
                                        top: Val::Px(-7.0), // Center on track
                                        ..default()
                                    },
                                    background_color: Color::srgb(0.8, 0.8, 1.0).into(),
                                    border_radius: BorderRadius::all(Val::Px(10.0)),
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
            match interaction {
                Interaction::Pressed => {
                    slider.is_dragging = true;
                }
                _ => {}
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
        track_query.get_single()
    ) {
        // Update slider value from time control when not dragging
        if !slider.is_dragging {
            slider.current_value = time_control.speed_multiplier;

            // Calculate handle position
            let track_width = track_node.size().x - 20.0; // Account for handle width
            let min_speed = 1.0;
            let max_speed = 100.0;
            let percentage = ((slider.current_value - min_speed) / (max_speed - min_speed)).clamp(0.0, 1.0);
            let position = percentage * track_width;

            handle_style.left = Val::Px(position);
        }
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
