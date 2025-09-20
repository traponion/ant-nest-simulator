use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Position component for entities in 2D space
#[derive(Component, Clone, Default, Serialize, Deserialize)]
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

/// Simulation time tracking resource for displaying elapsed time and day/night cycle
#[derive(Resource, Clone)]
pub struct SimulationTime {
    /// Total elapsed simulation time in seconds (affected by time scale)
    pub elapsed_seconds: f64,
    /// Current simulation day (starts at 1)
    pub current_day: u32,
    /// Current hour of the day (0-23)
    pub current_hour: u8,
    /// Current minute of the hour (0-59)
    pub current_minute: u8,
    /// Length of a simulated day in real seconds (at 1x speed)
    pub day_length_seconds: f64,
    /// When the simulation was started (for real-time tracking)
    pub start_time: f64,
}

impl Default for SimulationTime {
    fn default() -> Self {
        Self {
            elapsed_seconds: 0.0,
            current_day: 1,
            current_hour: 6, // Start at dawn (6:00 AM)
            current_minute: 0,
            day_length_seconds: 300.0, // 5 minutes = 1 simulated day at 1x speed
            start_time: 0.0,
        }
    }
}

impl SimulationTime {
    /// Update simulation time at natural speed (autonomous simulation)
    pub fn update(&mut self, delta_seconds: f32) {
        // Fixed natural speed - no player control
        self.elapsed_seconds += delta_seconds as f64;

        // Calculate current time within the day
        let total_seconds_in_day = self.elapsed_seconds % self.day_length_seconds;
        let seconds_per_hour = self.day_length_seconds / 24.0;
        let seconds_per_minute = seconds_per_hour / 60.0;

        // Calculate day, hour, and minute
        self.current_day = (self.elapsed_seconds / self.day_length_seconds) as u32 + 1;
        self.current_hour = (total_seconds_in_day / seconds_per_hour) as u8 % 24;
        self.current_minute =
            ((total_seconds_in_day % seconds_per_hour) / seconds_per_minute) as u8 % 60;
    }

    /// Get formatted time string (e.g., "Day 3, 14:30")
    pub fn format_time(&self) -> String {
        format!(
            "Day {}, {:02}:{:02}",
            self.current_day, self.current_hour, self.current_minute
        )
    }

    /// Get formatted elapsed time string (e.g., "2h 15m")
    pub fn format_elapsed_time(&self) -> String {
        let total_minutes = (self.elapsed_seconds / 60.0) as u32;
        let hours = total_minutes / 60;
        let minutes = total_minutes % 60;

        if hours > 0 {
            format!("{}h {}m", hours, minutes)
        } else {
            format!("{}m", minutes)
        }
    }

    /// Get time of day as a fraction (0.0 = midnight, 0.5 = noon)
    pub fn get_time_of_day_fraction(&self) -> f32 {
        (self.current_hour as f32 + self.current_minute as f32 / 60.0) / 24.0
    }
}

/// Component marker for simulation time display UI elements
#[derive(Component)]
pub struct SimulationTimeDisplay;

/// Component for different time display formats
#[derive(Component, Clone)]
pub struct TimeDisplayFormat {
    pub show_day: bool,
    pub show_time_of_day: bool,
    pub show_elapsed_time: bool,
    pub show_speed_indicator: bool,
}

