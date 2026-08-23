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
use renderer::{render_3d, render_maze, render_minimap, render_player, render_sprite};
use sprite::{Artifact, Enemy};
use textures::TextureManager;

fn select_level(
    window: &mut RaylibHandle,
    thread: &RaylibThread,
    background: &Texture2D,
    artifact_icon: &Texture2D,
) -> usize {
    let level_names = ["Nivel 1: Plaza de Zaculeu", "Nivel 2: Patio ceremonial"];
    let mut selected: usize = 0;

    while !window.window_should_close() {
        if window.is_key_pressed(KeyboardKey::KEY_UP) || window.is_key_pressed(KeyboardKey::KEY_W) {
            selected = selected.saturating_sub(1);
        }
        if window.is_key_pressed(KeyboardKey::KEY_DOWN) || window.is_key_pressed(KeyboardKey::KEY_S)
        {
            selected = (selected + 1).min(level_names.len() - 1);
        }
        if window.is_key_pressed(KeyboardKey::KEY_ENTER) {
            return selected;
        }

        let mut drawing = window.begin_drawing(thread);
        drawing.clear_background(Color::BLACK);
        drawing.draw_texture_pro(
            background,
            Rectangle::new(0.0, 0.0, background.width as f32, background.height as f32),
            Rectangle::new(0.0, 0.0, 800.0, 600.0),
            Vector2::zero(),
            0.0,
            Color::WHITE,
        );
        drawing.draw_rectangle(0, 0, 800, 600, Color::new(16, 28, 23, 65));
        drawing.draw_rectangle(195, 205, 550, 295, Color::new(24, 35, 31, 225));
        drawing.draw_rectangle_lines(195, 205, 550, 295, Color::new(202, 165, 72, 255));

        drawing.draw_text(
            "EL SECRETO DE ZACULEU",
            177,
            62,
            36,
            Color::new(42, 30, 18, 255),
        );
        drawing.draw_text(
            "EL SECRETO DE ZACULEU",
            175,
            60,
            36,
            Color::new(245, 231, 200, 255),
        );
        drawing.draw_text(
            "HUEHUETENANGO, GUATEMALA",
            268,
            108,
            18,
            Color::new(225, 197, 112, 255),
        );
        drawing.draw_text(
            "Explora las ruinas y encuentra el artefacto ceremonial",
            184,
            145,
            20,
            Color::new(245, 231, 200, 255),
        );

        drawing.draw_texture_pro(
            artifact_icon,
            Rectangle::new(
                0.0,
                0.0,
                artifact_icon.width as f32,
                artifact_icon.height as f32,
            ),
            Rectangle::new(42.0, 265.0, 130.0, 130.0),
            Vector2::zero(),
            0.0,
            Color::WHITE,
        );

        for (index, name) in level_names.iter().enumerate() {
            let item_y = 270 + index as i32 * 55;
            let color = if index == selected {
                Color::new(245, 207, 90, 255)
            } else {
                Color::new(225, 218, 190, 255)
            };

            if index == selected {
                drawing.draw_rectangle(230, item_y - 8, 480, 42, Color::new(41, 81, 68, 235));
                drawing.draw_rectangle_lines(
                    230,
                    item_y - 8,
                    480,
                    42,
                    Color::new(94, 182, 139, 255),
                );
            }

            let prefix = if index == selected { "> " } else { "  " };
            drawing.draw_text(&format!("{prefix}{name}"), 255, item_y, 24, color);
        }

        drawing.draw_text(
            "W/S o flechas: elegir",
            340,
            410,
            18,
            Color::new(211, 226, 190, 255),
        );
        drawing.draw_text(
            "Enter: comenzar",
            365,
            444,
            18,
            Color::new(245, 207, 90, 255),
        );
    }

    0
}

fn main() {
    const SCREEN_WIDTH: i32 = 800;
    const SCREEN_HEIGHT: i32 = 600;
    const LEVEL_FILES: [&str; 2] = ["assets/levels/zaculeu_1.txt", "assets/levels/zaculeu_2.txt"];

    let (mut window, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("El Secreto de Zaculeu")
        .build();
    window.set_target_fps(60);

    let menu_background = window
        .load_texture(&thread, "assets/textures/zaculeu_menu_background.png")
        .expect("No se pudo cargar el fondo del menu");
    let artifact_icon = window
        .load_texture(&thread, "assets/textures/zaculeu_artifact.png")
        .expect("No se pudo cargar el icono del artefacto");
    let selected_level = select_level(&mut window, &thread, &menu_background, &artifact_icon);

    let mut maze =
        load_maze(LEVEL_FILES[selected_level]).expect("No se pudo cargar el nivel de Zaculeu");
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

    window.disable_cursor();
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
            render_minimap(&mut framebuffer, &maze, &player, block_size);
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
