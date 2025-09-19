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
            .add_systems(Startup, (setup_world, spawn_soil_grid, spawn_initial_ants))
            .add_systems(Update, (
                ant_movement_system,
                ant_lifecycle_system,
                environmental_update_system,
            ));
    }
}

// Components for ECS entities
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

#[derive(Component)]
pub struct Ant;

#[derive(Component)]
pub struct Soil;

#[derive(Debug, Clone)]
pub enum AntState {
    Foraging,
    Returning,
    Resting,
    Digging,
}

// Basic camera setup for 2D pixel art view
fn setup_world(mut commands: Commands) {
    // Spawn 2D camera for side-view ant farm observation with better positioning
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1000.0)),
        ..default()
    });

    info!("Ant Nest Simulator initialized - ready for development!");
}

// Create soil grid with visual representation
fn spawn_soil_grid(mut commands: Commands) {
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

// Spawn initial ant colony with visual representation
fn spawn_initial_ants(mut commands: Commands) {
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

// System for ant movement and behavior
fn ant_movement_system(
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
        lifecycle.energy -= 2.0 * time.delta_seconds();

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
        soil.moisture += rng.gen_range(-0.05..0.05) * time.delta_seconds();
        soil.moisture = soil.moisture.clamp(0.0, 1.0);

        soil.temperature += rng.gen_range(-0.2..0.2) * time.delta_seconds();
        soil.temperature = soil.temperature.clamp(10.0, 35.0);

        // Nutrition slowly regenerates
        soil.nutrition += 0.01 * time.delta_seconds();
        soil.nutrition = soil.nutrition.clamp(0.0, 1.0);
    }
}