impl Default for TimeDisplayFormat {
    fn default() -> Self {
        Self {
            show_day: true,
            show_time_of_day: true,
            show_elapsed_time: false,
            show_speed_indicator: true,
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

/// Component for disaster status indicator background
#[derive(Component)]
pub struct DisasterStatusBackground {
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

/// Component for disaster cooldown progress bar
#[derive(Component)]
pub struct DisasterCooldownProgressBar {
    pub disaster_type: DisasterType,
    pub max_cooldown: f32,
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

    /// Get the icon emoji for disaster type
    pub fn get_icon(&self) -> &'static str {
        match self {
            DisasterType::Rain => "🌧️",
            DisasterType::Drought => "☀️",
            DisasterType::ColdSnap => "❄️",
            DisasterType::InvasiveSpecies => "🐛",
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

/// Grid cell coordinates for spatial partitioning
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridCell {
    pub x: i32,
    pub y: i32,
}

/// Resource for spatial partitioning of entities
/// Provides O(1) lookup time for nearby entities instead of O(n) brute force search
#[derive(Resource, Default)]
pub struct SpatialGrid {
    /// Grid cell size in world units
    pub cell_size: f32,
    /// Map from grid cells to lists of entities in that cell
    pub grid: std::collections::HashMap<GridCell, Vec<Entity>>,
    /// World bounds for the grid
    pub world_min: Position,
    pub world_max: Position,
}

impl SpatialGrid {
    /// Create new spatial grid with specified cell size and world bounds
    pub fn new(cell_size: f32, world_min: Position, world_max: Position) -> Self {
        Self {
            cell_size,
            grid: std::collections::HashMap::new(),
            world_min,
            world_max,
        }
    }

    /// Convert world position to grid cell coordinates
    pub fn world_to_grid(&self, position: &Position) -> GridCell {
        GridCell {
            x: (position.x / self.cell_size).floor() as i32,
            y: (position.y / self.cell_size).floor() as i32,
        }
    }

    /// Get all entities in a specific grid cell
    pub fn get_entities_in_cell(&self, cell: GridCell) -> Vec<Entity> {
        self.grid.get(&cell).cloned().unwrap_or_default()
    }

    /// Get all entities within a radius around a position
    /// Returns entities from all grid cells that could potentially contain entities within the radius
    pub fn get_entities_in_radius(&self, position: &Position, radius: f32) -> Vec<Entity> {
        let center_cell = self.world_to_grid(position);
        let cell_radius = (radius / self.cell_size).ceil() as i32;

        let mut entities = Vec::new();

        for dx in -cell_radius..=cell_radius {
            for dy in -cell_radius..=cell_radius {
                let cell = GridCell {
                    x: center_cell.x + dx,
                    y: center_cell.y + dy,
                };
                entities.extend(self.get_entities_in_cell(cell));
            }
        }

        entities
    }

    /// Add entity to the spatial grid at given position
    pub fn insert_entity(&mut self, entity: Entity, position: &Position) {
        let cell = self.world_to_grid(position);
        self.grid.entry(cell).or_default().push(entity);
    }

    /// Remove entity from the spatial grid
    pub fn remove_entity(&mut self, entity: Entity, position: &Position) {
        let cell = self.world_to_grid(position);
        if let Some(entities) = self.grid.get_mut(&cell) {
            entities.retain(|&e| e != entity);
            if entities.is_empty() {
                self.grid.remove(&cell);
            }
        }
    }

    /// Update entity position in the grid (remove from old cell, add to new cell)
    pub fn update_entity_position(
        &mut self,
        entity: Entity,
        old_position: &Position,
        new_position: &Position,
    ) {
        let old_cell = self.world_to_grid(old_position);
        let new_cell = self.world_to_grid(new_position);

        // Only update if the entity moved to a different cell
        if old_cell != new_cell {
            self.remove_entity(entity, old_position);
            self.insert_entity(entity, new_position);
        }
    }

    /// Clear all entities from the grid
    pub fn clear(&mut self) {
        self.grid.clear();
    }
}

/// Resource for tracking and displaying performance metrics
#[derive(Resource)]
pub struct PerformanceMetrics {
    /// Current frames per second
    pub fps: f32,
    /// Average frame time in milliseconds
    pub frame_time_ms: f32,
    /// Total number of ant entities
    pub ant_count: usize,
    /// Total number of food source entities
    pub food_count: usize,
    /// Total number of soil entities
    pub soil_count: usize,
    /// Number of occupied spatial grid cells
    pub spatial_grid_cells: usize,
    /// Average entities per spatial grid cell
    pub avg_entities_per_cell: f32,
    /// Frame time history for smoothing
    frame_times: Vec<f32>,
    /// Update timer for performance calculations
    pub update_timer: Timer,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            fps: 0.0,
            frame_time_ms: 0.0,
            ant_count: 0,
            food_count: 0,
            soil_count: 0,
            spatial_grid_cells: 0,
            avg_entities_per_cell: 0.0,
            frame_times: Vec::with_capacity(60), // Store last 60 frame times
            update_timer: Timer::from_seconds(0.1, TimerMode::Repeating), // Update 10 times per second
        }
    }
}

impl PerformanceMetrics {
    /// Add a new frame time measurement
    pub fn add_frame_time(&mut self, frame_time: f32) {
        self.frame_times.push(frame_time);

        // Keep only the last 60 measurements
        if self.frame_times.len() > 60 {
            self.frame_times.remove(0);
        }

        // Calculate average frame time and FPS
        if !self.frame_times.is_empty() {
            self.frame_time_ms =
                self.frame_times.iter().sum::<f32>() / self.frame_times.len() as f32 * 1000.0;
            self.fps = 1.0 / (self.frame_time_ms / 1000.0);
        }
    }

    /// Update entity counts
    pub fn update_entity_counts(&mut self, ants: usize, food: usize, soil: usize) {
        self.ant_count = ants;
        self.food_count = food;
        self.soil_count = soil;
    }

    /// Update spatial grid statistics
    pub fn update_spatial_stats(&mut self, occupied_cells: usize, total_entities: usize) {
        self.spatial_grid_cells = occupied_cells;
        self.avg_entities_per_cell = if occupied_cells > 0 {
            total_entities as f32 / occupied_cells as f32
        } else {
            0.0
        };
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
    pub young_ants: usize,   // Age < 30% of max_age
    pub adult_ants: usize,   // Age 30-70% of max_age
    pub elderly_ants: usize, // Age > 70% of max_age
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

    // Role Distribution Statistics
    pub role_general_workers: usize,
    pub role_foragers: usize,
    pub role_nest_maintainers: usize,
    pub role_nursery_workers: usize,
    pub role_waste_managers: usize,
    pub role_storage_workers: usize,

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

        format!(
            "Young: {:.0}%, Adult: {:.0}%, Elderly: {:.0}%",
            young_pct, adult_pct, elderly_pct
        )
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

    /// Get formatted role distribution text
    pub fn role_distribution_text(&self) -> String {
        if self.total_ant_count == 0 {
            return "No ants".to_string();
        }

        let general_pct = (self.role_general_workers as f32 / self.total_ant_count as f32) * 100.0;
        let forager_pct = (self.role_foragers as f32 / self.total_ant_count as f32) * 100.0;
        let maintainer_pct =
            (self.role_nest_maintainers as f32 / self.total_ant_count as f32) * 100.0;
        let nursery_pct = (self.role_nursery_workers as f32 / self.total_ant_count as f32) * 100.0;
        let waste_pct = (self.role_waste_managers as f32 / self.total_ant_count as f32) * 100.0;
        let storage_pct = (self.role_storage_workers as f32 / self.total_ant_count as f32) * 100.0;

        format!(
            "General: {:.0}%, Foragers: {:.0}%, Nest: {:.0}%, Nursery: {:.0}%, Waste: {:.0}%, Storage: {:.0}%",
            general_pct, forager_pct, maintainer_pct, nursery_pct, waste_pct, storage_pct
        )
    }

    /// Get total specialized workers (excluding general workers)
    pub fn total_specialized_workers(&self) -> usize {
        self.role_foragers
            + self.role_nest_maintainers
            + self.role_nursery_workers
            + self.role_waste_managers
            + self.role_storage_workers
    }

    /// Get specialization rate as percentage
    pub fn specialization_rate(&self) -> f32 {
        if self.total_ant_count == 0 {
            return 0.0;
        }
        (self.total_specialized_workers() as f32 / self.total_ant_count as f32) * 100.0
    }
}

/// Component marker for the statistics display panel
#[derive(Component)]
pub struct StatisticsPanel;

/// Component for statistics toggle functionality
#[derive(Component, Default)]
pub struct StatisticsToggle {
    pub is_visible: bool,
}

/// Component for the performance monitoring panel
#[derive(Component)]
pub struct PerformancePanel;

/// Component for FPS display text
#[derive(Component)]
pub struct FpsText;

/// Component for frame time display text
#[derive(Component)]
pub struct FrameTimeText;

/// Component for entity count display text
#[derive(Component)]
pub struct EntityCountText;

/// Component for spatial grid stats display text
#[derive(Component)]
pub struct SpatialStatsText;

/// Settings and configuration components
/// Resource for user settings and preferences
#[derive(Resource, Clone)]
pub struct UserSettings {
    // Visual settings
    pub visual_effects_enabled: bool,
    pub ui_scale: f32,
    pub color_theme: ColorTheme,
    pub performance_mode: bool,

    // Accessibility settings
    pub high_contrast: bool,
    pub reduced_motion: bool,
    pub large_ui_elements: bool,
    pub keyboard_navigation: bool,

    // Application settings
    pub auto_save_interval: f32,
    pub default_speed: f32,
    pub window_size: (u32, u32),
    pub panel_layout: PanelLayout,
}

/// Color theme options for the application
#[derive(Clone, Default, PartialEq)]
pub enum ColorTheme {
    #[default]
    Default,
    HighContrast,
    ColorblindFriendly,
}

/// Panel layout configuration options
#[derive(Clone, Default, PartialEq)]
pub enum PanelLayout {
    #[default]
    Standard,
    Compact,
    FullScreen,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            // Visual settings defaults
            visual_effects_enabled: true,
            ui_scale: 1.0,
            color_theme: ColorTheme::Default,
            performance_mode: false,

            // Accessibility settings defaults
            high_contrast: false,
            reduced_motion: false,
            large_ui_elements: false,
            keyboard_navigation: false,

            // Application settings defaults
            auto_save_interval: 300.0, // 5 minutes
            default_speed: 1.0,
            window_size: (1200, 800),
            panel_layout: PanelLayout::Standard,
        }
    }
}

/// Component marker for the settings panel
#[derive(Component)]
pub struct SettingsPanel;

/// Component for settings toggle button
#[derive(Component, Default)]
pub struct SettingsToggle {
    pub is_visible: bool,
}

/// Component for settings category tabs
#[derive(Component)]
pub struct SettingsCategory {
    pub category: SettingsCategoryType,
    pub is_active: bool,
}

/// Types of settings categories
#[derive(Clone, PartialEq)]
pub enum SettingsCategoryType {
    Visual,
    Accessibility,
    Application,
}

/// Component for individual setting items
#[derive(Component)]
pub struct SettingItem {
    pub setting_type: SettingType,
}

/// Types of individual settings
#[derive(Clone, PartialEq)]
pub enum SettingType {
    VisualEffectsToggle,
    UIScale,
    ColorTheme,
    PerformanceMode,
    HighContrast,
    ReducedMotion,
    LargeUIElements,
    KeyboardNavigation,
    AutoSaveInterval,
    DefaultSpeed,
    PanelLayout,
}

/// Component for settings control buttons
#[derive(Component)]
pub struct SettingsButton {
    pub action: SettingsAction,
}

/// Actions that settings buttons can perform
#[derive(Clone, PartialEq)]
pub enum SettingsAction {
    ApplySettings,
    ResetToDefaults,
    SaveSettings,
    LoadSettings,
    ClosePanel,
}

/// Tooltip system components for enhanced user experience
/// Component for tooltip data attached to UI elements
#[derive(Component, Clone)]
pub struct Tooltip {
    pub text: String,
    pub shortcut: Option<String>,
    pub position: TooltipPosition,
}

/// Position options for tooltip display
#[derive(Clone, PartialEq, Default)]
pub enum TooltipPosition {
    #[default]
    Below,
    Above,
    Left,
    Right,
}

/// Marker component for currently displayed tooltip
#[derive(Component)]
pub struct TooltipDisplay;

/// Component for tooltip trigger state
#[derive(Component)]
pub struct TooltipTrigger {
    pub is_hovered: bool,
    pub hover_timer: f32,
    pub show_delay: f32,
}

impl Default for TooltipTrigger {
    fn default() -> Self {
        Self {
            is_hovered: false,
            hover_timer: 0.0,
            show_delay: 0.5, // Show tooltip after 500ms hover
        }
    }
}

/// Resource for unified UI theme and design system
#[derive(Resource, Clone)]
pub struct UITheme {
    // Color Palette
    pub colors: UIColors,
    // Typography Scale
    pub typography: UITypography,
    // Spacing System
    pub spacing: UISpacing,
    // Border & Radius System
    pub borders: UIBorders,
    // Interactive States
    pub states: UIStates,
}

/// Unified color palette for consistent theming
#[derive(Clone)]
pub struct UIColors {
    // Surface Colors
    pub surface_primary: Color,   // Main panel backgrounds
    pub surface_secondary: Color, // Secondary panel areas
    pub surface_elevated: Color,  // Elevated elements (buttons, cards)

    // Border Colors
    pub border_primary: Color,   // Main borders
    pub border_secondary: Color, // Subtle borders
    pub border_focus: Color,     // Focus indicators

    // Text Colors
    pub text_primary: Color,   // Main text
    pub text_secondary: Color, // Secondary text
    pub text_accent: Color,    // Accent text (speed display)
    pub text_muted: Color,     // Muted text (instructions)

    // Interactive Colors
    pub action_primary: Color,   // Primary action buttons (play/pause)
    pub action_secondary: Color, // Secondary actions (speed presets)
    pub action_success: Color,   // Success states
    pub action_warning: Color,   // Warning states
    pub action_danger: Color,    // Danger states

    // Semantic Colors
    pub accent_blue: Color,   // Blue accents
    pub accent_green: Color,  // Green accents
    pub accent_orange: Color, // Orange accents
    pub accent_red: Color,    // Red accents
}

/// Typography scale for consistent text hierarchy
#[derive(Clone)]
pub struct UITypography {
    pub heading_large: f32,  // 24px - Main panel titles
    pub heading_medium: f32, // 20px - Section headers
    pub heading_small: f32,  // 18px - Subsection headers
    pub body_large: f32,     // 16px - Primary body text
    pub body_medium: f32,    // 14px - Secondary body text
    pub body_small: f32,     // 12px - Captions, labels
    pub caption: f32,        // 10px - Fine print, instructions
}

/// Spacing system for consistent layout rhythm
#[derive(Clone)]
pub struct UISpacing {
    pub xs: f32,  // 4px
    pub sm: f32,  // 8px
    pub md: f32,  // 12px
    pub lg: f32,  // 16px
    pub xl: f32,  // 24px
    pub xxl: f32, // 32px
}

/// Border and radius system for consistent shapes
#[derive(Clone)]
pub struct UIBorders {
    pub width_thin: f32,    // 1px
    pub width_medium: f32,  // 2px
    pub width_thick: f32,   // 3px
    pub radius_small: f32,  // 4px
    pub radius_medium: f32, // 8px
    pub radius_large: f32,  // 12px
    pub radius_round: f32,  // 50% (for circular elements)
}

/// Interactive state colors for hover, active, disabled states
#[derive(Clone)]
pub struct UIStates {
    pub hover_overlay: Color,    // Overlay for hover states
    pub active_overlay: Color,   // Overlay for active states
    pub disabled_overlay: Color, // Overlay for disabled states
    pub focus_outline: Color,    // Focus outline color
}

impl Default for UITheme {
    fn default() -> Self {
        Self {
            colors: UIColors {
                // Surface Colors - Dark theme with improved contrast
                surface_primary: Color::srgba(0.12, 0.12, 0.12, 0.95),
                surface_secondary: Color::srgba(0.18, 0.18, 0.18, 0.9),
                surface_elevated: Color::srgba(0.22, 0.22, 0.22, 0.9),

                // Border Colors
                border_primary: Color::srgb(0.35, 0.35, 0.35),
                border_secondary: Color::srgb(0.25, 0.25, 0.25),
                border_focus: Color::srgb(0.4, 0.7, 1.0),

                // Text Colors - Improved readability
                text_primary: Color::srgb(0.95, 0.95, 0.95),
                text_secondary: Color::srgb(0.8, 0.8, 0.8),
                text_accent: Color::srgb(0.7, 1.0, 0.7),
                text_muted: Color::srgb(0.6, 0.6, 0.6),

                // Interactive Colors - More vibrant and accessible
                action_primary: Color::srgb(0.2, 0.7, 0.2), // Green for play/pause
                action_secondary: Color::srgb(0.3, 0.4, 0.8), // Blue for speed controls
                action_success: Color::srgb(0.1, 0.8, 0.1),
                action_warning: Color::srgb(1.0, 0.7, 0.1),
                action_danger: Color::srgb(0.9, 0.2, 0.2),

                // Semantic Colors
                accent_blue: Color::srgb(0.3, 0.6, 1.0),
                accent_green: Color::srgb(0.2, 0.8, 0.3),
                accent_orange: Color::srgb(1.0, 0.6, 0.1),
                accent_red: Color::srgb(1.0, 0.3, 0.3),
            },

            typography: UITypography {
                heading_large: 24.0,
                heading_medium: 20.0,
                heading_small: 18.0,
                body_large: 16.0,
                body_medium: 14.0,
                body_small: 12.0,
                caption: 10.0,
            },

            spacing: UISpacing {
                xs: 4.0,
                sm: 8.0,
                md: 12.0,
                lg: 16.0,
                xl: 24.0,
                xxl: 32.0,
            },

            borders: UIBorders {
                width_thin: 1.0,
                width_medium: 2.0,
                width_thick: 3.0,
                radius_small: 4.0,
                radius_medium: 8.0,
                radius_large: 12.0,
                radius_round: 50.0,
            },

            states: UIStates {
                hover_overlay: Color::srgba(1.0, 1.0, 1.0, 0.1),
                active_overlay: Color::srgba(1.0, 1.0, 1.0, 0.2),
                disabled_overlay: Color::srgba(0.0, 0.0, 0.0, 0.5),
                focus_outline: Color::srgb(0.4, 0.7, 1.0),
            },
        }
    }
}

impl UITheme {
    /// Get hover state color for a base color
    pub fn get_hover_color(&self, base_color: Color) -> Color {
        // Use predefined hover colors based on common base colors
        match base_color {
            // Primary action button hover
            c if c.to_srgba() == self.colors.action_primary.to_srgba() => {
                Color::srgb(0.3, 0.8, 0.3)
            }
            // Secondary action button hover
            c if c.to_srgba() == self.colors.action_secondary.to_srgba() => {
                Color::srgb(0.4, 0.5, 0.9)
            }
            // Default: slightly lighter overlay effect
            _ => Color::srgba(0.8, 0.8, 0.8, 0.1),
        }
    }

    /// Get active state color for a base color
    pub fn get_active_color(&self, base_color: Color) -> Color {
        // Use predefined active colors based on common base colors
        match base_color {
            // Primary action button active
            c if c.to_srgba() == self.colors.action_primary.to_srgba() => {
                Color::srgb(0.1, 0.5, 0.1)
            }
            // Secondary action button active
            c if c.to_srgba() == self.colors.action_secondary.to_srgba() => {
                Color::srgb(0.2, 0.3, 0.6)
            }
            // Default: slightly darker overlay effect
            _ => Color::srgba(0.0, 0.0, 0.0, 0.2),
        }
    }

    /// Create a button style with theme colors
    pub fn create_button_style(&self, width: Val, height: Val) -> Style {
        Style {
            width,
            height,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(self.borders.width_medium)),
            padding: UiRect::all(Val::Px(self.spacing.sm)),
            margin: UiRect::all(Val::Px(self.spacing.xs)),
            ..default()
        }
    }

    /// Create a panel style with theme styling
    pub fn create_panel_style(&self, width: Val, height: Val) -> Style {
        Style {
            width,
            height,
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(self.spacing.lg)),
            border: UiRect::all(Val::Px(self.borders.width_medium)),
            row_gap: Val::Px(self.spacing.md),
            ..default()
        }
    }
}

// === Cross-Section Ant Farm Components ===

/// Marker component for ground surface boundary line
#[derive(Component, Serialize, Deserialize)]
pub struct GroundSurface;

/// Component for tunnel entities in the cross-section view
#[derive(Component, Clone, Serialize, Deserialize)]
pub struct Tunnel {
    /// Width of the tunnel for pathfinding
    pub width: f32,
    /// Connected tunnel/chamber positions for pathfinding network
    pub connections: Vec<Position>,
    /// Whether this tunnel is currently being excavated by ants
    pub under_construction: bool,
}

/// Component for chamber entities with specialized functions
#[derive(Component, Clone, Serialize, Deserialize)]
pub struct Chamber {
    /// Type and purpose of this chamber
    pub chamber_type: ChamberType,
    /// Radius/size of the chamber
    pub radius: f32,
    /// Connected tunnel entry/exit points
    pub tunnel_connections: Vec<Position>,
    /// Current capacity usage (0.0 = empty, 1.0 = full)
    pub capacity_usage: f32,
    /// Maximum capacity for this chamber type
    pub max_capacity: f32,
}

/// Types of chambers in the ant colony cross-section
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChamberType {
    /// Central queen's chamber for reproduction
    Queen,
    /// Nursery chambers for eggs and larvae
    Nursery,
    /// Food storage chambers
    FoodStorage,
    /// Worker rest areas and general activity
    Worker,
    /// Waste disposal chambers
    Waste,
}

impl ChamberType {
    /// Get the display name for this chamber type
    pub fn display_name(&self) -> &'static str {
        match self {
            ChamberType::Queen => "Queen Chamber",
            ChamberType::Nursery => "Nursery",
            ChamberType::FoodStorage => "Food Storage",
            ChamberType::Worker => "Worker Area",
            ChamberType::Waste => "Waste Disposal",
        }
    }

