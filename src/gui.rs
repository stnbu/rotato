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
        ui.label("\"b\" + mouse = rotate camera [b]oom about origin");
        ui.label("\"g\" + mouse = [g]imbal camera (look around)");
        ui.label("SPACE = (un)pause rotation ");
        ui.add(egui::Separator::default());
        // -
        ui.add(egui::Slider::new(&mut pending_parameters.y_bias, 0.0..=7.0).text("Y-bias"));
        ui.add(
            egui::Slider::new(&mut pending_parameters.max_children, 1..=10).text("Max Children"),
        );
        ui.add(egui::Slider::new(&mut pending_parameters.max_depth, 1..=10).text("Max Depth"));

        // Colors: how do they work?

        ui.horizontal(|ui| {
            let [r, g, b, a] = pending_parameters.color_endpoints.0.as_rgba_f32();
            let mut rgba = egui::Rgba::from_rgba_unmultiplied(r, g, b, a);
            ui.label("Start Color:");
            egui::color_picker::color_edit_button_rgba(
                ui,
                &mut rgba,
                egui::color_picker::Alpha::BlendOrAdditive,
            );
            let [r, g, b, a] = rgba.to_array();
            pending_parameters.color_endpoints.0 = Color::rgba(r, g, b, a);
        });

        ui.horizontal(|ui| {
            let [r, g, b, a] = pending_parameters.color_endpoints.1.as_rgba_f32();
            let mut rgba = egui::Rgba::from_rgba_unmultiplied(r, g, b, a);
            ui.label("End Color:");
            egui::color_picker::color_edit_button_rgba(
                ui,
                &mut rgba,
                egui::color_picker::Alpha::BlendOrAdditive,
            );
            let [r, g, b, a] = rgba.to_array();
            pending_parameters.color_endpoints.1 = Color::rgba(r, g, b, a);
        });
        ui.add(egui::Separator::default());
        if ui.add(egui::Button::new("Apply")).clicked() {
            *parameters = *pending_parameters;
        }
        if ui.add(egui::Button::new("Reset")).clicked() {
            *pending_parameters = *parameters;
        }
    });
}
