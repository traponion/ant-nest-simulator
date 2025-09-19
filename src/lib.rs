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
            .init_resource::<systems::ParticleConfig>()
            .add_systems(
                Startup,
                (
                    systems::setup_world,
                    systems::spawn_soil_grid,
                    systems::spawn_initial_ants,
                    systems::spawn_food_sources,
                    systems::spawn_queen_ant,
                    systems::setup_time_control_ui,
                ),
            )
            .add_systems(
                Update,
                systems::ant_movement_system,
            )
            .add_systems(
                Update,
                systems::ant_lifecycle_system,
            )
            .add_systems(
                Update,
                systems::environmental_update_system,
            )
            .add_systems(
                Update,
                systems::food_consumption_system,
            )
            .add_systems(
                Update,
                systems::food_regeneration_system,
            )
            .add_systems(
                Update,
                systems::queen_reproduction_system,
            )
            .add_systems(
                Update,
                systems::egg_hatching_system,
            )
            .add_systems(
                Update,
                systems::time_control_input_system,
            )
            .add_systems(
                Update,
                systems::update_speed_display_system,
            )
            .add_systems(
                Update,
                systems::disaster_input_system,
            )
            .add_systems(
                Update,
                systems::disaster_timer_system,
            )
            .add_systems(
                Update,
                systems::disaster_effect_system,
            )
            .add_systems(
                Update,
                systems::particle_spawner_system,
            )
            .add_systems(
                Update,
                systems::particle_update_system,
            )
            .add_systems(
                Update,
                systems::update_particle_config_system,
            );
    }
}
