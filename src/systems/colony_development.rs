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

/// Individual progress tracking component for UI display
#[derive(Component)]
pub struct PhaseProgressTracking {
    pub time_progress: f32,
    pub population_progress: f32,
    pub complexity_progress: f32,
    pub stability_progress: f32,
}

/// System for managing colony development phase progression
pub fn colony_development_management_system(
    time: Res<SimulationTime>,
    mut colony_phase: ResMut<ColonyDevelopmentPhase>,
    ant_query: Query<Entity, (With<Ant>, Without<Queen>)>,
    queen_query: Query<&Lifecycle, With<Queen>>,
    chamber_query: Query<&Chamber>,
    tunnel_query: Query<&Tunnel>,
    mut ant_behavior_query: Query<&mut PhaseSpecificBehavior, With<Ant>>,
    mut progress_tracking: ResMut<PhaseProgressTracking>,
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

    // Store individual progress for UI display
    progress_tracking.time_progress = time_progress;
    progress_tracking.population_progress = population_progress;
    progress_tracking.complexity_progress = complexity_progress;
    progress_tracking.stability_progress = stability_progress;

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

/// System to display colony development information in enhanced UI
pub fn colony_development_ui_system(
    colony_phase: Res<ColonyDevelopmentPhase>,
    progress_tracking: Res<PhaseProgressTracking>,
    mut text_query: Query<(&mut Text, &Name)>,
    mut progress_bar_query: Query<&mut Style, (With<ProgressBar>, Without<Text>)>,
    ui_theme: Res<UITheme>,
) {
    // Update text displays
    for (mut text, name) in text_query.iter_mut() {
        let new_text = match name.as_str() {
            "phase_name" => colony_phase.current_phase.display_name().to_string(),
            "phase_description" => colony_phase.current_phase.description().to_string(),
            "phase_day" => format!("Day {} in Phase", colony_phase.time_in_phase),
            "overall_progress" => format!("Overall Progress: {:.1}%", colony_phase.phase_progress * 100.0),
            "time_progress" => format!("Time: {:.1}%", progress_tracking.time_progress * 100.0),
            "population_progress" => format!("Population: {:.1}%", progress_tracking.population_progress * 100.0),
            "complexity_progress" => format!("Complexity: {:.1}%", progress_tracking.complexity_progress * 100.0),
            "stability_progress" => format!("Stability: {:.1}%", progress_tracking.stability_progress * 100.0),
            "next_phase" => {
                if let Some(next_phase) = colony_phase.current_phase.next_phase() {
                    format!("Next: {}", next_phase.display_name())
                } else {
                    "Final Phase".to_string()
                }
            },
            _ => continue,
        };

        if let Some(section) = text.sections.first_mut() {
            section.value = new_text;
        }
    }

    // Update progress bars
    let mut bar_index = 0;
    for mut style in progress_bar_query.iter_mut() {
        let progress = match bar_index {
            0 => colony_phase.phase_progress,
            1 => progress_tracking.time_progress,
            2 => progress_tracking.population_progress,
            3 => progress_tracking.complexity_progress,
            4 => progress_tracking.stability_progress,
            _ => 0.0,
        };

        style.width = Val::Percent(progress * 100.0);
        bar_index += 1;
    }
}

/// Component marker for colony development display UI
#[derive(Component)]
pub struct ColonyDevelopmentDisplay;

/// Component marker for progress bars
#[derive(Component)]
pub struct ProgressBar;

/// Component marker for phase timeline elements
#[derive(Component)]
pub struct PhaseTimelineElement;

/// System to spawn enhanced colony development UI panel
pub fn setup_colony_development_ui(mut commands: Commands, ui_theme: Res<UITheme>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                right: Val::Px(10.0),
                width: Val::Px(350.0),
                height: Val::Px(400.0),
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
            // Header Section
            create_section_header(parent, "Colony Development", &ui_theme);

            // Current Phase Info
            create_phase_info_section(parent, &ui_theme);

            // Overall Progress Bar
            create_overall_progress_section(parent, &ui_theme);

            // Individual Progress Bars
            create_individual_progress_section(parent, &ui_theme);

            // Phase Timeline
            create_phase_timeline_section(parent, &ui_theme);

            // Next Phase Preview
            create_next_phase_section(parent, &ui_theme);
        });
}

/// Create section header
fn create_section_header(parent: &mut ChildBuilder, title: &str, ui_theme: &UITheme) {
    parent.spawn(TextBundle::from_section(
        title,
        TextStyle {
            font_size: ui_theme.typography.heading_medium,
            color: ui_theme.colors.text_primary,
            ..default()
        },
    ));

    // Spacing
    parent.spawn(NodeBundle {
        style: Style {
            height: Val::Px(ui_theme.spacing.sm),
            ..default()
        },
        ..default()
    });
}

