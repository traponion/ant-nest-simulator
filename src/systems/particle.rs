use bevy::prelude::*;
use crate::components::{
    Particle, ParticleData, ParticleType, DisasterState, DisasterType, TimeControl
};
use crate::systems::time_control::effective_delta_time;
use rand::Rng;

/// Resource for managing particle system configuration
#[derive(Resource)]
pub struct ParticleConfig {
    /// Maximum number of particles that can exist simultaneously
    pub max_particles: usize,
    /// Current number of active particles
    pub active_particles: usize,
    /// Base spawn rate for particles (particles per second)
    pub base_spawn_rate: f32,
    /// Window dimensions for particle boundary calculations
    pub window_width: f32,
    pub window_height: f32,
}

impl Default for ParticleConfig {
    fn default() -> Self {
        Self {
            max_particles: 150, // Conservative limit for performance
            active_particles: 0,
            base_spawn_rate: 30.0, // 30 particles per second base rate
            window_width: 800.0, // Will be updated by window system
            window_height: 600.0, // Will be updated by window system
        }
    }
}

/// System for spawning particles based on active disasters
pub fn particle_spawner_system(
    mut commands: Commands,
    disaster_state: Res<DisasterState>,
    mut particle_config: ResMut<ParticleConfig>,
    time: Res<Time>,
    time_control: Res<TimeControl>,
    windows: Query<&Window>,
) {
    let delta_time = effective_delta_time(&time, &time_control);

    // Update window dimensions
    if let Ok(window) = windows.get_single() {
        particle_config.window_width = window.width();
        particle_config.window_height = window.height();
    }

    // Don't spawn if we're at particle limit
    if particle_config.active_particles >= particle_config.max_particles {
        return;
    }

    let mut rng = rand::thread_rng();

    // Spawn rain particles
    if disaster_state.is_active(DisasterType::Rain) {
        spawn_rain_particles(
            &mut commands,
            &mut particle_config,
            &mut rng,
            delta_time,
        );
    }

    // Spawn drought particles
    if disaster_state.is_active(DisasterType::Drought) {
        spawn_drought_particles(
            &mut commands,
            &mut particle_config,
            &mut rng,
            delta_time,
        );
    }

    // Spawn cold snap particles
    if disaster_state.is_active(DisasterType::ColdSnap) {
        spawn_cold_snap_particles(
            &mut commands,
            &mut particle_config,
            &mut rng,
            delta_time,
        );
    }

    // Spawn invasive species particles
    if disaster_state.is_active(DisasterType::InvasiveSpecies) {
        spawn_invasive_particles(
            &mut commands,
            &mut particle_config,
            &mut rng,
            delta_time,
        );
    }
}

/// System for updating particle positions, lifetimes, and cleanup
pub fn particle_update_system(
    mut commands: Commands,
    mut particle_config: ResMut<ParticleConfig>,
    mut particle_query: Query<(Entity, &mut Transform, &mut ParticleData, &mut Sprite), With<Particle>>,
    time: Res<Time>,
    time_control: Res<TimeControl>,
) {
    let delta_time = effective_delta_time(&time, &time_control);
    let mut particles_to_remove = Vec::new();

    for (entity, mut transform, mut particle_data, mut sprite) in particle_query.iter_mut() {
        // Update lifetime
        particle_data.lifetime -= delta_time;

        // Mark for removal if lifetime expired
        if particle_data.lifetime <= 0.0 {
            particles_to_remove.push(entity);
            continue;
        }

        // Update position based on velocity
        transform.translation.x += particle_data.velocity.x * delta_time;
        transform.translation.y += particle_data.velocity.y * delta_time;

        // Update color with fade effect
        sprite.color = particle_data.get_current_color();

        // Check bounds and mark for removal if out of screen
        if is_particle_out_of_bounds(
            transform.translation.x,
            transform.translation.y,
            particle_config.window_width,
            particle_config.window_height,
        ) {
            particles_to_remove.push(entity);
        }

        // Apply particle-type specific behavior updates
        update_particle_behavior(&mut particle_data, delta_time);
    }

    // Remove expired or out-of-bounds particles
    for entity in particles_to_remove {
        commands.entity(entity).despawn();
        particle_config.active_particles = particle_config.active_particles.saturating_sub(1);
    }
}

/// System for updating particle config when window is resized
pub fn update_particle_config_system(
    mut particle_config: ResMut<ParticleConfig>,
    windows: Query<&Window>,
    mut resize_events: EventReader<bevy::window::WindowResized>,
) {
    for _event in resize_events.read() {
        if let Ok(window) = windows.get_single() {
            particle_config.window_width = window.width();
            particle_config.window_height = window.height();
        }
    }
}

/// Helper function to spawn rain particles
fn spawn_rain_particles(
    commands: &mut Commands,
    particle_config: &mut ParticleConfig,
    rng: &mut impl Rng,
    delta_time: f32,
) {
    let spawn_count = calculate_spawn_count(particle_config.base_spawn_rate * 1.2, delta_time, rng);

    for _ in 0..spawn_count {
        if particle_config.active_particles >= particle_config.max_particles {
            break;
        }

        let x = rng.gen_range(0.0..particle_config.window_width);
        let y = particle_config.window_height + 10.0; // Start above screen
        let velocity = Vec2::new(
            rng.gen_range(-20.0..20.0), // Slight horizontal drift
            rng.gen_range(-200.0..-150.0), // Downward velocity
        );
        let lifetime = rng.gen_range(3.0..6.0);

        spawn_particle(
            commands,
            particle_config,
            ParticleData::new_rain_drop(lifetime, velocity),
            Vec3::new(x, y, 100.0), // Z=100 for particle layer
        );
    }
}

