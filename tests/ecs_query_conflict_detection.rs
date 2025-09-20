//! ECS Query Conflict Detection Tests
//!
//! This test module provides early detection of ECS query conflicts between Bevy systems.
//! Instead of testing all systems at once, this uses a selective approach to identify
//! potential conflicts between core simulation systems.
//!
//! Addresses Issue #81: Improve ECS Query Conflict Detection and Prevention

use ant_nest_simulator::components::*;
use bevy::prelude::*;

/// Test core ant simulation systems for query conflicts
///
/// This test focuses on the core ant simulation systems that most commonly
/// interact with ant entities and have historically caused query conflicts.
#[test]
fn test_core_ant_systems_compatibility() {
    let mut app = App::new();

    // Add minimal plugins required for ECS functionality
    app.add_plugins(MinimalPlugins);

    // Initialize only the core resources needed for ant simulation
    app.init_resource::<DisasterState>();
    app.init_resource::<SpatialGrid>();
    app.insert_resource(SpatialGrid::new(
        16.0,
        Position { x: -80.0, y: -60.0 },
        Position { x: 80.0, y: 60.0 },
    ));

    // Add core ant simulation systems that have previously caused conflicts
    app.add_systems(
        Update,
        (
            ant_nest_simulator::systems::ant_movement_system,
            ant_nest_simulator::systems::ant_lifecycle_system,
            ant_nest_simulator::systems::food_consumption_system,
        ),
    );

    // Spawn a test ant entity to ensure systems have something to process
    app.world_mut().spawn((
        Ant,
        Position { x: 0.0, y: 0.0 },
        AntBehavior {
            state: AntState::Foraging,
            target_position: None,
            speed: 10.0,
        },
        Lifecycle {
            age: 0.0,
            max_age: 100.0,
            energy: 50.0,
            max_energy: 100.0,
        },
    ));

    // Run multiple update cycles to catch any query conflicts
    for _ in 0..5 {
        app.update();
    }

    // If we reach here, no query conflicts occurred
    // Verify that the test ant entity still exists
    let world = app.world_mut();
    let ant_count = world.query::<&Ant>().iter(world).count();
    assert_eq!(
        ant_count, 1,
        "Core ant systems should maintain entities without conflicts"
    );
}

/// Test environmental and soil systems for compatibility
#[test]
fn test_environmental_systems_compatibility() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    // Initialize environmental resources
    app.init_resource::<DisasterState>();
    app.init_resource::<SpatialGrid>();
    app.insert_resource(SpatialGrid::new(
        16.0,
        Position { x: -80.0, y: -60.0 },
        Position { x: 80.0, y: 60.0 },
    ));

    // Add environmental systems
    app.add_systems(
        Update,
        (
            ant_nest_simulator::systems::environmental_update_system,
            ant_nest_simulator::systems::food_regeneration_system,
        ),
    );

    // Spawn test soil and food entities
    app.world_mut().spawn((
        Soil,
        Position { x: 0.0, y: 0.0 },
        SoilCell {
            moisture: 0.5,
            temperature: 20.0,
            nutrition: 0.7,
        },
    ));

    app.world_mut().spawn((
        Food,
        Position { x: 10.0, y: 10.0 },
        FoodSource {
            nutrition_value: 25.0,
            is_available: true,
            regeneration_timer: 0.0,
            regeneration_time: 30.0,
        },
    ));

    // Run update cycles
    for _ in 0..5 {
        app.update();
    }

    // Verify that soil and food entities still exist after updates
    let world = app.world_mut();
    let soil_count = world.query::<&Soil>().iter(world).count();
    let food_count = world.query::<&Food>().iter(world).count();
    assert_eq!(
        soil_count, 1,
        "Environmental systems should maintain soil entities"
    );
    assert_eq!(
        food_count, 1,
        "Environmental systems should maintain food entities"
    );
}

