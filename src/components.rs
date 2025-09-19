use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Position component for entities in 2D space
#[derive(Component, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

/// Ant behavior and AI state management
#[derive(Component, Clone, Serialize, Deserialize)]
pub struct AntBehavior {
    pub state: AntState,
    pub target_position: Option<Position>,
    pub speed: f32,
}

/// Lifecycle management for aging and energy
#[derive(Component, Clone, Serialize, Deserialize)]
pub struct Lifecycle {
    pub age: f32,
    pub max_age: f32,
    pub energy: f32,
    pub max_energy: f32,
}

/// Soil cell environmental properties
#[derive(Component, Clone, Serialize, Deserialize)]
pub struct SoilCell {
    pub moisture: f32,
    pub temperature: f32,
    pub nutrition: f32,
}

/// Marker component for ant entities
#[derive(Component, Serialize, Deserialize)]
pub struct Ant;

/// Marker component for soil entities
#[derive(Component, Serialize, Deserialize)]
pub struct Soil;

/// Ant behavioral states
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
#[derive(Component, Serialize, Deserialize)]
pub struct Queen;

/// Component for egg entities with incubation time
#[derive(Component, Serialize, Deserialize)]
pub struct Egg {
    /// Time remaining until hatching (in seconds)
    pub incubation_time: f32,
}

/// Component for managing ant reproduction behavior
#[derive(Component, Clone, Serialize, Deserialize)]
pub struct ReproductionState {
    /// Time since last egg laying (in seconds)
    pub time_since_last_egg: f32,
    /// Minimum time between egg laying attempts (in seconds)
    pub egg_laying_interval: f32,
    /// Current reproductive capacity based on colony resources
    pub reproductive_capacity: f32,
}

/// Marker component for food entities
#[derive(Component, Serialize, Deserialize)]
pub struct Food;

/// Component for food source properties
#[derive(Component, Clone, Serialize, Deserialize)]
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
#[derive(Component, Clone, Serialize, Deserialize)]
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
        self.cooldown_timers
            .get(&disaster_type)
            .is_some_and(|&timer| timer > 0.0)
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

/// Component for color overlay entities that provide visual feedback during disasters
#[derive(Component)]
pub struct ColorOverlay {
    /// The color of the overlay with alpha transparency
    pub color: bevy::prelude::Color,
    /// The disaster type this overlay represents
    pub disaster_type: DisasterType,
}

/// Resource for managing color overlay configuration and active overlays
#[derive(Resource)]
pub struct ColorOverlayConfig {
    /// Mapping of disaster types to their overlay colors (with alpha)
    pub disaster_colors: std::collections::HashMap<DisasterType, bevy::prelude::Color>,
    /// Entity ID of the active overlay entity (if any)
    pub overlay_entity: Option<bevy::prelude::Entity>,
}

impl Default for ColorOverlayConfig {
    fn default() -> Self {
        let mut disaster_colors = std::collections::HashMap::new();

        // Define overlay colors for each disaster type as specified in the requirements
        disaster_colors.insert(
            DisasterType::Rain,
            bevy::prelude::Color::srgba(0.0, 0.8, 1.0, 0.15),
        ); // Blue/cyan overlay
        disaster_colors.insert(
            DisasterType::Drought,
            bevy::prelude::Color::srgba(1.0, 0.6, 0.0, 0.2),
        ); // Yellow/orange overlay
        disaster_colors.insert(
            DisasterType::ColdSnap,
            bevy::prelude::Color::srgba(0.4, 0.7, 1.0, 0.25),
        ); // Blue/white overlay
        disaster_colors.insert(
            DisasterType::InvasiveSpecies,
            bevy::prelude::Color::srgba(1.0, 0.0, 0.0, 0.125),
        ); // Red overlay

        Self {
            disaster_colors,
            overlay_entity: None,
        }
    }
}

/// Marker component for particle entities
#[derive(Component)]
pub struct Particle;

