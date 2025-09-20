//! Colony Development System
//!
//! Manages the four-phase development cycle of Camponotus japonicus colonies:
//! 1. Queen's Independent Founding
//! 2. First Workers
//! 3. Colony Expansion
//! 4. Mature Colony
//!
//! This system tracks colony progress, manages phase transitions, and applies
//! phase-specific behavioral modifications to create a realistic ant colony
//! simulation with high replayability through randomized colony traits.

use crate::components::*;
use bevy::prelude::*;

/// System for managing colony development phase progression
pub fn colony_development_management_system(
    time: Res<SimulationTime>,
    mut colony_phase: ResMut<ColonyDevelopmentPhase>,
    ant_query: Query<Entity, (With<Ant>, Without<Queen>)>,
    queen_query: Query<&Lifecycle, With<Queen>>,
    chamber_query: Query<&Chamber>,
    tunnel_query: Query<&Tunnel>,
    mut ant_behavior_query: Query<&mut PhaseSpecificBehavior, With<Ant>>,
) {
    // Update time spent in current phase
    colony_phase.time_in_phase = time.current_day as f32;

    // Get current colony statistics
    let worker_count = ant_query.iter().count();
    let nest_complexity = chamber_query.iter().count() + tunnel_query.iter().count();
    let queen_alive = queen_query.iter().next().is_some();

    // Calculate phase progress based on multiple criteria
    let time_progress = calculate_time_progress(&colony_phase);
    let population_progress = calculate_population_progress(&colony_phase, worker_count);
    let complexity_progress = calculate_complexity_progress(&colony_phase, nest_complexity);
    let stability_progress = calculate_stability_progress(&colony_phase, queen_alive, worker_count);

    // Overall progress is the minimum of all criteria (bottleneck system)
    colony_phase.phase_progress = time_progress
        .min(population_progress)
        .min(complexity_progress)
        .min(stability_progress);

    // Check for phase transition
    if should_transition_phase(&colony_phase) {
        if let Some(next_phase) = colony_phase.current_phase.next_phase() {
            transition_to_phase(&mut colony_phase, next_phase);

            // Apply new phase behavioral modifiers to all ants
            apply_phase_behavioral_changes(&mut ant_behavior_query, &colony_phase);

            info!(
                "Colony transitioned to phase: {} after {} days",
                next_phase.display_name(),
                colony_phase.time_in_phase
            );
        }
    }

    // Update ant age groups and specialized roles based on current phase
    update_ant_roles_system(&mut ant_behavior_query, &colony_phase);
}

/// Calculate progress based on time spent in current phase
fn calculate_time_progress(colony_phase: &ColonyDevelopmentPhase) -> f32 {
    let min_days = colony_phase.phase_conditions.min_days_in_phase;
    if min_days == f32::INFINITY {
        return 1.0; // Mature colony phase never progresses based on time
    }

    (colony_phase.time_in_phase / min_days).min(1.0)
}

/// Calculate progress based on worker population
fn calculate_population_progress(
    colony_phase: &ColonyDevelopmentPhase,
    worker_count: usize,
) -> f32 {
    let target = colony_phase.phase_conditions.target_worker_count;
    if target == usize::MAX {
        return 1.0; // No population requirement
    }

    (worker_count as f32 / target as f32).min(1.0)
}

/// Calculate progress based on nest architectural complexity
fn calculate_complexity_progress(
    colony_phase: &ColonyDevelopmentPhase,
    nest_complexity: usize,
) -> f32 {
    let required = colony_phase.phase_conditions.required_nest_complexity;
    if required == usize::MAX {
        return 1.0; // No complexity requirement
    }

    (nest_complexity as f32 / required as f32).min(1.0)
}

/// Calculate progress based on colony stability (survival rates)
fn calculate_stability_progress(
    colony_phase: &ColonyDevelopmentPhase,
    queen_alive: bool,
    worker_count: usize,
) -> f32 {
    let threshold = colony_phase.phase_conditions.stability_threshold;

    match colony_phase.current_phase {
        DevelopmentPhase::QueenFounding => {
            // During founding phase, stability is based on queen survival
            if queen_alive {
                1.0
            } else {
                0.0
            }
        }
        _ => {
            // In later phases, stability is based on worker population maintenance
            // This is a simplified metric - in a full implementation, you'd track
            // birth/death rates over time
            let stability_score = if queen_alive && worker_count > 0 {
                // Simple stability metric based on having workers and queen
                0.9 // Assume high stability if basic conditions are met
            } else {
                0.0
            };

            if stability_score >= threshold {
                1.0
            } else {
                stability_score / threshold
            }
        }
    }
}

