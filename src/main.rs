use bevy::{prelude::*, sprite::collide_aabb::collide};

mod dino;
use dino::{DinoPlugin, Dino};

mod desert;
use desert::{DesertPlugin, Obstacle};

const WINDOW_WIDTH: f32 = 640.;
const WINDOW_HEIGHT: f32 = 320.;
const WINDOW_BACKGROUND_COLOR: Color = Color::WHITE;

const DINO_FPS: u8 = 4;

const DINO_X_LOCATION: f32 = -WINDOW_WIDTH / 3.;
const DINO_Y_LOCATION: f32 = -WINDOW_HEIGHT / 2.5;

const JUMP_VEL: f32 = 25.;
const GRAVITY: f32 = 2.;
const TRACK_SPEED: f32 = 5.;

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
        .add_plugin(DesertPlugin)
        .add_system(handle_collisions)
        .run();
}

fn spawn_camera_and_ui(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn handle_collisions(
    dino_query: Query<&Transform, With<Dino>>,
    obstacle_query: Query<&Transform, With<Obstacle>>,
) {
    let dino_transform = dino_query.get_single().unwrap();
    for obstacle_transform in obstacle_query.iter() {
        match collide(
            dino_transform.translation,
            Vec2::new(41., 44.),
            obstacle_transform.translation,
            Vec2::new(24., 49.)) {
                None => {},
                Some(_) => println!("Dino collision!!"),
            }

    }
}