    /// Get the typical size for this chamber type
    pub fn default_radius(&self) -> f32 {
        match self {
            ChamberType::Queen => 20.0,       // Large central chamber
            ChamberType::Nursery => 15.0,     // Medium nursery chambers
            ChamberType::FoodStorage => 12.0, // Medium storage chambers
            ChamberType::Worker => 8.0,       // Small worker areas
            ChamberType::Waste => 10.0,       // Medium waste chambers
        }
    }

    /// Get the maximum capacity for this chamber type
    pub fn default_capacity(&self) -> f32 {
        match self {
            ChamberType::Queen => 1.0,         // One queen
            ChamberType::Nursery => 50.0,      // 50 eggs/larvae
            ChamberType::FoodStorage => 100.0, // 100 food units
            ChamberType::Worker => 20.0,       // 20 resting workers
            ChamberType::Waste => 30.0,        // 30 waste units
        }
    }

    /// Get the visual color for this chamber type
    pub fn get_color(&self) -> Color {
        match self {
            ChamberType::Queen => Color::srgb(0.8, 0.6, 1.0), // Purple for queen
            ChamberType::Nursery => Color::srgb(1.0, 0.9, 0.7), // Light yellow for nursery
            ChamberType::FoodStorage => Color::srgb(0.6, 0.8, 0.4), // Green for food storage
            ChamberType::Worker => Color::srgb(0.7, 0.7, 0.7), // Gray for worker areas
            ChamberType::Waste => Color::srgb(0.6, 0.4, 0.2), // Brown for waste
        }
    }
}

