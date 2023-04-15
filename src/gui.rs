use crate::Parameters;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

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

        if ui.add(egui::Button::new("Apply")).clicked() {
            *parameters = *pending_parameters;
        }
        if ui.add(egui::Button::new("Reset")).clicked() {
            *pending_parameters = *parameters;
        }
    });
}

/*
use crate::Parameters;
use bevy::{prelude::*, render::camera::Projection, window::PrimaryWindow};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

pub fn gui(mut contexts: EguiContexts, mut occupied_screen_space: ResMut<Parameters>) {
    let ctx = contexts.ctx_mut();

    occupied_screen_space.left = egui::SidePanel::left("left_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
    occupied_screen_space.right = egui::SidePanel::right("right_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
    occupied_screen_space.top = egui::TopBottomPanel::top("top_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .height();
    occupied_screen_space.bottom = egui::TopBottomPanel::bottom("bottom_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .height();
}
*/
