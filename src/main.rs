//! **P**hysics **2D** **S**andbox.
//!
//! An interactive playground for exploring real-time 2d physics simulation.

#![warn(clippy::all)]

use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "p2ds".to_string(),
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .run();
}
