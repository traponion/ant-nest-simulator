use crate::components::{
    SettingItem, SettingType, SettingsAction, SettingsButton, SettingsCategory,
    SettingsCategoryType, SettingsPanel, SettingsToggle, UITheme, UserSettings,
};
use bevy::prelude::*;

/// Setup enhanced settings panel UI with tabbed interface and theme system
pub fn setup_settings_panel(mut commands: Commands, theme: Res<UITheme>) {
    // Main settings panel container with enhanced theme styling
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Px(theme.spacing.md),
                top: Val::Px(theme.spacing.md),
                width: Val::Px(420.0), // Slightly wider for better content fit
                max_height: Val::Px(650.0), // More height for enhanced content
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(theme.spacing.xl)),
                row_gap: Val::Px(theme.spacing.lg),
                border: UiRect::all(Val::Px(theme.borders.width_medium)),
                ..default()
            },
            background_color: theme.colors.surface_primary.into(),
            border_color: theme.colors.border_primary.into(),
            border_radius: BorderRadius::all(Val::Px(theme.borders.radius_large)),
            visibility: Visibility::Hidden, // Start hidden
            ..default()
        })
        .with_children(|parent| {
            // Enhanced panel title with close button using theme system
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        width: Val::Percent(100.0),
                        margin: UiRect::bottom(Val::Px(theme.spacing.md)),
                        padding: UiRect::all(Val::Px(theme.spacing.sm)),
                        border: UiRect::bottom(Val::Px(theme.borders.width_thin)),
                        ..default()
                    },
                    background_color: Color::NONE.into(),
                    border_color: theme.colors.border_secondary.into(),
                    ..default()
                })
                .with_children(|header_parent| {
                    // Enhanced title with theme typography
                    header_parent.spawn(TextBundle::from_section(
                        "⚙️ Settings",
                        TextStyle {
                            font_size: theme.typography.heading_large,
                            color: theme.colors.text_primary,
                            ..default()
                        },
                    ));

                    // Enhanced close button with theme styling
                    header_parent
                        .spawn(ButtonBundle {
                            style: theme.create_button_style(Val::Px(36.0), Val::Px(36.0)),
                            background_color: theme.colors.action_danger.into(),
                            border_color: theme.colors.border_primary.into(),
                            border_radius: BorderRadius::all(Val::Px(theme.borders.radius_medium)),
                            ..default()
                        })
                        .with_children(|button_parent| {
                            button_parent.spawn(TextBundle::from_section(
                                "✕",
                                TextStyle {
                                    font_size: theme.typography.body_large,
                                    color: theme.colors.text_primary,
                                    ..default()
                                },
                            ));
                        })
                        .insert(SettingsButton {
                            action: SettingsAction::ClosePanel,
                        });
                });

            // Enhanced tab navigation with theme styling
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        width: Val::Percent(100.0),
                        column_gap: Val::Px(theme.spacing.xs),
                        margin: UiRect::bottom(Val::Px(theme.spacing.lg)),
                        padding: UiRect::all(Val::Px(theme.spacing.xs)),
                        border: UiRect::all(Val::Px(theme.borders.width_thin)),
                        ..default()
                    },
                    background_color: theme.colors.surface_secondary.into(),
                    border_color: theme.colors.border_secondary.into(),
                    border_radius: BorderRadius::all(Val::Px(theme.borders.radius_medium)),
                    ..default()
                })
                .with_children(|tabs_parent| {
                    let categories = [
                        (SettingsCategoryType::Visual, "🎨 Visual", true),
                        (
                            SettingsCategoryType::Accessibility,
                            "♿ Accessibility",
                            false,
                        ),
                        (SettingsCategoryType::Application, "⚙️ Application", false),
                    ];

                    for (category_type, label, is_active) in categories {
                        let (bg_color, text_color) = if is_active {
                            (theme.colors.action_primary, theme.colors.text_primary)
                        } else {
                            (theme.colors.surface_elevated, theme.colors.text_secondary)
                        };

                        tabs_parent
                            .spawn(ButtonBundle {
                                style: Style {
                                    flex_grow: 1.0,
                                    height: Val::Px(42.0), // Slightly taller for better touch targets
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    border: UiRect::all(Val::Px(theme.borders.width_thin)),
                                    margin: UiRect::all(Val::Px(theme.spacing.xs / 2.0)),
                                    ..default()
                                },
                                background_color: bg_color.into(),
                                border_color: if is_active {
                                    theme.colors.border_focus
                                } else {
                                    theme.colors.border_primary
                                }
                                .into(),
                                border_radius: BorderRadius::all(Val::Px(
                                    theme.borders.radius_small,
                                )),
                                ..default()
                            })
                            .with_children(|tab_parent| {
                                tab_parent.spawn(TextBundle::from_section(
                                    label,
                                    TextStyle {
                                        font_size: theme.typography.body_medium,
                                        color: text_color,
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

            // Enhanced settings content area with theme styling
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        width: Val::Percent(100.0),
                        flex_grow: 1.0,
                        row_gap: Val::Px(theme.spacing.md),
                        padding: UiRect::all(Val::Px(theme.spacing.lg)),
                        border: UiRect::all(Val::Px(theme.borders.width_thin)),
                        overflow: Overflow::clip_y(),
                        ..default()
                    },
                    background_color: theme.colors.surface_secondary.into(),
                    border_color: theme.colors.border_secondary.into(),
                    border_radius: BorderRadius::all(Val::Px(theme.borders.radius_medium)),
                    ..default()
                })
                .with_children(|content_parent| {
                    setup_themed_visual_settings_content(content_parent, &theme);
                });

            // Enhanced action buttons at bottom with theme styling
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        width: Val::Percent(100.0),
                        column_gap: Val::Px(theme.spacing.sm),
                        margin: UiRect::top(Val::Px(theme.spacing.lg)),
                        padding: UiRect::all(Val::Px(theme.spacing.sm)),
                        border: UiRect::all(Val::Px(theme.borders.width_thin)),
                        ..default()
                    },
                    background_color: theme.colors.surface_elevated.into(),
                    border_color: theme.colors.border_secondary.into(),
                    border_radius: BorderRadius::all(Val::Px(theme.borders.radius_medium)),
                    ..default()
                })
                .with_children(|buttons_parent| {
                    let buttons = [
                        (
                            SettingsAction::ResetToDefaults,
                            "💫 Reset",
                            theme.colors.action_danger,
                        ),
                        (
                            SettingsAction::SaveSettings,
                            "💾 Save",
                            theme.colors.action_success,
                        ),
                        (
                            SettingsAction::ApplySettings,
                            "✓ Apply",
                            theme.colors.action_primary,
                        ),
                    ];

                    for (action, label, color) in buttons {
                        buttons_parent
                            .spawn(ButtonBundle {
                                style: theme.create_button_style(Val::Auto, Val::Px(40.0)),
                                background_color: color.into(),
                                border_color: theme.colors.border_primary.into(),
                                border_radius: BorderRadius::all(Val::Px(
                                    theme.borders.radius_small,
                                )),
                                ..default()
                            })
                            .with_children(|button_parent| {
                                button_parent.spawn(TextBundle::from_section(
                                    label,
                                    TextStyle {
                                        font_size: theme.typography.body_small,
                                        color: theme.colors.text_primary,
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

/// Setup enhanced visual settings content with theme styling (default tab)
fn setup_themed_visual_settings_content(parent: &mut ChildBuilder, theme: &UITheme) {
    // Visual Effects Toggle with enhanced styling
    setup_themed_setting_item(
        parent,
        "🎆 Visual Effects",
        "Enable particle effects and visual overlays for enhanced experience",
        SettingType::VisualEffectsToggle,
        theme,
    );

    // UI Scale Slider with enhanced styling
    setup_themed_setting_item(
        parent,
        "🔍 UI Scale",
        "Adjust the size of interface elements for better accessibility",
        SettingType::UIScale,
        theme,
    );

    // Color Theme Selection with enhanced styling
    setup_themed_setting_item(
        parent,
        "🎨 Color Theme",
        "Choose interface color scheme that suits your preferences",
        SettingType::ColorTheme,
        theme,
    );

    // Performance Mode Toggle with enhanced styling
    setup_themed_setting_item(
        parent,
        "⚡ Performance Mode",
        "Reduce visual effects for better performance on slower devices",
        SettingType::PerformanceMode,
        theme,
    );
}

/// Enhanced helper function to create a themed setting item with label and description
fn setup_themed_setting_item(
    parent: &mut ChildBuilder,
    label: &str,
    description: &str,
    setting_type: SettingType,
    theme: &UITheme,
) {
    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(theme.spacing.md)),
                row_gap: Val::Px(theme.spacing.sm),
                border: UiRect::all(Val::Px(theme.borders.width_thin)),
                margin: UiRect::bottom(Val::Px(theme.spacing.sm)),
                ..default()
            },
            background_color: theme.colors.surface_elevated.into(),
            border_color: theme.colors.border_secondary.into(),
            border_radius: BorderRadius::all(Val::Px(theme.borders.radius_small)),
            ..default()
        })
        .with_children(|item_parent| {
            // Enhanced setting label with theme typography
            item_parent.spawn(TextBundle::from_section(
                label,
                TextStyle {
                    font_size: theme.typography.body_large,
                    color: theme.colors.text_primary,
                    ..default()
                },
            ));

            // Enhanced setting description with theme colors
            item_parent.spawn(TextBundle::from_section(
                description,
                TextStyle {
                    font_size: theme.typography.body_small,
                    color: theme.colors.text_secondary,
                    ..default()
                },
            ));

            // Enhanced setting control with theme styling
            match setting_type {
                SettingType::VisualEffectsToggle | SettingType::PerformanceMode => {
                    setup_themed_toggle_control(item_parent, setting_type.clone(), theme);
                }
                SettingType::UIScale => {
                    setup_themed_slider_control(item_parent, setting_type.clone(), theme);
                }
                SettingType::ColorTheme => {
                    setup_themed_dropdown_control(item_parent, setting_type.clone(), theme);
                }
                _ => {
                    // Default control for other types with theme styling
                    setup_themed_toggle_control(item_parent, setting_type.clone(), theme);
                }
            }
        })
        .insert(SettingItem { setting_type });
}

/// Setup enhanced toggle control for boolean settings with theme styling
fn setup_themed_toggle_control(
    parent: &mut ChildBuilder,
    setting_type: SettingType,
    theme: &UITheme,
) {
    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                width: Val::Percent(100.0),
                margin: UiRect::top(Val::Px(theme.spacing.sm)),
                ..default()
            },
            ..default()
        })
        .with_children(|toggle_container| {
            // Toggle status text
            toggle_container.spawn(TextBundle::from_section(
                "Enabled",
                TextStyle {
                    font_size: theme.typography.body_small,
                    color: theme.colors.text_secondary,
                    ..default()
                },
            ));

            // Enhanced toggle button with theme styling
            toggle_container
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(70.0),
                        height: Val::Px(36.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(theme.borders.width_medium)),
                        ..default()
                    },
                    background_color: theme.colors.action_success.into(),
                    border_color: theme.colors.border_primary.into(),
                    border_radius: BorderRadius::all(Val::Px(theme.borders.radius_round)),
                    ..default()
                })
                .with_children(|toggle_parent| {
                    toggle_parent.spawn(TextBundle::from_section(
                        "✓ ON",
                        TextStyle {
                            font_size: theme.typography.body_small,
                            color: theme.colors.text_primary,
                            ..default()
                        },
                    ));
                })
                .insert(SettingItem { setting_type });
        });
}

