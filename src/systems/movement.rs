use crate::components::{
    Ant, DevelopmentStage, Egg, FoundingState, Larva, Position, Pupa, Queen, Soil, SoilCell,
};
use bevy::prelude::*;
use rand::prelude::*;

/// MVP: Simple ant movement with gravity and basic digging
pub fn ant_movement_system(
    mut commands: Commands,
    time: Res<Time>,
    mut ant_query: Query<(&mut Position, &mut Transform), With<Ant>>,
    soil_query: Query<(Entity, &Position), (With<Soil>, Without<Ant>)>,
) {
    let mut rng = thread_rng();
    let delta_time = time.delta_seconds();

    for (mut position, mut transform) in ant_query.iter_mut() {
        // MVP: Apply gravity - ants fall down unless supported by soil
        let gravity_force = -20.0 * delta_time; // Downward force
        let new_y = position.y + gravity_force;

        // Check if there's soil at the new position to stop falling
        let mut can_fall = true;
        for (_soil_entity, soil_position) in soil_query.iter() {
            let dx = soil_position.x - position.x;
            let dy = soil_position.y - new_y;
            let distance = (dx * dx + dy * dy).sqrt();

            // If ant is close to soil, it can't fall further
            if distance < 4.0 {
                can_fall = false;
                break;
            }
        }

        // Apply gravity if ant can fall
        if can_fall && new_y > -100.0 {
            // Don't fall below a certain depth
            position.y = new_y;
        }

        // MVP: Simple random movement
        if rng.gen_bool(0.1) {
            // 10% chance to move each frame
            let move_x = rng.gen_range(-8.0..8.0) * delta_time;
            let move_y = rng.gen_range(-4.0..4.0) * delta_time;

            position.x += move_x;
            position.y += move_y;

            // Keep ants within reasonable bounds
            position.x = position.x.clamp(-100.0, 100.0);
            position.y = position.y.clamp(-80.0, 20.0);
        }

        // MVP: Basic digging - remove soil that ants walk through
        if rng.gen_bool(0.05) {
            // 5% chance to dig each frame
            for (soil_entity, soil_position) in soil_query.iter() {
                let dx = soil_position.x - position.x;
                let dy = soil_position.y - position.y;
                let distance = (dx * dx + dy * dy).sqrt();

                // If ant is very close to soil, dig it out
                if distance < 3.0 {
                    commands.entity(soil_entity).despawn();
                    break; // Only dig one soil cell at a time
                }
            }
        }

        // Update transform to match position
        transform.translation.x = position.x;
        transform.translation.y = position.y;
    }
}

/// Queen ant founding behavior system
pub fn queen_founding_system(
    mut commands: Commands,
    mut queen_query: Query<(&mut Queen, &mut Position, &mut Transform), With<Ant>>,
    soil_query: Query<(Entity, &Position, &SoilCell), (With<Soil>, Without<Ant>)>,
) {
    let mut rng = thread_rng();

    for (mut queen, mut position, mut transform) in queen_query.iter_mut() {
        match queen.founding_state {
            FoundingState::Seeking => {
                // Queen seeks optimal founding location
                let current_site_quality = evaluate_founding_site(&position, &soil_query);

                if current_site_quality > 0.7 {
                    // Good site found, start digging
                    queen.founding_state = FoundingState::Digging;
                    info!(
                        "Queen found suitable founding site at ({}, {}) with quality {:.2}",
                        position.x, position.y, current_site_quality
                    );
                } else {
                    // Keep searching - move in random direction
                    if rng.gen_bool(0.3) {
                        let move_x = rng.gen_range(-12.0..12.0);
                        let move_y = rng.gen_range(-2.0..2.0); // Prefer staying near surface

                        position.x += move_x;
                        position.y += move_y;

                        // Keep queen within bounds and near surface
                        position.x = position.x.clamp(-90.0, 90.0);
                        position.y = position.y.clamp(-5.0, 5.0);
                    }
                }
            }

            FoundingState::Digging => {
                // Queen digs 3x3 founding chamber
                let mut dug_count = 0;

                // Remove soil in 3x3 area around queen
                for (soil_entity, soil_position, _soil_cell) in soil_query.iter() {
                    let dx = (soil_position.x - position.x).abs();
                    let dy = (soil_position.y - position.y).abs();

                    // Check if soil is within 3x3 area (allowing for 4-pixel spacing)
                    if dx <= 6.0 && dy <= 6.0 {
                        commands.entity(soil_entity).despawn();
                        dug_count += 1;
                    }
                }

                if dug_count > 0 {
                    info!("Queen dug {} soil cells for founding chamber", dug_count);
                }

                // After digging, establish the chamber
                queen.founding_state = FoundingState::Established;

                // Move queen slightly underground into the chamber
                position.y -= 8.0; // Move down into the dug chamber
            }

            FoundingState::Established => {
                // Queen stays in chamber - minimal movement
                if rng.gen_bool(0.02) {
                    // Very small movements within chamber
                    let move_x = rng.gen_range(-2.0..2.0);
                    let move_y = rng.gen_range(-2.0..2.0);

                    position.x += move_x;
                    position.y += move_y;
                }
            }
        }

        // Update transform to match position
        transform.translation.x = position.x;
        transform.translation.y = position.y;
    }
}

