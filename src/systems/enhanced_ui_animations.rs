use crate::components::{
    ColorAnimation, TransformAnimation, StyleAnimation, Tooltip, AccessibilityInfo,
    PlayPauseButton, SpeedSliderHandle, TimeControl,
};
use crate::systems::time_control::SpeedPresetButton;
use bevy::prelude::*;

/// System to update color animations for UI elements
pub fn update_color_animations_system(
    mut color_animation_query: Query<(&mut ColorAnimation, &mut BackgroundColor)>,
    time: Res<Time>,
) {
    for (mut animation, mut background_color) in color_animation_query.iter_mut() {
        if animation.is_active {
            animation.update(time.delta_seconds());
            background_color.0 = animation.current_color();
        }
    }
}

/// System to update transform animations for UI elements
pub fn update_transform_animations_system(
    mut transform_animation_query: Query<(&mut TransformAnimation, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut animation, mut transform) in transform_animation_query.iter_mut() {
        if animation.is_active {
            animation.update(time.delta_seconds());
            *transform = animation.current_transform();
        }
    }
}

/// System to update style animations for UI elements
pub fn update_style_animations_system(
    mut style_animation_query: Query<(&mut StyleAnimation, &mut Style)>,
    time: Res<Time>,
) {
    for (mut animation, mut style) in style_animation_query.iter_mut() {
        if animation.is_active {
            animation.update(time.delta_seconds());

            // Update style properties based on animation
            style.left = animation.current_left();
            // Add more style properties as needed
        }
    }
}

/// Enhanced button interaction system with smooth color animations
pub fn enhanced_button_interactions_system(
    mut button_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&mut ColorAnimation>,
            Option<&PlayPauseButton>,
            Option<&SpeedPresetButton>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    time_control: Res<TimeControl>,
) {
    for (interaction, mut background_color, color_animation, play_pause_button, speed_preset_button) in button_query.iter_mut() {
        let current_color = background_color.0;

        let target_color = match *interaction {
            Interaction::Pressed => {
                if play_pause_button.is_some() {
                    if time_control.is_paused {
                        Color::srgba(0.7, 0.5, 0.2, 0.95) // Amber for paused
                    } else {
                        Color::srgba(0.2, 0.7, 0.2, 0.95) // Green for playing
                    }
                } else if speed_preset_button.is_some() {
                    Color::srgba(0.5, 0.5, 0.8, 0.95) // Blue for speed buttons
                } else {
                    Color::srgba(0.4, 0.4, 0.4, 0.8) // Default pressed
                }
            }
            Interaction::Hovered => {
                if play_pause_button.is_some() {
                    if time_control.is_paused {
                        Color::srgba(0.6, 0.4, 0.15, 0.95) // Hover amber
                    } else {
                        Color::srgba(0.15, 0.6, 0.15, 0.95) // Hover green
                    }
                } else if speed_preset_button.is_some() {
                    Color::srgba(0.4, 0.4, 0.8, 0.9) // Hover blue
                } else {
                    Color::srgba(0.5, 0.5, 0.5, 0.8) // Default hover
                }
            }
            Interaction::None => {
                if play_pause_button.is_some() {
                    if time_control.is_paused {
                        Color::srgba(0.5, 0.3, 0.1, 0.8) // Normal amber
                    } else {
                        Color::srgba(0.1, 0.5, 0.1, 0.8) // Normal green
                    }
                } else if speed_preset_button.is_some() {
                    Color::srgba(0.3, 0.3, 0.6, 0.8) // Normal blue
                } else {
                    Color::srgba(0.3, 0.3, 0.3, 0.6) // Default normal
                }
            }
        };

        // Use existing animation component or set color directly
        if let Some(mut animation) = color_animation {
            animation.animate_to(target_color, current_color);
        } else {
            // For now, directly set the color without animation if no animation component exists
            background_color.0 = target_color;
        }
    }
}

/// System to handle smooth slider handle animations
pub fn animated_slider_handle_system(
    mut slider_handle_query: Query<
        (&mut Style, Option<&mut StyleAnimation>),
        With<SpeedSliderHandle>
    >,
    time_control: Res<TimeControl>,
) {
    for (mut style, style_animation) in slider_handle_query.iter_mut() {
        // Calculate target position based on speed
        let min_speed = 1.0;
        let max_speed = 100.0;
        let current_speed = time_control.speed_multiplier.clamp(min_speed, max_speed);
        let percentage = ((current_speed - min_speed) / (max_speed - min_speed)).clamp(0.0, 1.0);

        // Assume track width of 250px (should be calculated dynamically in real implementation)
        let track_width = 250.0;
        let target_position = Val::Px(percentage * track_width);

        // Use existing animation or create new one
        if let Some(mut animation) = style_animation {
            if animation.target_left != target_position {
                animation.animate_left(style.left, target_position, 0.3);
            }
        } else {
            // For now, directly set the position without animation if no animation component exists
            style.left = target_position;
        }
    }
}

