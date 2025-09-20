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

// Allow clippy warnings that are common in game development and don't affect functionality
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

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
        app.init_resource::<components::SimulationTime>()
            .init_resource::<components::ColorOverlayConfig>()
            .init_resource::<components::VisualEffectsSettings>()
            .init_resource::<components::PerformanceMetrics>()
            .init_resource::<components::ColonyStatistics>()
            .init_resource::<components::UserSettings>()
            .init_resource::<components::UITheme>()
            .init_resource::<components::ColonyDevelopmentPhase>()
            .init_resource::<components::DisasterState>()
            .init_resource::<systems::ParticleConfig>()
            .insert_resource(components::SpatialGrid::new(
                16.0,                                        // Cell size of 16 units
                components::Position { x: -80.0, y: -60.0 }, // World min
                components::Position { x: 80.0, y: 60.0 },   // World max
            ))
            .insert_resource(systems::PersistenceState::new())
            .add_systems(
                Startup,
                (
                    systems::setup_world,
                    systems::spawn_ground_surface,
                    systems::spawn_soil_grid,
                    systems::spawn_initial_tunnel_system,
                    systems::spawn_initial_chambers,
                    systems::spawn_initial_ants, // Now includes queen spawning
                    systems::spawn_food_sources,
                    systems::setup_performance_monitoring_ui,
                    systems::initialize_spatial_grid_system,
                    systems::setup_statistics_panel,
                    systems::settings_ui::setup_settings_panel,
                    systems::settings_ui::setup_settings_toggle_button,
                    systems::setup_colony_development_ui,
                    systems::disaster_ui::setup_disaster_control_panel,
                ),
            )
            // Core simulation systems
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
                    // Colony development systems
                    systems::colony_development_management_system,
                    systems::update_ant_age_groups_system,
                    systems::apply_phase_behavior_modifiers_system,
                    // Spatial optimization systems
                    systems::update_food_sources_in_grid_system,
                    systems::colony_statistics_calculation_system,
                ),
            )
            // UI systems (first part)
            .add_systems(
                Update,
                (
                    systems::visual_effects_toggle_system,
                    systems::settings_ui::settings_toggle_input_system,
                    systems::settings_ui::handle_settings_interactions_system,
                    // Disaster control systems
                    systems::disaster_ui::handle_disaster_control_interactions,
                    systems::disaster_ui::update_disaster_status_indicators,
                    systems::disaster_ui::disaster_keyboard_input_system,
                    // Tooltip system
                    systems::tooltip_trigger_system,
                    systems::tooltip_display_system,
                    systems::tooltip_cleanup_system,
                ),
            )
            // Visual effects systems
            .add_systems(
                Update,
                (
                    systems::color_overlay_system,
                    systems::update_overlay_size_system,
                    // TODO: Re-enable particle system after fixing DisasterState resource initialization order
                    // systems::particle_spawner_system,
                    systems::particle_update_system,
                    systems::update_particle_config_system,
                ),
            )
            // Performance monitoring, statistics, and persistence systems
            .add_systems(
                Update,
                (
                    systems::collect_performance_metrics,
                    systems::update_performance_monitoring_ui,
                    systems::toggle_performance_monitoring_system,
                    systems::statistics_toggle_input_system,
                    systems::update_statistics_display,
                    systems::colony_development_ui_system,
                    systems::save_game_system,
                    systems::load_game_system,
                    systems::persistence_status_system,
                ),
            );
    }
}
