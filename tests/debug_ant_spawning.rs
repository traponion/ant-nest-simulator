//! Debug test to investigate Issue #100: No ants visible in simulation

use ant_nest_simulator::components::*;
use ant_nest_simulator::systems;
use bevy::prelude::*;

#[test]
fn debug_ant_spawning_issue_100() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    // Initialize required resources
    app.init_resource::<SimulationTime>();
    app.init_resource::<DisasterState>();
    app.insert_resource(SpatialGrid::new(
        16.0,
        Position { x: -80.0, y: -60.0 },
        Position { x: 80.0, y: 60.0 },
    ));

    // Add startup systems to spawn entities
    app.add_systems(
        Startup,
        (
            systems::spawn_soil_grid,
            systems::spawn_initial_ants,
            systems::spawn_food_sources,
            systems::spawn_ground_surface,
            systems::spawn_initial_tunnel_system,
            systems::spawn_initial_chambers,
        ),
    );

    // Run startup to spawn entities
    app.update();

    let world = app.world_mut();

    // Count different entity types
    let ant_count = world.query::<&Ant>().iter(world).count();
    let queen_count = world.query::<&Queen>().iter(world).count();
    let soil_count = world.query::<&Soil>().iter(world).count();
    let food_count = world.query::<&Food>().iter(world).count();
    let chamber_count = world.query::<&Chamber>().iter(world).count();
    let tunnel_count = world.query::<&Tunnel>().iter(world).count();

    println!("=== Entity Count Report ===");
    println!("Ants (total): {}", ant_count);
    println!("Queens: {}", queen_count);
    println!("Soil cells: {}", soil_count);
    println!("Food sources: {}", food_count);
    println!("Chambers: {}", chamber_count);
    println!("Tunnel segments: {}", tunnel_count);

    // Get detailed ant information
    let mut ant_query = world.query::<(&Ant, &Position, &AntBehavior, Option<&Queen>)>();

    println!("\n=== Ant Details ===");
    for (i, (_, position, behavior, queen)) in ant_query.iter(world).enumerate() {
        let ant_type = if queen.is_some() { "Queen" } else { "Worker" };
        println!(
            "Ant {}: {} at ({:.1}, {:.1}) - State: {:?}",
            i + 1,
            ant_type,
            position.x,
            position.y,
            behavior.state
        );
    }

    // Check if any ants have sprite components (Transform, Sprite, Visibility, etc.)
    let mut transform_query = world.query::<(&Ant, &Transform)>();
    let ants_with_transforms = transform_query.iter(world).count();

    let mut visibility_query = world.query::<(&Ant, &Visibility)>();
    let ants_with_visibility = visibility_query.iter(world).count();

    println!("\n=== Rendering Components ===");
    println!("Ants with Transform: {}", ants_with_transforms);
    println!("Ants with Visibility: {}", ants_with_visibility);

    // Critical assertions for Issue #100
    assert!(
        ant_count > 0,
        "Issue #100: No ants found! Expected at least 9 ants (1 queen + 8 workers)"
    );
    assert_eq!(queen_count, 1, "Expected exactly 1 queen");
    assert!(
        ant_count >= 9,
        "Expected at least 9 ants total (1 queen + 8 workers), found {}",
        ant_count
    );

    // Check rendering components
    assert_eq!(
        ants_with_transforms, ant_count,
        "All ants should have Transform components for rendering"
    );
    assert_eq!(
        ants_with_visibility, ant_count,
        "All ants should have Visibility components for rendering"
    );

    if ant_count >= 9 && ants_with_transforms == ant_count {
        println!("\n✅ SUCCESS: Ants are being spawned correctly with rendering components!");
        println!("✅ Issue #100 root cause is NOT in the spawning system.");
    } else {
        println!("\n❌ FAILURE: Issue #100 confirmed - ants or rendering components missing!");
    }
}
