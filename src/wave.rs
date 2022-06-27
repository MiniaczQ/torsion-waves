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

/// Wraps values to [-pi; pi]
fn wrap(a: f32) -> f32 {
    (a + std::f32::consts::PI).rem_euclid(std::f32::consts::TAU) - std::f32::consts::PI
}

fn apply_forces(
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
    // Runs force calculations
    // Updates angular velocities
    for (edge, neighbour_angles, mut angular_velocity) in triplets {
        let w = wave_force(neighbour_angles, &soft_settings, &hard_settings);
        let d = damping_force(angular_velocity.0, &soft_settings);
        let a = agitation_force(edge, &soft_settings, time.total);
        let force = w + d + a;
        angular_velocity.0 += force * time.delta / soft_settings.moment_of_inertia;
    }
}

/// Calculates the wave-based force from 3 neighbouring angles and stiffness `k`
fn wave_force(
    (below, current, above): (f32, f32, f32),
    soft_settings: &SoftSettings,
    hard_settings: &HardSettings,
) -> f32 {
    let dda = wrap(above - 2.0 * current + below);
    dda * soft_settings.stiffness / (hard_settings.distance * hard_settings.distance)
}

/// Calculates damping force based on velocity and damping coefficient
fn damping_force(velocity: f32, settings: &SoftSettings) -> f32 {
    velocity * settings.damping
}

/// Calculates agitation forces on edge-most (top and bottom) poles
/// The force is currently omega * force (doesn't make much sense)
fn agitation_force((bottom, top): (bool, bool), settings: &SoftSettings, time: f64) -> f32 {
    let top = if top {
        (time * settings.top_frequency as f64 * std::f64::consts::TAU).sin() as f32
            * settings.top_force
    } else {
        0.0
    };
    let bottom = if bottom {
        (time * settings.bottom_frequency as f64 * std::f64::consts::TAU).sin() as f32
            * settings.bottom_force
    } else {
        0.0
    };
    top + bottom
}

/// Applies velocities for this frame
fn apply_velocities(
    mut query: Query<(&mut Transform, &AngularVelocity), With<Pole>>,
    time: Res<ScaledTime>,
) {
    query.iter_mut().for_each(|(mut transform, velocity)| {
        let angle_delta = velocity.0 * time.delta;
        let quat_delta = Quat::from_rotation_y(angle_delta);
        transform.rotation *= quat_delta;
    });
}

/// Adds functionality to the main application
pub struct WavePlugin;

impl Plugin for WavePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(collect_angles.chain(apply_forces).label("apply-forces"))
            .add_system(apply_velocities.after("apply-forces"));
    }
}
