use crate::components::{UIAnimation, GlowEffect, FadeAnimation, FocusIndicator, AccessibilityFeatures};
use bevy::prelude::*;

/// System to handle UI animation transitions
pub fn ui_animation_system(
    mut query: Query<(&Interaction, &mut UIAnimation, &mut Transform), (Changed<Interaction>, With<Button>)>,
    time: Res<Time>,
) {
    for (interaction, mut animation, mut transform) in &mut query {
        // Update target scale based on interaction
        let new_target = match *interaction {
            Interaction::Hovered => animation.hover_scale,
            Interaction::Pressed => animation.press_scale,
            Interaction::None => 1.0,
        };

        if (animation.target_scale - new_target).abs() > 0.001 {
            animation.target_scale = new_target;
            animation.is_animating = true;
        }

        // Animate towards target scale
        if animation.is_animating {
            let delta = time.delta_seconds();
            let speed = 1.0 / animation.transition_duration;

            animation.current_scale = animation.current_scale.lerp(
                animation.target_scale,
                speed * delta
            );

            // Update transform scale
            transform.scale = Vec3::splat(animation.current_scale);

            // Stop animating when close enough to target
            if (animation.current_scale - animation.target_scale).abs() < 0.001 {
                animation.current_scale = animation.target_scale;
                animation.is_animating = false;
                transform.scale = Vec3::splat(animation.current_scale);
            }
        }
    }
}

/// System to handle smooth UI animation updates for non-interaction based animations
pub fn ui_animation_update_system(
    mut query: Query<(&mut UIAnimation, &mut Transform), Without<Button>>,
    time: Res<Time>,
) {
    for (mut animation, mut transform) in &mut query {
        if animation.is_animating {
            let delta = time.delta_seconds();
            let speed = 1.0 / animation.transition_duration;

            animation.current_scale = animation.current_scale.lerp(
                animation.target_scale,
                speed * delta
            );

            transform.scale = Vec3::splat(animation.current_scale);

            if (animation.current_scale - animation.target_scale).abs() < 0.001 {
                animation.current_scale = animation.target_scale;
                animation.is_animating = false;
                transform.scale = Vec3::splat(animation.current_scale);
            }
        }
    }
}

/// System to handle glow effects and pulsing animations
pub fn glow_effect_system(
    mut query: Query<(&mut GlowEffect, &mut BackgroundColor)>,
    time: Res<Time>,
) {
    for (mut glow, mut background_color) in &mut query {
        if glow.is_active {
            let time_factor = time.elapsed_seconds() * glow.pulse_speed;
            let pulse = (time_factor.sin() * 0.5 + 0.5) * glow.intensity;

            // Apply glow effect by modifying background color brightness
            let base_color = glow.color;
            let glowed_color = Color::rgb(
                (base_color.r() + pulse).min(1.0),
                (base_color.g() + pulse).min(1.0),
                (base_color.b() + pulse).min(1.0),
            );

            *background_color = glowed_color.into();
        }
    }
}

/// System to handle fade in/out animations
pub fn fade_animation_system(
    mut query: Query<(&mut FadeAnimation, &mut BackgroundColor)>,
    time: Res<Time>,
) {
    for (mut fade, mut background_color) in &mut query {
        if fade.is_playing {
            fade.elapsed += time.delta_seconds();
            let progress = (fade.elapsed / fade.duration).min(1.0);

            // Interpolate between start and target alpha
            fade.current_alpha = fade.start_alpha.lerp(fade.target_alpha, progress);

            // Apply alpha to background color
            let mut color = background_color.0;
            color.set_a(fade.current_alpha);
            *background_color = color.into();

            // Stop animation when complete
            if progress >= 1.0 {
                fade.is_playing = false;
                fade.elapsed = 0.0;
            }
        }
    }
}

/// System to handle keyboard focus indicators
pub fn focus_indicator_system(
    mut query: Query<(&mut FocusIndicator, &mut BorderColor), With<Button>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    // Simple keyboard navigation (Tab key)
    if keyboard_input.just_pressed(KeyCode::Tab) {
        // Cycle through focusable elements
        let mut found_focused = false;
        let mut first_element = None;

        for (mut focus, mut border_color) in &mut query {
            if first_element.is_none() {
                first_element = Some((focus.as_mut(), border_color.as_mut()));
            }

            if focus.is_focused {
                focus.is_focused = false;
                border_color.0 = Color::TRANSPARENT;
                found_focused = true;
            } else if found_focused {
                focus.is_focused = true;
                border_color.0 = focus.focus_color;
                found_focused = false;
                break;
            }
        }

        // If no next element found, focus first element
        if found_focused {
            if let Some((focus, border_color)) = first_element {
                focus.is_focused = true;
                border_color.0 = focus.focus_color;
            }
        }
    }
}

/// System to handle accessibility features and ARIA labels
pub fn accessibility_system(
    query: Query<&AccessibilityFeatures, With<Button>>,
) {
    // This system would typically integrate with platform-specific accessibility APIs
    // For now, it serves as a placeholder for future accessibility enhancements
    for _accessibility in &query {
        // Platform-specific accessibility updates would go here
        // For example: updating screen reader information, announcing changes, etc.
    }
}

/// Helper function to start a fade animation
pub fn start_fade_animation(fade: &mut FadeAnimation, from: f32, to: f32, duration: f32) {
    fade.start_alpha = from;
    fade.target_alpha = to;
    fade.current_alpha = from;
    fade.duration = duration;
    fade.elapsed = 0.0;
    fade.is_playing = true;
}

/// Helper function to trigger a scale animation
pub fn trigger_scale_animation(animation: &mut UIAnimation, target_scale: f32) {
    animation.target_scale = target_scale;
    animation.is_animating = true;
}

/// Helper function to activate glow effect
pub fn activate_glow(glow: &mut GlowEffect, intensity: f32, color: Color) {
    glow.intensity = intensity;
    glow.color = color;
    glow.is_active = true;
}

/// Helper function to deactivate glow effect
pub fn deactivate_glow(glow: &mut GlowEffect) {
    glow.is_active = false;
    glow.intensity = 0.0;
}