/// Helper function to spawn drought particles
fn spawn_drought_particles(
    commands: &mut Commands,
    particle_config: &mut ParticleConfig,
    rng: &mut impl Rng,
    delta_time: f32,
) {
    let spawn_count = calculate_spawn_count(particle_config.base_spawn_rate * 0.8, delta_time, rng);

    for _ in 0..spawn_count {
        if particle_config.active_particles >= particle_config.max_particles {
            break;
        }

        let x = rng.gen_range(0.0..particle_config.window_width);
        let y = rng.gen_range(-10.0..50.0); // Start from ground level
        let velocity = Vec2::new(
            rng.gen_range(-30.0..30.0), // Horizontal drift
            rng.gen_range(50.0..120.0), // Upward velocity
        );
        let lifetime = rng.gen_range(4.0..8.0);

        spawn_particle(
            commands,
            particle_config,
            ParticleData::new_dust_mote(lifetime, velocity),
            Vec3::new(x, y, 100.0),
        );
    }
}

/// Helper function to spawn cold snap particles
fn spawn_cold_snap_particles(
    commands: &mut Commands,
    particle_config: &mut ParticleConfig,
    rng: &mut impl Rng,
    delta_time: f32,
) {
    let spawn_count = calculate_spawn_count(particle_config.base_spawn_rate * 1.0, delta_time, rng);

    for _ in 0..spawn_count {
        if particle_config.active_particles >= particle_config.max_particles {
            break;
        }

        let x = rng.gen_range(0.0..particle_config.window_width);
        let y = particle_config.window_height + 10.0; // Start above screen
        let velocity = Vec2::new(
            rng.gen_range(-40.0..40.0), // More horizontal drift than rain
            rng.gen_range(-100.0..-60.0), // Slower falling than rain
        );
        let lifetime = rng.gen_range(5.0..10.0);

        spawn_particle(
            commands,
            particle_config,
            ParticleData::new_snowflake(lifetime, velocity),
            Vec3::new(x, y, 100.0),
        );
    }
}

/// Helper function to spawn invasive species particles
fn spawn_invasive_particles(
    commands: &mut Commands,
    particle_config: &mut ParticleConfig,
    rng: &mut impl Rng,
    delta_time: f32,
) {
    let spawn_count = calculate_spawn_count(particle_config.base_spawn_rate * 0.6, delta_time, rng);

    for _ in 0..spawn_count {
        if particle_config.active_particles >= particle_config.max_particles {
            break;
        }

        let x = rng.gen_range(0.0..particle_config.window_width);
        let y = rng.gen_range(0.0..particle_config.window_height);
        let velocity = Vec2::new(
            rng.gen_range(-60.0..60.0), // Random horizontal movement
            rng.gen_range(-60.0..60.0), // Random vertical movement
        );
        let lifetime = rng.gen_range(2.0..5.0);

        spawn_particle(
            commands,
            particle_config,
            ParticleData::new_environmental_disturbance(lifetime, velocity),
            Vec3::new(x, y, 100.0),
        );
    }
}

/// Helper function to spawn a single particle
fn spawn_particle(
    commands: &mut Commands,
    particle_config: &mut ParticleConfig,
    particle_data: ParticleData,
    position: Vec3,
) {
    commands.spawn((
        Particle,
        SpriteBundle {
            sprite: Sprite {
                color: particle_data.get_current_color(),
                custom_size: Some(particle_data.size),
                ..default()
            },
            transform: Transform::from_translation(position),
            ..default()
        },
        particle_data,
    ));

    particle_config.active_particles += 1;
}

/// Calculate number of particles to spawn this frame based on spawn rate
fn calculate_spawn_count(spawn_rate: f32, delta_time: f32, rng: &mut impl Rng) -> usize {
    let base_count = spawn_rate * delta_time;
    let integer_part = base_count.floor() as usize;
    let fractional_part = base_count.fract();

    // Use fractional part as probability for one additional particle
    if rng.gen::<f32>() < fractional_part {
        integer_part + 1
    } else {
        integer_part
    }
}

/// Check if particle is outside screen bounds
fn is_particle_out_of_bounds(x: f32, y: f32, window_width: f32, window_height: f32) -> bool {
    x < -50.0 || x > window_width + 50.0 || y < -50.0 || y > window_height + 50.0
}

/// Update particle behavior based on type
fn update_particle_behavior(particle_data: &mut ParticleData, delta_time: f32) {
    match particle_data.particle_type {
        ParticleType::RainDrop => {
            // Rain drops might accelerate slightly due to gravity
            particle_data.velocity.y -= 20.0 * delta_time;
        }
        ParticleType::DustMote => {
            // Dust particles slow down over time
            particle_data.velocity *= 0.98;
        }
        ParticleType::Snowflake => {
            // Snowflakes might drift more horizontally
            particle_data.velocity.x += rand::thread_rng().gen_range(-10.0..10.0) * delta_time;
        }
        ParticleType::EnvironmentalDisturbance => {
            // Environmental disturbance changes direction randomly
            let mut rng = rand::thread_rng();
            particle_data.velocity.x += rng.gen_range(-30.0..30.0) * delta_time;
            particle_data.velocity.y += rng.gen_range(-30.0..30.0) * delta_time;
            // Cap velocity to prevent particles from moving too fast
            particle_data.velocity = particle_data.velocity.clamp_length_max(80.0);
        }
    }
}