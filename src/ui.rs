use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_egui::{egui, EguiContext};
use crate::marker::spawn_marker;


pub fn ui_system(
    mut egui_context: ResMut<EguiContext>,
    mut commands: Commands,
    mut windows: ResMut<Windows>
) {
    egui::Window::new("Scapper").show(egui_context.ctx_mut(), |ui| {

        if ui.button("Toggle Fullscreen").clicked() {
            if windows.primary().mode() != WindowMode::Fullscreen {
                windows.primary_mut().set_mode(WindowMode::Fullscreen);
            } else {
                windows.primary_mut().set_mode(WindowMode::Windowed);
            }
        }

        if ui.button("Add marker").clicked() {
            spawn_marker(commands)
        }
    });
}