/// System to handle tooltip display and management
pub fn tooltip_system(
    mut tooltip_query: Query<(&mut Tooltip, &Interaction), Changed<Interaction>>,
    time: Res<Time>,
) {
    // Update tooltip timers and visibility
    for (mut tooltip, interaction) in tooltip_query.iter_mut() {
        let is_hovered = matches!(interaction, Interaction::Hovered);
        tooltip.update(is_hovered, time.delta_seconds());

        // For now, just update tooltip state without spawning UI elements
        // In a full implementation, this would spawn/despawn tooltip UI entities
        if tooltip.is_visible && !tooltip.content.is_empty() {
            // Tooltip should be visible - in full implementation would spawn UI
            info!("Tooltip: {}", tooltip.content);
        }
    }
}


/// System to provide accessibility announcements for screen readers
pub fn accessibility_system(
    accessibility_query: Query<(&AccessibilityInfo, &Interaction), Changed<Interaction>>,
) {
    for (accessibility_info, interaction) in accessibility_query.iter() {
        match interaction {
            Interaction::Pressed => {
                // Log accessibility information for screen readers
                if accessibility_info.focusable {
                    info!(
                        "Accessibility: {} activated. Role: {:?}. State: {}",
                        accessibility_info.aria_label,
                        accessibility_info.role,
                        accessibility_info.state_description
                    );
                }
            }
            Interaction::Hovered => {
                if accessibility_info.focusable {
                    info!(
                        "Accessibility: {} focused. {}",
                        accessibility_info.aria_label,
                        accessibility_info.state_description
                    );
                }
            }
            _ => {}
        }
    }
}

/// System to handle keyboard focus navigation
pub fn keyboard_navigation_system(
    input: Res<ButtonInput<KeyCode>>,
    mut focusable_query: Query<(Entity, &AccessibilityInfo, &mut BackgroundColor), With<Button>>,
    mut current_focus: Local<Option<Entity>>,
) {
    // Handle Tab key for focus navigation
    if input.just_pressed(KeyCode::Tab) {
        let focusable_entities: Vec<_> = focusable_query
            .iter()
            .filter(|(_, accessibility, _)| accessibility.focusable)
            .collect();

        if !focusable_entities.is_empty() {
            let current_index = if let Some(focused) = *current_focus {
                focusable_entities
                    .iter()
                    .position(|(entity, _, _)| *entity == focused)
                    .unwrap_or(0)
            } else {
                0
            };

            // Move to next focusable element
            let next_index = (current_index + 1) % focusable_entities.len();
            let (next_entity, _, _) = focusable_entities[next_index];
            *current_focus = Some(next_entity);

            // Update visual focus indicator
            for (entity, _, mut background_color) in focusable_query.iter_mut() {
                if entity == next_entity {
                    // Add focus outline (simplified)
                    background_color.0 = Color::srgba(0.8, 0.8, 1.0, 0.3);
                } else {
                    // Remove focus from others (this would need proper state management)
                    background_color.0 = Color::srgba(0.0, 0.0, 0.0, 0.0);
                }
            }
        }
    }

    // Handle Enter key to activate focused element
    if input.just_pressed(KeyCode::Enter) {
        if let Some(focused_entity) = *current_focus {
            // Trigger activation for focused element
            info!("Keyboard activation for entity: {:?}", focused_entity);
        }
    }
}

/// Enhanced speed display with smooth transitions
pub fn enhanced_speed_display_system(
    time_control: Res<TimeControl>,
    mut speed_display_query: Query<(&mut Text, Option<&mut ColorAnimation>), With<crate::components::SpeedDisplay>>,
) {
    for (mut text, color_animation) in speed_display_query.iter_mut() {
        // Update text content
        let new_text = if time_control.is_paused {
            "Speed: PAUSED".to_string()
        } else {
            format!("Speed: {:.1}x", time_control.speed_multiplier)
        };

        let new_color = if time_control.is_paused {
            Color::srgb(1.0, 0.6, 0.6) // Red for paused
        } else {
            Color::srgb(0.8, 1.0, 0.8) // Green for running
        };

        // Update text if changed
        if text.sections[0].value != new_text {
            text.sections[0].value = new_text;
        }

        // Animate color change if needed
        let current_color = text.sections[0].style.color;
        if current_color != new_color {
            if let Some(mut animation) = color_animation {
                animation.animate_to(new_color, current_color);
            } else {
                // For now, directly set the color without animation if no animation component exists
                text.sections[0].style.color = new_color;
            }
        }
    }
}