/// Component for particle behavior and visual properties
#[derive(Component)]
pub struct ParticleData {
    /// Particle type determining its behavior and appearance
    pub particle_type: ParticleType,
    /// Remaining lifetime before particle despawns (in seconds)
    pub lifetime: f32,
    /// Maximum lifetime this particle started with
    pub max_lifetime: f32,
    /// Velocity vector for particle movement
    pub velocity: Vec2,
    /// Base color (may be modified by lifetime for fade effects)
    pub base_color: Color,
    /// Size of the particle sprite
    pub size: Vec2,
}

/// Types of particles for different disaster effects
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParticleType {
    /// Rain droplet particles - fall downward with blue color
    RainDrop,
    /// Dust particles for drought - rise upward with brown/yellow color
    DustMote,
    /// Snowflake particles for cold snap - fall downward with white/blue color
    Snowflake,
    /// Environmental disturbance particles for invasive species - random movement with red color
    EnvironmentalDisturbance,
}

impl ParticleData {
    /// Create a new rain drop particle
    pub fn new_rain_drop(lifetime: f32, initial_velocity: Vec2) -> Self {
        Self {
            particle_type: ParticleType::RainDrop,
            lifetime,
            max_lifetime: lifetime,
            velocity: initial_velocity,
            base_color: Color::srgba(0.2, 0.6, 1.0, 0.8), // Light blue with transparency
            size: Vec2::new(2.0, 4.0),                    // Small elongated droplet
        }
    }

    /// Create a new dust mote particle for drought
    pub fn new_dust_mote(lifetime: f32, initial_velocity: Vec2) -> Self {
        Self {
            particle_type: ParticleType::DustMote,
            lifetime,
            max_lifetime: lifetime,
            velocity: initial_velocity,
            base_color: Color::srgba(0.8, 0.6, 0.3, 0.6), // Sandy brown with transparency
            size: Vec2::new(3.0, 3.0),                    // Small square dust
        }
    }

    /// Create a new snowflake particle for cold snap
    pub fn new_snowflake(lifetime: f32, initial_velocity: Vec2) -> Self {
        Self {
            particle_type: ParticleType::Snowflake,
            lifetime,
            max_lifetime: lifetime,
            velocity: initial_velocity,
            base_color: Color::srgba(0.9, 0.9, 1.0, 0.7), // Light blue-white with transparency
            size: Vec2::new(3.0, 3.0),                    // Small snowflake
        }
    }

    /// Create a new environmental disturbance particle for invasive species
    pub fn new_environmental_disturbance(lifetime: f32, initial_velocity: Vec2) -> Self {
        Self {
            particle_type: ParticleType::EnvironmentalDisturbance,
            lifetime,
            max_lifetime: lifetime,
            velocity: initial_velocity,
            base_color: Color::srgba(1.0, 0.3, 0.2, 0.5), // Red with transparency
            size: Vec2::new(2.0, 2.0),                    // Small disturbance particle
        }
    }

    /// Get the current alpha based on lifetime for fade-out effect
    pub fn get_current_alpha(&self) -> f32 {
        let life_ratio = self.lifetime / self.max_lifetime;
        // Fade out over the last 25% of lifetime
        if life_ratio < 0.25 {
            life_ratio * 4.0 * self.base_color.alpha()
        } else {
            self.base_color.alpha()
        }
    }

    /// Get the current color with lifetime-based alpha
    pub fn get_current_color(&self) -> Color {
        let mut color = self.base_color;
        color.set_alpha(self.get_current_alpha());
        color
    }
}

/// UI components for disaster control panel
#[derive(Component)]
pub struct DisasterControlPanel;

/// Component for individual disaster control UI elements
#[derive(Component)]
pub struct DisasterControlButton {
    pub disaster_type: DisasterType,
}

/// Component for cooldown timer display
#[derive(Component)]
pub struct CooldownTimer {
    pub disaster_type: DisasterType,
}

/// Component for disaster status indicator
#[derive(Component)]
pub struct DisasterStatusIndicator {
    pub disaster_type: DisasterType,
}

/// Component for visual feedback when disaster is triggered
#[derive(Component)]
pub struct DisasterTriggerFeedback {
    pub disaster_type: DisasterType,
    pub fade_timer: f32,
}

