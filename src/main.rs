use ant_nest_simulator::AntNestPlugin;
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
