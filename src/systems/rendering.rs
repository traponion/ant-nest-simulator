use crate::components::{
    Ant, AntBehavior, AntState, Chamber, ChamberType, DepthLayer, Food, FoodSource, GroundSurface,
    Inventory, Lifecycle, PhaseSpecificBehavior, Position, Queen, Soil, SoilCell, Tunnel,
};
use bevy::prelude::*;
use rand::prelude::*;

/// Basic camera setup for 2D pixel art view with expanded view area
pub fn setup_world(mut commands: Commands) {
    // Spawn 2D camera for side-view ant farm observation with full screen view
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, -15.0, 1000.0)), // Slightly higher to see more surface activity
        projection: OrthographicProjection {
            scale: 0.4, // Further zoom out for full screen experience (smaller scale = wider view)
            ..default()
        },
        ..default()
    });

    info!("Ant Nest Simulator initialized with full screen view for pure observation gameplay!");
}

/// Create soil grid with depth layers for cross-section view
pub fn spawn_soil_grid(mut commands: Commands) {
    let mut rng = thread_rng();

    // Create a grid of soil cells below ground (y <= 0)
    for x in -25..25 {
        for y in -20..=0 {
            let world_x = x as f32 * 4.0; // 4 pixel spacing
            let world_y = y as f32 * 4.0;

            // Skip if this is exactly on the surface (y = 0)
            if y == 0 {
                continue;
            }

            // Calculate depth layer (0 = just below surface, higher = deeper)
            let depth = (-y) as u32;
            let depth_layer = DepthLayer::new(depth);

            // Adjust soil properties based on depth
            let base_moisture = rng.gen_range(0.3..0.8);
            let depth_moisture_bonus = (depth as f32 * 0.05).min(0.3); // Deeper = more moist
            let moisture = (base_moisture + depth_moisture_bonus).min(1.0);

            let base_temp = rng.gen_range(18.0..22.0);
            let depth_temp_stability = depth as f32 * 0.2; // Deeper = more stable temperature
            let temperature = base_temp + depth_temp_stability;

            commands.spawn((
                Position {
                    x: world_x,
                    y: world_y,
                },
                SoilCell {
                    moisture,
                    temperature,
                    nutrition: rng.gen_range(0.2..0.8),
                },
                depth_layer.clone(),
                Soil,
                SpriteBundle {
                    sprite: Sprite {
                        color: depth_layer.get_soil_color(),    // Color varies by depth
                        custom_size: Some(Vec2::new(3.0, 3.0)), // Small brown dots
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(world_x, world_y, 0.0)),
                    ..default()
                },
            ));
        }
    }

    info!(
        "Spawned underground soil grid: {} cells with depth layers",
        50 * 20
    );
}

/// Spawn initial ant colony positioned in tunnel system and chambers
pub fn spawn_initial_ants(mut commands: Commands) {
    let mut rng = thread_rng();

    // Predefined tunnel and chamber positions for realistic ant placement
    // Include some surface positions for better food access
    let colony_positions = [
        // Surface foraging positions (for better food access)
        Position { x: 0.0, y: 0.0 },
        Position { x: 8.0, y: 0.0 },
        Position { x: -8.0, y: 0.0 },
        // Main tunnel positions
        Position { x: 0.0, y: -8.0 },
        Position { x: 0.0, y: -16.0 },
        Position { x: 0.0, y: -32.0 },
        // Food storage branch
        Position { x: -12.0, y: -16.0 },
        Position { x: -24.0, y: -16.0 },
        // Worker area
        Position { x: 8.0, y: -16.0 },
    ];

    // Spawn the Queen in the queen chamber
    commands.spawn((
        Position { x: 0.0, y: -48.0 },
        AntBehavior {
            state: AntState::Resting, // Queen typically rests and lays eggs
            target_position: None,
            speed: 5.0, // Queen moves slowly
        },
        Lifecycle {
            age: 0.0,
            max_age: 300.0, // Queen lives much longer
            energy: 100.0,
            max_energy: 100.0,
        },
        Inventory {
            carried_food_value: 0.0,
            home_position: Position { x: 0.0, y: -48.0 }, // Queen's chamber
        },
        PhaseSpecificBehavior::default(), // Add phase-specific behavior
        Queen,                            // Mark as queen
        Ant,
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.6, 0.0, 0.6),      // Purple color for queen
                custom_size: Some(Vec2::new(3.0, 3.0)), // Slightly larger than workers
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, -48.0, 3.0)), // High Z to render above chambers
            ..default()
        },
    ));

    // Spawn worker ants throughout the tunnel system
    for (i, base_position) in colony_positions.iter().enumerate().take(8) {
        // Small random offset to avoid overlapping
        let x_offset = base_position.x + rng.gen_range(-4.0..4.0);
        let y_offset = base_position.y + rng.gen_range(-2.0..2.0);

        // Assign different behaviors based on location
        let ant_state = match i {
            0..=2 => AntState::Foraging,     // Ants near surface forage
            3..=4 => AntState::CarryingFood, // Ants in storage areas carry food
            5..=6 => AntState::Resting,      // Ants in nursery area rest
            _ => AntState::Digging,          // Other ants dig/expand tunnels
        };

        commands.spawn((
            Position {
                x: x_offset,
                y: y_offset,
            },
            AntBehavior {
                state: ant_state.clone(),
                target_position: None,
                speed: rng.gen_range(12.0..18.0),
            },
            Lifecycle {
                age: rng.gen_range(0.0..20.0), // Varied ages
                max_age: rng.gen_range(40.0..80.0),
                energy: rng.gen_range(70.0..100.0),
                max_energy: 100.0,
            },
            Inventory {
                carried_food_value: if ant_state == AntState::CarryingFood {
                    rng.gen_range(10.0..30.0)
                } else {
                    0.0
                },
                home_position: Position { x: 0.0, y: -48.0 }, // Queen's chamber as home
            },
            PhaseSpecificBehavior::default(), // Add phase-specific behavior
            Ant,
            SpriteBundle {
                sprite: Sprite {
                    color: Color::BLACK,                    // Black color for worker ants
                    custom_size: Some(Vec2::new(2.0, 2.0)), // 2-pixel black dots
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(x_offset, y_offset, 3.0)), // Above chambers and tunnels
                ..default()
            },
        ));
    }

    info!("Spawned ant colony: 1 Queen + 8 Worker ants in tunnel system");
}

