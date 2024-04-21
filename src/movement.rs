use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_position);
    }
}

fn update_position(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut position, velocity) in query.iter_mut() {
        position.translation += velocity.value * time.delta_seconds();
    }
}
