use bevy::prelude::*;

/// MVP: Position component for entities in 2D space
#[derive(Component, Clone, Default)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

/// MVP: Soil cell environmental properties (minimal)
#[derive(Component, Clone)]
pub struct SoilCell {
    pub moisture: f32,
    pub temperature: f32,
    pub nutrition: f32,
}

/// MVP: Marker component for ant entities
#[derive(Component)]
pub struct Ant;

/// Queen ant marker component with founding state
#[derive(Component)]
pub struct Queen {
    pub founding_state: FoundingState,
    pub last_egg_time: f32,
    pub egg_laying_interval: f32, // Time between egg laying (in seconds)
}

/// State machine for queen ant founding behavior
#[derive(Clone, PartialEq)]
pub enum FoundingState {
    Seeking,     // Looking for suitable founding location
    Digging,     // Creating the founding chamber
    Established, // Settled in the founding chamber
}

/// MVP: Marker component for soil entities
#[derive(Component)]
pub struct Soil;

/// Egg component for ant development cycle
#[derive(Component)]
pub struct Egg {
    pub development_time: f32,
    pub stage: DevelopmentStage,
}

/// Larva component for ant development cycle
#[derive(Component)]
pub struct Larva {
    pub development_time: f32,
    pub fed: bool,
}

/// Pupa component for ant development cycle
#[derive(Component)]
pub struct Pupa {
    pub development_time: f32,
}

/// Development stages for ant lifecycle
#[derive(Clone, PartialEq)]
pub enum DevelopmentStage {
    Egg,
    Larva,
    Pupa,
}
