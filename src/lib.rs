//! **P**hysics **2D** **S**andbox.
//!
//! An interactive playground for exploring real-time 2d physics simulation.

#![warn(clippy::all)]

use bevy::{
    core::FixedTimestep,
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    sprite::Mesh2dHandle,
};
use wasm_bindgen::prelude::*;

#[derive(Default, Component)]
struct ShapeShifter;

#[wasm_bindgen]
pub fn game() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "p2ds".to_string(),
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .add_startup_system(setup)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.15))
                .with_system(change_shape),
        )
        .run();
}

/// Add game entities to the world
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let circle_mesh = meshes.add(circle(3));
    let red_material = materials.add(Color::RED.into());

    commands
        .spawn_bundle(ColorMesh2dBundle {
            transform: Transform::identity().with_scale(Vec3::splat(128.0)),
            mesh: circle_mesh.into(),
            material: red_material,
            ..Default::default()
        })
        .insert(ShapeShifter::default());
}

fn change_shape(
    keyboard_input: Res<Input<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut segment_count: Local<u16>,
    mut shapeshifters: Query<&mut Mesh2dHandle, With<ShapeShifter>>,
) {
    if *segment_count == 0 {
        *segment_count = 3;
    }

    if keyboard_input.pressed(KeyCode::Left) {
        *segment_count -= 1;
        if *segment_count < 3 {
            *segment_count = 3;
        }
    } else if keyboard_input.pressed(KeyCode::Right) {
        *segment_count += 1;
    } else {
        return;
    }

    let mesh = shapeshifters.iter().next();

    if let Some(handle) = mesh {
        let new_handle: Mesh2dHandle = meshes.set(handle.0.id, circle(*segment_count)).into();

        for mut mesh in shapeshifters.iter_mut() {
            *mesh = new_handle.clone();
        }
    }
}

fn circle(segments: u16) -> Mesh {
    let vertex_count = segments as usize + 1;

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let mut positions = Vec::with_capacity(vertex_count);
    let mut uvs = Vec::with_capacity(vertex_count);
    let normals = vec![[0.0, 0.0, 1.0]; vertex_count];

    positions.push([0.0, 0.0, 0.0]);
    uvs.push([0.5, 0.5]);

    for i in 0..segments {
        let angle = (i as f32) / (segments as f32) * std::f32::consts::TAU;
        positions.push([angle.cos(), angle.sin(), 0.0]);
        uvs.push([(angle.cos() + 1.0) / 2.0, (angle.sin() + 1.0) / 2.0]);
    }

    let mut indices = Vec::with_capacity(segments as usize * 3);
    for i in 1..=segments {
        indices.push(0);
        indices.push(i);
        // faster modulo
        if i == segments {
            indices.push(1);
        } else {
            indices.push(i + 1);
        }
    }

    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(Indices::U16(indices)));

    mesh
}
