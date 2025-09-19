//! Game systems module
//!
//! Contains all ECS systems for the ant nest simulator:
//! - Movement: Ant pathfinding and movement behavior
//! - Lifecycle: Ant aging, energy management, and death
//! - Environment: Soil environmental simulation
//! - Rendering: Visual spawning and setup systems
//! - Time Control: Time acceleration and pause functionality
//! - Color Overlay: Visual effects for disaster feedback

pub mod active_disasters_ui;
pub mod colony_statistics;
pub mod color_overlay;
pub mod disaster;
pub mod disaster_ui;
pub mod environment;
pub mod foraging;
pub mod invasive_species;
pub mod lifecycle;
pub mod movement;
pub mod particle;
pub mod rendering;
pub mod reproduction;
pub mod statistics_ui;
pub mod time_control;
pub mod visual_effects_toggle;

// Re-export all system functions for easy importing
pub use active_disasters_ui::*;
pub use colony_statistics::*;
pub use color_overlay::*;
pub use disaster::*;
pub use disaster_ui::*;
pub use environment::*;
pub use foraging::*;
pub use invasive_species::*;
pub use lifecycle::*;
pub use movement::*;
pub use particle::*;
pub use rendering::*;
pub use reproduction::*;
pub use statistics_ui::*;
pub use time_control::*;
pub use visual_effects_toggle::*;
