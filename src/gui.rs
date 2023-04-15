use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

pub fn ui_example_system(mut contexts: EguiContexts) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
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
