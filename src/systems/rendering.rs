use crate::components::{Ant, FoundingState, Position, Queen, Soil, SoilCell};
use bevy::prelude::*;
use rand::prelude::*;

/// Basic camera setup for 2D pixel art view optimized for ant visibility
pub fn setup_world(mut commands: Commands) {
    // Spawn 2D camera for side-view ant farm observation with optimal zoom for ant visibility
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, -20.0, 1000.0)), // Better positioning for colony overview
        projection: OrthographicProjection {
            scale: 0.6, // Adjusted zoom for better ant visibility (Issue #102)
            ..default()
        },
        ..default()
    });

    info!("Ant Nest Simulator initialized with optimized view for clear ant visibility!");
}

/// Create soil grid with depth layers for cross-section view
pub fn spawn_soil_grid(mut commands: Commands) {
    let mut rng = thread_rng();

    // Create a grid of soil cells below ground (y <= 0)
    for x in -25i32..25i32 {
        for y in -20i32..=0i32 {
            let world_x = x as f32 * 4.0; // 4 pixel spacing
            let world_y = y as f32 * 4.0;

            // Skip if this is exactly on the surface (y = 0)
            if y == 0 {
                continue;
            }

            commands.spawn((
                Position {
                    x: world_x,
                    y: world_y,
                },
                SoilCell {
                    moisture: rng.gen_range(0.3..0.8),
                    temperature: rng.gen_range(18.0..22.0),
                    nutrition: rng.gen_range(0.2..0.8),
                },
                Soil,
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.6, 0.4, 0.2),      // Brown soil color
                        custom_size: Some(Vec2::new(4.0, 4.0)), // 4x4 pixel soil cells
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(world_x, world_y, 0.0)), // Background layer
                    ..default()
                },
            ));
        }
    }

    info!("Spawned underground soil grid: {} cells", 50 * 20);
}

/// Spawn simple ants for MVP
pub fn spawn_initial_ants(mut commands: Commands) {
    // MVP: Just spawn a few simple ants with basic components
    let ant_positions = [
        Position { x: 0.0, y: 0.0 },   // Surface ant
        Position { x: 4.0, y: 0.0 },   // Surface ant
        Position { x: -4.0, y: 0.0 },  // Surface ant
        Position { x: 0.0, y: -8.0 },  // Underground ant
        Position { x: 0.0, y: -16.0 }, // Underground ant
    ];

    for position in ant_positions.iter() {
        commands.spawn((
            position.clone(),
            Ant, // Simple marker component
            SpriteBundle {
                sprite: Sprite {
                    color: Color::BLACK,                    // Black ants as specified in MVP
                    custom_size: Some(Vec2::new(2.0, 2.0)), // 2-pixel dots as specified
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(position.x, position.y, 10.0)),
                ..default()
            },
        ));
    }

    info!("Spawned {} simple ants for MVP", ant_positions.len());
}

/// Spawn a single queen ant on the surface for nest founding
pub fn spawn_queen(mut commands: Commands) {
    let mut rng = thread_rng();

    // Random surface position for queen spawning
    let spawn_x = rng.gen_range(-80.0..80.0); // Within the soil grid range
    let spawn_y = 0.0; // Surface level

    commands.spawn((
        Position {
            x: spawn_x,
            y: spawn_y,
        },
        Queen {
            founding_state: FoundingState::Seeking,
        },
        Ant, // Queens are also ants, so they can use existing movement systems
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.2, 0.1, 0.0), // Dark brown for queen (distinct from black workers)
                custom_size: Some(Vec2::new(3.0, 3.0)), // 3x3 pixels (larger than 2x2 workers)
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(spawn_x, spawn_y, 10.0)),
            ..default()
        },
    ));

    info!(
        "Spawned queen ant at position ({}, {}) for nest founding",
        spawn_x, spawn_y
    );
}
