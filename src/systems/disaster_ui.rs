use crate::components::{
    DisasterControlButton, DisasterControlPanel, DisasterState, DisasterType, Tooltip,
    TooltipPosition, TooltipTrigger, UITheme,
};
use bevy::prelude::*;

/// Setup the disaster control panel UI
pub fn setup_disaster_control_panel(mut commands: Commands, ui_theme: Res<UITheme>) {
    // Main disaster control panel container
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(20.0),
                top: Val::Px(20.0),
                width: Val::Px(280.0),
                height: Val::Auto,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(ui_theme.spacing.lg)),
                row_gap: Val::Px(ui_theme.spacing.md),
                border: UiRect::all(Val::Px(ui_theme.borders.width_medium)),
                ..default()
            },
            background_color: ui_theme.colors.surface_primary.into(),
            border_color: ui_theme.colors.border_primary.into(),
            border_radius: BorderRadius::all(Val::Px(ui_theme.borders.radius_medium)),
            ..default()
        })
        .insert(DisasterControlPanel)
        .with_children(|parent| {
            // Panel Title
            parent.spawn(TextBundle::from_section(
                "Disaster Control",
                TextStyle {
                    font_size: ui_theme.typography.heading_small,
                    color: ui_theme.colors.text_primary,
                    ..default()
                },
            ));

            // Description text
            parent.spawn(TextBundle::from_section(
                "Trigger environmental disasters to observe colony adaptation",
                TextStyle {
                    font_size: ui_theme.typography.body_small,
                    color: ui_theme.colors.text_secondary,
                    ..default()
                },
            ));

            // Disaster control buttons container
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(ui_theme.spacing.sm),
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|buttons_parent| {
                    // Create buttons for each disaster type
                    create_disaster_button(buttons_parent, DisasterType::Rain, &ui_theme);
                    create_disaster_button(buttons_parent, DisasterType::Drought, &ui_theme);
                    create_disaster_button(buttons_parent, DisasterType::ColdSnap, &ui_theme);
                    create_disaster_button(
                        buttons_parent,
                        DisasterType::InvasiveSpecies,
                        &ui_theme,
                    );
                });
        });
}

/// Create a disaster control button with icon and status indicator
fn create_disaster_button(
    parent: &mut ChildBuilder,
    disaster_type: DisasterType,
    ui_theme: &UITheme,
) {
    parent
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(60.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(ui_theme.spacing.md)),
                border: UiRect::all(Val::Px(ui_theme.borders.width_thin)),
                margin: UiRect::bottom(Val::Px(ui_theme.spacing.xs)),
                ..default()
            },
            background_color: ui_theme.colors.surface_elevated.into(),
            border_color: ui_theme.colors.border_secondary.into(),
            border_radius: BorderRadius::all(Val::Px(ui_theme.borders.radius_small)),
            ..default()
        })
        .insert(DisasterControlButton { disaster_type })
        .insert(Tooltip {
            text: format!(
                "{}: {}",
                disaster_type.display_name(),
                get_disaster_description(disaster_type)
            ),
            shortcut: Some(format!("Press {}", disaster_type.shortcut_key())),
            position: TooltipPosition::Right,
        })
        .insert(TooltipTrigger::default())
        .with_children(|button_parent| {
            // Left side: Icon and label
            button_parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(ui_theme.spacing.sm),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|left_parent| {
                    // Disaster icon
                    left_parent.spawn(TextBundle::from_section(
                        disaster_type.get_icon(),
                        TextStyle {
                            font_size: ui_theme.typography.heading_medium,
                            color: ui_theme.colors.text_primary,
                            ..default()
                        },
                    ));

                    // Disaster name and shortcut
                    left_parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|text_parent| {
                            text_parent.spawn(TextBundle::from_section(
                                disaster_type.display_name(),
                                TextStyle {
                                    font_size: ui_theme.typography.body_medium,
                                    color: ui_theme.colors.text_primary,
                                    ..default()
                                },
                            ));
                            text_parent.spawn(TextBundle::from_section(
                                format!("({})", disaster_type.shortcut_key()),
                                TextStyle {
                                    font_size: ui_theme.typography.body_small,
                                    color: ui_theme.colors.text_muted,
                                    ..default()
                                },
                            ));
                        });
                });

            // Right side: Status indicator
            button_parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(12.0),
                        height: Val::Px(12.0),
                        ..default()
                    },
                    background_color: ui_theme.colors.accent_green.into(), // Default: available
                    border_radius: BorderRadius::all(Val::Px(6.0)),
                    ..default()
                },
                DisasterStatusIndicator { disaster_type },
            ));
        });
}

