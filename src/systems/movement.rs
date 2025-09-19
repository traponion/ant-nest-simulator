use crate::components::{Ant, AntBehavior, AntState, Food, FoodSource, Inventory, Lifecycle, Position, TimeControl};
use crate::systems::time_control::effective_delta_time;
use bevy::prelude::*;
use rand::prelude::*;

/// System for ant movement and behavior
pub fn ant_movement_system(
    time: Res<Time>,
    time_control: Res<TimeControl>,
    mut ant_query: Query<(&mut Position, &mut AntBehavior, &mut Transform, &Lifecycle, &mut Inventory), With<Ant>>,
    food_query: Query<(&Position, &FoodSource), With<Food>>,
) {
    let mut rng = thread_rng();

    for (mut position, mut behavior, mut transform, lifecycle, mut inventory) in ant_query.iter_mut() {
        match behavior.state {
            AntState::Foraging => {
                // Check if ant should look for food when energy is low
                if lifecycle.energy < 30.0 && behavior.target_position.is_none() {
                    // Look for nearest available food
                    let mut nearest_food: Option<Position> = None;
                    let mut nearest_distance = f32::INFINITY;

                    for (food_pos, food_source) in food_query.iter() {
                        if food_source.is_available {
                            let dx = food_pos.x - position.x;
                            let dy = food_pos.y - position.y;
                            let distance = (dx * dx + dy * dy).sqrt();

                            if distance < nearest_distance {
                                nearest_distance = distance;
                                nearest_food = Some(Position { x: food_pos.x, y: food_pos.y });
                            }
                        }
                    }

                    // Set target to nearest food or random position if no food available
                    behavior.target_position = match nearest_food {
                        Some(food_pos) => Some(food_pos),
                        None => Some(Position {
                            x: position.x + rng.gen_range(-50.0..50.0),
                            y: position.y + rng.gen_range(-50.0..50.0),
                        }),
                    };
                } else if behavior.target_position.is_none() {
                    // Normal random foraging when energy is sufficient
                    behavior.target_position = Some(Position {
                        x: position.x + rng.gen_range(-50.0..50.0),
                        y: position.y + rng.gen_range(-50.0..50.0),
                    });
                }

                // Move toward target
                if let Some(target) = &behavior.target_position {
                    let dx = target.x - position.x;
                    let dy = target.y - position.y;
                    let distance = (dx * dx + dy * dy).sqrt();

                    if distance > 1.0 {
                        let delta_time = effective_delta_time(&time, &time_control);
                        let move_distance = behavior.speed * delta_time;
                        position.x += (dx / distance) * move_distance;
                        position.y += (dy / distance) * move_distance;

                        // Update visual transform to match logical position
                        transform.translation.x = position.x;
                        transform.translation.y = position.y;
                    } else {
                        // Reached target, pick new one
                        behavior.target_position = None;
                    }
                }
            },
            AntState::CarryingFood => {
                // Move back to colony with food
                if let Some(target) = &behavior.target_position {
                    let dx = target.x - position.x;
                    let dy = target.y - position.y;
                    let distance = (dx * dx + dy * dy).sqrt();

                    if distance > 2.0 {
                        let delta_time = effective_delta_time(&time, &time_control);
                        let move_distance = behavior.speed * delta_time;
                        position.x += (dx / distance) * move_distance;
                        position.y += (dy / distance) * move_distance;

                        // Update visual transform to match logical position
                        transform.translation.x = position.x;
                        transform.translation.y = position.y;
                    } else {
                        // Reached colony, deliver food and return to foraging
                        inventory.carried_food_value = 0.0;
                        behavior.state = AntState::Foraging;
                        behavior.target_position = None;
                        info!("Ant delivered food to colony!");
                    }
                }
            },
            _ => {
                // Other states will be implemented later
            }
        }
    }
}
