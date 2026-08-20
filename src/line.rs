use crate::framebuffer::Framebuffer;
use raylib::prelude::Vector2;

pub fn line(framebuffer: &mut Framebuffer, start: Vector2, end: Vector2) {
    let x0 = start.x.round() as i32;
    let y0 = start.y.round() as i32;
    let x1 = end.x.round() as i32;
    let y1 = end.y.round() as i32;

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    let mut x = x0;
    let mut y = y0;

    loop {
        framebuffer.set_pixel(x as u32, y as u32);
        if x == x1 && y == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}