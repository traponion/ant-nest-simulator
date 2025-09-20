//! Integration tests for ECS system initialization and query conflict detection
//!
//! This test module verifies that all Bevy ECS systems can be initialized together
//! without query conflicts, helping to catch compatibility issues during development
//! rather than at runtime.

use ant_nest_simulator::AntNestPlugin;
use bevy::prelude::*;

/// Test that all ECS systems can be initialized without query conflicts
///
/// This test creates a mock application with all systems registered and runs
/// one update cycle. If there are ECS query conflicts between systems,
/// this test will panic and catch the issue during `cargo test`.
///
/// This addresses the problem described in Issue #81 where query conflicts
/// are only detected at runtime, causing panics on different platforms.
#[test]
fn test_all_systems_initialization() {
    let mut app = App::new();

    // Add plugins required for system testing (headless mode)
    app.add_plugins(DefaultPlugins.set(bevy::render::RenderPlugin {
        synchronous_pipeline_compilation: true,
        ..default()
    }).set(bevy::window::WindowPlugin {
        primary_window: None, // Headless mode - no window
        ..default()
    }));

    // Add the main AntNestPlugin which contains all our systems
    app.add_plugins(AntNestPlugin);

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

    // Add plugins required for system testing (headless mode)
    app.add_plugins(DefaultPlugins.set(bevy::render::RenderPlugin {
        synchronous_pipeline_compilation: true,
        ..default()
    }).set(bevy::window::WindowPlugin {
        primary_window: None, // Headless mode - no window
        ..default()
    }));

    // Add the main AntNestPlugin
    app.add_plugins(AntNestPlugin);

    // Run multiple update cycles to catch potential timing-related conflicts
    for _ in 0..10 {
        app.update();
    }

    // Verify that performance metrics resource was created and is functioning
    let performance_metrics = app.world().get_resource::<ant_nest_simulator::components::PerformanceMetrics>();
    assert!(performance_metrics.is_some(), "PerformanceMetrics resource should be initialized");
}

/// Test that core simulation systems can run with entities present
///
/// This test verifies that systems work correctly when there are actual entities
/// to process, which can reveal additional query conflicts.
#[test]
fn test_systems_with_entities() {
    let mut app = App::new();

    // Add minimal required plugins
    app.add_plugins(MinimalPlugins);
    app.add_plugins(AntNestPlugin);

    // Let startup systems run to spawn initial entities
    app.update();

    // Run several more update cycles with entities present
    for _ in 0..5 {
        app.update();
    }

    // Verify that entities were spawned and systems are processing them
    let world = app.world_mut();

    // Check that core entity types exist
    let ants = world.query::<&ant_nest_simulator::components::Ant>().iter(world).count();
    let soil = world.query::<&ant_nest_simulator::components::Soil>().iter(world).count();

    // We expect at least some ants and soil entities to be spawned
    assert!(ants > 0, "Should have spawned at least one ant entity");
    assert!(soil > 0, "Should have spawned at least one soil entity");
}

/// Test specific system combinations that previously caused conflicts
///
/// This test specifically targets system combinations that have caused
/// query conflicts in previous PRs (#79, #80).
#[test]
fn test_known_problematic_system_combinations() {
    let mut app = App::new();

    // Add plugins required for testing (headless mode)
    app.add_plugins(DefaultPlugins.set(bevy::render::RenderPlugin {
        synchronous_pipeline_compilation: true,
        ..default()
    }).set(bevy::window::WindowPlugin {
        primary_window: None, // Headless mode - no window
        ..default()
    }));

    // Manually add only the systems that have previously caused conflicts
    // This allows us to test specific combinations more precisely
    app.init_resource::<ant_nest_simulator::components::TimeControl>()
        .init_resource::<ant_nest_simulator::components::SpatialGrid>()
        .add_systems(
            Update,
            (
                // These systems have previously caused query conflicts
                ant_nest_simulator::systems::ant_movement_system,
                ant_nest_simulator::systems::invasive_species_behavior_system,
                ant_nest_simulator::systems::ant_defensive_behavior_system,
            ),
        );

    // Run multiple cycles to ensure no conflicts
    for _ in 0..5 {
        app.update();
    }
}

/// Performance test to ensure system initialization is reasonably fast
///
/// While not directly related to query conflicts, this helps ensure
/// that our test suite runs efficiently.
#[test]
fn test_system_initialization_performance() {
    use std::time::Instant;

    let start = Instant::now();

    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(AntNestPlugin);

    // Run initial update
    app.update();

    let duration = start.elapsed();

    // System initialization should complete within reasonable time
    // (This is a conservative limit - adjust if needed based on CI performance)
    assert!(duration.as_millis() < 5000,
           "System initialization took too long: {:?}", duration);
}