/// Component for depth-based soil layers in cross-section view
#[derive(Component, Clone, Serialize, Deserialize)]
pub struct DepthLayer {
    /// Depth level (0 = surface, higher = deeper)
    pub depth: u32,
    /// Soil hardness affecting digging difficulty
    pub hardness: f32,
    /// Visual color variation based on depth
    pub color_variation: f32,
}

impl DepthLayer {
    /// Create a depth layer with properties based on depth
    pub fn new(depth: u32) -> Self {
        let hardness = match depth {
            0..=2 => 0.2, // Easy to dig near surface
            3..=5 => 0.5, // Medium difficulty
            6..=8 => 0.8, // Hard deeper soil
            _ => 1.0,     // Very hard clay/rock
        };

        let color_variation = (depth as f32 * 0.1).min(0.5); // Darker with depth

        Self {
            depth,
            hardness,
            color_variation,
        }
    }

    /// Get the soil color for this depth layer
    pub fn get_soil_color(&self) -> Color {
        let base_brown = 0.6 - (self.depth as f32 * 0.05).min(0.3);
        Color::srgb(base_brown, base_brown * 0.7, base_brown * 0.3)
    }
}

/// Marker component for tunnel path nodes used in pathfinding
#[derive(Component, Clone, Serialize, Deserialize)]
pub struct TunnelNode {
    /// Position of this node in the tunnel network
    pub position: Position,
    /// Connected neighboring nodes for pathfinding
    pub neighbors: Vec<Position>,
    /// Node type for different pathfinding behaviors
    pub node_type: TunnelNodeType,
}

