//! Game systems module
//!
//! Contains all ECS systems for the ant nest simulator:
//! - Movement: Ant pathfinding and movement behavior
//! - Lifecycle: Ant aging, energy management, and death
//! - Environment: Soil environmental simulation
//! - Rendering: Visual spawning and setup systems
//! - Time Control: Time acceleration and pause functionality

pub mod environment;
pub mod lifecycle;
pub mod movement;
pub mod rendering;
pub mod time_control;

// Re-export all system functions for easy importing
pub use environment::*;
pub use lifecycle::*;
pub use movement::*;
pub use rendering::*;
pub use time_control::*;
