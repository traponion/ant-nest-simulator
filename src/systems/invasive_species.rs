use crate::components::{
    Ant, AntBehavior, AntState, DisasterState, DisasterType, FoodSource, InvasiveSpecies,
    Lifecycle, Position,
};
use crate::systems::time_control::effective_delta_time;
use bevy::prelude::*;
use rand::Rng;

/// System to spawn invasive species entities during invasive species disasters
pub fn invasive_species_spawning_system(
    mut commands: Commands,
    disaster_state: Res<DisasterState>,
    time: Res<Time>,
    time_control: Res<crate::components::TimeControl>,
    query: Query<&InvasiveSpecies>, // Check existing invasive species
) {
    // Only spawn if invasive species disaster is active and we don't have too many
    if !disaster_state.is_active(DisasterType::InvasiveSpecies) {
        return;
    }

    let current_count = query.iter().count();
    let max_invasive_species = 15; // Limit to prevent overwhelming the simulation

    if current_count >= max_invasive_species {
        return;
    }

    let delta_time = effective_delta_time(&time, &time_control);

    // Spawn rate: attempt to spawn every 2-3 seconds (adjusted for time acceleration)
    let spawn_probability = delta_time * 0.4; // ~40% chance per second

    if rand::thread_rng().gen::<f32>() < spawn_probability {
        spawn_invasive_species_entity(&mut commands);
    }
}

/// Spawn a single invasive species entity at a random location
fn spawn_invasive_species_entity(commands: &mut Commands) {
    let mut rng = rand::thread_rng();

    // Spawn at random location within the simulation area
    let x = rng.gen_range(-400.0..400.0);
    let y = rng.gen_range(-300.0..300.0);

    let lifetime = rng.gen_range(15.0..25.0); // Live for 15-25 seconds
    let food_consumption_rate = rng.gen_range(2.0..4.0); // Consume 2-4 food per second

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(1.0, 0.2, 0.2), // Bright red to distinguish from ants
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(x, y, 0.0),
                scale: Vec3::splat(4.0), // Slightly larger than ants for visibility
                ..default()
            },
            ..default()
        },
        Position { x, y },
        InvasiveSpecies {
            lifetime,
            food_consumption_rate,
        },
    ));

    info!(
        "Spawned invasive species at ({:.1}, {:.1}) with {:.1}s lifetime",
        x, y, lifetime
    );
}

/// System to manage invasive species behavior and lifecycle
pub fn invasive_species_behavior_system(
    mut commands: Commands,
    time: Res<Time>,
    time_control: Res<crate::components::TimeControl>,
    mut invasive_query: Query<(Entity, &mut Position, &mut InvasiveSpecies, &mut Transform)>,
    mut food_query: Query<(&Position, &mut FoodSource), Without<InvasiveSpecies>>,
) {
    let delta_time = effective_delta_time(&time, &time_control);
    let mut entities_to_despawn = Vec::new();

    for (entity, mut position, mut invasive_species, mut transform) in invasive_query.iter_mut() {
        // Update lifetime
        invasive_species.lifetime -= delta_time;

        // Despawn if lifetime expired
        if invasive_species.lifetime <= 0.0 {
            entities_to_despawn.push(entity);
            continue;
        }

        // Random movement behavior - invasive species move in random patterns
        let mut rng = rand::thread_rng();
        let movement_speed = 50.0; // pixels per second

        let direction_x = rng.gen_range(-1.0..1.0);
        let direction_y = rng.gen_range(-1.0..1.0);

        position.x += direction_x * movement_speed * delta_time;
        position.y += direction_y * movement_speed * delta_time;

        // Keep within simulation bounds
        position.x = position.x.clamp(-400.0, 400.0);
        position.y = position.y.clamp(-300.0, 300.0);

        // Update visual transform
        transform.translation.x = position.x;
        transform.translation.y = position.y;

        // Consume nearby food sources
        for (food_position, mut food_source) in food_query.iter_mut() {
            let distance = ((position.x - food_position.x).powi(2)
                + (position.y - food_position.y).powi(2))
            .sqrt();

            // If close enough to food source (within 30 pixels)
            if distance < 30.0 && food_source.is_available {
                let consumption = invasive_species.food_consumption_rate * delta_time;
                food_source.nutrition_value -= consumption;

                // Deplete food source if consumed too much
                if food_source.nutrition_value <= 0.0 {
                    food_source.is_available = false;
                    food_source.regeneration_timer = food_source.regeneration_time * 2.0; // Longer regen time
                    info!(
                        "Invasive species depleted food source at ({:.1}, {:.1})",
                        food_position.x, food_position.y
                    );
                }
            }
        }
    }

    // Despawn expired invasive species
    for entity in entities_to_despawn {
        commands.entity(entity).despawn();
    }
}

/// System to influence ant behavior when invasive species are present
pub fn ant_defensive_behavior_system(
    invasive_query: Query<&Position, (With<InvasiveSpecies>, Without<Ant>)>,
    mut ant_query: Query<(&Position, &mut AntBehavior, &mut Lifecycle), With<Ant>>,
    time: Res<Time>,
    time_control: Res<crate::components::TimeControl>,
) {
    let delta_time = effective_delta_time(&time, &time_control);

    // Only apply defensive behavior if invasive species are present
    if invasive_query.is_empty() {
        return;
    }

    for (ant_position, mut ant_behavior, mut lifecycle) in ant_query.iter_mut() {
        let mut nearest_invasive_distance = f32::INFINITY;

        // Find the nearest invasive species
        for invasive_position in invasive_query.iter() {
            let distance = ((ant_position.x - invasive_position.x).powi(2)
                + (ant_position.y - invasive_position.y).powi(2))
            .sqrt();
            nearest_invasive_distance = nearest_invasive_distance.min(distance);
        }

        // If invasive species is close (within 80 pixels), trigger defensive behavior
        if nearest_invasive_distance < 80.0 {
            // Increase energy consumption due to stress
            lifecycle.energy -= 1.5 * delta_time;

            // Modify behavior to be more clustered/defensive
            match ant_behavior.state {
                AntState::Foraging => {
                    // Chance to switch to resting (defensive clustering)
                    if rand::thread_rng().gen::<f32>() < 0.3 * delta_time {
                        ant_behavior.state = AntState::Resting;
                        ant_behavior.target_position = None;
                    }
                }
                AntState::Returning => {
                    // Speed up return to nest when invasive species nearby
                    ant_behavior.speed *= 1.3;
                }
                _ => {}
            }
        }
    }
}

/// System to clean up invasive species when disaster ends
pub fn invasive_species_cleanup_system(
    mut commands: Commands,
    disaster_state: Res<DisasterState>,
    invasive_query: Query<Entity, With<InvasiveSpecies>>,
) {
    // If invasive species disaster is not active, despawn all invasive species
    if !disaster_state.is_active(DisasterType::InvasiveSpecies) {
        for entity in invasive_query.iter() {
            commands.entity(entity).despawn();
        }

        if !invasive_query.is_empty() {
            info!(
                "Cleaned up {} invasive species entities after disaster ended",
                invasive_query.iter().count()
            );
        }
    }
}
