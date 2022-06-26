use bevy::prelude::*;

use crate::{settings::HardSettings, wave::Velocity};

const BOX_COLOR: Color = Color::rgb(1., 1., 1.);
const HEIGHT: f32 = 10.;

#[derive(Default)]
pub struct PoleRelations(pub Vec<Entity>);

#[derive(Component)]
pub struct Pole;

pub fn despawn(
    mut commands: Commands,
    query: Query<Entity, With<Pole>>,
    mut relations: ResMut<PoleRelations>,
) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
    relations.0 = vec![];
}

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    settings: Res<HardSettings>,
    mut relations: ResMut<PoleRelations>,
) {
    let c = settings.n as f32;
    let c = c + (c - 1.) / 2.;
    let a = HEIGHT / c;
    let mesh_handle = meshes.add(shape::Box::new(settings.l, a, a).into());
    let material_handle = materials.add(BOX_COLOR.into());
    let mut rels = vec![];
    for i in 0..settings.n {
        let y = i as f32 * a * 1.5 + a / 2.;
        rels.push(
            commands
                .spawn_bundle(PbrBundle {
                    mesh: mesh_handle.clone(),
                    material: material_handle.clone(),
                    transform: Transform::from_translation(Vec3::new(0., y, 0.))
                        .with_rotation(Quat::from_rotation_y(i as f32 / 30.)),
                    ..Default::default()
                })
                .insert(Pole)
                .insert(Velocity(0.))
                .id(),
        );
    }
    relations.0 = rels;
}
