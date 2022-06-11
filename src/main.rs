use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy::window::PresentMode;
use bevy_egui::EguiPlugin;
use bevy_mod_picking;
use bevy_mod_picking::{DebugCursorPickingPlugin, DebugEventsPickingPlugin, DefaultPickingPlugins, PickableBundle, PickingCameraBundle};
use bevy_prototype_lyon::prelude::*;

mod grid;
mod marker;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        .insert_resource(WindowDescriptor {
            present_mode: PresentMode::Mailbox, // Reduce input latency
            ..Default::default()
        })

        .add_plugin(EguiPlugin)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(DebugEventsPickingPlugin)
        .add_plugin(DebugCursorPickingPlugin)
        .add_plugin(ShapePlugin)

        .add_startup_system(setup)
        .add_startup_system_set(grid::startup())
        .add_system(ui::ui_system)

        .add_system_set(marker::marker_system_set())

        .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert_bundle(PickingCameraBundle::default())
    ;
}