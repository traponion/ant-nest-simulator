use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use directories::ProjectDirs;

use crate::components::*;

/// Saveable game state structure
#[derive(Serialize, Deserialize)]
pub struct SaveData {
    /// All ant entities with their components
    pub ants: Vec<AntData>,
    /// All soil entities with their components
    pub soil_cells: Vec<SoilData>,
    /// All food entities with their components
    pub food_sources: Vec<FoodSourceData>,
    /// Queen ant data
    pub queen: Option<QueenData>,
    /// Game settings and state
    pub game_state: GameStateData,
    /// Save metadata
    pub metadata: SaveMetadata,
}

/// Ant entity data for serialization
#[derive(Serialize, Deserialize)]
pub struct AntData {
    pub position: Position,
    pub behavior: AntBehavior,
    pub lifecycle: Lifecycle,
    pub inventory: Option<Inventory>,
}

/// Soil cell data for serialization
#[derive(Serialize, Deserialize)]
pub struct SoilData {
    pub position: Position,
    pub soil_cell: SoilCell,
}

/// Food source data for serialization
#[derive(Serialize, Deserialize)]
pub struct FoodSourceData {
    pub position: Position,
    pub food_source: FoodSource,
}

/// Queen ant data for serialization
#[derive(Serialize, Deserialize)]
pub struct QueenData {
    pub position: Position,
    pub lifecycle: Lifecycle,
    pub reproduction_state: ReproductionState,
}

/// Game state data for serialization
#[derive(Serialize, Deserialize)]
pub struct GameStateData {
    pub simulation_time: f32,
    pub is_paused: bool,
    pub speed_multiplier: f32,
}

/// Save file metadata
#[derive(Serialize, Deserialize)]
pub struct SaveMetadata {
    pub save_name: String,
    pub creation_time: String,
    pub colony_age: f32,
    pub ant_population: usize,
    pub version: String,
}

/// Resource for managing save/load operations
#[derive(Resource, Default)]
pub struct PersistenceState {
    pub save_directory: Option<PathBuf>,
    pub last_save_time: f32,
    pub auto_save_interval: f32, // in seconds
    pub is_saving: bool,
    pub is_loading: bool,
}

impl PersistenceState {
    pub fn new() -> Self {
        let save_directory = if let Some(proj_dirs) = ProjectDirs::from("com", "traponion", "ant-nest-simulator") {
            let save_dir = proj_dirs.data_dir().join("saves");
            if let Err(e) = fs::create_dir_all(&save_dir) {
                warn!("Failed to create save directory: {}", e);
                None
            } else {
                Some(save_dir)
            }
        } else {
            warn!("Failed to determine save directory");
            None
        };

        Self {
            save_directory,
            last_save_time: 0.0,
            auto_save_interval: 300.0, // Auto-save every 5 minutes
            is_saving: false,
            is_loading: false,
        }
    }
}

