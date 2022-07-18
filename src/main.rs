use bevy::prelude::*;

mod dino;
use dino::DinoPlugin;

const WINDOW_WIDTH: f32 = 640.;
const WINDOW_HEIGHT: f32 = 320.;
const WINDOW_BACKGROUND_COLOR: Color = Color::WHITE;

const JUMP_VEL: f32 = 10.;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Dino Runner".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(WINDOW_BACKGROUND_COLOR))
        .add_startup_system_to_stage(StartupStage::PreStartup, spawn_camera_and_ui)
        .add_plugin(DinoPlugin)
        .run();
}

fn spawn_camera_and_ui (
    mut commands: Commands,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}