/// Component for active disasters display panel
#[derive(Component)]
pub struct ActiveDisastersPanel;

/// Component for individual active disaster entry
#[derive(Component)]
pub struct ActiveDisasterEntry {
    pub disaster_type: DisasterType,
}

/// Component for disaster duration progress bar
#[derive(Component)]
pub struct DisasterProgressBar {
    pub disaster_type: DisasterType,
    pub max_duration: f32,
}

/// Component for disaster duration text display
#[derive(Component)]
pub struct DisasterDurationText {
    pub disaster_type: DisasterType,
}
impl DisasterType {
    /// Get the display name for UI
    pub fn display_name(&self) -> &'static str {
        match self {
            DisasterType::Rain => "Rain",
            DisasterType::Drought => "Drought",
            DisasterType::ColdSnap => "Cold Snap",
            DisasterType::InvasiveSpecies => "Invasive Species",
        }
    }

    /// Get the display color for active disaster UI
    pub fn get_active_color(&self) -> Color {
        match self {
            DisasterType::Rain => Color::srgb(0.3, 0.8, 1.0), // Blue
            DisasterType::Drought => Color::srgb(1.0, 0.7, 0.2), // Orange
            DisasterType::ColdSnap => Color::srgb(0.7, 0.9, 1.0), // Light blue
            DisasterType::InvasiveSpecies => Color::srgb(1.0, 0.4, 0.4), // Red
        }
    }

    /// Get the keyboard shortcut key for UI
    pub fn shortcut_key(&self) -> &'static str {
        match self {
            DisasterType::Rain => "R",
            DisasterType::Drought => "D",
            DisasterType::ColdSnap => "C",
            DisasterType::InvasiveSpecies => "I",
        }
    }

    /// Get the status color based on current state
    pub fn get_status_color(&self, disaster_state: &DisasterState) -> Color {
        if disaster_state.is_active(*self) {
            Color::srgb(1.0, 0.3, 0.3) // Red for active
        } else if disaster_state.is_on_cooldown(*self) {
            Color::srgb(1.0, 0.6, 0.0) // Orange for cooldown
        } else {
            Color::srgb(0.3, 1.0, 0.3) // Green for available
        }
    }
}

/// Resource for managing visual effects accessibility settings
#[derive(Resource, Default)]
pub struct VisualEffectsSettings {
    /// Whether particle effects are enabled (can be toggled for accessibility)
    pub particles_enabled: bool,
    /// Whether color overlays are enabled (can be toggled for accessibility)
    pub overlays_enabled: bool,
}

impl VisualEffectsSettings {
    /// Create new settings with effects enabled by default
    pub fn new() -> Self {
        Self {
            particles_enabled: true,
            overlays_enabled: true,
        }
    }

    /// Toggle particle effects on/off
    pub fn toggle_particles(&mut self) {
        self.particles_enabled = !self.particles_enabled;
    }

    /// Toggle color overlays on/off
    pub fn toggle_overlays(&mut self) {
        self.overlays_enabled = !self.overlays_enabled;
    }

    /// Toggle all visual effects on/off
    pub fn toggle_all(&mut self) {
        self.particles_enabled = !self.particles_enabled;
        self.overlays_enabled = !self.overlays_enabled;
    }
}

/// Resource for tracking comprehensive colony statistics and metrics
#[derive(Resource, Default)]
pub struct ColonyStatistics {
    // Population Statistics
    pub total_ant_count: usize,
    pub queen_count: usize,
    pub egg_count: usize,
    pub average_incubation_time: f32,
    pub young_ants: usize,      // Age < 30% of max_age
    pub adult_ants: usize,      // Age 30-70% of max_age
    pub elderly_ants: usize,    // Age > 70% of max_age
    pub recent_births: usize,
    pub recent_deaths: usize,

    // Resource Management
    pub available_food_sources: usize,
    pub total_food_nutrition: f32,
    pub average_ant_energy: f32,
    pub min_ant_energy: f32,
    pub max_ant_energy: f32,
    pub ants_carrying_food: usize,
    pub total_carried_food_value: f32,

