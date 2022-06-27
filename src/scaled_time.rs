//! Global time and delta time, with consideration to time scale
use bevy::prelude::*;

use crate::settings::SoftSettings;

#[derive(Default)]
pub struct ScaledTime {
    pub delta: f32,
    pub total: f64,
}

pub struct ScaledTimePlugin;

impl Plugin for ScaledTimePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ScaledTime>()
            .add_system_to_stage(CoreStage::PreUpdate, update);
    }
}

fn update(mut scaled_time: ResMut<ScaledTime>, time: Res<Time>, settings: Res<SoftSettings>) {
    scaled_time.delta = time.delta_seconds() * settings.time_scale;
    scaled_time.total += scaled_time.delta as f64;
}
