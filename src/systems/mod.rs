//! MVP Systems Module
//!
//! Simplified systems for core ant nest simulation:
//! - Environment: Basic soil environmental simulation
//! - Movement: Simple ant movement with gravity and digging
//! - Rendering: Basic world setup and entity spawning

pub mod environment;
pub mod movement;
pub mod rendering;

// Re-export only MVP system functions
pub use environment::*;
pub use movement::*;
pub use rendering::*;
