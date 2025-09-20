use crate::components::{
    SettingItem, SettingType, SettingsAction, SettingsButton, SettingsCategory,
    SettingsCategoryType, SettingsPanel, SettingsToggle, UserSettings,
};
use bevy::prelude::*;

/// Setup settings panel UI with tabbed interface
pub fn setup_settings_panel(mut commands: Commands) {
    // Main settings panel container - initially hidden
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Px(10.0),
                top: Val::Px(10.0),
                width: Val::Px(400.0),
                max_height: Val::Px(600.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                row_gap: Val::Px(15.0),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: Color::srgba(0.1, 0.1, 0.1, 0.95).into(),
            border_color: Color::srgb(0.4, 0.4, 0.4).into(),
            border_radius: BorderRadius::all(Val::Px(10.0)),
            visibility: Visibility::Hidden, // Start hidden
            ..default()
        })
        .with_children(|parent| {
            // Panel title with close button
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        width: Val::Percent(100.0),
                        margin: UiRect::bottom(Val::Px(10.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|header_parent| {
                    // Title
                    header_parent.spawn(TextBundle::from_section(
                        "Settings",
                        TextStyle {
                            font_size: 24.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));

                    // Close button
                    header_parent
                        .spawn(ButtonBundle {
                            style: Style {
                                width: Val::Px(30.0),
                                height: Val::Px(30.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(1.0)),
                                ..default()
                            },
                            background_color: Color::srgb(0.6, 0.2, 0.2).into(),
                            border_color: Color::srgb(0.8, 0.4, 0.4).into(),
                            border_radius: BorderRadius::all(Val::Px(4.0)),
                            ..default()
                        })
                        .with_children(|button_parent| {
                            button_parent.spawn(TextBundle::from_section(
                                "×",
                                TextStyle {
                                    font_size: 18.0,
                                    color: Color::WHITE,
                                    ..default()
                                },
                            ));
                        })
                        .insert(SettingsButton {
                            action: SettingsAction::ClosePanel,
                        });
                });

            // Tab navigation
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        width: Val::Percent(100.0),
                        column_gap: Val::Px(5.0),
                        margin: UiRect::bottom(Val::Px(15.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|tabs_parent| {
                    let categories = [
                        (SettingsCategoryType::Visual, "Visual", true),
                        (SettingsCategoryType::Accessibility, "Accessibility", false),
                        (SettingsCategoryType::Application, "Application", false),
                    ];

                    for (category_type, label, is_active) in categories {
                        let bg_color = if is_active {
                            Color::srgb(0.3, 0.5, 0.7)
                        } else {
                            Color::srgb(0.2, 0.2, 0.2)
                        };

                        tabs_parent
                            .spawn(ButtonBundle {
                                style: Style {
                                    flex_grow: 1.0,
                                    height: Val::Px(35.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    border: UiRect::all(Val::Px(1.0)),
                                    ..default()
                                },
                                background_color: bg_color.into(),
                                border_color: Color::srgb(0.4, 0.4, 0.4).into(),
                                border_radius: BorderRadius::all(Val::Px(4.0)),
                                ..default()
                            })
                            .with_children(|tab_parent| {
                                tab_parent.spawn(TextBundle::from_section(
                                    label,
                                    TextStyle {
                                        font_size: 14.0,
                                        color: Color::WHITE,
                                        ..default()
                                    },
                                ));
                            })
                            .insert(SettingsCategory {
                                category: category_type,
                                is_active,
                            });
                    }
                });

            // Settings content area (will be populated based on active tab)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        width: Val::Percent(100.0),
                        flex_grow: 1.0,
                        row_gap: Val::Px(12.0),
                        padding: UiRect::all(Val::Px(10.0)),
                        overflow: Overflow::clip_y(),
                        ..default()
                    },
                    background_color: Color::srgba(0.05, 0.05, 0.05, 0.8).into(),
                    border_radius: BorderRadius::all(Val::Px(6.0)),
                    ..default()
                })
                .with_children(|content_parent| {
                    setup_visual_settings_content(content_parent);
                });

            // Action buttons at bottom
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        width: Val::Percent(100.0),
                        column_gap: Val::Px(10.0),
                        margin: UiRect::top(Val::Px(15.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|buttons_parent| {
                    let buttons = [
                        (
                            SettingsAction::ResetToDefaults,
                            "Reset to Defaults",
                            Color::srgb(0.6, 0.3, 0.3),
                        ),
                        (
                            SettingsAction::SaveSettings,
                            "Save Settings",
                            Color::srgb(0.2, 0.6, 0.2),
                        ),
                        (
                            SettingsAction::ApplySettings,
                            "Apply",
                            Color::srgb(0.3, 0.5, 0.7),
                        ),
                    ];

                    for (action, label, color) in buttons {
                        buttons_parent
                            .spawn(ButtonBundle {
                                style: Style {
                                    flex_grow: 1.0,
                                    height: Val::Px(35.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    border: UiRect::all(Val::Px(1.0)),
                                    ..default()
                                },
                                background_color: color.into(),
                                border_color: Color::srgb(0.5, 0.5, 0.5).into(),
                                border_radius: BorderRadius::all(Val::Px(4.0)),
                                ..default()
                            })
                            .with_children(|button_parent| {
                                button_parent.spawn(TextBundle::from_section(
                                    label,
                                    TextStyle {
                                        font_size: 12.0,
                                        color: Color::WHITE,
                                        ..default()
                                    },
                                ));
                            })
                            .insert(SettingsButton { action });
                    }
                });
        })
        .insert(SettingsPanel);
}

/// Setup visual settings content (default tab)
fn setup_visual_settings_content(parent: &mut ChildBuilder) {
    // Visual Effects Toggle
    setup_setting_item(
        parent,
        "Visual Effects",
        "Enable particle effects and visual overlays",
        SettingType::VisualEffectsToggle,
    );

    // UI Scale Slider
    setup_setting_item(
        parent,
        "UI Scale",
        "Adjust the size of interface elements",
        SettingType::UIScale,
    );

    // Color Theme Selection
    setup_setting_item(
        parent,
        "Color Theme",
        "Choose interface color scheme",
        SettingType::ColorTheme,
    );

    // Performance Mode Toggle
    setup_setting_item(
        parent,
        "Performance Mode",
        "Reduce visual effects for better performance",
        SettingType::PerformanceMode,
    );
}

/// Helper function to create a setting item with label and description
fn setup_setting_item(
    parent: &mut ChildBuilder,
    label: &str,
    description: &str,
    setting_type: SettingType,
) {
    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(8.0)),
                row_gap: Val::Px(5.0),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            background_color: Color::srgba(0.1, 0.1, 0.1, 0.5).into(),
            border_color: Color::srgb(0.3, 0.3, 0.3).into(),
            border_radius: BorderRadius::all(Val::Px(4.0)),
            ..default()
        })
        .with_children(|item_parent| {
            // Setting label
            item_parent.spawn(TextBundle::from_section(
                label,
                TextStyle {
                    font_size: 16.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));

            // Setting description
            item_parent.spawn(TextBundle::from_section(
                description,
                TextStyle {
                    font_size: 12.0,
                    color: Color::srgb(0.8, 0.8, 0.8),
                    ..default()
                },
            ));

            // Setting control (will be customized based on setting type)
            match setting_type {
                SettingType::VisualEffectsToggle | SettingType::PerformanceMode => {
                    setup_toggle_control(item_parent, setting_type.clone());
                }
                SettingType::UIScale => {
                    setup_slider_control(item_parent, setting_type.clone());
                }
                SettingType::ColorTheme => {
                    setup_dropdown_control(item_parent, setting_type.clone());
                }
                _ => {
                    // Default control for other types
                    setup_toggle_control(item_parent, setting_type.clone());
                }
            }
        })
        .insert(SettingItem { setting_type });
}

