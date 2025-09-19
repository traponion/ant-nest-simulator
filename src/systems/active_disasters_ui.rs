use crate::components::{DisasterState, DisasterType};
use bevy::prelude::*;

/// UI components for active disasters display
#[derive(Component)]
pub struct ActiveDisastersUI;

#[derive(Component)]
pub struct DisasterListDisplay;

/// Setup active disasters UI
pub fn setup_active_disasters_ui(mut commands: Commands, existing_ui: Query<Entity, With<crate::systems::time_control::TimeControlUI>>) {
    // Find existing UI root to add disasters UI as a sibling
    if let Ok(ui_root) = existing_ui.get_single() {
        commands.entity(ui_root).with_children(|parent| {
            // Add some spacing
            parent.spawn(NodeBundle {
                style: Style {
                    height: Val::Px(20.0),
                    ..default()
                },
                ..default()
            });

            // Active disasters section
            parent.spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                },
                ActiveDisastersUI,
            )).with_children(|disasters_parent| {
                // Header
                disasters_parent.spawn(TextBundle::from_section(
                    "Active Disasters:",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::srgb(1.0, 0.8, 0.8), // Light red
                        ..default()
                    },
                ));

                // Disaster list (will be updated dynamically)
                disasters_parent.spawn((
                    TextBundle::from_section(
                        "None",
                        TextStyle {
                            font_size: 16.0,
                            color: Color::srgb(0.8, 0.8, 0.8),
                            ..default()
                        },
                    ),
                    DisasterListDisplay,
                ));
            });
        });
    } else {
        // Fallback: create standalone UI if time control UI not found
        create_standalone_disasters_ui(&mut commands);
    }
}

/// Create standalone disasters UI (fallback)
fn create_standalone_disasters_ui(commands: &mut Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                top: Val::Px(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            background_color: Color::srgba(0.0, 0.0, 0.0, 0.7).into(),
            ..default()
        })
        .with_children(|parent| {
            // Header
            parent.spawn(TextBundle::from_section(
                "Active Disasters:",
                TextStyle {
                    font_size: 20.0,
                    color: Color::srgb(1.0, 0.8, 0.8), // Light red
                    ..default()
                },
            ));

            // Disaster list
            parent.spawn((
                TextBundle::from_section(
                    "None",
                    TextStyle {
                        font_size: 16.0,
                        color: Color::srgb(0.8, 0.8, 0.8),
                        ..default()
                    },
                ),
                DisasterListDisplay,
            ));
        })
        .insert(ActiveDisastersUI);
}

/// Update active disasters display
pub fn update_active_disasters_display_system(
    disaster_state: Res<DisasterState>,
    mut disaster_display_query: Query<&mut Text, With<DisasterListDisplay>>,
) {
    if let Ok(mut text) = disaster_display_query.get_single_mut() {
        if disaster_state.active_disasters.is_empty() {
            text.sections[0].value = "None".to_string();
            text.sections[0].style.color = Color::srgb(0.8, 0.8, 0.8);
        } else {
            let mut display_text = String::new();

            for (disaster_type, remaining_time) in &disaster_state.active_disasters {
                let disaster_name = format_disaster_name(disaster_type);
                let color_emoji = get_disaster_emoji(disaster_type);

                if !display_text.is_empty() {
                    display_text.push('\n');
                }

                display_text.push_str(&format!(
                    "{} {}: {:.1}s",
                    color_emoji,
                    disaster_name,
                    remaining_time
                ));
            }

            text.sections[0].value = display_text;
            text.sections[0].style.color = Color::srgb(1.0, 0.9, 0.7); // Warm white for active disasters
        }
    }
}

/// Format disaster type name for display
fn format_disaster_name(disaster_type: &DisasterType) -> &'static str {
    match disaster_type {
        DisasterType::Rain => "Rain",
        DisasterType::Drought => "Drought",
        DisasterType::ColdSnap => "Cold Snap",
        DisasterType::InvasiveSpecies => "Invasive Species",
    }
}

/// Get emoji/symbol for disaster type
fn get_disaster_emoji(disaster_type: &DisasterType) -> &'static str {
    match disaster_type {
        DisasterType::Rain => "🌧",
        DisasterType::Drought => "🌵",
        DisasterType::ColdSnap => "❄️",
        DisasterType::InvasiveSpecies => "🐛",
    }
}