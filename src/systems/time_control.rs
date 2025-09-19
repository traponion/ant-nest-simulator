use crate::components::TimeControl;
use bevy::prelude::*;

/// UI components for time control
#[derive(Component)]
pub struct TimeControlUI;

#[derive(Component)]
pub struct SpeedDisplay;

/// Button for play/pause toggle
#[derive(Component)]
pub struct PlayPauseButton;

/// Button for speed presets
#[derive(Component)]
pub struct SpeedPresetButton(pub f32);

/// Button hover interaction
#[derive(Component)]
pub struct ButtonHover;

/// Setup time control UI
pub fn setup_time_control_ui(mut commands: Commands) {
    // Time Control Panel - Top left corner
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                padding: UiRect::all(Val::Px(12.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: Color::srgba(0.1, 0.1, 0.1, 0.9).into(),
            border_color: Color::srgb(0.3, 0.3, 0.3).into(),
            ..default()
        })
        .with_children(|parent| {
            // Speed display
            parent.spawn((
                TextBundle::from_section(
                    "Speed: 1.0x",
                    TextStyle {
                        font_size: 18.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                SpeedDisplay,
            ));

            // Control buttons container
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(8.0),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|buttons_parent| {
                    // Play/Pause button
                    buttons_parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(50.0),
                                    height: Val::Px(35.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    border: UiRect::all(Val::Px(2.0)),
                                    ..default()
                                },
                                background_color: Color::srgb(0.2, 0.6, 0.2).into(),
                                border_color: Color::srgb(0.3, 0.7, 0.3).into(),
                                ..default()
                            },
                            PlayPauseButton,
                        ))
                        .with_children(|button| {
                            button.spawn(TextBundle::from_section(
                                "▶",
                                TextStyle {
                                    font_size: 16.0,
                                    color: Color::WHITE,
                                    ..default()
                                },
                            ));
                        });

                    // Speed preset buttons
                    let speed_presets = [
                        (1.0, "1x", "Normal"),
                        (5.0, "5x", "Fast"),
                        (20.0, "20x", "Very Fast"),
                        (100.0, "100x", "Maximum"),
                    ];

                    for (speed, label, _tooltip) in speed_presets {
                        buttons_parent
                            .spawn((
                                ButtonBundle {
                                    style: Style {
                                        width: Val::Px(45.0),
                                        height: Val::Px(35.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        border: UiRect::all(Val::Px(2.0)),
                                        ..default()
                                    },
                                    background_color: Color::srgb(0.3, 0.3, 0.6).into(),
                                    border_color: Color::srgb(0.4, 0.4, 0.7).into(),
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

            // Keyboard shortcuts hint
            parent.spawn(TextBundle::from_section(
                "Keyboard: SPACE=Pause, 1-9=Speed, 0=Normal",
                TextStyle {
                    font_size: 12.0,
                    color: Color::srgb(0.7, 0.7, 0.7),
                    ..default()
                },
            ));
        })
        .insert(TimeControlUI);
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
    play_pause_button_query: Query<Entity, With<PlayPauseButton>>,
    children_query: Query<&Children>,
    mut text_query: Query<&mut Text, Without<SpeedDisplay>>,
) {
    // Update speed display
    if let Ok(mut text) = speed_display_query.get_single_mut() {
        if time_control.is_paused {
            text.sections[0].value = "Speed: PAUSED".to_string();
        } else {
            text.sections[0].value = format!("Speed: {:.1}x", time_control.speed_multiplier);
        }
    }

    // Update play/pause button icon
    if let Ok(button_entity) = play_pause_button_query.get_single() {
        if let Ok(children) = children_query.get(button_entity) {
            for &child in children.iter() {
                if let Ok(mut text) = text_query.get_mut(child) {
                    if time_control.is_paused {
                        text.sections[0].value = "▶".to_string();
                    } else {
                        text.sections[0].value = "⏸".to_string();
                    }
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
