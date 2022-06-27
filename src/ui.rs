//! UI related stuff

use bevy::{ecs::schedule::ShouldRun, prelude::*};
use bevy_egui::{egui, EguiContext, EguiPlugin};

use crate::settings::{HardReset, HardSettings, SoftSettings};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(EguiPlugin)
            .init_resource::<HelpMessage>()
            .add_system(settings_ui.with_run_criteria(cursor_unlocked))
            .add_system(toggle_help)
            .add_system(help_ui);
    }
}

fn settings_ui(
    mut soft_settings: ResMut<SoftSettings>,
    mut hard_settings: ResMut<HardSettings>,
    mut hard_reset: ResMut<HardReset>,
    mut egui_context: ResMut<EguiContext>,
) {
    egui::Window::new("Settings")
        .default_pos([10.0, 10.0])
        .resizable(false)
        .show(egui_context.ctx_mut(), |ui| {
            ui.heading("General");
            ui.add(
                egui::Slider::new(&mut soft_settings.time_scale, 0.0..=10.0)
                    .clamp_to_range(true)
                    .text("Time scale"),
            );

            ui.add(
                egui::Slider::new(&mut soft_settings.stiffness, 0.0..=1.0)
                    .clamp_to_range(false)
                    .prefix("k = ")
                    .suffix(" N / m")
                    .text("Stiffness"),
            );
            if soft_settings.stiffness < 0.0 {
                soft_settings.stiffness = 0.0;
            }

            ui.add(
                egui::Slider::new(&mut soft_settings.moment_of_inertia, 0.01..=1.0)
                    .clamp_to_range(false)
                    .prefix("I = ")
                    .suffix(" kg * m^2")
                    .text("Moment of inertia"),
            );
            if soft_settings.moment_of_inertia < 0.01 {
                soft_settings.moment_of_inertia = 0.01;
            }

            let mut speed = (soft_settings.stiffness / soft_settings.moment_of_inertia).sqrt();
            ui.horizontal(|ui| {
                ui.add(
                    egui::DragValue::new(&mut speed)
                        .prefix("v = ")
                        .suffix(" m / s"),
                );
                ui.label("Wave speed (derived)");
            });

            ui.add(
                egui::Slider::new(&mut soft_settings.damping, -1.0..=0.0)
                    .clamp_to_range(false)
                    .prefix("α = ")
                    .suffix(" [...]")
                    .text("Damping"),
            );
            if soft_settings.damping > 0.0 {
                soft_settings.damping = 0.0;
            }

            ui.separator();
            ui.heading("Anchors");
            ui.checkbox(&mut soft_settings.anchor_top, "Top");
            ui.checkbox(&mut soft_settings.anchor_bottom, "Bottom");

            ui.separator();
            ui.heading("Top agitation");
            ui.add(
                egui::Slider::new(&mut soft_settings.top_frequency, -0.5..=0.5)
                    .clamp_to_range(false)
                    .prefix("f = ")
                    .suffix(" [...]")
                    .text("Frequency"),
            );
            ui.add(
                egui::Slider::new(&mut soft_settings.top_force, -1.0..=1.0)
                    .clamp_to_range(false)
                    .prefix("M = ")
                    .suffix(" [...]")
                    .text("Moment of force"),
            );

            ui.separator();
            ui.heading("Bottom agitation");
            ui.add(
                egui::Slider::new(&mut soft_settings.bottom_frequency, -0.5..=0.5)
                    .clamp_to_range(false)
                    .prefix("f = ")
                    .suffix(" [...]")
                    .text("Frequency"),
            );
            ui.add(
                egui::Slider::new(&mut soft_settings.bottom_force, -1.0..=1.0)
                    .clamp_to_range(false)
                    .prefix("M = ")
                    .suffix(" [...]")
                    .text("Moment of force"),
            );

            ui.separator();
            ui.heading("Requiring reset");
            ui.add(
                egui::Slider::new(&mut hard_settings.amount, 1..=64)
                    .clamp_to_range(false)
                    .text("Amount of poles"),
            );
            if hard_settings.amount < 1 {
                hard_settings.amount = 1;
            }
            ui.add(
                egui::Slider::new(&mut hard_settings.length, 2.0..=16.0)
                    .clamp_to_range(false)
                    .text("Length of poles"),
            );
            if hard_settings.length < 2.0 {
                hard_settings.length = 2.0;
            }

            if ui.button("Reset simulation").clicked() {
                hard_reset.0 = true;
            }
        });
}

struct HelpMessage(bool);

impl Default for HelpMessage {
    fn default() -> Self {
        Self(true)
    }
}

fn toggle_help(input: Res<Input<KeyCode>>, mut help_message: ResMut<HelpMessage>) {
    if input.just_pressed(KeyCode::H) {
        help_message.0 = !help_message.0;
    }
}

fn help_ui(mut egui_context: ResMut<EguiContext>, help_message: ResMut<HelpMessage>) {
    if help_message.0 {
        egui::Area::new("help")
            .anchor(egui::Align2::RIGHT_BOTTOM, [-10.0, -10.0])
            .movable(false)
            .show(egui_context.ctx_mut(), |ui| {
                let color = egui::Color32::BLACK;
                let bg_color = egui::Color32::from_rgba_unmultiplied(255, 255, 255, 127);
                ui.label(
                    egui::RichText::new("Toggle between settings and movement using `Q`")
                        .color(color)
                        .background_color(bg_color),
                );
                ui.label(
                    egui::RichText::new("Move with WSAD, `Shift` and `Spacebar`.")
                        .color(color)
                        .background_color(bg_color),
                );
                ui.label(
                    egui::RichText::new("Adjust simulation settings with UI.")
                        .color(color)
                        .background_color(bg_color),
                );
                ui.label(
                    egui::RichText::new("You can escape the slider limits by dragging the value directly.")
                        .color(color)
                        .background_color(bg_color),
                );
                ui.label(
                    egui::RichText::new("Although some limits are still kept to keep the math safe. (no division by 0)")
                        .color(color)
                        .background_color(bg_color),
                );
                ui.label(
                    egui::RichText::new("To close this message press `H`.")
                        .color(color)
                        .background_color(bg_color),
                );
            });
    }
}

fn cursor_unlocked(windows: Res<Windows>) -> ShouldRun {
    match windows.get_primary().unwrap().cursor_locked() {
        true => ShouldRun::No,
        false => ShouldRun::Yes,
    }
}
