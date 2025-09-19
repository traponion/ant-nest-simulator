use bevy::prelude::*;

/// Position component for entities in 2D space
#[derive(Component, Clone)]
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
#[derive(Debug, Clone, PartialEq)]
pub enum AntState {
    Foraging,
    Returning,
    Resting,
    Digging,
    /// Carrying food back to the colony
    CarryingFood,
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

/// Marker component for food entities
#[derive(Component)]
pub struct Food;

/// Component for food source properties
#[derive(Component)]
pub struct FoodSource {
    /// Nutritional value provided when consumed (energy points)
    pub nutrition_value: f32,
    /// Whether this food source is currently available for consumption
    pub is_available: bool,
    /// Time remaining until this food source regenerates (if consumed)
    pub regeneration_timer: f32,
    /// Base regeneration time for this food source
    pub regeneration_time: f32,
}

/// Component for tracking what an ant is currently carrying
#[derive(Component)]
pub struct Inventory {
    /// Energy value of the food being carried (0.0 if nothing)
    pub carried_food_value: f32,
    /// Position where the ant should return to (colony center)
    pub home_position: Position,
}

/// Natural disaster types that can affect the environment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DisasterType {
    Rain,
    Drought,
    ColdSnap,
    InvasiveSpecies,
}

/// Resource for managing active natural disasters
#[derive(Resource, Default)]
pub struct DisasterState {
    /// Currently active disasters with their remaining duration
    pub active_disasters: std::collections::HashMap<DisasterType, f32>,
    /// Cooldown timers to prevent disaster spam
    pub cooldown_timers: std::collections::HashMap<DisasterType, f32>,
}

impl DisasterState {
    /// Check if a disaster type is currently active
    pub fn is_active(&self, disaster_type: DisasterType) -> bool {
        self.active_disasters.contains_key(&disaster_type)
    }

    /// Check if a disaster type is on cooldown
    pub fn is_on_cooldown(&self, disaster_type: DisasterType) -> bool {
        self.cooldown_timers.get(&disaster_type).map_or(false, |&timer| timer > 0.0)
    }

    /// Start a new disaster with the given duration
    pub fn start_disaster(&mut self, disaster_type: DisasterType, duration: f32) {
        self.active_disasters.insert(disaster_type, duration);
    }

    /// Get the remaining time for an active disaster
    pub fn get_remaining_time(&self, disaster_type: DisasterType) -> Option<f32> {
        self.active_disasters.get(&disaster_type).copied()
    }
}

/// Component for invasive species entities (temporary during invasive species disaster)
#[derive(Component)]
pub struct InvasiveSpecies {
    /// How long this invasive species will remain active
    pub lifetime: f32,
    /// Rate at which this species consumes food sources
    pub food_consumption_rate: f32,
}
