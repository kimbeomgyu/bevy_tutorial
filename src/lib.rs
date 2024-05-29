mod asset_loader;
mod asteroids;
mod camera;
mod collision_detection;
mod debug;
mod despawn;
mod health;
mod movement;
mod schedule;
mod spaceship;
mod state;

use bevy::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn start() {
    App::new()
        // Bevy built-in plugins
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 750.0,
        })
        .add_plugins(DefaultPlugins)
        // User Custom plugins
        .add_plugins(asset_loader::AssetLoaderPlugin)
        .add_plugins(asteroids::AsteroidsPlugin)
        .add_plugins(spaceship::SpaceshipPlugin)
        .add_plugins(movement::MovementPlugin)
        .add_plugins(collision_detection::CollisionDetectionPlugin)
        .add_plugins(despawn::DespawnPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(schedule::SchedulePlugin)
        .add_plugins(state::StatePlugin)
        // .add_plugins(debug::DebugPlugin)
        .run();
}
