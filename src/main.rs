mod pins;
mod settings;
mod wave;

use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;
use pins::PoleRelations;
use settings::{HardSettings, SoftSettings};

fn main() {
    App::new()
        .init_resource::<SoftSettings>()
        .init_resource::<HardSettings>()
        .init_resource::<PoleRelations>()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_startup_system(pins::spawn)
        .add_system(wave::update_velocity.label("velocity_update"))
        .add_system(wave::update_translation.after("velocity_update"))
        .run();
}
