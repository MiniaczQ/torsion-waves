//! Setting objects

use bevy::prelude::*;

/// Settings that don't require restart
pub struct SoftSettings {
    pub stiffness: f32,
    pub moment_of_inertia: f32,
    pub time_scale: f32,
    pub damping: f32,
    pub anchor_top: bool,
    pub anchor_bottom: bool,

    pub top_frequency: f32,
    pub top_force: f32,

    pub bottom_frequency: f32,
    pub bottom_force: f32,
}

impl Default for SoftSettings {
    fn default() -> Self {
        Self {
            stiffness: 1.0,
            moment_of_inertia: 0.01,
            time_scale: 1.0,
            damping: -0.01,
            anchor_bottom: false,
            anchor_top: false,

            top_frequency: 0.0,
            top_force: 0.0,

            bottom_frequency: 0.0,
            bottom_force: 0.0,
        }
    }
}

/// Settings that require restart
pub struct HardSettings {
    /// Amount of the poles
    pub amount: u32,
    /// Length of the poles
    pub length: f32,
    /// Distance between poles (derived)
    pub distance: f32,
}

impl Default for HardSettings {
    fn default() -> Self {
        Self {
            amount: 32,
            length: 5.0,
            distance: 0.0,
        }
    }
}

/// Signal for reset
pub struct HardReset(pub bool);

impl Default for HardReset {
    fn default() -> Self {
        Self(true)
    }
}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SoftSettings>()
            .init_resource::<HardSettings>()
            .init_resource::<HardReset>();
    }
}
