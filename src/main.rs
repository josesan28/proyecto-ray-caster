mod caster;
mod combat;
mod controller;
mod framebuffer;
mod game;
pub mod maze;
mod player;
mod renderer;
mod sprite;
mod textures;

use caster::cast_rays;
use combat::hits_enemy;
use controller::process_events;
use framebuffer::Framebuffer;
use game::{collect_clue, open_doors, update_objective};
use maze::load_maze;
use player::Player;
use raylib::prelude::*;
use renderer::{
    render_3d, render_enemy, render_maze, render_minimap, render_player, render_sprite,
};
use sprite::{Artifact, Clue, Enemy};
use textures::TextureManager;

const MUSIC_FILE: &str = "assets/audio/music.mp3";
const SPEAR_SOUND_FILE: &str = "assets/audio/spear.mp3";
const ARTIFACT_SOUND_FILE: &str = "assets/audio/artifact.mp3";
const KUKULKAN_SOUND_FILE: &str = "assets/audio/kukulkan.mp3";

fn select_level(
    window: &mut RaylibHandle,
    thread: &RaylibThread,
    background: &Texture2D,
    artifact_icon: &Texture2D,
    music: Option<&Music<'_>>,
) -> usize {
    let level_names = ["Nivel 1: Plaza de Zaculeu", "Nivel 2: Patio ceremonial"];
    let mut selected: usize = 0;
    let mut music_enabled = music.is_some();

    while !window.window_should_close() {
        if let Some(track) = music {
            track.update_stream();
        }
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
        if window.is_key_pressed(KeyboardKey::KEY_M) {
            if let Some(track) = music {
                if music_enabled {
                    track.pause_stream();
                } else {
                    track.resume_stream();
                }
                music_enabled = !music_enabled;
            }
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
        let music_text = if music.is_none() {
            "Música no encontrada"
        } else if music_enabled {
            "M: música activada"
        } else {
            "M: música silenciada"
        };
        drawing.draw_text(music_text, 350, 474, 17, Color::new(167, 207, 180, 255));
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

    let audio = RaylibAudio::init_audio_device().ok();
    let mut background_music = audio
        .as_ref()
        .and_then(|device| device.new_music(MUSIC_FILE).ok());
    if let Some(track) = background_music.as_mut() {
        track.set_looping(true);
        track.set_volume(0.3);
        track.play_stream();
    }
    let spear_sound = audio
        .as_ref()
        .and_then(|device| device.new_sound(SPEAR_SOUND_FILE).ok());
    if let Some(sound) = spear_sound.as_ref() {
        sound.set_volume(0.55);
    }
    let artifact_sound = audio
        .as_ref()
        .and_then(|device| device.new_sound(ARTIFACT_SOUND_FILE).ok());
    if let Some(sound) = artifact_sound.as_ref() {
        sound.set_volume(0.45);
    }
    let kukulkan_sound = audio
        .as_ref()
        .and_then(|device| device.new_sound(KUKULKAN_SOUND_FILE).ok());
    if let Some(sound) = kukulkan_sound.as_ref() {
        sound.set_volume(0.6);
    }

    let menu_background = window
        .load_texture(&thread, "assets/textures/zaculeu_menu_background.png")
        .expect("No se pudo cargar el fondo del menu");
    let artifact_icon = window
        .load_texture(&thread, "assets/textures/zaculeu_artifact.png")
        .expect("No se pudo cargar el icono del artefacto");
    let selected_level = select_level(
        &mut window,
        &thread,
        &menu_background,
        &artifact_icon,
        background_music.as_ref(),
    );

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
    let mut enemy = Enemy::from_maze(&maze, block_size);
    let mural_position = Vector2::new(enemy.pos.x, enemy.pos.y);
    let artifact = Artifact::from_maze(&maze, block_size);
    let mut clues = Clue::from_maze(&maze, block_size);
    let mut has_artifact = false;
    let mut enemy_defeated = false;
    let mut kukulkan_sound_played = false;
    let mut last_shot_time = -1.0;
    let mut last_clue_collected: Option<(char, f64)> = None;
    let mut mural_input = String::new();
    let mut wrong_code_attempts = 0;
    let mut enemy_speed_multiplier = 1.0;
    let mut last_wrong_code_time: Option<f64> = None;
    let mut level_complete = false;
    let mut mode_3d = true;

    let mut framebuffer = Framebuffer::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, Color::BLACK);

    window.disable_cursor();
    let texture_manager = TextureManager::new(&mut window, &thread);

    let mut texture = window
        .load_texture_from_image(&thread, &framebuffer.color_buffer)
        .expect("No se pudo crear la textura del framebuffer");

    while !window.window_should_close() {
        if let Some(track) = background_music.as_ref() {
            track.update_stream();
        }
        let mut mural_active = false;
        if !level_complete {
            process_events(&window, &mut player, &maze, block_size);
            if !enemy_defeated {
                enemy.update(
                    &player.pos,
                    &maze,
                    block_size,
                    window.get_frame_time().min(0.05),
                    enemy_speed_multiplier,
                );
            }
            if let Some(digit) = collect_clue(&mut maze, &player, block_size, &mut clues) {
                last_clue_collected = Some((digit, window.get_time()));
                if let Some(sound) = artifact_sound.as_ref() {
                    sound.play();
                }
            }
            let had_artifact = has_artifact;
            let has_all_clues = clues.len() == 3 && clues.iter().all(|clue| clue.collected);
            mural_active = update_objective(
                &mut maze,
                &player,
                block_size,
                &mut has_artifact,
                has_all_clues,
            );
            if !had_artifact && has_artifact {
                if let Some(sound) = artifact_sound.as_ref() {
                    sound.play();
                }
            }

            if mural_active {
                while let Some(character) = window.get_char_pressed() {
                    if character.is_ascii_digit() && mural_input.len() < 3 {
                        mural_input.push(character);
                    }
                }

                if window.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
                    mural_input.pop();
                }

                if (window.is_key_pressed(KeyboardKey::KEY_ENTER)
                    || window.is_key_pressed(KeyboardKey::KEY_KP_ENTER))
                    && mural_input.len() == 3
                {
                    if mural_input == "250" {
                        open_doors(&mut maze);
                        level_complete = true;
                        mural_active = false;
                    } else {
                        wrong_code_attempts += 1;
                        enemy_speed_multiplier = (1.0 + wrong_code_attempts as f32 * 0.35).min(2.4);
                        last_wrong_code_time = Some(window.get_time());
                        mural_input.clear();
                    }
                }
            }
        }
        if !mural_active && window.is_key_pressed(KeyboardKey::KEY_M) {
            mode_3d = !mode_3d;
        }

        let enemy_distance =
            ((player.pos.x - enemy.pos.x).powi(2) + (player.pos.y - enemy.pos.y).powi(2)).sqrt();
        if !enemy_defeated && !kukulkan_sound_played && enemy_distance <= block_size as f32 * 5.0 {
            if let Some(sound) = kukulkan_sound.as_ref() {
                sound.play();
            }
            kukulkan_sound_played = true;
        }

        if mode_3d && !mural_active && window.is_key_pressed(KeyboardKey::KEY_SPACE) {
            last_shot_time = window.get_time();
            if let Some(sound) = spear_sound.as_ref() {
                sound.play();
            }
            if !enemy_defeated && hits_enemy(&player, &enemy.pos, &maze, block_size) {
                enemy_defeated = true;
            }
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

            let mut visible_sprites = Vec::new();
            visible_sprites.push((&mural_position, 'm', 68.0, 0.0));
            if !enemy_defeated {
                let enemy_animation = window.get_time() as f32 * 2.2;
                let enemy_scale = 82.0 + enemy_animation.sin() * 2.0;
                let enemy_height = enemy_animation.sin() * 1.5;
                visible_sprites.push((&enemy.pos, 'e', enemy_scale, enemy_height));
            }
            if !has_artifact {
                let animation_time = window.get_time() as f32;
                let artifact_scale = 55.0 + animation_time.sin() * 4.0;
                let artifact_height = animation_time.sin() * 9.0;
                visible_sprites.push((&artifact.pos, 'a', artifact_scale, artifact_height));
            }
            let clue_animation = window.get_time() as f32 * 1.8;
            for (index, clue) in clues.iter().filter(|clue| !clue.collected).enumerate() {
                let movement = clue_animation + index as f32 * 1.7;
                visible_sprites.push((
                    &clue.pos,
                    clue.digit,
                    44.0 + movement.sin() * 2.0,
                    movement.sin() * 6.0,
                ));
            }

            visible_sprites.sort_by(|left, right| {
                let left_distance =
                    (left.0.x - player.pos.x).powi(2) + (left.0.y - player.pos.y).powi(2);
                let right_distance =
                    (right.0.x - player.pos.x).powi(2) + (right.0.y - player.pos.y).powi(2);
                right_distance.total_cmp(&left_distance)
            });

            for (position, sprite_texture, scale, vertical_offset) in visible_sprites {
                render_sprite(
                    &mut framebuffer,
                    &player,
                    position,
                    sprite_texture,
                    scale,
                    vertical_offset,
                    &texture_manager,
                    &z_buffer,
                );
            }
            render_minimap(
                &mut framebuffer,
                &maze,
                &player,
                (!enemy_defeated).then_some(&enemy.pos),
                block_size,
            );
        } else {
            render_maze(&mut framebuffer, &maze, block_size);
            cast_rays(&mut framebuffer, &maze, &player, block_size);
            if !enemy_defeated {
                render_enemy(&mut framebuffer, &enemy.pos, block_size);
            }
            render_player(&mut framebuffer, &player, block_size);
        }

        let pixels = framebuffer.color_buffer.get_image_data_u8(false);
        texture
            .update_texture(&pixels)
            .expect("No se pudo actualizar la textura");

        let current_time = window.get_time();
        let shot_elapsed = current_time - last_shot_time;
        let shot_progress = if mode_3d && (0.0..0.38).contains(&shot_elapsed) {
            Some((shot_elapsed / 0.38) as f32)
        } else {
            None
        };
        let mut drawing = window.begin_drawing(&thread);
        drawing.clear_background(Color::BLACK);
        drawing.draw_texture(&texture, 0, 0, Color::WHITE);
        drawing.draw_fps(10, 10);

        if let Some(progress) = shot_progress {
            let travel_progress = (progress / 0.86).min(1.0);
            let spear_x = 400.0 + (travel_progress * std::f32::consts::PI).sin() * 3.0;
            let spear_tip_y = 555.0 - 255.0 * travel_progress;
            let spear_base_y = (spear_tip_y + 105.0).min(600.0);

            drawing.draw_line_ex(
                Vector2::new(spear_x, spear_tip_y + 17.0),
                Vector2::new(spear_x, spear_base_y),
                8.0,
                Color::new(91, 55, 32, 255),
            );
            drawing.draw_line_ex(
                Vector2::new(spear_x - 2.0, spear_tip_y + 20.0),
                Vector2::new(spear_x - 2.0, spear_base_y),
                2.0,
                Color::new(190, 128, 65, 255),
            );
            drawing.draw_triangle(
                Vector2::new(spear_x, spear_tip_y),
                Vector2::new(spear_x - 13.0, spear_tip_y + 22.0),
                Vector2::new(spear_x + 13.0, spear_tip_y + 22.0),
                Color::new(221, 192, 117, 255),
            );
            drawing.draw_triangle(
                Vector2::new(spear_x, spear_tip_y + 4.0),
                Vector2::new(spear_x - 7.0, spear_tip_y + 19.0),
                Vector2::new(spear_x + 7.0, spear_tip_y + 19.0),
                Color::new(255, 235, 171, 255),
            );

            let feather_y = (spear_tip_y + 64.0).min(575.0);
            drawing.draw_triangle(
                Vector2::new(spear_x - 3.0, feather_y),
                Vector2::new(spear_x - 22.0, feather_y + 8.0),
                Vector2::new(spear_x - 3.0, feather_y + 16.0),
                Color::new(54, 145, 104, 255),
            );
            drawing.draw_triangle(
                Vector2::new(spear_x + 3.0, feather_y),
                Vector2::new(spear_x + 3.0, feather_y + 16.0),
                Vector2::new(spear_x + 22.0, feather_y + 8.0),
                Color::new(45, 111, 87, 255),
            );

            if progress > 0.86 {
                let impact = (progress - 0.86) / 0.14;
                drawing.draw_circle_lines(
                    400,
                    300,
                    8.0 + impact * 32.0,
                    Color::new(255, 225, 120, ((1.0 - impact) * 255.0) as u8),
                );
            }
        }

        if mode_3d {
            let aim_color = Color::new(255, 232, 158, 235);
            drawing.draw_line_ex(
                Vector2::new(386.0, 300.0),
                Vector2::new(414.0, 300.0),
                3.0,
                aim_color,
            );
            drawing.draw_line_ex(
                Vector2::new(400.0, 286.0),
                Vector2::new(400.0, 314.0),
                3.0,
                aim_color,
            );
            drawing.draw_circle_lines(400, 300, 4.0, Color::new(82, 55, 31, 255));
        }

        let clues_found = clues.iter().filter(|clue| clue.collected).count();
        let has_all_clues = clues.len() == 3 && clues_found == 3;
        let objective = if !has_all_clues && enemy_defeated {
            format!("Kukulkán vencido: encuentra los fragmentos ({clues_found}/3)")
        } else if !has_all_clues {
            format!("Kukulkán te persigue: encuentra los fragmentos ({clues_found}/3)")
        } else if !has_artifact {
            "Código 250 descubierto: busca el artefacto ceremonial".to_string()
        } else {
            "Artefacto y código listos: ve al mural de la salida".to_string()
        };
        drawing.draw_text(&objective, 10, 40, 20, Color::DARKBROWN);

        let discovered_digit = |digit| {
            if clues
                .iter()
                .any(|clue| clue.digit == digit && clue.collected)
            {
                digit
            } else {
                '?'
            }
        };
        let code_text = format!(
            "Código: {} - {} - {}",
            discovered_digit('2'),
            discovered_digit('5'),
            discovered_digit('0')
        );
        drawing.draw_rectangle(10, 528, 280, 60, Color::new(35, 43, 38, 220));
        drawing.draw_text(
            "Año de fundación de Zaculeu",
            20,
            536,
            17,
            Color::new(232, 213, 164, 255),
        );
        drawing.draw_text(&code_text, 20, 558, 22, Color::new(83, 211, 151, 255));

        if wrong_code_attempts > 0 && !enemy_defeated {
            drawing.draw_text(
                &format!("Kukulkán acelerado x{enemy_speed_multiplier:.2}"),
                10,
                500,
                18,
                Color::new(190, 35, 45, 255),
            );
        }

        if !mural_active && let Some((digit, collected_time)) = last_clue_collected {
            if current_time - collected_time < 3.0 {
                const MESSAGE_FONT_SIZE: i32 = 22;
                const MESSAGE_PADDING: i32 = 20;
                const MESSAGE_BOX_Y: i32 = 140;
                const MESSAGE_BOX_HEIGHT: i32 = 56;
                let message = format!("Fragmento encontrado: número maya {digit}");
                let message_width = drawing.measure_text(&message, MESSAGE_FONT_SIZE);
                let box_width = message_width + MESSAGE_PADDING * 2;
                let box_x = (SCREEN_WIDTH - box_width) / 2;

                drawing.draw_rectangle(
                    box_x + 4,
                    MESSAGE_BOX_Y + 4,
                    box_width,
                    MESSAGE_BOX_HEIGHT,
                    Color::new(20, 25, 22, 120),
                );
                drawing.draw_rectangle(
                    box_x,
                    MESSAGE_BOX_Y,
                    box_width,
                    MESSAGE_BOX_HEIGHT,
                    Color::new(35, 43, 38, 235),
                );
                drawing.draw_rectangle_lines(
                    box_x,
                    MESSAGE_BOX_Y,
                    box_width,
                    MESSAGE_BOX_HEIGHT,
                    Color::new(198, 175, 126, 255),
                );
                drawing.draw_text(
                    &message,
                    box_x + MESSAGE_PADDING,
                    MESSAGE_BOX_Y + 17,
                    MESSAGE_FONT_SIZE,
                    Color::new(255, 225, 142, 255),
                );
            }
        }

        if mural_active {
            const PANEL_X: i32 = 120;
            const PANEL_Y: i32 = 165;
            const PANEL_WIDTH: i32 = 560;
            const PANEL_HEIGHT: i32 = 265;

            drawing.draw_rectangle(
                PANEL_X + 6,
                PANEL_Y + 6,
                PANEL_WIDTH,
                PANEL_HEIGHT,
                Color::new(20, 25, 22, 140),
            );
            drawing.draw_rectangle(
                PANEL_X,
                PANEL_Y,
                PANEL_WIDTH,
                PANEL_HEIGHT,
                Color::new(42, 48, 40, 245),
            );
            drawing.draw_rectangle_lines(
                PANEL_X,
                PANEL_Y,
                PANEL_WIDTH,
                PANEL_HEIGHT,
                Color::new(198, 175, 126, 255),
            );

            let title = "Mural ceremonial";
            let title_width = drawing.measure_text(title, 32);
            drawing.draw_text(
                title,
                (SCREEN_WIDTH - title_width) / 2,
                PANEL_Y + 20,
                32,
                Color::new(232, 213, 164, 255),
            );

            let instruction = "Ingresa el año revelado por los fragmentos";
            let instruction_width = drawing.measure_text(instruction, 19);
            drawing.draw_text(
                instruction,
                (SCREEN_WIDTH - instruction_width) / 2,
                PANEL_Y + 64,
                19,
                Color::new(205, 196, 169, 255),
            );

            for index in 0..3 {
                let slot_x = 286 + index as i32 * 76;
                let digit = mural_input.chars().nth(index).unwrap_or('_');
                drawing.draw_rectangle(slot_x, PANEL_Y + 100, 60, 64, Color::new(24, 29, 25, 255));
                drawing.draw_rectangle_lines(
                    slot_x,
                    PANEL_Y + 100,
                    60,
                    64,
                    Color::new(83, 211, 151, 255),
                );
                let digit_text = digit.to_string();
                let digit_width = drawing.measure_text(&digit_text, 38);
                drawing.draw_text(
                    &digit_text,
                    slot_x + (60 - digit_width) / 2,
                    PANEL_Y + 112,
                    38,
                    Color::new(255, 225, 142, 255),
                );
            }

            let controls = "Números: escribir | Backspace: borrar | Enter: confirmar";
            let controls_width = drawing.measure_text(controls, 16);
            drawing.draw_text(
                controls,
                (SCREEN_WIDTH - controls_width) / 2,
                PANEL_Y + 180,
                16,
                Color::new(205, 196, 169, 255),
            );

            let incorrect_code =
                last_wrong_code_time.is_some_and(|attempt_time| current_time - attempt_time < 3.0);
            let feedback = if incorrect_code {
                format!("Código incorrecto: Kukulkán acelera x{enemy_speed_multiplier:.2}")
            } else {
                "Escribe los tres dígitos y confirma el código".to_string()
            };
            let feedback_width = drawing.measure_text(&feedback, 19);
            let feedback_color = if incorrect_code {
                Color::new(255, 110, 95, 255)
            } else {
                Color::new(83, 211, 151, 255)
            };
            drawing.draw_text(
                &feedback,
                (SCREEN_WIDTH - feedback_width) / 2,
                PANEL_Y + 218,
                19,
                feedback_color,
            );
        }

        if level_complete {
            drawing.draw_rectangle(170, 245, 460, 110, Color::new(245, 231, 200, 235));
            drawing.draw_text("¡Código correcto!", 265, 275, 32, Color::DARKGREEN);
            drawing.draw_text(
                "La puerta se abrió. Escapaste de Kukulkán",
                196,
                320,
                19,
                Color::DARKBROWN,
            );
        }
    }
}
