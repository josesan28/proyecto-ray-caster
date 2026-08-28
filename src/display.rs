use raylib::prelude::*;

pub const SCREEN_WIDTH: i32 = 800;
pub const SCREEN_HEIGHT: i32 = 600;

pub fn handle_fullscreen_toggle(window: &mut RaylibHandle) {
    if window.is_key_pressed(KeyboardKey::KEY_F11) {
        window.toggle_borderless_windowed();
    }
}

pub fn virtual_camera(window: &RaylibHandle) -> Camera2D {
    let window_width = window.get_screen_width().max(1) as f32;
    let window_height = window.get_screen_height().max(1) as f32;
    let scale = (window_width / SCREEN_WIDTH as f32)
        .min(window_height / SCREEN_HEIGHT as f32)
        .max(0.01);

    Camera2D {
        offset: Vector2::new(
            (window_width - SCREEN_WIDTH as f32 * scale) / 2.0,
            (window_height - SCREEN_HEIGHT as f32 * scale) / 2.0,
        ),
        target: Vector2::zero(),
        rotation: 0.0,
        zoom: scale,
    }
}
