//! Here be physics
use bevy::{prelude::*, utils::HashMap};

use crate::{
    poles::{Neighbour, Pole},
    scaled_time::ScaledTime,
    settings::{HardSettings, SoftSettings},
};

/// Angle around Y axis from rotation quaternion
fn quat_around_y(q: Quat) -> f32 {
    (2. * (q.x * q.z + q.w * q.y)).atan2(q.w * q.w - q.x * q.x - q.y * q.y + q.z * q.z)
}

#[derive(Component)]
pub struct AngularVelocity(pub f32);

/// Collects angles and stores them in a hashmap
/// Helper for collecting neighbour angles
fn collect_angles(query: Query<(Entity, &Transform)>) -> HashMap<Entity, f32> {
    query
        .iter()
        .map(|(e, t)| (e, quat_around_y(t.rotation)))
        .collect()
}

/// Wraps values to `[-pi; pi]`
fn wrap(a: f32) -> f32 {
    (a + std::f32::consts::PI).rem_euclid(std::f32::consts::TAU) - std::f32::consts::PI
}

/// Applies torques for this frame
/// Executed before angular velocities are applied
fn apply_torques(
    In(angles): In<HashMap<Entity, f32>>,
    mut query: Query<(Entity, &Pole, &mut AngularVelocity)>,
    soft_settings: Res<SoftSettings>,
    hard_settings: Res<HardSettings>,
    time: Res<ScaledTime>,
) {
    // Collect all angles
    // Prepend and append with anchor values
    // If anchor is enabled, the angle is 0
    // If anchor is disabled (pole on that end is loose), the angle is the same as pole's
    //
    // With bottom anchored and 4 poles, looks something like this:
    // [0, 0.7, 0.8, 0.9, 1.0, 1.0]
    let triplets = query.iter_mut().map(|(entity, p, angular_velocity)| {
        (
            (
                matches!(p.below, Neighbour::Empty),
                matches!(p.above, Neighbour::Empty),
            ),
            (
                match p.below {
                    Neighbour::Pole(n) => *angles.get(&n).unwrap(),
                    _ => match soft_settings.anchor_bottom {
                        true => 0.0,
                        false => *angles.get(&entity).unwrap(),
                    },
                },
                *angles.get(&entity).unwrap(),
                match p.above {
                    Neighbour::Pole(n) => *angles.get(&n).unwrap(),
                    _ => match soft_settings.anchor_top {
                        true => 0.0,
                        false => *angles.get(&entity).unwrap(),
                    },
                },
            ),
            angular_velocity,
        )
    });

    // Iterates over 3-wide windows of angles
    // Runs torque calculations
    // Updates angular velocities
    //
    // `edge` is a pair of booleans indicating bottom-most or top-most pole
    // `neighbour_angles` is a tuple of 3 angles: angle of pole below, angle of current pole, angle of pole above
    // `angular_velocity` is self-explainatory
    for (edge, neighbour_angles, mut angular_velocity) in triplets {
        let w = wave_torque(neighbour_angles, &soft_settings, &hard_settings);
        let d = damping_torque(angular_velocity.0, &soft_settings);
        let a = agitation_torque(edge, &soft_settings, time.total);
        let total_torque = w + d + a;
        angular_velocity.0 += total_torque * time.delta / soft_settings.moment_of_inertia;
    }
}

/// Calculates the wave-based torque from 3 neighbouring angles, stiffness `k` and distance between poles
fn wave_torque(
    (below, current, above): (f32, f32, f32),
    soft_settings: &SoftSettings,
    hard_settings: &HardSettings,
) -> f32 {
    let dda = wrap(above - 2.0 * current + below) / hard_settings.distance;
    dda * soft_settings.stiffness
}

/// Calculates damping torque based on velocity and damping coefficient
fn damping_torque(velocity: f32, settings: &SoftSettings) -> f32 {
    velocity * settings.damping
}

/// Calculates agitation torque on edge-most (top and bottom) poles
fn agitation_torque((bottom, top): (bool, bool), settings: &SoftSettings, time: f64) -> f32 {
    let top = if top {
        (time * settings.top_frequency as f64 * std::f64::consts::TAU + settings.top_phase as f64)
            .sin() as f32
            * settings.top_force
    } else {
        0.0
    };
    let bottom = if bottom {
        (time * settings.bottom_frequency as f64 * std::f64::consts::TAU
            + settings.bottom_phase as f64)
            .sin() as f32
            * settings.bottom_force
    } else {
        0.0
    };
    top + bottom
}

/// Applies angular velocities for this frame
/// Executed after torques are applied
fn apply_angular_velocities(
    mut query: Query<(&mut Transform, &AngularVelocity), With<Pole>>,
    hard_settings: Res<HardSettings>,
    time: Res<ScaledTime>,
) {
    query.iter_mut().for_each(|(mut transform, velocity)| {
        let angle_delta = velocity.0 * time.delta / hard_settings.distance;
        let quat_delta = Quat::from_rotation_y(angle_delta);
        transform.rotation *= quat_delta;
    });
}

/// Adds functionality to the main application
pub struct WavePlugin;

impl Plugin for WavePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(collect_angles.chain(apply_torques).label("apply-forces"))
            .add_system(apply_angular_velocities.after("apply-forces"));
    }
}