    // Environmental Status
    pub average_soil_moisture: f32,
    pub min_soil_moisture: f32,
    pub max_soil_moisture: f32,
    pub average_soil_temperature: f32,
    pub min_soil_temperature: f32,
    pub max_soil_temperature: f32,
    pub average_soil_nutrition: f32,
    pub min_soil_nutrition: f32,
    pub max_soil_nutrition: f32,
    pub active_disasters_count: usize,

    // Behavioral Insights
    pub ants_foraging: usize,
    pub ants_returning: usize,
    pub ants_resting: usize,
    pub ants_digging: usize,
    pub ants_carrying: usize,

    // Queen Reproduction Statistics
    pub queen_reproduction_capacity: f32,
    pub time_since_last_egg: f32,

    // Performance tracking
    pub last_update_time: f32,
}

impl ColonyStatistics {
    /// Create a new empty statistics resource
    pub fn new() -> Self {
        Self::default()
    }

    /// Reset all statistics to default values
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Get total population (all ants + queen)
    pub fn total_population(&self) -> usize {
        self.total_ant_count + self.queen_count
    }

    /// Get foraging efficiency as a percentage
    pub fn foraging_efficiency(&self) -> f32 {
        if self.total_ant_count == 0 {
            return 0.0;
        }

        let foraging_ants = self.ants_foraging + self.ants_carrying;
        (foraging_ants as f32 / self.total_ant_count as f32) * 100.0
    }

    /// Get average energy percentage across all ants
    pub fn average_energy_percentage(&self) -> f32 {
        if self.total_ant_count == 0 || self.max_ant_energy == 0.0 {
            return 0.0;
        }
        (self.average_ant_energy / self.max_ant_energy) * 100.0
    }

    /// Get formatted age distribution text
    pub fn age_distribution_text(&self) -> String {
        if self.total_ant_count == 0 {
            return "No ants".to_string();
        }

        let young_pct = (self.young_ants as f32 / self.total_ant_count as f32) * 100.0;
        let adult_pct = (self.adult_ants as f32 / self.total_ant_count as f32) * 100.0;
        let elderly_pct = (self.elderly_ants as f32 / self.total_ant_count as f32) * 100.0;

        format!("Young: {:.0}%, Adult: {:.0}%, Elderly: {:.0}%", young_pct, adult_pct, elderly_pct)
    }

    /// Get formatted behavioral state distribution
    pub fn behavior_distribution_text(&self) -> String {
        if self.total_ant_count == 0 {
            return "No ants".to_string();
        }

        let foraging_pct = (self.ants_foraging as f32 / self.total_ant_count as f32) * 100.0;
        let returning_pct = (self.ants_returning as f32 / self.total_ant_count as f32) * 100.0;
        let resting_pct = (self.ants_resting as f32 / self.total_ant_count as f32) * 100.0;
        let digging_pct = (self.ants_digging as f32 / self.total_ant_count as f32) * 100.0;
        let carrying_pct = (self.ants_carrying as f32 / self.total_ant_count as f32) * 100.0;

        format!(
            "Foraging: {:.0}%, Returning: {:.0}%, Resting: {:.0}%, Digging: {:.0}%, Carrying: {:.0}%",
            foraging_pct, returning_pct, resting_pct, digging_pct, carrying_pct
        )
    }
}

/// Component marker for the statistics display panel
#[derive(Component)]
pub struct StatisticsPanel;

/// Component for statistics toggle functionality
#[derive(Component)]
pub struct StatisticsToggle {
    pub is_visible: bool,
}

impl Default for StatisticsToggle {
    fn default() -> Self {
        Self {
            is_visible: false, // Start hidden by default
        }
    }
}

/// UI components for time control panel
#[derive(Component)]
pub struct TimeControlPanel;

/// Component for play/pause button
#[derive(Component)]
pub struct PlayPauseButton;

/// Component for speed control buttons
#[derive(Component)]
pub struct SpeedButton {
    pub target_speed: f32,
}

/// Component for speed display text
#[derive(Component)]
pub struct SpeedDisplay;
