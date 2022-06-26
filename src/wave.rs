use bevy::prelude::*;

use crate::{
    pins::{Pole, PoleRelations},
    settings::SoftSettings,
};

fn quat_around_y(q: Quat) -> f32 {
    (2. * (q.x * q.z + q.w * q.y)).atan2(q.w * q.w - q.x * q.x - q.y * q.y + q.z * q.z)
}

#[derive(Component)]
pub struct Velocity(pub f32);

pub fn update_velocity(
    mut query: Query<(&Transform, &mut Velocity), With<Pole>>,
    relations: Res<PoleRelations>,
    settings: Res<SoftSettings>,
) {
    // Get angles around Y axis
    let mut angles = relations.0.iter().map(|e| {
        if let Ok((t, _)) = query.get(*e) {
            quat_around_y(t.rotation)
        } else {
            0.
        }
    });
    // Prepend with bottom anchor
    let first = angles.next().unwrap_or(0.);
    let mut angles = if settings.anchor_bottom {
        [0., first].into_iter()
    } else {
        [first, first].into_iter()
    }
    .chain(angles)
    .collect::<Vec<_>>();
    // Postpend with top anchor
    if settings.anchor_top {
        angles.push(0.);
    } else {
        angles.push(*angles.last().unwrap_or(&0.));
    }
    // Calculate forces
    relations
        .0
        .iter()
        .zip(angles.windows(3).map(|t| t[0] - 2. * t[1] + t[2]))
        .for_each(|(e, f)| {
            if let Ok((_, mut v)) = query.get_mut(*e) {
                v.0 += f * 0.01;
                v.0 %= std::f32::consts::TAU;
            }
        });
}

pub fn update_translation(mut query: Query<(&mut Transform, &Velocity), With<Pole>>) {
    query.iter_mut().for_each(|(mut t, v)| {
        let r = Quat::from_rotation_y(v.0);
        t.rotation *= r;
    });
}