/// Setup enhanced slider control for numeric settings with theme styling
fn setup_themed_slider_control(
    parent: &mut ChildBuilder,
    setting_type: SettingType,
    theme: &UITheme,
) {
    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                row_gap: Val::Px(theme.spacing.sm),
                margin: UiRect::top(Val::Px(theme.spacing.sm)),
                ..default()
            },
            ..default()
        })
        .with_children(|slider_container| {
            // Value display and slider container
            slider_container
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        width: Val::Percent(100.0),
                        column_gap: Val::Px(theme.spacing.md),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|slider_parent| {
                    // Enhanced slider track with theme styling
                    slider_parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(75.0),
                                height: Val::Px(8.0), // Slightly thicker for better usability
                                border: UiRect::all(Val::Px(theme.borders.width_thin)),
                                ..default()
                            },
                            background_color: theme.colors.surface_elevated.into(),
                            border_color: theme.colors.border_secondary.into(),
                            border_radius: BorderRadius::all(Val::Px(theme.borders.radius_small)),
                            ..default()
                        })
                        .with_children(|track_parent| {
                            // Enhanced slider handle with better visibility
                            track_parent.spawn(NodeBundle {
                                style: Style {
                                    width: Val::Px(20.0), // Larger handle for better touch targets
                                    height: Val::Px(20.0),
                                    position_type: PositionType::Absolute,
                                    left: Val::Percent(100.0), // Default to 100% (will be updated based on value)
                                    top: Val::Px(-6.0),        // Center on track
                                    border: UiRect::all(Val::Px(theme.borders.width_medium)),
                                    ..default()
                                },
                                background_color: theme.colors.accent_blue.into(),
                                border_color: theme.colors.text_primary.into(),
                                border_radius: BorderRadius::all(Val::Px(
                                    theme.borders.radius_round,
                                )),
                                ..default()
                            });
                        });

                    // Enhanced value display with theme styling
                    slider_parent
                        .spawn(NodeBundle {
                            style: Style {
                                padding: UiRect::all(Val::Px(theme.spacing.xs)),
                                border: UiRect::all(Val::Px(theme.borders.width_thin)),
                                min_width: Val::Px(50.0),
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            background_color: theme.colors.surface_secondary.into(),
                            border_color: theme.colors.border_secondary.into(),
                            border_radius: BorderRadius::all(Val::Px(theme.borders.radius_small)),
                            ..default()
                        })
                        .with_children(|value_parent| {
                            value_parent.spawn(TextBundle::from_section(
                                "100%",
                                TextStyle {
                                    font_size: theme.typography.body_small,
                                    color: theme.colors.text_accent,
                                    ..default()
                                },
                            ));
                        });
                });
        })
        .insert(SettingItem { setting_type });
}