/// Spawn food sources on the surface for foraging ants to collect
pub fn spawn_food_sources(mut commands: Commands) {
    let mut rng = thread_rng();
    let food_count = 20; // Number of food sources to spawn

    for _i in 0..food_count {
        // Scatter food sources on or near the surface (y >= -4.0)
        let x_offset: f32 = rng.gen_range(-80.0..80.0);
        let y_offset: f32 = rng.gen_range(-4.0..8.0); // Surface to slightly above ground

        // Avoid spawning food directly on tunnel entrance
        if x_offset.abs() < 8.0 && y_offset.abs() < 4.0 {
            continue;
        }

        // Cluster some food sources for more realistic foraging
        let cluster_chance = rng.gen_range(0.0..1.0);
        let (final_x, final_y) = if cluster_chance < 0.3 {
            // 30% chance to spawn near existing food clusters
            let cluster_centers = [
                (-40.0, 4.0),  // Left cluster
                (45.0, 2.0),   // Right cluster
                (-20.0, -2.0), // Left-center cluster
                (25.0, 6.0),   // Right-center cluster
            ];
            let cluster = cluster_centers[rng.gen_range(0..cluster_centers.len())];
            (
                cluster.0 + rng.gen_range(-12.0..12.0),
                cluster.1 + rng.gen_range(-2.0..2.0),
            )
        } else {
            (x_offset, y_offset)
        };

        commands.spawn((
            Position {
                x: final_x,
                y: final_y,
            },
            FoodSource {
                nutrition_value: rng.gen_range(15.0..35.0), // Energy recovery
                is_available: true,
                regeneration_timer: 0.0,
                regeneration_time: rng.gen_range(20.0..40.0), // Longer regeneration for realism
            },
            Food,
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.2, 0.9, 0.2), // Bright green color for food
                    custom_size: Some(Vec2::new(2.0, 2.0)), // Slightly larger for visibility
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(final_x, final_y, 1.5)), // Above ground surface
                ..default()
            },
        ));
    }

    info!(
        "Spawned {} food sources on surface with clustering",
        food_count
    );
}

/// Spawn ground surface line to separate above/below ground areas
pub fn spawn_ground_surface(mut commands: Commands) {
    let surface_y = 0.0; // Ground level at y = 0
    let surface_length = 200.0; // Length of the visible surface
    let surface_segments = 50; // Number of segments for the surface line

    for i in 0..surface_segments {
        let x_pos = -surface_length / 2.0 + (i as f32 * (surface_length / surface_segments as f32));

        commands.spawn((
            Position {
                x: x_pos,
                y: surface_y,
            },
            GroundSurface,
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.4, 0.6, 0.2),      // Green grass color
                    custom_size: Some(Vec2::new(4.0, 2.0)), // Small grass segments
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(x_pos, surface_y, 0.5)),
                ..default()
            },
        ));
    }

    info!("Spawned ground surface with {} segments", surface_segments);
}

