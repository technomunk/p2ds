//! Specialized systems for wasm builds.

use bevy::{prelude::*, window::WindowResized};
use wasm_bindgen::prelude::*;

/// Make sure the window occupies the whole browser window
pub fn fullscreen_window(mut windows: ResMut<Windows>, mut last_size: Local<Vec2>) {
    let browser_window = web_sys::window().expect("No browser window found");
    let width = browser_window.inner_width().unwrap().as_f64().unwrap() as f32;
    let height = browser_window.inner_height().unwrap().as_f64().unwrap() as f32;

    if let Some(window) = windows.get_primary_mut() {
        if width != last_size.x || height != last_size.y {
            *last_size = Vec2::new(width, height);
            window.set_resolution(width, height);
        }
    }
}
