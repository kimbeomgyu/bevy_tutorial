mod asset_loader;
mod asteroids;
mod camera;
mod collision_detection;
mod debug;
mod despawn;
mod movement;
mod spaceship;

use bevy::prelude::*;

fn main() {
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
        // .add_plugins(debug::DebugPlugin)
        .run();
}
