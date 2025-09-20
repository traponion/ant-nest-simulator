//! Game systems module
//!
//! Contains all ECS systems for the ant nest simulator:
//! - Movement: Ant pathfinding and movement behavior
//! - Lifecycle: Ant aging, energy management, and death
//! - Environment: Soil environmental simulation
//! - Rendering: Visual spawning and setup systems
//! - Color Overlay: Visual effects for environmental feedback

pub mod colony_development;
pub mod colony_statistics;
pub mod color_overlay;
pub mod disaster_ui;
pub mod environment;
pub mod foraging;
pub mod invasive_species;
pub mod lifecycle;
pub mod movement;
pub mod particle;
pub mod performance_monitoring;
pub mod persistence;
pub mod rendering;
pub mod reproduction;
pub mod settings_ui;
pub mod spatial_grid;
pub mod statistics_ui;
pub mod tooltip;
pub mod visual_effects_toggle;

// Re-export all system functions for easy importing
pub use colony_development::*;
pub use colony_statistics::*;
pub use color_overlay::*;
pub use disaster_ui::*;
pub use environment::*;
pub use foraging::*;
pub use invasive_species::*;
pub use lifecycle::*;
pub use movement::*;
pub use particle::*;
pub use performance_monitoring::*;
pub use persistence::*;
pub use rendering::*;
pub use reproduction::*;
pub use settings_ui::*;
pub use spatial_grid::*;
pub use statistics_ui::*;
pub use tooltip::*;
pub use visual_effects_toggle::*;