/// Check if the colony should transition to the next phase
fn should_transition_phase(colony_phase: &ColonyDevelopmentPhase) -> bool {
    // All progress criteria must be 100% complete
    colony_phase.phase_progress >= 1.0 && colony_phase.current_phase.next_phase().is_some()
}

/// Transition the colony to a new development phase
fn transition_to_phase(colony_phase: &mut ColonyDevelopmentPhase, new_phase: DevelopmentPhase) {
    colony_phase.current_phase = new_phase;
    colony_phase.time_in_phase = 0.0;
    colony_phase.phase_progress = 0.0;
    colony_phase.phase_conditions = PhaseConditions::for_phase(new_phase);
}

/// Apply phase-specific behavioral changes to all ants
fn apply_phase_behavioral_changes(
    ant_behavior_query: &mut Query<&mut PhaseSpecificBehavior, With<Ant>>,
    colony_phase: &ColonyDevelopmentPhase,
) {
    for mut behavior in ant_behavior_query.iter_mut() {
        behavior.behavior_modifiers =
            calculate_phase_modifiers(colony_phase.current_phase, &colony_phase.colony_traits);
    }
}

/// Calculate behavioral modifiers based on colony phase and traits
fn calculate_phase_modifiers(phase: DevelopmentPhase, traits: &ColonyTraits) -> BehaviorModifiers {
    match phase {
        DevelopmentPhase::QueenFounding => BehaviorModifiers {
            speed_modifier: 0.7 * traits.worker_efficiency, // Slower, cautious movement
            foraging_efficiency: 0.5,                       // No foraging - queen does everything
            construction_skill: 0.8 * traits.architectural_skill, // Basic nest construction
            energy_efficiency: 1.1 * traits.environmental_adaptation, // Efficient energy use
        },
        DevelopmentPhase::FirstWorkers => BehaviorModifiers {
            speed_modifier: 0.9 * traits.worker_efficiency, // Still learning
            foraging_efficiency: 0.7 * traits.worker_efficiency, // Basic foraging
            construction_skill: 0.9 * traits.architectural_skill, // Improving construction
            energy_efficiency: 1.0 * traits.environmental_adaptation, // Normal efficiency
        },
        DevelopmentPhase::ColonyExpansion => BehaviorModifiers {
            speed_modifier: 1.1 * traits.worker_efficiency, // Confident movement
            foraging_efficiency: 1.0 * traits.worker_efficiency, // Good foraging
            construction_skill: 1.1 * traits.architectural_skill, // Advanced construction
            energy_efficiency: 0.95 * traits.environmental_adaptation, // High activity cost
        },
        DevelopmentPhase::MatureColony => BehaviorModifiers {
            speed_modifier: 1.0 * traits.worker_efficiency, // Optimized movement
            foraging_efficiency: 1.2 * traits.worker_efficiency, // Expert foraging
            construction_skill: 1.0 * traits.architectural_skill, // Maintenance focus
            energy_efficiency: 1.0 * traits.environmental_adaptation, // Balanced efficiency
        },
    }
}

/// Update ant age groups and assign specialized roles based on current phase
fn update_ant_roles_system(
    ant_behavior_query: &mut Query<&mut PhaseSpecificBehavior, With<Ant>>,
    colony_phase: &ColonyDevelopmentPhase,
) {
    for mut behavior in ant_behavior_query.iter_mut() {
        // Age groups would be updated by the lifecycle system
        // Here we just assign appropriate roles based on age and phase
        behavior.specialized_role =
            assign_specialized_role(behavior.age_group, colony_phase.current_phase);
    }
}

