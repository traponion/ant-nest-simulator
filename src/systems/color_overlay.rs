use bevy::prelude::*;
use crate::components::{ColorOverlay, ColorOverlayConfig, DisasterState, DisasterType, VisualEffectsSettings};

/// System for managing color overlay entities based on active disasters
pub fn color_overlay_system(
    mut commands: Commands,
    disaster_state: Res<DisasterState>,
    mut overlay_config: ResMut<ColorOverlayConfig>,
    overlay_query: Query<Entity, With<ColorOverlay>>,
    windows: Query<&Window>,
    visual_effects_settings: Res<VisualEffectsSettings>,
) {
    // Get window size for fullscreen overlay
    let window = windows.single();
    let window_width = window.width();
    let window_height = window.height();

    // Remove existing overlay entities
    for entity in overlay_query.iter() {
        commands.entity(entity).despawn();
    }
    overlay_config.overlay_entity = None;

    // Skip overlay creation if overlays are disabled for accessibility
    if !visual_effects_settings.overlays_enabled {
        return;
    }

    // Calculate blended color for active disasters
    let blended_color = calculate_blended_overlay_color(&disaster_state, &overlay_config);

    // Spawn new overlay entity if any disasters are active
    if let Some(color) = blended_color {
        let overlay_entity = commands
            .spawn((
                ColorOverlay {
                    color,
                    disaster_type: DisasterType::Rain, // Primary disaster type (for reference)
                },
                SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::new(window_width, window_height)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1000.0)), // High Z to render on top
                    ..default()
                },
            ))
            .id();

        overlay_config.overlay_entity = Some(overlay_entity);
    }
}

/// Calculate the blended color for multiple active disasters
fn calculate_blended_overlay_color(
    disaster_state: &DisasterState,
    overlay_config: &ColorOverlayConfig,
) -> Option<Color> {
    let active_disasters: Vec<DisasterType> =
        disaster_state.active_disasters.keys().copied().collect();

    if active_disasters.is_empty() {
        return None;
    }

    // If only one disaster is active, return its color directly
    if active_disasters.len() == 1 {
        return overlay_config
            .disaster_colors
            .get(&active_disasters[0])
            .copied();
    }

    // For multiple disasters, blend the colors using additive blending with alpha management
    let mut total_red = 0.0;
    let mut total_green = 0.0;
    let mut total_blue = 0.0;
    let mut total_alpha = 0.0;

    for disaster_type in &active_disasters {
        if let Some(color) = overlay_config.disaster_colors.get(disaster_type) {
            let [r, g, b, a] = color.to_srgba().to_f32_array();
            total_red += r * a; // Weight by alpha for proper blending
            total_green += g * a;
            total_blue += b * a;
            total_alpha += a;
        }
    }

    if total_alpha > 0.0 {
        // Normalize by total alpha and cap alpha at reasonable level
        let blended_alpha = (total_alpha * 0.7).min(0.4); // Cap combined alpha to prevent too strong overlay
        let normalized_red = total_red / total_alpha;
        let normalized_green = total_green / total_alpha;
        let normalized_blue = total_blue / total_alpha;

        Some(Color::srgba(
            normalized_red,
            normalized_green,
            normalized_blue,
            blended_alpha,
        ))
    } else {
        None
    }
}

/// System for updating overlay size when window is resized
pub fn update_overlay_size_system(
    mut overlay_query: Query<(&mut Transform, &mut Sprite), With<ColorOverlay>>,
    windows: Query<&Window>,
    mut resize_events: EventReader<bevy::window::WindowResized>,
) {
    for _event in resize_events.read() {
        let window = windows.single();
        let window_width = window.width();
        let window_height = window.height();

        for (mut transform, mut sprite) in overlay_query.iter_mut() {
            sprite.custom_size = Some(Vec2::new(window_width, window_height));
            // Keep overlay centered
            transform.translation.x = 0.0;
            transform.translation.y = 0.0;
        }
    }
}