/// Test invasive species systems compatibility
///
/// This specifically tests the systems that were involved in previous
/// query conflicts (Issues #79, #80).
#[test]
fn test_invasive_species_systems_compatibility() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    // Initialize required resources
    app.init_resource::<DisasterState>();
    app.init_resource::<SpatialGrid>();
    app.insert_resource(SpatialGrid::new(
        16.0,
        Position { x: -80.0, y: -60.0 },
        Position { x: 80.0, y: 60.0 },
    ));

    // Add invasive species systems that previously caused conflicts
    app.add_systems(
        Update,
        (
            ant_nest_simulator::systems::invasive_species_behavior_system,
            ant_nest_simulator::systems::ant_defensive_behavior_system,
            ant_nest_simulator::systems::invasive_species_cleanup_system,
        ),
    );

    // Spawn test entities
    app.world_mut().spawn((
        Ant,
        Position { x: 0.0, y: 0.0 },
        AntBehavior {
            state: AntState::Foraging,
            target_position: None,
            speed: 10.0,
        },
        Lifecycle {
            age: 10.0,
            max_age: 100.0,
            energy: 80.0,
            max_energy: 100.0,
        },
    ));

    app.world_mut().spawn((
        InvasiveSpecies {
            lifetime: 60.0,
            food_consumption_rate: 2.0,
        },
        Position { x: 5.0, y: 5.0 },
    ));

    // Run update cycles
    for _ in 0..5 {
        app.update();
    }

    // Verify that ant and invasive species entities still exist
    let world = app.world_mut();
    let ant_count = world.query::<&Ant>().iter(world).count();
    let invasive_count = world.query::<&InvasiveSpecies>().iter(world).count();
    assert_eq!(
        ant_count, 1,
        "Invasive species systems should maintain ant entities"
    );
    // Note: Invasive species may be cleaned up by systems during updates
    // The important thing is that no ECS conflicts occurred
    assert!(
        invasive_count <= 1,
        "Invasive species systems should handle entities without conflicts (found {})",
        invasive_count
    );
}

/// Test reproduction and lifecycle systems
#[test]
fn test_reproduction_systems_compatibility() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    // Initialize resources
    app.init_resource::<DisasterState>();

    // Add reproduction and lifecycle systems
    app.add_systems(
        Update,
        (
            ant_nest_simulator::systems::queen_reproduction_system,
            ant_nest_simulator::systems::egg_hatching_system,
            ant_nest_simulator::systems::ant_lifecycle_system,
        ),
    );

    // Spawn queen ant for testing
    app.world_mut().spawn((
        Queen,
        Position { x: 0.0, y: -30.0 },
        Lifecycle {
            age: 50.0,
            max_age: 500.0,
            energy: 90.0,
            max_energy: 100.0,
        },
        ReproductionState {
            time_since_last_egg: 0.0,
            egg_laying_interval: 10.0,
            reproductive_capacity: 0.8,
        },
    ));

    // Spawn test egg
    app.world_mut().spawn((
        Egg {
            incubation_time: 15.0,
        },
        Position { x: 1.0, y: -30.0 },
    ));

    // Run update cycles
    for _ in 0..10 {
        app.update();
    }

    // Verify that queen and egg entities still exist
    let world = app.world_mut();
    let queen_count = world.query::<&Queen>().iter(world).count();
    let egg_count = world.query::<&Egg>().iter(world).count();
    assert_eq!(
        queen_count, 1,
        "Reproduction systems should maintain queen entities"
    );
    assert!(
        egg_count >= 1,
        "Reproduction systems should maintain or create egg entities"
    );
}

/// Comprehensive test that gradually adds system groups
///
/// This test starts with minimal systems and gradually adds more,
/// helping to isolate which combination causes conflicts if any occur.
#[test]
fn test_gradual_system_addition() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    // Initialize all required resources first
    app.init_resource::<SpatialGrid>();
    app.init_resource::<DisasterState>();
    app.insert_resource(SpatialGrid::new(
        16.0,
        Position { x: -80.0, y: -60.0 },
        Position { x: 80.0, y: 60.0 },
    ));

    // Test 1: Add basic ant systems
    app.add_systems(
        Update,
        (
            ant_nest_simulator::systems::ant_movement_system,
            ant_nest_simulator::systems::ant_lifecycle_system,
        ),
    );

    // Spawn test ant
    app.world_mut().spawn((
        Ant,
        Position { x: 0.0, y: 0.0 },
        AntBehavior {
            state: AntState::Foraging,
            target_position: None,
            speed: 10.0,
        },
        Lifecycle {
            age: 0.0,
            max_age: 100.0,
            energy: 50.0,
            max_energy: 100.0,
        },
    ));

    // Test basic systems
    for _ in 0..3 {
        app.update();
    }

    // Test 2: Add environmental systems
    app.add_systems(
        Update,
        (
            ant_nest_simulator::systems::environmental_update_system,
            ant_nest_simulator::systems::food_consumption_system,
        ),
    );

    // Test with environmental systems
    for _ in 0..3 {
        app.update();
    }

    // Test 3: Add invasive species systems
    app.add_systems(
        Update,
        (
            ant_nest_simulator::systems::invasive_species_behavior_system,
            ant_nest_simulator::systems::ant_defensive_behavior_system,
        ),
    );

    // Test with invasive species systems
    for _ in 0..3 {
        app.update();
    }

    // Verify that the test ant entity still exists after all system additions
    let world = app.world_mut();
    let ant_count = world.query::<&Ant>().iter(world).count();
    assert_eq!(
        ant_count, 1,
        "All system groups should work together without conflicts"
    );
}
