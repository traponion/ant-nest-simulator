use crate::components::{TimeControl, TimeControlPanel, PlayPauseButton, SpeedButton, SpeedDisplay};
use bevy::prelude::*;

/// Setup time control UI panel
pub fn setup_time_control_ui(mut commands: Commands) {
    // Main time control panel container - positioned at top-left
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                top: Val::Px(10.0),
                width: Val::Px(260.0),
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

            // Play/Pause button
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::srgba(0.2, 0.6, 0.2, 0.8).into(),
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

            // Speed control section
            parent.spawn(TextBundle::from_section(
                "Speed Presets:",
                TextStyle {
                    font_size: 14.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ));

            // Speed buttons grid
            let speed_presets = [
                (1.0, "1x (Key: 1)"),
                (2.0, "2x (Key: 2)"),
                (5.0, "5x (Key: 3)"),
                (10.0, "10x (Key: 4)"),
                (20.0, "20x (Key: 5)"),
                (50.0, "50x (Key: 7)"),
                (100.0, "Max (Key: 9)"),
            ];

            for (speed, label) in speed_presets.iter() {
                parent
                    .spawn(ButtonBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Px(32.0),
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
                            *label,
                            TextStyle {
                                font_size: 14.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ));
                    })
                    .insert(SpeedButton { target_speed: *speed });
            }

            // Instructions
            parent.spawn(TextBundle::from_section(
                "Click buttons or use keyboard shortcuts",
                TextStyle {
                    font_size: 12.0,
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

/// Update speed display UI
pub fn update_speed_display_system(
    time_control: Res<TimeControl>,
    mut speed_display_query: Query<&mut Text, With<SpeedDisplay>>,
) {
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
