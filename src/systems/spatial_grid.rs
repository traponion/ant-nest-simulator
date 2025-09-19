use crate::components::{Food, FoodSource, Position, SpatialGrid};
use bevy::prelude::*;

/// System to initialize the spatial grid with food source entities at startup
pub fn initialize_spatial_grid_system(
    mut spatial_grid: ResMut<SpatialGrid>,
    food_query: Query<(Entity, &Position), (With<Food>, With<FoodSource>)>,
) {
    // Clear any existing entries
    spatial_grid.clear();

    // Add all food sources to the spatial grid
    for (entity, position) in food_query.iter() {
        spatial_grid.insert_entity(entity, position);
    }

    info!("Spatial grid initialized with {} food sources", food_query.iter().count());
}

/// System to track food source position changes and update spatial grid accordingly
/// This system should run whenever food sources are added, removed, or moved
pub fn update_food_sources_in_grid_system(
    mut spatial_grid: ResMut<SpatialGrid>,
    // Query for food sources that have changed (added or moved)
    changed_food_query: Query<(Entity, &Position), (With<Food>, With<FoodSource>, Changed<Position>)>,
    // Query for all food sources to handle additions
    all_food_query: Query<(Entity, &Position), (With<Food>, With<FoodSource>)>,
    mut last_food_count: Local<usize>,
) {
    let current_food_count = all_food_query.iter().count();

    // Handle new food sources being added
    if current_food_count != *last_food_count {
        // Rebuild the grid to catch new entities
        spatial_grid.clear();
        for (entity, position) in all_food_query.iter() {
            spatial_grid.insert_entity(entity, position);
        }
        *last_food_count = current_food_count;
        return;
    }

    // Handle position changes for existing food sources
    for (_entity, _new_position) in changed_food_query.iter() {
        // For position changes, we need the old position, but Change detection doesn't provide it
        // For now, we'll rebuild the entire grid when any food source moves
        // TODO: Optimize this by storing previous positions
        spatial_grid.clear();
        for (entity, position) in all_food_query.iter() {
            spatial_grid.insert_entity(entity, position);
        }
        break; // Only need to rebuild once per frame
    }
}

/// System to remove despawned food sources from the spatial grid
/// Note: Currently simplified to work with the update system
pub fn cleanup_despawned_food_sources_system(
    mut _spatial_grid: ResMut<SpatialGrid>,
    mut removed_food_events: RemovedComponents<Food>,
) {
    // Check if any food sources were removed
    let _removed_count = removed_food_events.read().count();

    // TODO: Implement proper cleanup when we have access to remaining food entities
    // For now, the update_food_sources_in_grid_system handles this by detecting count changes
}

/// Debug system to print spatial grid statistics
pub fn debug_spatial_grid_system(
    spatial_grid: Res<SpatialGrid>,
    mut timer: Local<Timer>,
    time: Res<Time>,
) {
    // Only run this debug system every 5 seconds
    if timer.duration().is_zero() {
        *timer = Timer::from_seconds(5.0, TimerMode::Repeating);
    }

    if timer.tick(time.delta()).just_finished() {
        let total_entities = spatial_grid.grid.values().map(|v| v.len()).sum::<usize>();
        let occupied_cells = spatial_grid.grid.len();

        debug!(
            "Spatial Grid Stats: {} entities across {} cells (avg: {:.1} per cell)",
            total_entities,
            occupied_cells,
            if occupied_cells > 0 { total_entities as f32 / occupied_cells as f32 } else { 0.0 }
        );
    }
}