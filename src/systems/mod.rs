//! Game systems module
//!
//! Contains all ECS systems for the ant nest simulator:
//! - Movement: Ant pathfinding and movement behavior
//! - Lifecycle: Ant aging, energy management, and death
//! - Environment: Soil environmental simulation
//! - Rendering: Visual spawning and setup systems

pub mod movement;
pub mod lifecycle;
pub mod environment;
pub mod rendering;

// Re-export all system functions for easy importing
pub use movement::*;
pub use lifecycle::*;
pub use environment::*;
pub use rendering::*;