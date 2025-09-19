use bevy::prelude::*;
use rand::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Ant Nest Simulator".into(),
                resolution: (800.0, 600.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(AntNestPlugin)
        .run();
}

pub struct AntNestPlugin;

impl Plugin for AntNestPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (setup_world, spawn_initial_ants))
            .add_systems(Update, (
                ant_movement_system,
                ant_lifecycle_system,
                environmental_update_system,
            ));
    }
}

// Components for ant entities
#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct AntBehavior {
    pub state: AntState,
    pub target_position: Option<Position>,
    pub speed: f32,
}

#[derive(Component)]
pub struct Lifecycle {
    pub age: f32,
    pub max_age: f32,
    pub energy: f32,
    pub max_energy: f32,
}

#[derive(Component)]
pub struct SoilCell {
    pub moisture: f32,
    pub temperature: f32,
    pub nutrition: f32,
}

#[derive(Debug, Clone)]
pub enum AntState {
    Foraging,
    Returning,
    Resting,
    Digging,
}

// Basic camera setup for 2D pixel art view
fn setup_world(mut commands: Commands) {
    // Spawn 2D camera for side-view ant farm observation
    commands.spawn(Camera2dBundle::default());

    // Create basic soil grid (simplified for now)
    let mut rng = thread_rng();
    for x in -20..20 {
        for y in -15..15 {
            commands.spawn((
                Position {
                    x: x as f32 * 10.0,
                    y: y as f32 * 10.0
                },
                SoilCell {
                    moisture: rng.gen_range(0.0..1.0),
                    temperature: rng.gen_range(15.0..25.0),
                    nutrition: rng.gen_range(0.0..1.0),
                },
            ));
        }
    }

    info!("Ant Nest Simulator initialized - ready for development!");
}

// Spawn initial ant colony
fn spawn_initial_ants(mut commands: Commands) {
    let mut rng = thread_rng();

    // Spawn 10 initial ants near the center
    for i in 0..10 {
        let x_offset = rng.gen_range(-50.0..50.0);
        let y_offset = rng.gen_range(-50.0..50.0);

        commands.spawn((
            Position {
                x: x_offset,
                y: y_offset
            },
            AntBehavior {
                state: AntState::Foraging,
                target_position: None,
                speed: rng.gen_range(20.0..40.0),
            },
            Lifecycle {
                age: 0.0,
                max_age: rng.gen_range(30.0..60.0), // seconds for now
                energy: 100.0,
                max_energy: 100.0,
            },
        ));
    }

    info!("Spawned {} initial ants", 10);
}

// System for ant movement and behavior
fn ant_movement_system(
    time: Res<Time>,
    mut ant_query: Query<(&mut Position, &mut AntBehavior), With<Lifecycle>>,
) {
    let mut rng = thread_rng();

    for (mut position, mut behavior) in ant_query.iter_mut() {
        match behavior.state {
            AntState::Foraging => {
                // Simple random movement for foraging
                if behavior.target_position.is_none() {
                    behavior.target_position = Some(Position {
                        x: position.x + rng.gen_range(-100.0..100.0),
                        y: position.y + rng.gen_range(-100.0..100.0),
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

// System for ant aging and energy management
fn ant_lifecycle_system(
    time: Res<Time>,
    mut commands: Commands,
    mut ant_query: Query<(Entity, &mut Lifecycle), With<AntBehavior>>,
) {
    for (entity, mut lifecycle) in ant_query.iter_mut() {
        // Age the ant
        lifecycle.age += time.delta_seconds();

        // Decrease energy over time (simplified)
        lifecycle.energy -= 5.0 * time.delta_seconds();

        // Check if ant should die
        if lifecycle.age >= lifecycle.max_age || lifecycle.energy <= 0.0 {
            commands.entity(entity).despawn();
            info!("Ant died at age {:.1}s with {:.1} energy", lifecycle.age, lifecycle.energy);
        }
    }
}

// System for environmental simulation
fn environmental_update_system(
    time: Res<Time>,
    mut soil_query: Query<&mut SoilCell>,
) {
    let mut rng = thread_rng();

    for mut soil in soil_query.iter_mut() {
        // Simple environmental changes over time
        soil.moisture += rng.gen_range(-0.1..0.1) * time.delta_seconds();
        soil.moisture = soil.moisture.clamp(0.0, 1.0);

        soil.temperature += rng.gen_range(-0.5..0.5) * time.delta_seconds();
        soil.temperature = soil.temperature.clamp(10.0, 35.0);

        // Nutrition slowly regenerates
        soil.nutrition += 0.01 * time.delta_seconds();
        soil.nutrition = soil.nutrition.clamp(0.0, 1.0);
    }
}