/// Setup toggle control for boolean settings
fn setup_toggle_control(parent: &mut ChildBuilder, setting_type: SettingType) {
    parent
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(60.0),
                height: Val::Px(30.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.0)),
                margin: UiRect::top(Val::Px(5.0)),
                ..default()
            },
            background_color: Color::srgb(0.2, 0.6, 0.2).into(), // Default: ON
            border_color: Color::srgb(0.4, 0.8, 0.4).into(),
            border_radius: BorderRadius::all(Val::Px(15.0)),
            ..default()
        })
        .with_children(|toggle_parent| {
            toggle_parent.spawn(TextBundle::from_section(
                "ON",
                TextStyle {
                    font_size: 12.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        })
        .insert(SettingItem { setting_type });
}

/// Setup slider control for numeric settings
fn setup_slider_control(parent: &mut ChildBuilder, setting_type: SettingType) {
    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                column_gap: Val::Px(10.0),
                margin: UiRect::top(Val::Px(5.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|slider_parent| {
            // Slider track
            slider_parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(70.0),
                        height: Val::Px(6.0),
                        ..default()
                    },
                    background_color: Color::srgb(0.3, 0.3, 0.3).into(),
                    border_radius: BorderRadius::all(Val::Px(3.0)),
                    ..default()
                })
                .with_children(|track_parent| {
                    // Slider handle (positioned based on current value)
                    track_parent.spawn(NodeBundle {
                        style: Style {
                            width: Val::Px(16.0),
                            height: Val::Px(16.0),
                            position_type: PositionType::Absolute,
                            left: Val::Percent(0.0), // Will be updated based on value
                            top: Val::Px(-5.0),
                            ..default()
                        },
                        background_color: Color::srgb(0.8, 0.8, 1.0).into(),
                        border_radius: BorderRadius::all(Val::Px(8.0)),
                        ..default()
                    });
                });

            // Value display
            slider_parent.spawn(TextBundle::from_section(
                "100%",
                TextStyle {
                    font_size: 12.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ));
        })
        .insert(SettingItem { setting_type });
}

/// Setup dropdown control for selection settings
fn setup_dropdown_control(parent: &mut ChildBuilder, setting_type: SettingType) {
    parent
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(30.0),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(Val::Px(10.0)),
                border: UiRect::all(Val::Px(1.0)),
                margin: UiRect::top(Val::Px(5.0)),
                ..default()
            },
            background_color: Color::srgb(0.2, 0.2, 0.2).into(),
            border_color: Color::srgb(0.4, 0.4, 0.4).into(),
            border_radius: BorderRadius::all(Val::Px(4.0)),
            ..default()
        })
        .with_children(|dropdown_parent| {
            dropdown_parent.spawn(TextBundle::from_section(
                "Default Theme",
                TextStyle {
                    font_size: 12.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));

            dropdown_parent.spawn(TextBundle::from_section(
                "▼",
                TextStyle {
                    font_size: 10.0,
                    color: Color::srgb(0.8, 0.8, 0.8),
                    ..default()
                },
            ));
        })
        .insert(SettingItem { setting_type });
}

