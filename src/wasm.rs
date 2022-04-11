//! Specialized systems for wasm builds.

use bevy::{prelude::*, window::WindowResized};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn resize_canvas(width: f32, height: f32);
}

/// Make sure the window occupies the whole browser window
pub fn fullscreen_window(
    mut windows: ResMut<Windows>,
    mut window_resized_events: EventWriter<WindowResized>,
    mut last_size: Local<Vec2>,
) {
    let browser_window = web_sys::window().expect("No browser window found");
    let width = browser_window.inner_width().unwrap().as_f64().unwrap() as f32;
    let height = browser_window.inner_height().unwrap().as_f64().unwrap() as f32;

    if let Some(window) = windows.get_primary_mut() {
        if width != last_size.x || height != last_size.y {
            *last_size = Vec2::new(width, height);

            let physical_width = width * window.scale_factor() as f32;
            let physical_height = height * window.scale_factor() as f32;

            window.update_actual_size_from_backend(physical_width as u32, physical_height as u32);
            window_resized_events.send(WindowResized {
                id: window.id(),
                width,
                height,
            })
        }
    }
}