/// Assign specialized roles based on ant age group and colony phase
fn assign_specialized_role(age_group: AntAgeGroup, phase: DevelopmentPhase) -> SpecializedRole {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    match (phase, age_group) {
        (DevelopmentPhase::QueenFounding, _) => {
            // No specialized roles during founding - queen does everything
            SpecializedRole::GeneralWorker
        }
        (DevelopmentPhase::FirstWorkers, AntAgeGroup::Young) => {
            // Young workers stay in nest
            if rng.gen_bool(0.7) {
                SpecializedRole::NurseryWorker
            } else {
                SpecializedRole::GeneralWorker
            }
        }
        (DevelopmentPhase::FirstWorkers, AntAgeGroup::Adult) => {
            // Adult workers do general tasks
            if rng.gen_bool(0.3) {
                SpecializedRole::Forager
            } else {
                SpecializedRole::GeneralWorker
            }
        }
        (DevelopmentPhase::FirstWorkers, AntAgeGroup::Senior) => {
            // Senior workers become foragers
            SpecializedRole::Forager
        }
        (DevelopmentPhase::ColonyExpansion, AntAgeGroup::Young) => {
            // More specialization emerges
            match rng.gen_range(0..3) {
                0 => SpecializedRole::NurseryWorker,
                1 => SpecializedRole::NestMaintainer,
                _ => SpecializedRole::GeneralWorker,
            }
        }
        (DevelopmentPhase::ColonyExpansion, AntAgeGroup::Adult) => {
            // Adults take on varied roles
            match rng.gen_range(0..4) {
                0 => SpecializedRole::Forager,
                1 => SpecializedRole::NestMaintainer,
                2 => SpecializedRole::StorageWorker,
                _ => SpecializedRole::GeneralWorker,
            }
        }
        (DevelopmentPhase::ColonyExpansion, AntAgeGroup::Senior) => {
            // Experienced workers become specialized foragers or maintainers
            if rng.gen_bool(0.6) {
                SpecializedRole::Forager
            } else {
                SpecializedRole::NestMaintainer
            }
        }
        (DevelopmentPhase::MatureColony, AntAgeGroup::Young) => {
            // Full specialization in mature colonies
            match rng.gen_range(0..4) {
                0 => SpecializedRole::NurseryWorker,
                1 => SpecializedRole::StorageWorker,
                2 => SpecializedRole::WasteManager,
                _ => SpecializedRole::GeneralWorker,
            }
        }
        (DevelopmentPhase::MatureColony, AntAgeGroup::Adult) => {
            // Adults in mature colonies have diverse specialized roles
            match rng.gen_range(0..6) {
                0 => SpecializedRole::Forager,
                1 => SpecializedRole::NestMaintainer,
                2 => SpecializedRole::StorageWorker,
                3 => SpecializedRole::WasteManager,
                4 => SpecializedRole::NurseryWorker,
                _ => SpecializedRole::GeneralWorker,
            }
        }
        (DevelopmentPhase::MatureColony, AntAgeGroup::Senior) => {
            // Senior ants become expert foragers or specialized maintainers
            match rng.gen_range(0..3) {
                0 => SpecializedRole::Forager,
                1 => SpecializedRole::NestMaintainer,
                _ => SpecializedRole::WasteManager,
            }
        }
    }
}

/// System to update ant age groups based on their lifecycle
pub fn update_ant_age_groups_system(
    mut ant_query: Query<(&mut PhaseSpecificBehavior, &Lifecycle), With<Ant>>,
) {
    for (mut behavior, lifecycle) in ant_query.iter_mut() {
        let age_ratio = lifecycle.age / lifecycle.max_age;
        behavior.age_group = AntAgeGroup::from_age_ratio(age_ratio);
    }
}

/// System to apply phase-specific behavior modifiers to ant actions
pub fn apply_phase_behavior_modifiers_system(
    mut ant_query: Query<(&mut AntBehavior, &PhaseSpecificBehavior), With<Ant>>,
) {
    for (mut ant_behavior, phase_behavior) in ant_query.iter_mut() {
        // Apply speed modifier
        ant_behavior.speed *= phase_behavior.behavior_modifiers.speed_modifier;

        // Clamp speed to reasonable bounds
        ant_behavior.speed = ant_behavior.speed.clamp(5.0, 50.0);
    }
}

/// System to display colony development information in UI
pub fn colony_development_ui_system(
    colony_phase: Res<ColonyDevelopmentPhase>,
    mut text_query: Query<&mut Text, With<ColonyDevelopmentDisplay>>,
) {
    for mut text in text_query.iter_mut() {
        if let Some(section) = text.sections.first_mut() {
            section.value = format!(
                "Colony Phase: {}\nDay {} in Phase\nProgress: {:.1}%\nTraits: Vigor {:.2}, Efficiency {:.2}",
                colony_phase.current_phase.display_name(),
                colony_phase.time_in_phase,
                colony_phase.phase_progress * 100.0,
                colony_phase.colony_traits.queen_vigor,
                colony_phase.colony_traits.worker_efficiency,
            );
        }
    }
}

/// Component marker for colony development display UI
#[derive(Component)]
pub struct ColonyDevelopmentDisplay;

/// System to spawn colony development UI panel
pub fn setup_colony_development_ui(mut commands: Commands, ui_theme: Res<UITheme>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                right: Val::Px(10.0),
                width: Val::Px(280.0),
                height: Val::Px(120.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(ui_theme.spacing.md)),
                border: UiRect::all(Val::Px(ui_theme.borders.width_medium)),
                ..default()
            },
            background_color: ui_theme.colors.surface_primary.into(),
            border_color: ui_theme.colors.border_primary.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Colony Development Loading...",
                    TextStyle {
                        font_size: ui_theme.typography.body_medium,
                        color: ui_theme.colors.text_primary,
                        ..default()
                    },
                ),
                ColonyDevelopmentDisplay,
            ));
        });
}