/// Types of tunnel nodes for pathfinding behavior
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TunnelNodeType {
    /// Standard tunnel pathway
    Pathway,
    /// Junction where multiple tunnels meet
    Junction,
    /// Chamber entrance/exit point
    ChamberEntrance,
    /// Surface exit point
    SurfaceExit,
}

// === Camponotus japonicus Colony Development System ===

/// Resource for tracking the colony's current development phase
/// Based on realistic Camponotus japonicus (Japanese carpenter ant) lifecycle
#[derive(Resource, Clone, Serialize, Deserialize)]
pub struct ColonyDevelopmentPhase {
    /// Current phase of colony development
    pub current_phase: DevelopmentPhase,
    /// Time spent in current phase (in simulation days)
    pub time_in_phase: f32,
    /// Progress toward next phase (0.0 to 1.0)
    pub phase_progress: f32,
    /// Unique conditions and events for this colony run
    pub colony_traits: ColonyTraits,
    /// Phase transition conditions and thresholds
    pub phase_conditions: PhaseConditions,
}

/// Four distinct phases of Camponotus japonicus colony development
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DevelopmentPhase {
    /// Phase 1: Queen's Independent Founding (Days 1-365)
    /// - Single queen starts colony alone after nuptial flight
    /// - Self-sufficient nest creation and first brood care
    /// - Minimal external activity during first year
    QueenFounding,

    /// Phase 2: First Workers (Days 365-730)
    /// - Birth of initial worker population (5-20 workers)
    /// - Limited foraging and maintenance activities
    /// - Colony remains mostly underground
    FirstWorkers,

    /// Phase 3: Colony Expansion (Days 730-1095)
    /// - Explosive growth in activity and population (50-200 workers)
    /// - Complex social behaviors emerge
    /// - Extensive nest construction (10-30cm depth)
    ColonyExpansion,

    /// Phase 4: Mature Colony (Days 1095+)
    /// - Age-based division of labor system (200+ workers)
    /// - Seasonal behavioral patterns
    /// - Sophisticated waste management
    /// - Potential for reproductive alates
    MatureColony,
}

