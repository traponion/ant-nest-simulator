use bevy::prelude::*;
use ant_nest_simulator::components::*;
use ant_nest_simulator::systems;

fn main() {
    // Create a minimal app to test ant spawning
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

    let world = app.world();

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
        println!("Ant {}: {} at ({:.1}, {:.1}) - State: {:?}",
                i + 1, ant_type, position.x, position.y, behavior.state);
    }

    // Check if any ants have rendering components
    let mut sprite_query = world.query::<(&Ant, &SpriteBundle)>();
    let ants_with_sprites = sprite_query.iter(world).count();
    println!("\nAnts with SpriteBundle: {}", ants_with_sprites);

    if ant_count == 0 {
        println!("\n❌ NO ANTS FOUND! This confirms Issue #100.");
    } else {
        println!("\n✅ Ants are being spawned correctly.");
        if ants_with_sprites < ant_count {
            println!("⚠️  Some ants are missing SpriteBundle components!");
        }
    }
}