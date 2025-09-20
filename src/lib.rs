//! # Ant Nest Simulator - MVP
//!
//! Simplified ant colony simulation focusing on core mechanics:
//! - Brown soil dots that ants can dig through
//! - Black ant dots with basic gravity physics
//! - Pure observation experience (no player controls)
//!
//! ## MVP Features
//!
//! - **Soil System**: Brown dots representing soil cells
//! - **Ant System**: Black 2-pixel dots that obey gravity
//! - **Digging**: Ants excavate soil to create tunnels
//! - **Movement**: Basic ant movement through tunnels
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
        app
            // MVP: Only basic setup systems
            .add_systems(
                Startup,
                (
                    systems::setup_world,
                    systems::spawn_soil_grid,
                    systems::spawn_initial_ants,
                ),
            )
            // MVP: Only core simulation systems
            .add_systems(
                Update,
                (
                    systems::ant_movement_system,
                    systems::environmental_update_system,
                ),
            );
    }
}