/// Unique traits and characteristics for each colony simulation
/// Provides replayability through randomization
#[derive(Clone, Serialize, Deserialize)]
pub struct ColonyTraits {
    /// Queen's initial energy and reproductive capacity
    pub queen_vigor: f32, // 0.7-1.0, affects egg laying rate
    /// Environmental adaptation rating
    pub environmental_adaptation: f32, // 0.6-1.0, affects survival in disasters
    /// Worker efficiency and intelligence
    pub worker_efficiency: f32, // 0.8-1.2, affects foraging and construction
    /// Colony's resilience to environmental challenges
    pub disaster_resistance: f32, // 0.5-1.0, affects disaster impact
    /// Social organization efficiency
    pub social_organization: f32, // 0.7-1.1, affects coordination
    /// Nest construction skill
    pub architectural_skill: f32, // 0.6-1.0, affects tunnel/chamber quality
}

/// Conditions and thresholds for phase transitions
#[derive(Clone, Serialize, Deserialize)]
pub struct PhaseConditions {
    /// Minimum days required for current phase
    pub min_days_in_phase: f32,
    /// Target worker population for next phase
    pub target_worker_count: usize,
    /// Required nest development (tunnel/chamber count)
    pub required_nest_complexity: usize,
    /// Colony stability threshold (survival rate)
    pub stability_threshold: f32,
}

