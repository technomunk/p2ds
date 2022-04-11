//! Physics module. Represents the meat of the project.

use bevy::{
    math::Vec2,
    prelude::{App, Plugin, Query, Transform, Vec3},
};

pub mod coord;
pub mod gravity;
pub mod momentum;

/// Combination of all the physics systems for developer convenience.
#[derive(Debug, Clone, Copy, Default)]
pub struct PhysicsPlugin;

/// Convert physical world positions to default Bevy Transform coordinates
pub fn convert_transform(mut entities: Query<(&coord::Position, &mut Transform)>) {
    for (position, mut transform) in entities.iter_mut() {
        let xy = position.to_range(Vec2::splat(-256.0), Vec2::splat(256.0));
        transform.translation = Vec3::new(xy.x, xy.y, transform.translation.z);
    }
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(gravity::gravity);
        app.add_system(momentum::momentum);
        app.add_system(convert_transform);
    }
}
