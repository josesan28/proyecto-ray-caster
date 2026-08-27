mod caster;
mod combat;
mod controller;
mod framebuffer;
mod game;
mod hud;
pub mod maze;
mod menu;
mod player;
mod renderer;
mod sprite;
mod textures;

use combat::hits_enemy;
use controller::{GAMEPAD_ID, gamepad_button_pressed, process_events, process_mural_input};
use framebuffer::Framebuffer;
use game::{GameState, SHOT_COOLDOWN, collect_clue, open_doors, update_objective};
use hud::draw_hud;
use maze::load_maze;
use menu::select_level;
use raylib::prelude::*;
use renderer::render_world;
use textures::TextureManager;

const MUSIC_FILE: &str = "assets/audio/music.mp3";
const SPEAR_SOUND_FILE: &str = "assets/audio/spear.mp3";
const ARTIFACT_SOUND_FILE: &str = "assets/audio/artifact.mp3";
const KUKULKAN_SOUND_FILE: &str = "assets/audio/kukulkan.mp3";
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
    while !window.window_should_close() {
        window.enable_cursor();
        let selected_level = select_level(
            &mut window,
            &thread,
            &menu_background,
            &artifact_icon,
            background_music.as_ref(),
        );
        if window.window_should_close() {
            break;
        }

        let floor_color = if selected_level == 0 {
            Color::new(145, 151, 150, 255)
        } else {
            Color::new(82, 126, 72, 255)
        };

        let maze =
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
        let mut state = GameState::new(maze, block_size, floor_color);

        let mut framebuffer =
            Framebuffer::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, Color::BLACK);

        window.disable_cursor();
        let texture_manager = TextureManager::new(&mut window, &thread);

        let mut texture = window
            .load_texture_from_image(&thread, &framebuffer.color_buffer)
            .expect("No se pudo crear la textura del framebuffer");

        while !window.window_should_close() {
            if let Some(track) = background_music.as_ref() {
                track.update_stream();
            }

            let gamepad_connected = window.is_gamepad_available(GAMEPAD_ID);
            let return_to_menu = window.is_key_pressed(KeyboardKey::KEY_R)
                || gamepad_button_pressed(&window, GamepadButton::GAMEPAD_BUTTON_MIDDLE_RIGHT);
            if return_to_menu {
                window.enable_cursor();
                break;
            }

            if !state.level_complete && !state.player_defeated {
                if !state.mural_active {
                    process_events(&window, &mut state.player, &state.maze, state.block_size);
                }
                if !state.enemy_defeated {
                    state.enemy.update(
                        &state.player.pos,
                        &state.maze,
                        state.block_size,
                        window.get_frame_time().min(0.05),
                        state.enemy_speed_multiplier,
                    );
                }
                if let Some(digit) = collect_clue(
                    &mut state.maze,
                    &state.player,
                    state.block_size,
                    &mut state.clues,
                ) {
                    state.last_clue_collected = Some((digit, window.get_time()));
                    if let Some(sound) = artifact_sound.as_ref() {
                        sound.play();
                    }
                }
                let had_artifact = state.has_artifact;
                let has_all_clues = state.has_all_clues();
                state.mural_active = update_objective(
                    &mut state.maze,
                    &state.player,
                    state.block_size,
                    &mut state.has_artifact,
                    has_all_clues,
                );
                if !had_artifact && state.has_artifact {
                    if let Some(sound) = artifact_sound.as_ref() {
                        sound.play();
                    }
                }

                let confirm_code = process_mural_input(&mut window, &mut state, gamepad_connected);
                if confirm_code && state.mural_input.len() == 3 {
                    if state.mural_input == "250" {
                        open_doors(&mut state.maze);
                        state.level_complete = true;
                        state.mural_active = false;
                    } else {
                        state.wrong_code_attempts += 1;
                        state.enemy_speed_multiplier =
                            (1.0 + state.wrong_code_attempts as f32 * 0.35).min(2.4);
                        state.last_wrong_code_time = Some(window.get_time());
                        state.mural_input.clear();
                        state.mural_slot = 0;
                    }
                }
            }
            if !state.mural_active
                && !state.player_defeated
                && !state.level_complete
                && (window.is_key_pressed(KeyboardKey::KEY_M)
                    || gamepad_button_pressed(&window, GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_UP))
            {
                state.mode_3d = !state.mode_3d;
            }

            let enemy_distance = state.enemy_distance();
            if !state.enemy_defeated
                && !state.player_defeated
                && !state.kukulkan_sound_played
                && enemy_distance <= state.block_size as f32 * 5.0
            {
                if let Some(sound) = kukulkan_sound.as_ref() {
                    sound.play();
                }
                state.kukulkan_sound_played = true;
            }

            let shot_time = window.get_time();
            if state.mode_3d
                && !state.mural_active
                && !state.player_defeated
                && !state.level_complete
                && shot_time - state.last_shot_time >= SHOT_COOLDOWN
                && (window.is_key_pressed(KeyboardKey::KEY_SPACE)
                    || gamepad_button_pressed(
                        &window,
                        GamepadButton::GAMEPAD_BUTTON_RIGHT_TRIGGER_2,
                    ))
            {
                state.last_shot_time = shot_time;
                if let Some(sound) = spear_sound.as_ref() {
                    sound.play();
                }
                if !state.enemy_defeated
                    && hits_enemy(
                        &state.player,
                        &state.enemy.pos,
                        &state.maze,
                        state.block_size,
                    )
                {
                    state.enemy_health -= 1;
                    state.last_enemy_hit_time = Some(shot_time);
                    if state.enemy_health <= 0 {
                        state.enemy_health = 0;
                        state.enemy_defeated = true;
                    }
                }
            }

            if !state.level_complete
                && !state.player_defeated
                && !state.enemy_defeated
                && enemy_distance <= state.block_size as f32 * 0.70
            {
                state.player_defeated = true;
                state.mural_active = false;
            }

            render_world(
                &mut framebuffer,
                &state,
                &texture_manager,
                window.get_time(),
            );

            let pixels = framebuffer.color_buffer.get_image_data_u8(false);
            texture
                .update_texture(&pixels)
                .expect("No se pudo actualizar la textura");

            let current_time = window.get_time();
            let shot_elapsed = current_time - state.last_shot_time;
            let shot_progress = if state.mode_3d
                && !state.player_defeated
                && !state.level_complete
                && !state.mural_active
                && (0.0..0.38).contains(&shot_elapsed)
            {
                Some((shot_elapsed / 0.38) as f32)
            } else {
                None
            };
            let mut drawing = window.begin_drawing(&thread);
            drawing.clear_background(Color::BLACK);
            drawing.draw_texture(&texture, 0, 0, Color::WHITE);
            drawing.draw_fps(10, 10);
            draw_hud(
                &mut drawing,
                &state,
                current_time,
                shot_progress,
                gamepad_connected,
                SCREEN_WIDTH,
            );
        }
    }
}