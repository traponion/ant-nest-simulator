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