/// Spawn initial tunnel system for the ant colony cross-section
pub fn spawn_initial_tunnel_system(mut commands: Commands) {
    // Main tunnel from surface to queen chamber
    let main_tunnel_positions = [
        Position { x: 0.0, y: 0.0 },   // Surface entrance
        Position { x: 0.0, y: -8.0 },  // First depth
        Position { x: 0.0, y: -16.0 }, // Second depth
        Position { x: 0.0, y: -32.0 }, // Third depth
        Position { x: 0.0, y: -48.0 }, // Queen chamber depth
    ];

    // Horizontal tunnel to the left (food storage branch)
    let left_tunnel_positions = [
        Position { x: 0.0, y: -16.0 }, // Junction point
        Position { x: -12.0, y: -16.0 },
        Position { x: -24.0, y: -16.0 }, // Food storage area
    ];

    // Horizontal tunnel to the right (nursery branch)
    let right_tunnel_positions = [
        Position { x: 0.0, y: -32.0 }, // Junction point
        Position { x: 16.0, y: -32.0 },
        Position { x: 32.0, y: -32.0 }, // Nursery area
    ];

    // Spawn tunnel segments for main vertical tunnel
    for i in 0..main_tunnel_positions.len() - 1 {
        let start = &main_tunnel_positions[i];
        let end = &main_tunnel_positions[i + 1];
        spawn_tunnel_segment(&mut commands, start, end, 6.0);
    }

    // Spawn tunnel segments for left branch
    for i in 0..left_tunnel_positions.len() - 1 {
        let start = &left_tunnel_positions[i];
        let end = &left_tunnel_positions[i + 1];
        spawn_tunnel_segment(&mut commands, start, end, 4.0);
    }

    // Spawn tunnel segments for right branch
    for i in 0..right_tunnel_positions.len() - 1 {
        let start = &right_tunnel_positions[i];
        let end = &right_tunnel_positions[i + 1];
        spawn_tunnel_segment(&mut commands, start, end, 4.0);
    }

    info!("Spawned initial tunnel system with main shaft and branches");
}

/// Helper function to spawn tunnel segments between two points
fn spawn_tunnel_segment(commands: &mut Commands, start: &Position, end: &Position, width: f32) {
    let dx = end.x - start.x;
    let dy = end.y - start.y;
    let distance = (dx * dx + dy * dy).sqrt();
    let segments = (distance / 4.0).max(1.0) as usize; // 4-pixel segments

    for i in 0..=segments {
        let t = i as f32 / segments as f32;
        let x = start.x + dx * t;
        let y = start.y + dy * t;

        commands.spawn((
            Position { x, y },
            Tunnel {
                width,
                connections: vec![start.clone(), end.clone()],
                under_construction: false,
            },
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.3, 0.2, 0.1), // Dark brown tunnel color
                    custom_size: Some(Vec2::new(width, 4.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(x, y, 1.0)), // Above soil layer
                ..default()
            },
        ));
    }
}

/// Spawn initial chambers for specialized colony functions
pub fn spawn_initial_chambers(mut commands: Commands) {
    // Queen chamber - central and deep
    commands.spawn((
        Position { x: 0.0, y: -48.0 },
        Chamber {
            chamber_type: ChamberType::Queen,
            radius: ChamberType::Queen.default_radius(),
            tunnel_connections: vec![Position { x: 0.0, y: -32.0 }],
            capacity_usage: 0.0,
            max_capacity: ChamberType::Queen.default_capacity(),
        },
        SpriteBundle {
            sprite: Sprite {
                color: ChamberType::Queen.get_color(),
                custom_size: Some(Vec2::new(40.0, 40.0)), // Large queen chamber
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, -48.0, 2.0)), // Above tunnels
            ..default()
        },
    ));

    // Nursery chamber - right branch
    commands.spawn((
        Position { x: 32.0, y: -32.0 },
        Chamber {
            chamber_type: ChamberType::Nursery,
            radius: ChamberType::Nursery.default_radius(),
            tunnel_connections: vec![Position { x: 16.0, y: -32.0 }],
            capacity_usage: 0.0,
            max_capacity: ChamberType::Nursery.default_capacity(),
        },
        SpriteBundle {
            sprite: Sprite {
                color: ChamberType::Nursery.get_color(),
                custom_size: Some(Vec2::new(30.0, 30.0)), // Medium nursery chamber
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(32.0, -32.0, 2.0)),
            ..default()
        },
    ));

    // Food storage chamber - left branch
    commands.spawn((
        Position { x: -24.0, y: -16.0 },
        Chamber {
            chamber_type: ChamberType::FoodStorage,
            radius: ChamberType::FoodStorage.default_radius(),
            tunnel_connections: vec![Position { x: -12.0, y: -16.0 }],
            capacity_usage: 0.0,
            max_capacity: ChamberType::FoodStorage.default_capacity(),
        },
        SpriteBundle {
            sprite: Sprite {
                color: ChamberType::FoodStorage.get_color(),
                custom_size: Some(Vec2::new(24.0, 24.0)), // Medium storage chamber
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-24.0, -16.0, 2.0)),
            ..default()
        },
    ));

    // Worker areas near junctions
    commands.spawn((
        Position { x: 8.0, y: -16.0 },
        Chamber {
            chamber_type: ChamberType::Worker,
            radius: ChamberType::Worker.default_radius(),
            tunnel_connections: vec![Position { x: 0.0, y: -16.0 }],
            capacity_usage: 0.0,
            max_capacity: ChamberType::Worker.default_capacity(),
        },
        SpriteBundle {
            sprite: Sprite {
                color: ChamberType::Worker.get_color(),
                custom_size: Some(Vec2::new(16.0, 16.0)), // Small worker chamber
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(8.0, -16.0, 2.0)),
            ..default()
        },
    ));

    info!("Spawned initial chambers: Queen, Nursery, Food Storage, and Worker areas");
}