/// Create current phase information section
fn create_phase_info_section(parent: &mut ChildBuilder, ui_theme: &UITheme) {
    parent.spawn((
        TextBundle::from_section(
            "Loading...",
            TextStyle {
                font_size: ui_theme.typography.body_large,
                color: ui_theme.colors.text_primary,
                ..default()
            },
        ),
        Name::new("phase_name".to_string()),
    ));

    parent.spawn((
        TextBundle::from_section(
            "Loading phase information...",
            TextStyle {
                font_size: ui_theme.typography.body_small,
                color: ui_theme.colors.text_secondary,
                ..default()
            },
        ),
        Name::new("phase_description".to_string()),
    ));

    parent.spawn((
        TextBundle::from_section(
            "Day 0 in Phase",
            TextStyle {
                font_size: ui_theme.typography.body_small,
                color: ui_theme.colors.text_secondary,
                ..default()
            },
        ),
        Name::new("phase_day".to_string()),
    ));

    // Spacing
    parent.spawn(NodeBundle {
        style: Style {
            height: Val::Px(ui_theme.spacing.md),
            ..default()
        },
        ..default()
    });
}

/// Create overall progress section with main progress bar
fn create_overall_progress_section(parent: &mut ChildBuilder, ui_theme: &UITheme) {
    parent.spawn((
        TextBundle::from_section(
            "Overall Progress: 0%",
            TextStyle {
                font_size: ui_theme.typography.body_medium,
                color: ui_theme.colors.text_primary,
                ..default()
            },
        ),
        Name::new("overall_progress".to_string()),
    ));

    // Progress bar container
    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Px(20.0),
            margin: UiRect::vertical(Val::Px(ui_theme.spacing.xs)),
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        },
        background_color: ui_theme.colors.surface_secondary.into(),
        border_color: ui_theme.colors.border_primary.into(),
        ..default()
    }).with_children(|parent| {
        // Progress bar fill
        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(0.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: Color::srgb(0.2, 0.8, 0.2).into(), // Green
                ..default()
            },
            ProgressBar,
        ));
    });

    // Spacing
    parent.spawn(NodeBundle {
        style: Style {
            height: Val::Px(ui_theme.spacing.md),
            ..default()
        },
        ..default()
    });
}

/// Create individual progress indicators section
fn create_individual_progress_section(parent: &mut ChildBuilder, ui_theme: &UITheme) {
    let progress_items = [
        ("Time", "time_progress", Color::srgb(0.2, 0.6, 0.8)),
        ("Population", "population_progress", Color::srgb(0.8, 0.6, 0.2)),
        ("Complexity", "complexity_progress", Color::srgb(0.6, 0.2, 0.8)),
        ("Stability", "stability_progress", Color::srgb(0.8, 0.2, 0.4)),
    ];

    for (label, name_id, color) in progress_items.iter() {
        // Progress label
        parent.spawn((
            TextBundle::from_section(
                format!("{}: 0%", label),
                TextStyle {
                    font_size: ui_theme.typography.body_small,
                    color: ui_theme.colors.text_primary,
                    ..default()
                },
            ),
            Name::new(name_id.to_string()),
        ));

        // Mini progress bar container
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(8.0),
                margin: UiRect {
                    top: Val::Px(2.0),
                    bottom: Val::Px(ui_theme.spacing.xs),
                    ..default()
                },
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            background_color: ui_theme.colors.surface_secondary.into(),
            border_color: ui_theme.colors.border_secondary.into(),
            ..default()
        }).with_children(|parent| {
            // Mini progress bar fill
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(0.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: (*color).into(),
                    ..default()
                },
                ProgressBar,
            ));
        });
    }

    // Spacing
    parent.spawn(NodeBundle {
        style: Style {
            height: Val::Px(ui_theme.spacing.md),
            ..default()
        },
        ..default()
    });
}

/// Create phase timeline section
fn create_phase_timeline_section(parent: &mut ChildBuilder, ui_theme: &UITheme) {
    parent.spawn(TextBundle::from_section(
        "Phase Timeline",
        TextStyle {
            font_size: ui_theme.typography.body_medium,
            color: ui_theme.colors.text_primary,
            ..default()
        },
    ));

    // Timeline container
    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Px(30.0),
            margin: UiRect::vertical(Val::Px(ui_theme.spacing.xs)),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    }).with_children(|parent| {
        let phases = [
            ("Q", "Queen's Founding"),
            ("F", "First Workers"),
            ("E", "Colony Expansion"),
            ("M", "Mature Colony"),
        ];

        for (short_name, _full_name) in phases.iter() {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(25.0),
                        height: Val::Px(25.0),
                        border: UiRect::all(Val::Px(2.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: ui_theme.colors.surface_secondary.into(),
                    border_color: ui_theme.colors.border_primary.into(),
                    ..default()
                },
                PhaseTimelineElement,
            )).with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    *short_name,
                    TextStyle {
                        font_size: ui_theme.typography.body_small,
                        color: ui_theme.colors.text_primary,
                        ..default()
                    },
                ));
            });
        }
    });

    // Spacing
    parent.spawn(NodeBundle {
        style: Style {
            height: Val::Px(ui_theme.spacing.md),
            ..default()
        },
        ..default()
    });
}

/// Create next phase preview section
fn create_next_phase_section(parent: &mut ChildBuilder, ui_theme: &UITheme) {
    parent.spawn((
        TextBundle::from_section(
            "Next: Loading...",
            TextStyle {
                font_size: ui_theme.typography.body_medium,
                color: ui_theme.colors.text_secondary,
                ..default()
            },
        ),
        Name::new("next_phase".to_string()),
    ));
}
