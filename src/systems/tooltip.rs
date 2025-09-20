use crate::components::{Tooltip, TooltipDisplay, TooltipTrigger, TooltipPosition, UITheme};
use bevy::prelude::*;

/// System to manage tooltip triggers and display timing
pub fn tooltip_trigger_system(
    mut interaction_query: Query<
        (&Interaction, &mut TooltipTrigger, Option<&Tooltip>),
        (Changed<Interaction>, With<Button>),
    >,
    time: Res<Time>,
) {
    for (interaction, mut trigger, tooltip) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                if !trigger.is_hovered {
                    trigger.is_hovered = true;
                    trigger.hover_timer = 0.0;
                }
                trigger.hover_timer += time.delta_seconds();

                // Show tooltip after delay if we have tooltip data
                if trigger.hover_timer >= trigger.show_delay && tooltip.is_some() {
                    // Trigger tooltip display (will be handled by display system)
                }
            }
            Interaction::None => {
                if trigger.is_hovered {
                    trigger.is_hovered = false;
                    trigger.hover_timer = 0.0;
                    // Hide tooltip immediately when not hovering
                }
            }
            Interaction::Pressed => {
                // Hide tooltip when pressed
                trigger.is_hovered = false;
                trigger.hover_timer = 0.0;
            }
        }
    }
}

/// System to display and position tooltips
pub fn tooltip_display_system(
    mut commands: Commands,
    theme: Res<UITheme>,
    trigger_query: Query<(&TooltipTrigger, &Tooltip, &GlobalTransform), With<Button>>,
    existing_tooltips: Query<Entity, With<TooltipDisplay>>,
) {
    // First, remove existing tooltips
    for entity in &existing_tooltips {
        commands.entity(entity).despawn_recursive();
    }

    // Then, create new tooltips for elements that should show them
    for (trigger, tooltip, transform) in &trigger_query {
        if trigger.is_hovered && trigger.hover_timer >= trigger.show_delay {
            spawn_tooltip(&mut commands, &theme, tooltip, transform);
        }
    }
}

/// Helper function to spawn a tooltip entity
fn spawn_tooltip(
    commands: &mut Commands,
    theme: &UITheme,
    tooltip: &Tooltip,
    target_transform: &GlobalTransform,
) {
    let tooltip_text = if let Some(shortcut) = &tooltip.shortcut {
        format!("{}\n\nKeyboard shortcut: {}", tooltip.text, shortcut)
    } else {
        tooltip.text.clone()
    };

    // Calculate position based on target transform and tooltip position preference
    let (x_offset, y_offset) = match tooltip.position {
        TooltipPosition::Below => (0.0, -60.0),
        TooltipPosition::Above => (0.0, 60.0),
        TooltipPosition::Left => (-200.0, 0.0),
        TooltipPosition::Right => (200.0, 0.0),
    };

    let target_translation = target_transform.translation();

    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(target_translation.x + x_offset),
                top: Val::Px(target_translation.y + y_offset),
                max_width: Val::Px(280.0),
                padding: UiRect::all(Val::Px(theme.spacing.md)),
                border: UiRect::all(Val::Px(theme.borders.width_thin)),
                ..default()
            },
            background_color: theme.colors.surface_primary.into(),
            border_color: theme.colors.border_primary.into(),
            border_radius: BorderRadius::all(Val::Px(theme.borders.radius_small)),
            z_index: ZIndex::Global(1000), // Ensure tooltips appear above other UI
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                tooltip_text,
                TextStyle {
                    font_size: theme.typography.body_small,
                    color: theme.colors.text_primary,
                    ..default()
                },
            ));
        })
        .insert(TooltipDisplay);
}

/// System to cleanup tooltips when not needed
pub fn tooltip_cleanup_system(
    mut commands: Commands,
    tooltip_query: Query<Entity, With<TooltipDisplay>>,
    trigger_query: Query<&TooltipTrigger>,
) {
    let any_tooltip_should_show = trigger_query.iter().any(|trigger| {
        trigger.is_hovered && trigger.hover_timer >= trigger.show_delay
    });

    if !any_tooltip_should_show {
        for entity in &tooltip_query {
            commands.entity(entity).despawn_recursive();
        }
    }
}