impl Default for ColonyDevelopmentPhase {
    fn default() -> Self {
        Self {
            current_phase: DevelopmentPhase::QueenFounding,
            time_in_phase: 0.0,
            phase_progress: 0.0,
            colony_traits: ColonyTraits::generate_random(),
            phase_conditions: PhaseConditions::for_phase(DevelopmentPhase::QueenFounding),
        }
    }
}

impl ColonyTraits {
    /// Generate randomized colony traits for unique simulation experience
    pub fn generate_random() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        Self {
            queen_vigor: rng.gen_range(0.7..=1.0),
            environmental_adaptation: rng.gen_range(0.6..=1.0),
            worker_efficiency: rng.gen_range(0.8..=1.2),
            disaster_resistance: rng.gen_range(0.5..=1.0),
            social_organization: rng.gen_range(0.7..=1.1),
            architectural_skill: rng.gen_range(0.6..=1.0),
        }
    }
}

impl PhaseConditions {
    /// Get appropriate conditions for a specific development phase
    pub fn for_phase(phase: DevelopmentPhase) -> Self {
        match phase {
            DevelopmentPhase::QueenFounding => Self {
                min_days_in_phase: 300.0,    // Minimum 300 simulation days
                target_worker_count: 5,      // At least 5 workers to advance
                required_nest_complexity: 2, // Basic queen chamber + nursery
                stability_threshold: 0.8,    // 80% queen survival rate
            },
            DevelopmentPhase::FirstWorkers => Self {
                min_days_in_phase: 200.0,    // Minimum 200 simulation days
                target_worker_count: 25,     // At least 25 workers
                required_nest_complexity: 5, // More chambers and tunnels
                stability_threshold: 0.7,    // 70% worker survival rate
            },
            DevelopmentPhase::ColonyExpansion => Self {
                min_days_in_phase: 365.0,     // Minimum 365 simulation days
                target_worker_count: 100,     // At least 100 workers
                required_nest_complexity: 10, // Complex tunnel network
                stability_threshold: 0.8,     // 80% colony stability
            },
            DevelopmentPhase::MatureColony => Self {
                min_days_in_phase: f32::INFINITY,     // Permanent final phase
                target_worker_count: usize::MAX,      // No upper limit
                required_nest_complexity: usize::MAX, // No upper limit
                stability_threshold: 0.9,             // 90% mature colony stability
            },
        }
    }
}

