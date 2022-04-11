//! **P**hysics **2D** **S**andbox.
//!
//! An interactive playground for exploring real-time 2d physics simulation.

#![warn(clippy::all)]

use bevy::prelude::*;
use physics::{coord::Position, momentum::Velocity, PhysicsPlugin};
use wasm_bindgen::prelude::*;

pub mod physics;
#[cfg(target_arch = "wasm32")]
mod wasm;

#[wasm_bindgen]
pub fn game() {
    let mut app = App::new();

    // default systems, relevant for all target builds
    app.insert_resource(WindowDescriptor {
        title: "p2ds".to_string(),
        vsync: true,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(PhysicsPlugin)
    .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
    .add_startup_system(setup);

    // target-specific systems
    #[cfg(target_arch = "wasm32")]
    app.add_system(wasm::fullscreen_window);
    #[cfg(not(target_arch = "wasm32"))]
    app.add_system(bevy::input::system::exit_on_esc_system);

    // finally run the application
    app.run();
}

/// Add game entities to the world
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            scale: Vec3::splat(512.0 + 128.0),
            ..Default::default()
        },
        ..Default::default()
    });

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                scale: Vec3::splat(0.5),
                ..Default::default()
            },
            texture: asset_server.load("ball.png"),
            ..Default::default()
        })
        .insert_bundle((Position([0, i32::MAX].into()), Velocity::default()));
}