/// Setup enhanced dropdown control for selection settings with theme styling
fn setup_themed_dropdown_control(
    parent: &mut ChildBuilder,
    setting_type: SettingType,
    theme: &UITheme,
) {
    parent
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(40.0), // Taller for better touch targets
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(Val::Px(theme.spacing.md)),
                border: UiRect::all(Val::Px(theme.borders.width_medium)),
                margin: UiRect::top(Val::Px(theme.spacing.sm)),
                ..default()
            },
            background_color: theme.colors.surface_elevated.into(),
            border_color: theme.colors.border_primary.into(),
            border_radius: BorderRadius::all(Val::Px(theme.borders.radius_small)),
            ..default()
        })
        .with_children(|dropdown_parent| {
            // Current selection with icon
            dropdown_parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(theme.spacing.xs),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|selection_parent| {
                    selection_parent.spawn(TextBundle::from_section(
                        "🎨",
                        TextStyle {
                            font_size: theme.typography.body_medium,
                            color: theme.colors.text_accent,
                            ..default()
                        },
                    ));

                    selection_parent.spawn(TextBundle::from_section(
                        "Default Theme",
                        TextStyle {
                            font_size: theme.typography.body_small,
                            color: theme.colors.text_primary,
                            ..default()
                        },
                    ));
                });

            // Dropdown arrow with enhanced styling
            dropdown_parent
                .spawn(NodeBundle {
                    style: Style {
                        padding: UiRect::all(Val::Px(theme.spacing.xs)),
                        border: UiRect::all(Val::Px(theme.borders.width_thin)),
                        ..default()
                    },
                    background_color: theme.colors.surface_secondary.into(),
                    border_color: theme.colors.border_secondary.into(),
                    border_radius: BorderRadius::all(Val::Px(theme.borders.radius_small)),
                    ..default()
                })
                .with_children(|arrow_parent| {
                    arrow_parent.spawn(TextBundle::from_section(
                        "▼",
                        TextStyle {
                            font_size: theme.typography.caption,
                            color: theme.colors.text_secondary,
                            ..default()
                        },
                    ));
                });
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
