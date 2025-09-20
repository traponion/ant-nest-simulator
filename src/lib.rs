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
        app.init_resource::<components::TimeControl>()
            .init_resource::<components::SimulationTime>()
            .init_resource::<components::DisasterState>()
            .init_resource::<components::ColorOverlayConfig>()
            .init_resource::<components::VisualEffectsSettings>()
            .init_resource::<components::PerformanceMetrics>()
            .init_resource::<components::ColonyStatistics>()
            .init_resource::<components::UserSettings>()
            .init_resource::<components::UITheme>()
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
                    systems::spawn_soil_grid,
                    systems::spawn_initial_ants,
                    systems::spawn_food_sources,
                    systems::spawn_queen_ant,
                    systems::setup_themed_time_control_ui,
                    systems::setup_simulation_time_display,
                    systems::setup_active_disasters_panel,
                    systems::setup_enhanced_disaster_control_ui_v3,
                    systems::setup_performance_monitoring_ui,
                    systems::initialize_spatial_grid_system,
                    systems::setup_statistics_panel,
                    systems::settings_ui::setup_settings_panel,
                    systems::settings_ui::setup_settings_toggle_button,
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
                    // Spatial optimization systems
                    systems::update_food_sources_in_grid_system,
                    systems::colony_statistics_calculation_system,
                ),
            )
            .add_systems(
                Update,
                (
                    // Disaster and visual effects systems
                    systems::disaster_input_system,
                    systems::disaster_timer_system,
                    systems::disaster_effect_system,
                    systems::invasive_species_spawning_system,
                    systems::invasive_species_food_consumption_system,
                ),
            )
            // UI systems (first part)
            .add_systems(
                Update,
                (
                    systems::time_control_input_system,
                    systems::update_speed_display_system,
                    systems::update_simulation_time_system,
                    systems::update_time_display_system,
                    systems::initialize_simulation_time_system,
                    systems::handle_themed_time_control_buttons,
                    systems::update_play_pause_button_system,
                    systems::button_click_system,
                    systems::handle_speed_slider_system,
                    systems::update_slider_handle_position_system,
                    systems::update_slider_progress_system,
                    systems::handle_speed_preset_buttons_system,
                    systems::visual_effects_toggle_system,
                    systems::settings_ui::settings_toggle_input_system,
                    systems::settings_ui::handle_settings_interactions_system,
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
                    systems::particle_spawner_system,
                    systems::particle_update_system,
                    systems::update_particle_config_system,
                ),
            )
            // Disaster UI systems
            .add_systems(
                Update,
                (
                    systems::update_disaster_status_system,
                    systems::update_cooldown_timers_system,
                    systems::disaster_trigger_feedback_system,
                    systems::update_active_disasters_display,
                    systems::update_disaster_progress_bars,
                    systems::update_disaster_duration_text,
                    systems::handle_disaster_control_interactions,
                    systems::handle_disaster_control_button_interactions,
                    systems::update_cooldown_progress_bars_system,
                    systems::visual_effects_toggle_system,
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
                    systems::save_game_system,
                    systems::load_game_system,
                    systems::persistence_status_system,
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
