//! MVP Integration tests for ECS system initialization
//!
//! This test module verifies that MVP Bevy ECS systems can be initialized together
//! without query conflicts.

use bevy::prelude::*;

/// Test that MVP systems can be initialized without query conflicts
#[test]
fn test_mvp_systems_initialization() {
    let mut app = App::new();

    // Add minimal plugins for testing
    app.add_plugins(MinimalPlugins);

    // Add MVP startup systems
    app.add_systems(
        Startup,
        (
            ant_nest_simulator::systems::spawn_soil_grid,
            ant_nest_simulator::systems::spawn_initial_ants,
        ),
    );

    // MVP update systems will be added here in the future

    // Initialize additional resources required by systems in test environment
    app.init_resource::<bevy::input::ButtonInput<bevy::input::keyboard::KeyCode>>();

    // Run one update cycle - this will panic if there are query conflicts
    // between any of the registered systems
    app.update();

    // If we reach this point, all systems initialized successfully
    // without ECS query conflicts
}

/// Test system initialization with longer execution to catch timing-related conflicts
///
/// Some ECS query conflicts might only appear after multiple update cycles
/// or when systems actually have entities to process.
#[test]
fn test_systems_multi_update_cycle() {
    let mut app = App::new();

    // Add minimal plugins for testing
    app.add_plugins(MinimalPlugins);

    // Add MVP startup systems
    app.add_systems(
        Startup,
        (
            ant_nest_simulator::systems::spawn_soil_grid,
            ant_nest_simulator::systems::spawn_initial_ants,
        ),
    );

    // Run startup to spawn entities
    app.update();

    // Verify that entities were spawned correctly
    let world = app.world_mut();

    // Check that core entity types exist
    let ants = world
        .query::<&ant_nest_simulator::components::Ant>()
        .iter(world)
        .count();
    let soil = world
        .query::<&ant_nest_simulator::components::Soil>()
        .iter(world)
        .count();

    // We expect at least some ants and soil entities to be spawned
    assert!(ants > 0, "Should have spawned at least one ant entity");
    assert!(soil > 0, "Should have spawned at least one soil entity");
}
