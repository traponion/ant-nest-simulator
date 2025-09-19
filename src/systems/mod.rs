//! Game systems module
//!
//! Contains all ECS systems for the ant nest simulator:
//! - Movement: Ant pathfinding and movement behavior
//! - Lifecycle: Ant aging, energy management, and death
//! - Environment: Soil environmental simulation
//! - Rendering: Visual spawning and setup systems
//! - Time Control: Time acceleration and pause functionality
//! - Color Overlay: Visual effects for disaster feedback

pub mod color_overlay;
pub mod disaster;
pub mod environment;
pub mod foraging;
pub mod lifecycle;
pub mod movement;
pub mod particle;
pub mod rendering;
pub mod reproduction;
pub mod time_control;

// Re-export all system functions for easy importing
pub use color_overlay::*;
pub use disaster::*;
pub use environment::*;
pub use foraging::*;
pub use lifecycle::*;
pub use movement::*;
pub use particle::*;
pub use rendering::*;
pub use reproduction::*;
pub use time_control::*;
