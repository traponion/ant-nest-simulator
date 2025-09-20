//! MVP Test: Simple ant spawning verification

use ant_nest_simulator::components::*;
use ant_nest_simulator::systems;
use bevy::prelude::*;

#[test]
fn mvp_simple_ant_spawning() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    // Add MVP startup systems
    app.add_systems(
        Startup,
        (systems::spawn_soil_grid, systems::spawn_initial_ants),
    );

    // Run startup to spawn entities
    app.update();

    let world = app.world_mut();

    // Count MVP entity types
    let ant_count = world.query::<&Ant>().iter(world).count();
    let soil_count = world.query::<&Soil>().iter(world).count();

    println!("=== MVP Entity Count Report ===");
    println!("Ants: {}", ant_count);
    println!("Soil cells: {}", soil_count);

    // Get ant position information
    let mut ant_query = world.query::<(&Ant, &Position)>();

    println!("\n=== Ant Details ===");
    for (i, (_, position)) in ant_query.iter(world).enumerate() {
        println!("Ant {}: at ({:.1}, {:.1})", i + 1, position.x, position.y,);
    }

    // Check rendering components
    let mut transform_query = world.query::<(&Ant, &Transform)>();
    let ants_with_transforms = transform_query.iter(world).count();

    let mut visibility_query = world.query::<(&Ant, &Visibility)>();
    let ants_with_visibility = visibility_query.iter(world).count();

    println!("\n=== Rendering Components ===");
    println!("Ants with Transform: {}", ants_with_transforms);
    println!("Ants with Visibility: {}", ants_with_visibility);

    // MVP assertions
    assert!(
        ant_count > 0,
        "MVP: No ants found! Expected some ants to be spawned"
    );
    assert!(
        soil_count > 0,
        "MVP: No soil found! Expected soil grid to be spawned"
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

    if ant_count > 0 && soil_count > 0 && ants_with_transforms == ant_count {
        println!("\n✅ SUCCESS: MVP spawning system working correctly!");
    }
}