/// Save the current game state to a file
pub fn save_game_system(
    mut persistence_state: ResMut<PersistenceState>,
    time: Res<Time>,
    ant_query: Query<(&Position, &AntBehavior, &Lifecycle, Option<&Inventory>), (With<Ant>, Without<Queen>)>,
    soil_query: Query<(&Position, &SoilCell), With<Soil>>,
    food_query: Query<(&Position, &FoodSource), With<Food>>,
    queen_query: Query<(&Position, &Lifecycle, &ReproductionState), With<Queen>>,
    time_control: Res<TimeControl>,
    input: Res<ButtonInput<KeyCode>>,
) {
    // Manual save with Ctrl+S
    let should_manual_save = input.pressed(KeyCode::ControlLeft) && input.just_pressed(KeyCode::KeyS);

    // Auto-save check
    let time_since_last_save = time.elapsed_seconds() - persistence_state.last_save_time;
    let should_auto_save = time_since_last_save >= persistence_state.auto_save_interval;

    if !should_manual_save && !should_auto_save {
        return;
    }

    if persistence_state.is_saving {
        return; // Already saving
    }

    persistence_state.is_saving = true;

    // Collect ant data
    let ants: Vec<AntData> = ant_query
        .iter()
        .map(|(pos, behavior, lifecycle, inventory)| AntData {
            position: pos.clone(),
            behavior: behavior.clone(),
            lifecycle: lifecycle.clone(),
            inventory: inventory.cloned(),
        })
        .collect();

    // Collect soil data
    let soil_cells: Vec<SoilData> = soil_query
        .iter()
        .map(|(pos, soil_cell)| SoilData {
            position: pos.clone(),
            soil_cell: soil_cell.clone(),
        })
        .collect();

    // Collect food source data
    let food_sources: Vec<FoodSourceData> = food_query
        .iter()
        .map(|(pos, food_source)| FoodSourceData {
            position: pos.clone(),
            food_source: food_source.clone(),
        })
        .collect();

    // Collect queen data
    let queen = queen_query
        .get_single()
        .ok()
        .map(|(pos, lifecycle, reproduction)| QueenData {
            position: pos.clone(),
            lifecycle: lifecycle.clone(),
            reproduction_state: reproduction.clone(),
        });

    // Create game state
    let game_state = GameStateData {
        simulation_time: time.elapsed_seconds(),
        is_paused: time_control.is_paused,
        speed_multiplier: time_control.speed_multiplier,
    };

    // Create metadata
    let metadata = SaveMetadata {
        save_name: format!("colony_{}", chrono::Utc::now().format("%Y%m%d_%H%M%S")),
        creation_time: chrono::Utc::now().to_rfc3339(),
        colony_age: time.elapsed_seconds(),
        ant_population: ants.len(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };

    // Create save data
    let save_data = SaveData {
        ants,
        soil_cells,
        food_sources,
        queen,
        game_state,
        metadata,
    };

    // Save to file
    if let Some(save_dir) = &persistence_state.save_directory {
        let filename = if should_manual_save {
            format!("manual_save_{}.dat", chrono::Utc::now().format("%Y%m%d_%H%M%S"))
        } else {
            "auto_save.dat".to_string()
        };

        let save_path = save_dir.join(filename);

        match bincode::serialize(&save_data) {
            Ok(encoded) => {
                match fs::write(&save_path, encoded) {
                    Ok(_) => {
                        info!("Game saved successfully to {:?}", save_path);
                        persistence_state.last_save_time = time.elapsed_seconds();
                    }
                    Err(e) => {
                        error!("Failed to write save file: {}", e);
                    }
                }
            }
            Err(e) => {
                error!("Failed to serialize save data: {}", e);
            }
        }
    }

    persistence_state.is_saving = false;
}

/// Load game state from a file
pub fn load_game_system(
    mut commands: Commands,
    mut persistence_state: ResMut<PersistenceState>,
    input: Res<ButtonInput<KeyCode>>,
    existing_ants: Query<Entity, With<Ant>>,
    existing_soil: Query<Entity, With<Soil>>,
    existing_food: Query<Entity, With<Food>>,
    existing_queen: Query<Entity, With<Queen>>,
    mut time_control: ResMut<TimeControl>,
) {
    // Load with Ctrl+L
    if !(input.pressed(KeyCode::ControlLeft) && input.just_pressed(KeyCode::KeyL)) {
        return;
    }

    if persistence_state.is_loading {
        return; // Already loading
    }

    persistence_state.is_loading = true;

    if let Some(save_dir) = &persistence_state.save_directory {
        let save_path = save_dir.join("auto_save.dat");

        if !save_path.exists() {
            warn!("No save file found at {:?}", save_path);
            persistence_state.is_loading = false;
            return;
        }

        match fs::read(&save_path) {
            Ok(data) => {
                match bincode::deserialize::<SaveData>(&data) {
                    Ok(save_data) => {
                        info!("Loading game from {:?}", save_path);

                        // Clear existing entities
                        for entity in existing_ants.iter() {
                            commands.entity(entity).despawn();
                        }
                        for entity in existing_soil.iter() {
                            commands.entity(entity).despawn();
                        }
                        for entity in existing_food.iter() {
                            commands.entity(entity).despawn();
                        }
                        for entity in existing_queen.iter() {
                            commands.entity(entity).despawn();
                        }

                        // Restore ants
                        for ant_data in save_data.ants {
                            let mut entity = commands.spawn((
                                ant_data.position,
                                ant_data.behavior,
                                ant_data.lifecycle,
                                Ant,
                            ));

                            if let Some(inventory) = ant_data.inventory {
                                entity.insert(inventory);
                            }
                        }

                        // Restore soil
                        for soil_data in save_data.soil_cells {
                            commands.spawn((
                                soil_data.position,
                                soil_data.soil_cell,
                                Soil,
                            ));
                        }

                        // Restore food sources
                        for food_data in save_data.food_sources {
                            commands.spawn((
                                food_data.position,
                                food_data.food_source,
                                Food,
                            ));
                        }

                        // Restore queen
                        if let Some(queen_data) = save_data.queen {
                            commands.spawn((
                                queen_data.position,
                                queen_data.lifecycle,
                                queen_data.reproduction_state,
                                Queen,
                            ));
                        }

                        // Restore game state
                        time_control.is_paused = save_data.game_state.is_paused;
                        time_control.speed_multiplier = save_data.game_state.speed_multiplier;

                        info!("Game loaded successfully! Colony age: {:.1}s, Population: {}",
                              save_data.metadata.colony_age, save_data.metadata.ant_population);
                    }
                    Err(e) => {
                        error!("Failed to deserialize save data: {}", e);
                    }
                }
            }
            Err(e) => {
                error!("Failed to read save file: {}", e);
            }
        }
    }

    persistence_state.is_loading = false;
}

/// System to display save/load status
pub fn persistence_status_system(
    persistence_state: Res<PersistenceState>,
    time: Res<Time>,
) {
    if persistence_state.is_saving {
        info!("Saving game...");
    }

    if persistence_state.is_loading {
        info!("Loading game...");
    }

    // Show auto-save countdown (optional debug info)
    let time_until_autosave = persistence_state.auto_save_interval -
        (time.elapsed_seconds() - persistence_state.last_save_time);

    if time_until_autosave <= 30.0 && time_until_autosave > 0.0 {
        debug!("Auto-save in {:.1} seconds", time_until_autosave);
    }
}