impl DevelopmentPhase {
    /// Get the display name for this development phase
    pub fn display_name(&self) -> &'static str {
        match self {
            DevelopmentPhase::QueenFounding => "Queen's Founding",
            DevelopmentPhase::FirstWorkers => "First Workers",
            DevelopmentPhase::ColonyExpansion => "Colony Expansion",
            DevelopmentPhase::MatureColony => "Mature Colony",
        }
    }

    /// Get a description of what happens in this phase
    pub fn description(&self) -> &'static str {
        match self {
            DevelopmentPhase::QueenFounding => "The queen establishes her nest alone, caring for the first brood without worker assistance.",
            DevelopmentPhase::FirstWorkers => "The first workers emerge and begin basic foraging and nest maintenance activities.",
            DevelopmentPhase::ColonyExpansion => "Rapid population growth with complex social behaviors and extensive nest construction.",
            DevelopmentPhase::MatureColony => "Fully developed colony with sophisticated division of labor and seasonal behaviors.",
        }
    }

    /// Get the expected duration range for this phase
    pub fn expected_duration_days(&self) -> (f32, f32) {
        match self {
            DevelopmentPhase::QueenFounding => (300.0, 400.0),
            DevelopmentPhase::FirstWorkers => (200.0, 365.0),
            DevelopmentPhase::ColonyExpansion => (365.0, 500.0),
            DevelopmentPhase::MatureColony => (f32::INFINITY, f32::INFINITY),
        }
    }

    /// Get the next phase in the development sequence
    pub fn next_phase(&self) -> Option<DevelopmentPhase> {
        match self {
            DevelopmentPhase::QueenFounding => Some(DevelopmentPhase::FirstWorkers),
            DevelopmentPhase::FirstWorkers => Some(DevelopmentPhase::ColonyExpansion),
            DevelopmentPhase::ColonyExpansion => Some(DevelopmentPhase::MatureColony),
            DevelopmentPhase::MatureColony => None, // Final phase
        }
    }
}

/// Component for ants with phase-specific behavioral modifications
#[derive(Component, Clone, Serialize, Deserialize)]
pub struct PhaseSpecificBehavior {
    /// How this ant's behavior is modified by the current colony phase
    pub behavior_modifiers: BehaviorModifiers,
    /// Age group of this ant affecting its role
    pub age_group: AntAgeGroup,
    /// Specialized role within the current phase
    pub specialized_role: SpecializedRole,
}

/// Behavioral modifiers based on colony development phase
#[derive(Clone, Serialize, Deserialize)]
pub struct BehaviorModifiers {
    /// Movement speed modifier (0.5-1.5)
    pub speed_modifier: f32,
    /// Foraging efficiency modifier (0.5-1.5)
    pub foraging_efficiency: f32,
    /// Construction skill modifier (0.5-1.5)
    pub construction_skill: f32,
    /// Energy efficiency modifier (0.8-1.2)
    pub energy_efficiency: f32,
}

/// Age-based categorization of ants for phase-appropriate roles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AntAgeGroup {
    /// Young ants (0-30% of max age) - stay in nest, tend brood
    Young,
    /// Adult ants (30-70% of max age) - versatile workers
    Adult,
    /// Senior ants (70%+ of max age) - experienced foragers
    Senior,
}

/// Specialized roles that emerge during different colony phases
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpecializedRole {
    /// General purpose worker (all phases)
    GeneralWorker,
    /// Specialized forager (FirstWorkers+)
    Forager,
    /// Nest maintenance specialist (ColonyExpansion+)
    NestMaintainer,
    /// Nursery caretaker (FirstWorkers+)
    NurseryWorker,
    /// Waste management specialist (MatureColony)
    WasteManager,
    /// Food storage organizer (ColonyExpansion+)
    StorageWorker,
}

impl Default for PhaseSpecificBehavior {
    fn default() -> Self {
        Self {
            behavior_modifiers: BehaviorModifiers::default(),
            age_group: AntAgeGroup::Adult,
            specialized_role: SpecializedRole::GeneralWorker,
        }
    }
}

impl Default for BehaviorModifiers {
    fn default() -> Self {
        Self {
            speed_modifier: 1.0,
            foraging_efficiency: 1.0,
            construction_skill: 1.0,
            energy_efficiency: 1.0,
        }
    }
}

impl AntAgeGroup {
    /// Determine age group based on ant's current age and max age
    pub fn from_age_ratio(age_ratio: f32) -> Self {
        if age_ratio < 0.3 {
            AntAgeGroup::Young
        } else if age_ratio < 0.7 {
            AntAgeGroup::Adult
        } else {
            AntAgeGroup::Senior
        }
    }
}

impl SpecializedRole {
    /// Get the display name for this specialized role
    pub fn display_name(&self) -> &'static str {
        match self {
            SpecializedRole::GeneralWorker => "General Worker",
            SpecializedRole::Forager => "Forager",
            SpecializedRole::NestMaintainer => "Nest Maintainer",
            SpecializedRole::NurseryWorker => "Nursery Worker",
            SpecializedRole::WasteManager => "Waste Manager",
            SpecializedRole::StorageWorker => "Storage Worker",
        }
    }
}
