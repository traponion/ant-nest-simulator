use crate::components::{
    Ant, AntBehavior, DisasterState, DisasterType, InvasiveSpecies, Lifecycle, Position, SoilCell, TimeControl,
};
use crate::systems::time_control::effective_delta_time;
use bevy::prelude::*;
use rand::prelude::*;

/// System for handling disaster keyboard input
pub fn disaster_input_system(
    input: Res<ButtonInput<KeyCode>>,
    mut disaster_state: ResMut<DisasterState>,
) {
    // Rain disaster - R key
    if input.just_pressed(KeyCode::KeyR) {
        if !disaster_state.is_active(DisasterType::Rain)
            && !disaster_state.is_on_cooldown(DisasterType::Rain)
        {
            disaster_state.start_disaster(DisasterType::Rain, 20.0); // 20 seconds duration
            disaster_state
                .cooldown_timers
                .insert(DisasterType::Rain, 5.0); // 5 second cooldown
            info!("Rain disaster started! Duration: 20 seconds");
        } else if disaster_state.is_on_cooldown(DisasterType::Rain) {
            info!("Rain is on cooldown");
        } else {
            info!("Rain is already active");
        }
    }

    // Drought disaster - D key
    if input.just_pressed(KeyCode::KeyD) {
        if !disaster_state.is_active(DisasterType::Drought)
            && !disaster_state.is_on_cooldown(DisasterType::Drought)
        {
            disaster_state.start_disaster(DisasterType::Drought, 45.0); // 45 seconds duration
            disaster_state
                .cooldown_timers
                .insert(DisasterType::Drought, 8.0); // 8 second cooldown
            info!("Drought disaster started! Duration: 45 seconds");
        } else if disaster_state.is_on_cooldown(DisasterType::Drought) {
            info!("Drought is on cooldown");
        } else {
            info!("Drought is already active");
        }
    }

    // Cold Snap disaster - C key
    if input.just_pressed(KeyCode::KeyC) {
        if !disaster_state.is_active(DisasterType::ColdSnap)
            && !disaster_state.is_on_cooldown(DisasterType::ColdSnap)
        {
            disaster_state.start_disaster(DisasterType::ColdSnap, 30.0); // 30 seconds duration
            disaster_state
                .cooldown_timers
                .insert(DisasterType::ColdSnap, 6.0); // 6 second cooldown
            info!("Cold Snap disaster started! Duration: 30 seconds");
        } else if disaster_state.is_on_cooldown(DisasterType::ColdSnap) {
            info!("Cold Snap is on cooldown");
        } else {
            info!("Cold Snap is already active");
        }
    }

    // Invasive Species disaster - I key
    if input.just_pressed(KeyCode::KeyI) {
        if !disaster_state.is_active(DisasterType::InvasiveSpecies)
            && !disaster_state.is_on_cooldown(DisasterType::InvasiveSpecies)
        {
            disaster_state.start_disaster(DisasterType::InvasiveSpecies, 60.0); // 60 seconds duration
            disaster_state
                .cooldown_timers
                .insert(DisasterType::InvasiveSpecies, 10.0); // 10 second cooldown
            info!("Invasive Species disaster started! Duration: 60 seconds");
        } else if disaster_state.is_on_cooldown(DisasterType::InvasiveSpecies) {
            info!("Invasive Species is on cooldown");
        } else {
            info!("Invasive Species is already active");
        }
    }
}

/// System for managing disaster timers and cleanup
pub fn disaster_timer_system(
    time: Res<Time>,
    time_control: Res<TimeControl>,
    mut disaster_state: ResMut<DisasterState>,
) {
    let delta_time = effective_delta_time(&time, &time_control);

    // Update active disaster timers
    let mut disasters_to_remove = Vec::new();
    for (disaster_type, remaining_time) in disaster_state.active_disasters.iter_mut() {
        *remaining_time -= delta_time;
        if *remaining_time <= 0.0 {
            disasters_to_remove.push(*disaster_type);
        }
    }

    // Remove expired disasters
    for disaster_type in disasters_to_remove {
        disaster_state.active_disasters.remove(&disaster_type);
        info!("{:?} disaster has ended", disaster_type);
    }

    // Update cooldown timers
    for (_, cooldown_time) in disaster_state.cooldown_timers.iter_mut() {
        *cooldown_time -= delta_time;
        if *cooldown_time < 0.0 {
            *cooldown_time = 0.0;
        }
    }
}

