use bevy::prelude::*;
use rand::prelude::*;
use crate::components::SoilCell;

/// System for environmental simulation
pub fn environmental_update_system(
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