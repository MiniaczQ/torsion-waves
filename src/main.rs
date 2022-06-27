mod flycam;
mod poles;
mod scaled_time;
mod settings;
mod ui;
mod wave;

use bevy::prelude::*;
use flycam::{FlyCam, FlycamPlugin};
use poles::PolePlugin;
use scaled_time::ScaledTimePlugin;
use settings::SettingsPlugin;
use ui::UIPlugin;
use wave::WavePlugin;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb_u8(0x8C, 0xC0, 0xDE)))
        .add_plugins(DefaultPlugins)
        .add_plugin(FlycamPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(SettingsPlugin)
        .add_plugin(PolePlugin)
        .add_plugin(WavePlugin)
        .add_plugin(ScaledTimePlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0)),
            ..Default::default()
        })
        .insert(FlyCam);

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.5,
    });

    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 7500.0,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(10.0, 20.0, 50.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..Default::default()
        },
        ..Default::default()
    });
}
