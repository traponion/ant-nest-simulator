use crate::components::VisualEffectsSettings;
use bevy::prelude::*;

/// System for handling visual effects toggle input (accessibility feature)
pub fn visual_effects_toggle_system(
    input: Res<ButtonInput<KeyCode>>,
    mut visual_effects_settings: ResMut<VisualEffectsSettings>,
) {
    // Toggle all visual effects with 'V' key
    if input.just_pressed(KeyCode::KeyV) {
        visual_effects_settings.toggle_all();

        let status = if visual_effects_settings.particles_enabled && visual_effects_settings.overlays_enabled {
            "enabled"
        } else {
            "disabled"
        };

        info!("Visual effects toggled: {} (particles: {}, overlays: {})",
              status,
              visual_effects_settings.particles_enabled,
              visual_effects_settings.overlays_enabled);
    }

    // Toggle only particles with 'P' key (advanced option)
    if input.just_pressed(KeyCode::KeyP) {
        visual_effects_settings.toggle_particles();
        let status = if visual_effects_settings.particles_enabled { "enabled" } else { "disabled" };
        info!("Particle effects {}", status);
    }

    // Toggle only overlays with 'O' key (advanced option)
    if input.just_pressed(KeyCode::KeyO) {
        visual_effects_settings.toggle_overlays();
        let status = if visual_effects_settings.overlays_enabled { "enabled" } else { "disabled" };
        info!("Color overlay effects {}", status);
    }
}