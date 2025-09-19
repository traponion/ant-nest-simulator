//! # Ant Nest Simulator
//!
//! A realistic ant colony simulation inspired by SimEarth, featuring simple dot-based graphics
//! and complex emergent behavior. Players observe ant colonies developing naturally with minimal intervention.
//!
//! ## Architecture
//!
//! The simulator is built using Bevy's Entity Component System (ECS) architecture:
//!
//! - **Components**: Define data structures for ants, soil, and environmental properties
//! - **Systems**: Implement behavior logic for movement, lifecycle, and environmental changes
//! - **Rendering**: Handle visual representation and spawning of entities
//!
//! ## Usage
//!
//! ```rust,no_run
//! use ant_nest_simulator::AntNestPlugin;
//! use bevy::prelude::*;
//!
//! App::new()
//!     .add_plugins(DefaultPlugins)
//!     .add_plugins(AntNestPlugin)
//!     .run();
//! ```

pub mod components;
pub mod systems;

/// Convenient prelude for common imports
pub mod prelude {
    pub use crate::components::*;
    pub use crate::systems::*;
}

use bevy::prelude::*;

/// Main plugin for the Ant Nest Simulator
pub struct AntNestPlugin;

impl Plugin for AntNestPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<components::TimeControl>()
            .init_resource::<components::DisasterState>()
            .init_resource::<components::ColorOverlayConfig>()
            .init_resource::<components::VisualEffectsSettings>()
            .init_resource::<systems::ParticleConfig>()
            .insert_resource(components::SpatialGrid::new(
                16.0, // Cell size of 16 units
                components::Position { x: -80.0, y: -60.0 }, // World min
                components::Position { x: 80.0, y: 60.0 },   // World max
            ))
            .add_systems(
                Startup,
                (
                    systems::setup_world,
                    systems::spawn_soil_grid,
                    systems::spawn_initial_ants,
                    systems::spawn_food_sources,
                    systems::spawn_queen_ant,
                    systems::setup_time_control_ui,
                    systems::setup_active_disasters_panel,
                    systems::setup_disaster_control_panel,
                    systems::initialize_spatial_grid_system,
                ),
            )
            .add_systems(
                Update,
                (
                    // Core simulation systems
                    systems::ant_movement_system,
                    systems::ant_lifecycle_system,
                    systems::environmental_update_system,
                    systems::food_consumption_system,
                    systems::food_regeneration_system,
                    systems::queen_reproduction_system,
                    systems::egg_hatching_system,
                    // Spatial optimization systems
                    systems::update_food_sources_in_grid_system,
                ),
            )
            .add_systems(
                Update,
                (
                    // Disaster and visual effects systems
                    systems::disaster_input_system,
                    systems::disaster_timer_system,
                    systems::disaster_effect_system,
                    systems::color_overlay_system,
                    systems::update_overlay_size_system,
                    systems::particle_spawner_system,
                    systems::particle_update_system,
                    systems::update_particle_config_system,
                    systems::update_disaster_status_system,
                    systems::update_cooldown_timers_system,
                    systems::disaster_trigger_feedback_system,
                ),
            )
            .add_systems(
                Update,
                (
                    // UI systems
                    systems::time_control_input_system,
                    systems::update_speed_display_system,
                    systems::update_active_disasters_display,
                    systems::update_disaster_progress_bars,
                    systems::update_disaster_duration_text,
                    systems::visual_effects_toggle_system,
                ),
            )
            .add_systems(
                Update,
                (
                    // Invasive species systems
                    systems::invasive_species_spawning_system,
                    systems::invasive_species_behavior_system,
                    systems::ant_defensive_behavior_system,
                    systems::invasive_species_cleanup_system,
                ),
            );
    }
}
