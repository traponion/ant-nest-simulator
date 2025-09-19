use bevy::prelude::*;

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
            .add_systems(Startup, setup_world)
            .add_systems(Update, (
                // Placeholder systems - will be implemented later
            ));
    }
}

// Basic camera setup for 2D pixel art view
fn setup_world(mut commands: Commands) {
    // Spawn 2D camera for side-view ant farm observation
    commands.spawn(Camera2dBundle::default());

    info!("Ant Nest Simulator initialized - ready for development!");
}