/// Setup settings toggle button in main UI
pub fn setup_settings_toggle_button(mut commands: Commands) {
    commands
        .spawn(ButtonBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                right: Val::Px(10.0),
                width: Val::Px(40.0),
                height: Val::Px(40.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: Color::srgba(0.2, 0.2, 0.2, 0.8).into(),
            border_color: Color::srgb(0.4, 0.4, 0.4).into(),
            border_radius: BorderRadius::all(Val::Px(6.0)),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "⚙️",
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        })
        .insert(SettingsToggle { is_visible: false });
}

/// System for handling settings toggle input (S key)
pub fn settings_toggle_input_system(
    input: Res<ButtonInput<KeyCode>>,
    mut settings_toggle_query: Query<&mut SettingsToggle>,
    mut panel_query: Query<&mut Visibility, With<SettingsPanel>>,
) {
    if input.just_pressed(KeyCode::KeyS) {
        if let Ok(mut settings_toggle) = settings_toggle_query.get_single_mut() {
            settings_toggle.is_visible = !settings_toggle.is_visible;

            if let Ok(mut panel_visibility) = panel_query.get_single_mut() {
                *panel_visibility = if settings_toggle.is_visible {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                };
            }
        }
    }
}

/// System for handling settings button interactions
pub fn handle_settings_interactions_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&SettingsButton>,
            Option<&SettingsToggle>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut panel_query: Query<&mut Visibility, With<SettingsPanel>>,
    mut settings_toggle_query: Query<&mut SettingsToggle, Without<Button>>,
    mut user_settings: ResMut<UserSettings>,
) {
    for (interaction, mut background_color, settings_button, settings_toggle_button) in
        &mut interaction_query
    {
        match *interaction {
            Interaction::Pressed => {
                // Handle settings button actions
                if let Some(settings_button) = settings_button {
                    match settings_button.action {
                        SettingsAction::ClosePanel => {
                            if let Ok(mut panel_visibility) = panel_query.get_single_mut() {
                                *panel_visibility = Visibility::Hidden;
                            }
                            if let Ok(mut toggle) = settings_toggle_query.get_single_mut() {
                                toggle.is_visible = false;
                            }
                        }
                        SettingsAction::ResetToDefaults => {
                            *user_settings = UserSettings::default();
                            info!("Settings reset to defaults");
                        }
                        SettingsAction::SaveSettings => {
                            // TODO: Implement save to file
                            info!("Settings saved");
                        }
                        SettingsAction::ApplySettings => {
                            // TODO: Apply settings to systems
                            info!("Settings applied");
                        }
                        SettingsAction::LoadSettings => {
                            // TODO: Implement load from file
                            info!("Settings loaded");
                        }
                    }
                }

                // Handle settings toggle button
                if settings_toggle_button.is_some() {
                    if let Ok(mut settings_toggle) = settings_toggle_query.get_single_mut() {
                        settings_toggle.is_visible = !settings_toggle.is_visible;

                        if let Ok(mut panel_visibility) = panel_query.get_single_mut() {
                            *panel_visibility = if settings_toggle.is_visible {
                                Visibility::Visible
                            } else {
                                Visibility::Hidden
                            };
                        }
                    }
                }
            }
            Interaction::Hovered => {
                // Visual feedback for hover state
                if settings_button.is_some() || settings_toggle_button.is_some() {
                    *background_color = Color::srgb(0.3, 0.3, 0.3).into();
                }
            }
            Interaction::None => {
                // Reset to default colors
                if let Some(settings_button) = settings_button {
                    *background_color = match settings_button.action {
                        SettingsAction::ResetToDefaults => Color::srgb(0.6, 0.3, 0.3).into(),
                        SettingsAction::SaveSettings => Color::srgb(0.2, 0.6, 0.2).into(),
                        SettingsAction::ApplySettings => Color::srgb(0.3, 0.5, 0.7).into(),
                        SettingsAction::ClosePanel => Color::srgb(0.6, 0.2, 0.2).into(),
                        _ => Color::srgb(0.2, 0.2, 0.2).into(),
                    };
                } else if settings_toggle_button.is_some() {
                    *background_color = Color::srgba(0.2, 0.2, 0.2, 0.8).into();
                }
            }
        }
    }
}
