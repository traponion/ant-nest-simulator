use crate::components::{
    Ant, AntBehavior, AntState, Egg, Lifecycle, Position, Queen, ReproductionState, SoilCell,
    TimeControl,
};
use crate::systems::time_control::effective_delta_time;
use bevy::prelude::*;
use rand::prelude::*;

/// Spawn initial queen ant at the center of the nest
pub fn spawn_queen_ant(mut commands: Commands) {
    commands.spawn((
        Position { x: 0.0, y: 0.0 },
        Queen,
        AntBehavior {
            state: AntState::Resting,
            target_position: None,
            speed: 5.0, // Queens move slower than workers
        },
        Lifecycle {
            age: 0.0,
            max_age: 300.0, // Queens live much longer
            energy: 200.0,  // Queens have more energy
            max_energy: 200.0,
        },
        ReproductionState {
            time_since_last_egg: 0.0,
            egg_laying_interval: 10.0, // Lay egg every 10 seconds
            reproductive_capacity: 1.0,
        },
        Ant,
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.8, 0.0, 0.8),      // Purple color for queen
                custom_size: Some(Vec2::new(3.0, 3.0)), // Slightly larger than workers
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..default()
        },
    ));

    info!("Queen ant spawned at the center of the nest");
}

/// System for queen ant egg laying behavior
#[allow(clippy::type_complexity)]
pub fn queen_reproduction_system(
    time: Res<Time>,
    time_control: Res<TimeControl>,
    mut commands: Commands,
    mut queen_query: Query<
        (&Position, &mut ReproductionState, &Lifecycle),
        (With<Queen>, With<Ant>),
    >,
    ant_count: Query<&Ant>,
    soil_query: Query<&SoilCell>,
) {
    let delta_time = effective_delta_time(&time, &time_control);
    let current_ant_population = ant_count.iter().count();

    // Calculate average soil nutrition for reproductive capacity
    let total_nutrition: f32 = soil_query.iter().map(|soil| soil.nutrition).sum();
    let avg_nutrition = if soil_query.iter().count() > 0 {
        total_nutrition / soil_query.iter().count() as f32
    } else {
        0.5
    };

    for (position, mut reproduction_state, lifecycle) in queen_query.iter_mut() {
        reproduction_state.time_since_last_egg += delta_time;

        // Update reproductive capacity based on nutrition and population
        reproduction_state.reproductive_capacity = (avg_nutrition * 2.0).min(1.0)
            * if current_ant_population < 20 {
                1.0
            } else {
                0.3
            }; // Slow down if overpopulated

        // Check if it's time to lay an egg and conditions are favorable
        if reproduction_state.time_since_last_egg >= reproduction_state.egg_laying_interval
            && lifecycle.energy > 50.0  // Queen needs enough energy
            && reproduction_state.reproductive_capacity > 0.3
            && current_ant_population < 50
        // Population cap
        {
            lay_egg(&mut commands, position);
            reproduction_state.time_since_last_egg = 0.0;
        }
    }
}

/// Helper function to spawn an egg near the queen
fn lay_egg(commands: &mut Commands, queen_position: &Position) {
    let mut rng = thread_rng();

    // Place egg near queen with small random offset
    let egg_x = queen_position.x + rng.gen_range(-5.0..5.0);
    let egg_y = queen_position.y + rng.gen_range(-5.0..5.0);

    commands.spawn((
        Position { x: egg_x, y: egg_y },
        Egg {
            incubation_time: rng.gen_range(8.0..15.0), // 8-15 seconds to hatch
        },
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(1.0, 1.0, 0.8),      // Pale yellow for eggs
                custom_size: Some(Vec2::new(1.5, 1.5)), // Small eggs
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(egg_x, egg_y, 0.5)), // Between soil and ants
            ..default()
        },
    ));

    info!("Queen laid an egg at ({:.1}, {:.1})", egg_x, egg_y);
}

/// System for egg incubation and hatching
pub fn egg_hatching_system(
    time: Res<Time>,
    time_control: Res<TimeControl>,
    mut commands: Commands,
    mut egg_query: Query<(Entity, &Position, &mut Egg)>,
) {
    let delta_time = effective_delta_time(&time, &time_control);

    for (egg_entity, position, mut egg) in egg_query.iter_mut() {
        egg.incubation_time -= delta_time;

        // Check if egg is ready to hatch
        if egg.incubation_time <= 0.0 {
            hatch_egg(&mut commands, egg_entity, position);
        }
    }
}

/// Helper function to hatch an egg into a new worker ant
fn hatch_egg(commands: &mut Commands, egg_entity: Entity, position: &Position) {
    let mut rng = thread_rng();

    // Remove the egg
    commands.entity(egg_entity).despawn();

    // Spawn a new worker ant
    commands.spawn((
        Position {
            x: position.x,
            y: position.y,
        },
        AntBehavior {
            state: AntState::Foraging,
            target_position: None,
            speed: rng.gen_range(10.0..20.0),
        },
        Lifecycle {
            age: 0.0,
            max_age: rng.gen_range(30.0..60.0),
            energy: 100.0,
            max_energy: 100.0,
        },
        Ant,
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK, // Standard black for worker ants
                custom_size: Some(Vec2::new(2.0, 2.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(position.x, position.y, 1.0)),
            ..default()
        },
    ));

    info!(
        "Egg hatched into new worker ant at ({:.1}, {:.1})",
        position.x, position.y
    );
}
