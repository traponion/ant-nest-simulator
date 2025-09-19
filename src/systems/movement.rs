use bevy::prelude::*;
use rand::prelude::*;
use crate::components::{Position, AntBehavior, AntState, Ant, Lifecycle};

/// System for ant movement and behavior
pub fn ant_movement_system(
    time: Res<Time>,
    mut ant_query: Query<(&mut Position, &mut AntBehavior, &mut Transform), (With<Ant>, With<Lifecycle>)>,
) {
    let mut rng = thread_rng();

    for (mut position, mut behavior, mut transform) in ant_query.iter_mut() {
        match behavior.state {
            AntState::Foraging => {
                // Simple random movement for foraging
                if behavior.target_position.is_none() {
                    behavior.target_position = Some(Position {
                        x: position.x + rng.gen_range(-50.0..50.0),
                        y: position.y + rng.gen_range(-50.0..50.0),
                    });
                }

                if let Some(target) = &behavior.target_position {
                    let dx = target.x - position.x;
                    let dy = target.y - position.y;
                    let distance = (dx * dx + dy * dy).sqrt();

                    if distance > 1.0 {
                        let move_distance = behavior.speed * time.delta_seconds();
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
            _ => {
                // Other states will be implemented later
            }
        }
    }
}