/// Handle disaster control button interactions
pub fn handle_disaster_control_interactions(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &DisasterControlButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut disaster_state: ResMut<DisasterState>,
    ui_theme: Res<UITheme>,
) {
    for (interaction, mut color, button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // Trigger disaster if not on cooldown and not already active
                if !disaster_state.is_active(button.disaster_type)
                    && !disaster_state.is_on_cooldown(button.disaster_type)
                {
                    // Start disaster with appropriate duration
                    let duration = match button.disaster_type {
                        DisasterType::Rain => 30.0,
                        DisasterType::Drought => 45.0,
                        DisasterType::ColdSnap => 25.0,
                        DisasterType::InvasiveSpecies => 40.0,
                    };
                    disaster_state.start_disaster(button.disaster_type, duration);
                }
                *color = ui_theme
                    .get_active_color(ui_theme.colors.surface_elevated)
                    .into();
            }
            Interaction::Hovered => {
                *color = ui_theme
                    .get_hover_color(ui_theme.colors.surface_elevated)
                    .into();
            }
            Interaction::None => {
                *color = ui_theme.colors.surface_elevated.into();
            }
        }
    }
}

/// Update disaster status indicators based on current disaster state
pub fn update_disaster_status_indicators(
    mut status_query: Query<(&mut BackgroundColor, &DisasterStatusIndicator)>,
    disaster_state: Res<DisasterState>,
    _ui_theme: Res<UITheme>,
) {
    for (mut color, indicator) in &mut status_query {
        *color = indicator
            .disaster_type
            .get_status_color(&disaster_state)
            .into();
    }
}

/// Handle keyboard shortcuts for disaster control
pub fn disaster_keyboard_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut disaster_state: ResMut<DisasterState>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        trigger_disaster_if_available(&mut disaster_state, DisasterType::Rain);
    }
    if keyboard_input.just_pressed(KeyCode::KeyD) {
        trigger_disaster_if_available(&mut disaster_state, DisasterType::Drought);
    }
    if keyboard_input.just_pressed(KeyCode::KeyC) {
        trigger_disaster_if_available(&mut disaster_state, DisasterType::ColdSnap);
    }
    if keyboard_input.just_pressed(KeyCode::KeyI) {
        trigger_disaster_if_available(&mut disaster_state, DisasterType::InvasiveSpecies);
    }
}

/// Helper function to trigger disaster if available
fn trigger_disaster_if_available(disaster_state: &mut DisasterState, disaster_type: DisasterType) {
    if !disaster_state.is_active(disaster_type) && !disaster_state.is_on_cooldown(disaster_type) {
        let duration = match disaster_type {
            DisasterType::Rain => 30.0,
            DisasterType::Drought => 45.0,
            DisasterType::ColdSnap => 25.0,
            DisasterType::InvasiveSpecies => 40.0,
        };
        disaster_state.start_disaster(disaster_type, duration);
    }
}

/// Get description text for each disaster type
fn get_disaster_description(disaster_type: DisasterType) -> &'static str {
    match disaster_type {
        DisasterType::Rain => "Increases soil moisture, affects ant movement",
        DisasterType::Drought => "Reduces soil moisture, stresses food sources",
        DisasterType::ColdSnap => "Lowers temperature, slows ant activity",
        DisasterType::InvasiveSpecies => "Introduces competing species, reduces food",
    }
}

use crate::components::DisasterStatusIndicator;