/// System for applying disaster effects to the environment and ants
pub fn disaster_effect_system(
    disaster_state: Res<DisasterState>,
    time: Res<Time>,
    time_control: Res<TimeControl>,
    mut soil_query: Query<&mut SoilCell>,
    mut ant_query: Query<(&Position, &mut AntBehavior, &mut Lifecycle), With<Ant>>,
) {
    let delta_time = effective_delta_time(&time, &time_control);

    // Apply Rain effects
    if disaster_state.is_active(DisasterType::Rain) {
        // Increase soil moisture
        for mut soil in soil_query.iter_mut() {
            soil.moisture += 0.8 * delta_time; // Rapid moisture increase
            soil.moisture = soil.moisture.min(1.0);
        }

        // Rain effects on ants are applied through movement system speed modifier
        // No direct ant behavior changes needed here
    }

    // Apply Drought effects
    if disaster_state.is_active(DisasterType::Drought) {
        // Decrease soil moisture and nutrition
        for mut soil in soil_query.iter_mut() {
            soil.moisture -= 0.6 * delta_time; // Rapid moisture loss
            soil.moisture = soil.moisture.max(0.0);

            soil.nutrition -= 0.1 * delta_time; // Nutrition loss
            soil.nutrition = soil.nutrition.max(0.0);
        }

        // Increase ant energy consumption (heat stress)
        for (_, _, mut lifecycle) in ant_query.iter_mut() {
            lifecycle.energy -= 2.0 * delta_time; // Additional energy loss
        }
    }

    // Apply Cold Snap effects
    if disaster_state.is_active(DisasterType::ColdSnap) {
        // Decrease soil temperature
        for mut soil in soil_query.iter_mut() {
            soil.temperature -= 15.0 * delta_time; // Rapid temperature drop
            soil.temperature = soil.temperature.max(5.0); // Don't go below 5°C
        }

        // Increase ant energy consumption (cold stress)
        for (_, _, mut lifecycle) in ant_query.iter_mut() {
            lifecycle.energy -= 3.0 * delta_time; // Higher energy loss than drought
        }
    }

    // Invasive Species effects are handled by dedicated spawning and behavior systems
    // The spawning is triggered by invasive_species_spawn_system
    // Movement and consumption is handled by invasive_species_behavior_system
}

/// Helper function to check if any disaster is affecting ant movement
pub fn get_movement_speed_modifier(disaster_state: &DisasterState) -> f32 {
    let mut modifier = 1.0;

    if disaster_state.is_active(DisasterType::Rain) {
        modifier *= 0.8; // 20% speed reduction
    }

    if disaster_state.is_active(DisasterType::ColdSnap) {
        modifier *= 0.5; // 50% speed reduction
    }

    modifier
}

/// System for spawning invasive species entities when disaster starts
pub fn invasive_species_spawn_system(
    mut commands: Commands,
    disaster_state: Res<DisasterState>,
    existing_invasive: Query<Entity, With<InvasiveSpecies>>,
) {
    // Check if invasive species disaster just started and no invasive species exist
    if disaster_state.is_active(DisasterType::InvasiveSpecies) && existing_invasive.is_empty() {
        let mut rng = thread_rng();

        // Spawn 8-12 invasive species entities scattered across the environment
        let spawn_count = rng.gen_range(8..=12);

        for _i in 0..spawn_count {
            // Random spawn locations across simulation area
            let x_offset = rng.gen_range(-70.0..70.0);
            let y_offset = rng.gen_range(-50.0..50.0);

            commands.spawn((
                Position {
                    x: x_offset,
                    y: y_offset,
                },
                InvasiveSpecies {
                    lifetime: 60.0, // Match disaster duration
                    food_consumption_rate: rng.gen_range(0.8..1.2),
                },
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(1.0, 0.0, 0.0), // Bright red color for invasive species
                        custom_size: Some(Vec2::new(2.0, 2.0)), // Same size as ants but red
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(x_offset, y_offset, 1.5)), // Z=1.5 to render above ants
                    ..default()
                },
            ));
        }

        info!("Spawned {} invasive species entities", spawn_count);
    }
}

/// System for invasive species movement and behavior
pub fn invasive_species_behavior_system(
    time: Res<Time>,
    time_control: Res<TimeControl>,
    mut invasive_query: Query<
        (&mut Position, &mut Transform, &mut InvasiveSpecies),
        Without<Ant>,
    >,
) {
    let mut rng = thread_rng();
    let delta_time = effective_delta_time(&time, &time_control);

    for (mut position, mut transform, mut invasive) in invasive_query.iter_mut() {
        // Update lifetime
        invasive.lifetime -= delta_time;

        // Random movement behavior - more erratic than ants
        let move_speed = 15.0; // Slightly faster than ants
        let direction_x: f32 = rng.gen_range(-1.0..1.0);
        let direction_y: f32 = rng.gen_range(-1.0..1.0);

        // Normalize direction
        let length = (direction_x * direction_x + direction_y * direction_y).sqrt();
        if length > 0.0 {
            let normalized_x = direction_x / length;
            let normalized_y = direction_y / length;

            position.x += normalized_x * move_speed * delta_time;
            position.y += normalized_y * move_speed * delta_time;

            // Keep within simulation bounds
            position.x = position.x.clamp(-80.0, 80.0);
            position.y = position.y.clamp(-60.0, 60.0);

            // Update visual transform
            transform.translation.x = position.x;
            transform.translation.y = position.y;
        }
    }
}

/// System for cleaning up invasive species when disaster ends or entities expire
pub fn invasive_species_cleanup_system(
    mut commands: Commands,
    disaster_state: Res<DisasterState>,
    invasive_query: Query<(Entity, &InvasiveSpecies)>,
) {
    // Remove invasive species if disaster has ended or if their lifetime has expired
    for (entity, invasive) in invasive_query.iter() {
        if !disaster_state.is_active(DisasterType::InvasiveSpecies) || invasive.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
