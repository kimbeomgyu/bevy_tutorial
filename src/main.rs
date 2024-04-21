mod camera;
mod debug;
mod movement;
mod spaceship;

use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 750.0,
        })
        .add_plugins(spaceship::SpaceshipPlugin)
        .add_plugins(movement::MovementPlugin)
        .add_plugins(debug::DebugPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(DefaultPlugins)
        .run();
}
