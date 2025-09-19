use bevy::prelude::*;
use rand::prelude::*;
use crate::components::{Position, SoilCell, Soil, AntBehavior, AntState, Lifecycle, Ant};

/// Basic camera setup for 2D pixel art view
pub fn setup_world(mut commands: Commands) {
    // Spawn 2D camera for side-view ant farm observation with better positioning
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1000.0)),
        ..default()
    });

    info!("Ant Nest Simulator initialized - ready for development!");
}

/// Create soil grid with visual representation
pub fn spawn_soil_grid(mut commands: Commands) {
    let mut rng = thread_rng();

    // Create a grid of soil cells
    for x in -20..20 {
        for y in -15..15 {
            let world_x = x as f32 * 4.0; // 4 pixel spacing
            let world_y = y as f32 * 4.0;

            commands.spawn((
                Position {
                    x: world_x,
                    y: world_y,
                },
                SoilCell {
                    moisture: rng.gen_range(0.0..1.0),
                    temperature: rng.gen_range(15.0..25.0),
                    nutrition: rng.gen_range(0.0..1.0),
                },
                Soil,
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.6, 0.4, 0.2), // Brown color for soil
                        custom_size: Some(Vec2::new(3.0, 3.0)), // Small brown dots
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(world_x, world_y, 0.0)),
                    ..default()
                },
            ));
        }
    }

    info!("Spawned soil grid: {} cells", 40 * 30);
}

/// Spawn initial ant colony with visual representation
pub fn spawn_initial_ants(mut commands: Commands) {
    let mut rng = thread_rng();

    // Spawn ants near the center
    for _i in 0..10 {
        let x_offset = rng.gen_range(-20.0..20.0);
        let y_offset = rng.gen_range(-20.0..20.0);

        commands.spawn((
            Position {
                x: x_offset,
                y: y_offset,
            },
            AntBehavior {
                state: AntState::Foraging,
                target_position: None,
                speed: rng.gen_range(10.0..20.0),
            },
            Lifecycle {
                age: 0.0,
                max_age: rng.gen_range(30.0..60.0), // seconds for now
                energy: 100.0,
                max_energy: 100.0,
            },
            Ant,
            SpriteBundle {
                sprite: Sprite {
                    color: Color::BLACK, // Black color for ants
                    custom_size: Some(Vec2::new(2.0, 2.0)), // 2-pixel black dots
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(x_offset, y_offset, 1.0)), // Z=1 to render above soil
                ..default()
            },
        ));
    }

    info!("Spawned {} initial ants", 10);
}