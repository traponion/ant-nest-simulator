use crate::components::{
    Ant, EntityCountText, Food, FpsText, FrameTimeText, PerformanceMetrics, PerformancePanel, Soil,
    SpatialGrid, SpatialStatsText,
};
use bevy::prelude::*;

/// System to set up the performance monitoring UI panel
pub fn setup_performance_monitoring_ui(mut commands: Commands) {
    // Create performance monitoring panel (top-left corner)
    let panel_entity = commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                top: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            background_color: Color::srgba(0.0, 0.0, 0.0, 0.8).into(),
            border_radius: BorderRadius::all(Val::Px(5.0)),
            ..default()
        })
        .insert(PerformancePanel)
        .id();

    // Title text
    let title_entity = commands
        .spawn(TextBundle::from_section(
            "Performance Metrics",
            TextStyle {
                font_size: 16.0,
                color: Color::WHITE,
                ..default()
            },
        ))
        .id();

    // FPS text
    let fps_entity = commands
        .spawn(TextBundle::from_section(
            "FPS: 0",
            TextStyle {
                font_size: 14.0,
                color: Color::srgb(0.0, 1.0, 0.0), // Green
                ..default()
            },
        ))
        .insert(FpsText)
        .id();

    // Frame time text
    let frame_time_entity = commands
        .spawn(TextBundle::from_section(
            "Frame Time: 0.0ms",
            TextStyle {
                font_size: 14.0,
                color: Color::srgb(1.0, 1.0, 0.0), // Yellow
                ..default()
            },
        ))
        .insert(FrameTimeText)
        .id();

    // Entity count text
    let entity_count_entity = commands
        .spawn(TextBundle::from_section(
            "Entities: Ants: 0, Food: 0, Soil: 0",
            TextStyle {
                font_size: 14.0,
                color: Color::srgb(0.0, 0.8, 1.0), // Cyan
                ..default()
            },
        ))
        .insert(EntityCountText)
        .id();

    // Spatial grid stats text
    let spatial_stats_entity = commands
        .spawn(TextBundle::from_section(
            "Spatial Grid: 0 cells, 0.0 avg/cell",
            TextStyle {
                font_size: 14.0,
                color: Color::srgb(1.0, 0.5, 0.0), // Orange
                ..default()
            },
        ))
        .insert(SpatialStatsText)
        .id();

    // Add all text entities as children of the panel
    commands.entity(panel_entity).push_children(&[
        title_entity,
        fps_entity,
        frame_time_entity,
        entity_count_entity,
        spatial_stats_entity,
    ]);

    info!("Performance monitoring UI panel created");
}

/// System to collect performance metrics and update the resource
pub fn collect_performance_metrics(
    time: Res<Time>,
    mut performance_metrics: ResMut<PerformanceMetrics>,
    spatial_grid: Res<SpatialGrid>,
    ant_query: Query<Entity, With<Ant>>,
    food_query: Query<Entity, With<Food>>,
    soil_query: Query<Entity, With<Soil>>,
) {
    // Add current frame time to metrics
    performance_metrics.add_frame_time(time.delta_seconds());

    // Update metrics periodically (every 0.1 seconds)
    if performance_metrics
        .update_timer
        .tick(time.delta())
        .just_finished()
    {
        // Count entities
        let ant_count = ant_query.iter().count();
        let food_count = food_query.iter().count();
        let soil_count = soil_query.iter().count();

        performance_metrics.update_entity_counts(ant_count, food_count, soil_count);

        // Update spatial grid statistics
        let occupied_cells = spatial_grid.grid.len();
        let total_entities_in_grid = spatial_grid.grid.values().map(|v| v.len()).sum::<usize>();

        performance_metrics.update_spatial_stats(occupied_cells, total_entities_in_grid);
    }
}

/// System to update the performance monitoring UI display
pub fn update_performance_monitoring_ui(
    performance_metrics: Res<PerformanceMetrics>,
    mut fps_query: Query<
        &mut Text,
        (
            With<FpsText>,
            Without<FrameTimeText>,
            Without<EntityCountText>,
            Without<SpatialStatsText>,
        ),
    >,
    mut frame_time_query: Query<
        &mut Text,
        (
            With<FrameTimeText>,
            Without<FpsText>,
            Without<EntityCountText>,
            Without<SpatialStatsText>,
        ),
    >,
    mut entity_count_query: Query<
        &mut Text,
        (
            With<EntityCountText>,
            Without<FpsText>,
            Without<FrameTimeText>,
            Without<SpatialStatsText>,
        ),
    >,
    mut spatial_stats_query: Query<
        &mut Text,
        (
            With<SpatialStatsText>,
            Without<FpsText>,
            Without<FrameTimeText>,
            Without<EntityCountText>,
        ),
    >,
) {
    // Update FPS text
    for mut text in fps_query.iter_mut() {
        text.sections[0].value = format!("FPS: {:.0}", performance_metrics.fps);

        // Color code based on FPS (Green >= 60, Yellow >= 30, Red < 30)
        text.sections[0].style.color = if performance_metrics.fps >= 60.0 {
            Color::srgb(0.0, 1.0, 0.0) // Green
        } else if performance_metrics.fps >= 30.0 {
            Color::srgb(1.0, 1.0, 0.0) // Yellow
        } else {
            Color::srgb(1.0, 0.0, 0.0) // Red
        };
    }

    // Update frame time text
    for mut text in frame_time_query.iter_mut() {
        text.sections[0].value = format!("Frame Time: {:.1}ms", performance_metrics.frame_time_ms);

        // Color code based on frame time (Green <= 16.7ms, Yellow <= 33.3ms, Red > 33.3ms)
        text.sections[0].style.color = if performance_metrics.frame_time_ms <= 16.7 {
            Color::srgb(0.0, 1.0, 0.0) // Green (60+ FPS)
        } else if performance_metrics.frame_time_ms <= 33.3 {
            Color::srgb(1.0, 1.0, 0.0) // Yellow (30+ FPS)
        } else {
            Color::srgb(1.0, 0.0, 0.0) // Red (<30 FPS)
        };
    }

    // Update entity count text
    for mut text in entity_count_query.iter_mut() {
        text.sections[0].value = format!(
            "Entities: Ants: {}, Food: {}, Soil: {}",
            performance_metrics.ant_count,
            performance_metrics.food_count,
            performance_metrics.soil_count
        );
    }

    // Update spatial grid stats text
    for mut text in spatial_stats_query.iter_mut() {
        text.sections[0].value = format!(
            "Spatial Grid: {} cells, {:.1} avg/cell",
            performance_metrics.spatial_grid_cells, performance_metrics.avg_entities_per_cell
        );
    }
}

/// System to toggle performance monitoring panel visibility with F3 key
pub fn toggle_performance_monitoring_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    performance_panel_query: Query<Entity, With<PerformancePanel>>,
    mut visibility_query: Query<&mut Visibility, With<PerformancePanel>>,
) {
    if keyboard_input.just_pressed(KeyCode::F3) {
        // Toggle visibility of existing panel
        for mut visibility in visibility_query.iter_mut() {
            *visibility = match *visibility {
                Visibility::Visible => {
                    info!("Performance monitoring panel hidden");
                    Visibility::Hidden
                }
                Visibility::Hidden => {
                    info!("Performance monitoring panel shown");
                    Visibility::Visible
                }
                Visibility::Inherited => Visibility::Hidden,
            };
        }

        // If no panel exists, create one
        if performance_panel_query.is_empty() {
            info!("Creating performance monitoring panel");
            // This would need to call the setup function, but for simplicity
            // we assume the panel is created at startup
        }
    }
}
