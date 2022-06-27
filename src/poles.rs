//! Pole spawning and despawning

use bevy::{ecs::schedule::ShouldRun, prelude::*};
use itertools::Itertools;

use crate::{
    settings::{HardReset, HardSettings},
    wave::AngularVelocity,
};

const TOTAL_HEIGHT: f32 = 10.;

#[derive(Clone, Copy)]
pub enum Neighbour {
    Empty,
    Pole(Entity),
}

#[derive(Component)]
pub struct Pole {
    pub below: Neighbour,
    pub above: Neighbour,
}

pub fn despawn(mut commands: Commands, query: Query<Entity, With<Pole>>) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
}

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut settings: ResMut<HardSettings>,
    mut hard_reset: ResMut<HardReset>,
) {
    let side = TOTAL_HEIGHT / (settings.amount as f32 + (settings.amount as f32 - 1.) / 2.);
    settings.distance = side * 1.5;
    let length = settings.length * side;
    let mesh_handle = meshes.add(shape::Box::new(length, side, side).into());
    let material_handle = materials.add(Color::rgb_u8(0xFF, 0xB7, 0x2B).into());

    let commands = &mut commands;

    #[allow(clippy::needless_collect)]
    let poles = [Neighbour::Empty]
        .into_iter()
        .chain((0..settings.amount).map(|i| {
            let y = i as f32 * side * 1.5 + (side - TOTAL_HEIGHT) / 2.;
            let id = commands
                .spawn_bundle(PbrBundle {
                    mesh: mesh_handle.clone(),
                    material: material_handle.clone(),
                    transform: Transform::from_translation(Vec3::new(0., y, 0.)),
                    ..Default::default()
                })
                .id();
            Neighbour::Pole(id)
        }))
        .chain([Neighbour::Empty].into_iter())
        .collect::<Vec<_>>();

    poles
        .into_iter()
        .tuple_windows::<(_, _, _)>()
        .for_each(|(below, current, above)| {
            if let Neighbour::Pole(entity) = current {
                let pole = Pole { below, above };
                commands
                    .entity(entity)
                    .insert(pole)
                    .insert(AngularVelocity(0.));
            }
        });

    hard_reset.0 = false;
}

fn hard_reset(hard_reset: Res<HardReset>) -> ShouldRun {
    match hard_reset.0 {
        true => ShouldRun::Yes,
        false => ShouldRun::No,
    }
}

pub struct PolePlugin;

impl Plugin for PolePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(
            CoreStage::PreUpdate,
            despawn.chain(spawn).with_run_criteria(hard_reset),
        );
    }
}
