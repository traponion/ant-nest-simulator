//! MVP ECS Query Conflict Detection Tests
//!
//! Simple tests to verify MVP systems work without conflicts.

use ant_nest_simulator::components::*;
use bevy::prelude::*;

/// Test MVP systems compatibility
#[test]
fn test_mvp_systems_compatibility() {
    let mut app = App::new();

    // Add minimal plugins required for ECS functionality
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

    // Spawn test entities
    let mut commands = app.world_mut().commands();
    commands.spawn((Ant, Position { x: 0.0, y: 0.0 }));
    commands.spawn((Soil, Position { x: 4.0, y: 4.0 }));

    // Run several update cycles to check for conflicts
    for _ in 0..5 {
        app.update();
    }

    // Verify entities exist and systems ran without panicking
    let world = app.world_mut();
    let ant_count = world.query::<&Ant>().iter(world).count();
    let soil_count = world.query::<&Soil>().iter(world).count();

    assert!(ant_count > 0, "Should have at least one ant entity");
    assert!(soil_count > 0, "Should have at least one soil entity");
}
