mod caster;
mod controller;
mod framebuffer;
mod game;
pub mod maze;
mod player;
mod renderer;
mod sprite;
mod textures;

use caster::cast_rays;
use controller::process_events;
use framebuffer::Framebuffer;
use game::update_objective;
use maze::load_maze;
use player::Player;
use raylib::prelude::*;
use renderer::{render_3d, render_maze, render_player, render_sprite};
use sprite::{Artifact, Enemy};
use textures::TextureManager;

fn main() {
    const SCREEN_WIDTH: i32 = 800;
    const SCREEN_HEIGHT: i32 = 600;

    let mut maze =
        load_maze("assets/levels/zaculeu_1.txt").expect("No se pudo cargar el nivel de Zaculeu");
    println!(
        "Nivel cargado: {} columnas x {} filas",
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
    let enemy = Enemy::from_maze(&maze, block_size);
    let artifact = Artifact::from_maze(&maze, block_size);
    let mut has_artifact = false;
    let mut level_complete = false;
    let mut mode_3d = true;

    let mut framebuffer = Framebuffer::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, Color::BLACK);

    let (mut window, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("El Secreto de Zaculeu")
        .build();

    window.disable_cursor();
    window.set_target_fps(60);
    let texture_manager = TextureManager::new(&mut window, &thread);

    let mut texture = window
        .load_texture_from_image(&thread, &framebuffer.color_buffer)
        .expect("No se pudo crear la textura del framebuffer");

    while !window.window_should_close() {
        if !level_complete {
            process_events(&window, &mut player, &maze, block_size);
            level_complete = update_objective(&mut maze, &player, block_size, &mut has_artifact);
        }
        if window.is_key_pressed(KeyboardKey::KEY_M) {
            mode_3d = !mode_3d;
        }

        framebuffer.clear();
        if mode_3d {
            let z_buffer = render_3d(
                &mut framebuffer,
                &maze,
                &player,
                block_size,
                &texture_manager,
            );
            render_sprite(
                &mut framebuffer,
                &player,
                &enemy.pos,
                'e',
                70.0,
                &texture_manager,
                &z_buffer,
            );
            if !has_artifact {
                render_sprite(
                    &mut framebuffer,
                    &player,
                    &artifact.pos,
                    'a',
                    55.0,
                    &texture_manager,
                    &z_buffer,
                );
            }
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
        drawing.draw_fps(10, 10);

        let objective = if has_artifact {
            "Artefacto encontrado: ve a la salida"
        } else {
            "Busca el artefacto ceremonial"
        };
        drawing.draw_text(objective, 10, 40, 20, Color::DARKBROWN);

        if level_complete {
            drawing.draw_rectangle(170, 245, 460, 110, Color::new(245, 231, 200, 235));
            drawing.draw_text("¡Nivel completado!", 265, 275, 32, Color::DARKGREEN);
            drawing.draw_text(
                "Encontraste el secreto de Zaculeu",
                220,
                320,
                20,
                Color::DARKBROWN,
            );
        }
    }
}