/// Evaluate the quality of a founding site based on soil conditions
fn evaluate_founding_site(
    queen_position: &Position,
    soil_query: &Query<(Entity, &Position, &SoilCell), (With<Soil>, Without<Ant>)>,
) -> f32 {
    let mut total_quality = 0.0;
    let mut sample_count = 0;

    // Sample soil conditions in a small area around queen
    for (_entity, soil_position, soil_cell) in soil_query.iter() {
        let dx = (soil_position.x - queen_position.x).abs();
        let dy = (soil_position.y - queen_position.y).abs();

        // Check soil within a reasonable founding area
        if dx <= 12.0 && dy <= 12.0 {
            // Evaluate soil quality based on Camponotus japonicus preferences
            let moisture_score = if soil_cell.moisture >= 0.4 && soil_cell.moisture <= 0.7 {
                1.0
            } else {
                0.3
            };
            let temp_score = if soil_cell.temperature >= 19.0 && soil_cell.temperature <= 21.0 {
                1.0
            } else {
                0.5
            };
            let nutrition_score = if soil_cell.nutrition >= 0.3 { 1.0 } else { 0.4 };

            let site_quality = (moisture_score + temp_score + nutrition_score) / 3.0;
            total_quality += site_quality;
            sample_count += 1;
        }
    }

    if sample_count > 0 {
        total_quality / sample_count as f32
    } else {
        0.0 // No soil nearby - poor site
    }
}

/// Queen egg laying system for established queens
pub fn queen_egg_laying_system(
    mut commands: Commands,
    time: Res<Time>,
    mut queen_query: Query<(&mut Queen, &Position), With<Ant>>,
) {
    let current_time = time.elapsed_seconds();

    for (mut queen, position) in queen_query.iter_mut() {
        // Only established queens can lay eggs
        if queen.founding_state != FoundingState::Established {
            continue;
        }

        // Check if it's time to lay an egg
        let time_since_last_egg = current_time - queen.last_egg_time;
        if time_since_last_egg >= queen.egg_laying_interval {
            // Create egg near queen position (within chamber)
            let mut rng = thread_rng();
            let egg_x = position.x + rng.gen_range(-4.0..4.0);
            let egg_y = position.y + rng.gen_range(-4.0..4.0);

            commands.spawn((
                Position { x: egg_x, y: egg_y },
                Egg {
                    development_time: 0.0,
                    stage: DevelopmentStage::Egg,
                },
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.9, 0.9, 0.8),      // Off-white color for eggs
                        custom_size: Some(Vec2::new(1.0, 1.0)), // 1x1 pixel for tiny eggs
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(egg_x, egg_y, 5.0)),
                    ..default()
                },
            ));

            // Update queen's egg laying timer
            queen.last_egg_time = current_time;

            info!("Queen laid egg at position ({:.1}, {:.1})", egg_x, egg_y);
        }
    }
}

/// Brood development system for egg → larva → pupa → worker progression
pub fn brood_development_system(
    mut commands: Commands,
    time: Res<Time>,
    mut egg_query: Query<(Entity, &mut Egg, &Position, &mut Transform)>,
    mut larva_query: Query<(Entity, &mut Larva, &Position, &mut Transform), Without<Egg>>,
    mut pupa_query: Query<
        (Entity, &mut Pupa, &Position, &mut Transform),
        (Without<Egg>, Without<Larva>),
    >,
) {
    let delta_time = time.delta_seconds();

    // Process eggs
    for (entity, mut egg, position, mut transform) in egg_query.iter_mut() {
        egg.development_time += delta_time;

        // Egg stage: 2-3 weeks (simplified as 20-30 seconds for demo)
        if egg.development_time >= 25.0 {
            // Remove egg component and add larva component
            commands.entity(entity).remove::<Egg>();
            commands.entity(entity).insert(Larva {
                development_time: 0.0,
                fed: true, // Assume queen feeds larva
            });

            // Update visual appearance to yellow larva
            transform.scale = Vec3::new(1.0, 1.0, 1.0);
            commands.entity(entity).insert(SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(1.0, 1.0, 0.6),      // Pale yellow for larvae
                    custom_size: Some(Vec2::new(1.5, 1.5)), // Slightly larger than eggs
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(position.x, position.y, 5.0)),
                ..default()
            });

            info!(
                "Egg hatched into larva at position ({:.1}, {:.1})",
                position.x, position.y
            );
        }
    }

    // Process larvae
    for (entity, mut larva, position, _transform) in larva_query.iter_mut() {
        larva.development_time += delta_time;

        // Larva stage: 3-4 weeks (simplified as 35 seconds for demo)
        if larva.development_time >= 35.0 && larva.fed {
            // Remove larva component and add pupa component
            commands.entity(entity).remove::<Larva>();
            commands.entity(entity).insert(Pupa {
                development_time: 0.0,
            });

            // Update visual appearance to brown pupa
            commands.entity(entity).insert(SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.6, 0.4, 0.2),      // Brown for pupae
                    custom_size: Some(Vec2::new(1.8, 1.8)), // Larger than larvae
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(position.x, position.y, 5.0)),
                ..default()
            });

            info!(
                "Larva pupated at position ({:.1}, {:.1})",
                position.x, position.y
            );
        }
    }

    // Process pupae
    for (entity, mut pupa, position, _transform) in pupa_query.iter_mut() {
        pupa.development_time += delta_time;

        // Pupa stage: 2-3 weeks (simplified as 25 seconds for demo)
        if pupa.development_time >= 25.0 {
            // Remove pupa component and add worker ant components
            commands.entity(entity).remove::<Pupa>();
            commands.entity(entity).insert(Ant);

            // Update visual appearance to black worker ant
            commands.entity(entity).insert(SpriteBundle {
                sprite: Sprite {
                    color: Color::BLACK,                    // Black worker ants
                    custom_size: Some(Vec2::new(2.0, 2.0)), // Standard worker size
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(position.x, position.y, 10.0)),
                ..default()
            });

            info!(
                "Pupa emerged as worker ant at position ({:.1}, {:.1})",
                position.x, position.y
            );
        }
    }
}
