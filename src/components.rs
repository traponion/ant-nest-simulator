use bevy::prelude::*;

/// Position component for entities in 2D space
#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

/// Ant behavior and AI state management
#[derive(Component)]
pub struct AntBehavior {
    pub state: AntState,
    pub target_position: Option<Position>,
    pub speed: f32,
}

/// Lifecycle management for aging and energy
#[derive(Component)]
pub struct Lifecycle {
    pub age: f32,
    pub max_age: f32,
    pub energy: f32,
    pub max_energy: f32,
}

/// Soil cell environmental properties
#[derive(Component)]
pub struct SoilCell {
    pub moisture: f32,
    pub temperature: f32,
    pub nutrition: f32,
}

/// Marker component for ant entities
#[derive(Component)]
pub struct Ant;

/// Marker component for soil entities
#[derive(Component)]
pub struct Soil;

/// Ant behavioral states
#[derive(Debug, Clone)]
pub enum AntState {
    Foraging,
    Returning,
    Resting,
    Digging,
}

/// Time control resource for managing simulation speed
#[derive(Resource)]
pub struct TimeControl {
    /// Current speed multiplier (1.0 = normal speed, 0.0 = paused, 100.0 = max speed)
    pub speed_multiplier: f32,
    /// Whether the simulation is paused
    pub is_paused: bool,
}

impl Default for TimeControl {
    fn default() -> Self {
        Self {
            speed_multiplier: 1.0,
            is_paused: false,
        }
    }
}

/// Marker component for queen ant entities
#[derive(Component)]
pub struct Queen;

/// Component for egg entities with incubation time
#[derive(Component)]
pub struct Egg {
    /// Time remaining until hatching (in seconds)
    pub incubation_time: f32,
}

/// Component for managing ant reproduction behavior
#[derive(Component)]
pub struct ReproductionState {
    /// Time since last egg laying (in seconds)
    pub time_since_last_egg: f32,
    /// Minimum time between egg laying attempts (in seconds)
    pub egg_laying_interval: f32,
    /// Current reproductive capacity based on colony resources
    pub reproductive_capacity: f32,
}