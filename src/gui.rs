use crate::Parameters;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

// egui::widgets::color_picker::color_edit_button_rgb

pub fn menu(
    mut contexts: EguiContexts,
    mut parameters: ResMut<Parameters>,
    mut pending_parameters: Local<Parameters>,
) {
    if parameters.is_added() {
        *pending_parameters = *parameters;
    }
    egui::Window::new("Configuration...").show(contexts.ctx_mut(), |ui| {
        ui.add(egui::Slider::new(&mut pending_parameters.y_bias, 0.0..=7.0).text("Y-bias"));
        ui.add(
            egui::Slider::new(&mut pending_parameters.max_children, 1..=10).text("Max Children"),
        );
        ui.add(egui::Slider::new(&mut pending_parameters.max_depth, 1..=10).text("Max Depth"));

        let mut color = [0.0, 1.0, 0.0];
        ui.horizontal(|ui| {
            ui.label("Start Color:");
            egui::color_picker::color_edit_button_rgb(ui, &mut color);
        });
        let mut color2 = [1.0, 0.0, 0.0];
        ui.horizontal(|ui| {
            ui.label("End Color:");
            egui::color_picker::color_edit_button_rgb(ui, &mut color2);
        });
        // -
        if ui.add(egui::Button::new("Apply")).clicked() {
            *parameters = *pending_parameters;
        }
        if ui.add(egui::Button::new("Reset")).clicked() {
            *pending_parameters = *parameters;
        }
    });
}
