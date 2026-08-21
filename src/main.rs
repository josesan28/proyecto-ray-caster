mod caster;
mod controller;
mod framebuffer;
pub mod maze;
mod player;
mod renderer;

use caster::cast_rays;
use controller::process_events;
use framebuffer::Framebuffer;
use maze::generate_maze;
use player::Player;
use raylib::prelude::*;
use renderer::{render_3d, render_maze, render_player};

fn main() {
    const SCREEN_WIDTH: i32 = 800;
    const SCREEN_HEIGHT: i32 = 600;

    println!("Holaaaa");

    let maze = generate_maze(10, 10);
    println!(
        "Laberinto generado: {} columnas x {} filas",
        maze[0].len(),
        maze.len()
    );

    for row in &maze {
        let line: String = row.iter().collect();
        println!("{line}");
    }

    let block_size = (SCREEN_WIDTH as usize / maze[0].len())
        .min(SCREEN_HEIGHT as usize / maze.len())
        .max(1);
    let mut player = Player::from_maze(&maze, block_size);
    let mut mode_3d = false;

    let mut framebuffer = Framebuffer::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, Color::BLACK);

    let (mut window, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Laberinto")
        .build();

    window.set_target_fps(60);

    let mut texture = window
        .load_texture_from_image(&thread, &framebuffer.color_buffer)
        .expect("No se pudo crear la textura del framebuffer");

    while !window.window_should_close() {
        process_events(&window, &mut player, &maze, block_size);
        if window.is_key_pressed(KeyboardKey::KEY_M) {
            mode_3d = !mode_3d;
        }

        framebuffer.clear();
        if mode_3d {
            render_3d(&mut framebuffer, &maze, &player, block_size);
        } else {
            render_maze(&mut framebuffer, &maze, block_size);
            cast_rays(&mut framebuffer, &maze, &player, block_size);
            render_player(&mut framebuffer, &player, block_size);
        }

        let pixels = framebuffer.color_buffer.get_image_data_u8(false);
        texture
            .update_texture(&pixels)
            .expect("No se pudo actualizar la textura");

        let mut drawing = window.begin_drawing(&thread);
        drawing.clear_background(Color::BLACK);
        drawing.draw_texture(&texture, 0, 0, Color::WHITE);
    }
}
