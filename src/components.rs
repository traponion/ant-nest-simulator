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
        disaster_colors.insert(DisasterType::Rain, bevy::prelude::Color::srgba(0.0, 0.8, 1.0, 0.15)); // Blue/cyan overlay
        disaster_colors.insert(DisasterType::Drought, bevy::prelude::Color::srgba(1.0, 0.6, 0.0, 0.2)); // Yellow/orange overlay
        disaster_colors.insert(DisasterType::ColdSnap, bevy::prelude::Color::srgba(0.4, 0.7, 1.0, 0.25)); // Blue/white overlay
        disaster_colors.insert(DisasterType::InvasiveSpecies, bevy::prelude::Color::srgba(1.0, 0.0, 0.0, 0.125)); // Red overlay

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
            DisasterType::Rain => Color::srgb(0.3, 0.8, 1.0),          // Blue
            DisasterType::Drought => Color::srgb(1.0, 0.7, 0.2),       // Orange
            DisasterType::ColdSnap => Color::srgb(0.7, 0.9, 1.0),      // Light blue
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

/// Age groups for population analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AgeGroup {
    Egg,
    Young,     // 0-25% of max age
    Adult,     // 25-75% of max age
    Elderly,   // 75-100% of max age
}

/// Types of metrics displayed in analytics dashboard
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricType {
    Population,
    FoodCollected,
    AverageEnergy,
    ForagingSuccess,
    BirthRate,
    DeathRate,
}

/// Snapshot of population data at a specific time
#[derive(Debug, Clone)]
pub struct PopulationSnapshot {
    pub timestamp: f32,
    pub total_population: usize,
    pub population_by_age: std::collections::HashMap<AgeGroup, usize>,
    pub birth_count: usize,
    pub death_count: usize,
}

/// Snapshot of resource data at a specific time
#[derive(Debug, Clone)]
pub struct ResourceSnapshot {
    pub timestamp: f32,
    pub food_collected: f32,
    pub average_energy: f32,
    pub foraging_attempts: usize,
    pub successful_foraging: usize,
}

/// Resource for managing colony analytics and statistics
#[derive(Resource)]
pub struct ColonyAnalytics {
    // Population metrics
    pub total_population: usize,
    pub population_by_age: std::collections::HashMap<AgeGroup, usize>,
    pub birth_count: usize,
    pub death_count: usize,

    // Resource metrics
    pub food_collected: f32,
    pub average_energy: f32,
    pub foraging_attempts: usize,
    pub successful_foraging: usize,

    // Historical data (circular buffers for memory efficiency)
    pub population_history: std::collections::VecDeque<PopulationSnapshot>,
    pub resource_history: std::collections::VecDeque<ResourceSnapshot>,

    // Update timing
    pub update_timer: f32,
    pub update_interval: f32, // Update frequency (e.g., 1.0 = once per second)

    // Dashboard visibility
    pub is_visible: bool,

    // Historical data limits
    pub max_history_entries: usize,
}

impl Default for ColonyAnalytics {
    fn default() -> Self {
        let mut population_by_age = std::collections::HashMap::new();
        population_by_age.insert(AgeGroup::Egg, 0);
        population_by_age.insert(AgeGroup::Young, 0);
        population_by_age.insert(AgeGroup::Adult, 0);
        population_by_age.insert(AgeGroup::Elderly, 0);

        Self {
            total_population: 0,
            population_by_age,
            birth_count: 0,
            death_count: 0,
            food_collected: 0.0,
            average_energy: 0.0,
            foraging_attempts: 0,
            successful_foraging: 0,
            population_history: std::collections::VecDeque::new(),
            resource_history: std::collections::VecDeque::new(),
            update_timer: 0.0,
            update_interval: 1.0, // Update once per second
            is_visible: true, // Start with dashboard visible
            max_history_entries: 300, // Keep 5 minutes of data at 1Hz
        }
    }
}

impl ColonyAnalytics {
    /// Calculate foraging success rate as a percentage
    pub fn foraging_success_rate(&self) -> f32 {
        if self.foraging_attempts == 0 {
            0.0
        } else {
            (self.successful_foraging as f32 / self.foraging_attempts as f32) * 100.0
        }
    }

    /// Calculate birth rate (births per minute)
    pub fn birth_rate(&self) -> f32 {
        if self.population_history.len() < 2 {
            return 0.0;
        }

        let time_span = 60.0; // Calculate per minute
        let recent_entries: Vec<_> = self.population_history
            .iter()
            .rev()
            .take_while(|snapshot| {
                let latest_time = self.population_history.back().unwrap().timestamp;
                latest_time - snapshot.timestamp <= time_span
            })
            .collect();

        if recent_entries.len() < 2 {
            return 0.0;
        }

        let oldest = recent_entries.last().unwrap();
        let newest = recent_entries.first().unwrap();
        let time_diff = newest.timestamp - oldest.timestamp;

        if time_diff > 0.0 {
            let birth_diff = newest.birth_count.saturating_sub(oldest.birth_count);
            (birth_diff as f32 / time_diff) * 60.0 // Convert to per minute
        } else {
            0.0
        }
    }

    /// Calculate death rate (deaths per minute)
    pub fn death_rate(&self) -> f32 {
        if self.population_history.len() < 2 {
            return 0.0;
        }

        let time_span = 60.0; // Calculate per minute
        let recent_entries: Vec<_> = self.population_history
            .iter()
            .rev()
            .take_while(|snapshot| {
                let latest_time = self.population_history.back().unwrap().timestamp;
                latest_time - snapshot.timestamp <= time_span
            })
            .collect();

        if recent_entries.len() < 2 {
            return 0.0;
        }

        let oldest = recent_entries.last().unwrap();
        let newest = recent_entries.first().unwrap();
        let time_diff = newest.timestamp - oldest.timestamp;

        if time_diff > 0.0 {
            let death_diff = newest.death_count.saturating_sub(oldest.death_count);
            (death_diff as f32 / time_diff) * 60.0 // Convert to per minute
        } else {
            0.0
        }
    }

    /// Add a new population snapshot to history
    pub fn add_population_snapshot(&mut self, timestamp: f32) {
        let snapshot = PopulationSnapshot {
            timestamp,
            total_population: self.total_population,
            population_by_age: self.population_by_age.clone(),
            birth_count: self.birth_count,
            death_count: self.death_count,
        };

        self.population_history.push_back(snapshot);

        // Maintain maximum history size
        while self.population_history.len() > self.max_history_entries {
            self.population_history.pop_front();
        }
    }

    /// Add a new resource snapshot to history
    pub fn add_resource_snapshot(&mut self, timestamp: f32) {
        let snapshot = ResourceSnapshot {
            timestamp,
            food_collected: self.food_collected,
            average_energy: self.average_energy,
            foraging_attempts: self.foraging_attempts,
            successful_foraging: self.successful_foraging,
        };

        self.resource_history.push_back(snapshot);

        // Maintain maximum history size
        while self.resource_history.len() > self.max_history_entries {
            self.resource_history.pop_front();
        }
    }
}

/// Marker component for analytics dashboard panel
#[derive(Component)]
pub struct AnalyticsDashboard;

/// Component for individual metric displays in the analytics dashboard
#[derive(Component)]
pub struct MetricDisplay {
    pub metric_type: MetricType,
}

/// Component for analytics toggle button
#[derive(Component)]
pub struct AnalyticsToggleButton;
